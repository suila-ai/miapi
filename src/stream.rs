//! ストリーミング API（WebSocket の `/streaming`）の接続。
//!
//! チャンネルに依らない部分だけをここに置く。チャンネルごとの型（`Params`・`Event`・`Message`）と、
//! それらでつなぐビルダーは、バージョンセットの `streaming` モジュールに生成してある。
//!
//! プロトコルは misskey-js と同じで、やり取りはすべて `{"type": …, "body": …}` の JSON である。
//! チャンネルへは `connect`（`pong: true` を付け、サーバの `connected` を待つ）・`disconnect`・`ch` を送り、
//! サーバからは `channel`（`body.id` で振り分ける）・`connected`・それ以外
//! （`noteUpdated` や `emojiAdded` などのストリーム全体のイベント）を受け取る。
//!
//! WebSocket が切れたら、つなぎ直して、つないであったチャンネルと購読中のノートをサーバに登録し直す。
//! 切れている間に送ったものは、つなぎ直した後に順に送る。

use std::collections::HashMap;
use std::fmt;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, PoisonError};
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use reqwest_websocket::{CloseCode, Message, Upgrade, WebSocket};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Value, json};
use tokio::sync::{Notify, mpsc, oneshot, watch};

use crate::{Error, endpoint};

/// チャンネルにつないでから、サーバが受け付けた応答（`connected`）を待つ上限。
/// 典型的には、往復の時間とチャンネルの初期化（DB の問い合わせ）で 1 秒に収まる。
pub const CONNECT_ACK_TIMEOUT: Duration = Duration::from_secs(5);
/// WebSocket のハンドシェイクを待つ上限。
pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
/// 再接続の間隔の上限。最初は待たずにつなぎ直し、失敗するたびに 1 秒から 2 倍ずつ延ばす。
pub const RECONNECT_MAX_DELAY: Duration = Duration::from_secs(10);

/// 1 つのチャンネルの定義。
///
/// 生成される `Params` がこのトレイトを実装し、[`Stream::connect`] がそれでつなぐ。
/// REST の `Request: Endpoint` と対になる。
pub trait Channel: Serialize + Send + 'static {
    /// `connect` で送るチャンネル名。`localTimeline` など。
    const NAME: &'static str;
    /// サーバから受け取るイベント。`{type, body}` の形からデコードする。
    type Event: DeserializeOwned + Send + 'static;
    /// チャンネルへ送るメッセージ。`{type, body}` の形にシリアライズする。
    type Message: Serialize;
}

/// `receives` の無いチャンネルの [`Channel::Message`]。値が無いので、何も送れない。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoMessage {}

impl Serialize for NoMessage {
    fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        match *self {}
    }
}

/// `{type, body}` の形。チャンネルのイベントとメッセージ、ストリーム全体のイベントに共通の外形で、
/// 生成されるそれらの enum は、この形を介してシリアライズ・デシリアライズする。
#[derive(Serialize, Deserialize)]
pub(crate) struct Envelope {
    pub r#type: String,
    #[serde(default)]
    pub body: Value,
}

/// ストリームの状態。[`Stream::state`] で見る。
#[derive(Debug, Clone)]
pub enum State {
    /// WebSocket がつながっている。
    Connected,
    /// 切れた後、つなぎ直している。`attempts` は切れてから、つなぎ直しに失敗した回数（ハンドシェイクの失敗と、
    /// その後の connect などの送り直しの失敗を数える）、`last_error` は切れた理由か、最後の失敗のエラー。
    Reconnecting {
        attempts: u32,
        last_error: Arc<Error>,
    },
    /// [`Stream::close`] で閉じたか、`Stream`・`Connection`・`Receiver` を全部 Drop した。
    Closed,
}

/// 1 本の WebSocket。[`Client::streaming`](crate::client::Client::streaming) で開く。
///
/// `V` はバージョンセットのマーカーで、チャンネルのビルダーは、バージョンセットの `streaming` モジュールにある
/// 別名の `Stream` の側にある。clone はこの WebSocket を共有する。
///
/// 切れたらつなぎ直す。その間に取りこぼしたタイムラインなどは、[`state`](Self::state) が
/// [`State::Connected`] に戻ったのを見て、REST で取り直す。
// #[must_use] は付けない。接続そのものの型で、開いてすぐ捨てる `client.streaming().await?;` も、
// トークンと upgrade が通るかを確かめる使い方になる。
// 持っている間だけ意味がある Connection と Receiver には付ける。
pub struct Stream<V> {
    shared: Arc<Shared>,
    _version: PhantomData<fn() -> V>,
}

/// チャンネルへの 1 つの接続。[`Stream::connect`] で作る。
///
/// Drop すると、サーバへ `disconnect` を送る。
#[must_use = "dropping a Connection disconnects from the channel"]
pub struct Connection<C: Channel> {
    id: String,
    shared: Arc<Shared>,
    events: mpsc::UnboundedReceiver<Value>,
    _channel: PhantomData<fn() -> C>,
}

/// ストリーム全体のイベント（`noteUpdated` や `emojiAdded` など、チャンネルに属さないもの）の受け口。
#[must_use = "broadcast events reach a Receiver only while it is kept"]
pub struct Receiver<T> {
    /// 受け口が残っている間は WebSocket を保つために持つ。
    _shared: Arc<Shared>,
    events: mpsc::UnboundedReceiver<Value>,
    _event: PhantomData<fn() -> T>,
}

/// 利用側の `Stream`・`Connection`・`Receiver` が共有するもの。全部の持ち手が無くなったらタスクを止める。
struct Shared {
    outgoing: mpsc::UnboundedSender<Outgoing>,
    inner: Arc<Inner>,
    task: tokio::task::JoinHandle<()>,
}

/// WebSocket を持つタスクと、利用側が共有するもの。
struct Inner {
    registry: Mutex<Registry>,
    state: watch::Sender<State>,
    /// [`Stream::close`] がつなぎ直しの途中（待ち・ハンドシェイク・送り直し）を打ち切るために使う。
    closing: Notify,
}

/// つないであるチャンネル、`connected` を待っている接続、購読中のノート。
#[derive(Default)]
struct Registry {
    next_id: u64,
    channels: HashMap<String, ChannelEntry>,
    broadcast: Vec<mpsc::UnboundedSender<Value>>,
    /// ノートの id と、購読した回数。回数は `send_note` が送るときに数える。サーバも回数を数えるので、
    /// つなぎ直したときは回数ぶん送る。
    subscribed_notes: HashMap<String, usize>,
    closed: bool,
}

struct ChannelEntry {
    name: &'static str,
    params: Value,
    events: mpsc::UnboundedSender<Value>,
    /// いまの WebSocket で `connect` を送ったかどうか。つなぎ直したら全部送り直す。
    sent: bool,
    /// `connect` をソケットへ書けたことを知らせる。[`Stream::connect`] が `connected` を待ち始める起点にする。
    written: Option<oneshot::Sender<()>>,
    /// サーバが `connected` を返したことを [`Stream::connect`] に知らせる。
    ack: Option<oneshot::Sender<()>>,
}

enum Outgoing {
    Text(String),
    /// その id のチャンネルの `connect`。本文は送るときに登録から作る。
    Connect(String),
    /// ノートの `subNote`（`subscribe` が true のとき）か `unsubNote`。購読の回数は送るときに数える。
    Note {
        id: String,
        subscribe: bool,
    },
    Close,
}

impl<V> Stream<V> {
    pub(crate) async fn open(http: reqwest::Client, url: reqwest::Url) -> Result<Self, Error> {
        let ws = handshake(&http, url.clone()).await?;
        let (outgoing, rx) = mpsc::unbounded_channel();
        let inner = Arc::new(Inner {
            registry: Mutex::new(Registry::default()),
            state: watch::Sender::new(State::Connected),
            closing: Notify::new(),
        });
        let task = tokio::spawn(run(ws, http, url, rx, Arc::clone(&inner)));
        Ok(Stream {
            shared: Arc::new(Shared {
                outgoing,
                inner,
                task,
            }),
            _version: PhantomData,
        })
    }

    /// チャンネルにつなぐ。生成されたビルダーを await したときに呼ばれる。
    ///
    /// `connect` を送り、サーバが受け付けた応答（`connected`）を [`CONNECT_ACK_TIMEOUT`] まで待つ。
    /// 来なければ [`Error::ChannelUnavailable`]。再接続中なら、つなぎ直してから `connect` を送り、
    /// そこから待つ。
    pub async fn connect<C: Channel>(&self, params: C) -> Result<Connection<C>, Error> {
        let params = endpoint::to_value(&params)?;
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let (ack_tx, ack_rx) = oneshot::channel();
        let (written_tx, written_rx) = oneshot::channel();
        let id = {
            let mut registry = self.shared.inner.lock();
            if registry.closed {
                return Err(Error::StreamClosed);
            }
            registry.next_id += 1;
            let id = registry.next_id.to_string();
            registry.channels.insert(
                id.clone(),
                ChannelEntry {
                    name: C::NAME,
                    params,
                    events: events_tx,
                    sent: false,
                    written: Some(written_tx),
                    ack: Some(ack_tx),
                },
            );
            id
        };
        // 先に Connection にしておくと、失敗したときも Drop が登録を消して disconnect を送る。
        let connection = Connection {
            id: id.clone(),
            shared: Arc::clone(&self.shared),
            events: events_rx,
            _channel: PhantomData,
        };
        self.shared.send(Outgoing::Connect(id))?;
        if written_rx.await.is_err() {
            return Err(Error::StreamClosed);
        }
        match tokio::time::timeout(CONNECT_ACK_TIMEOUT, ack_rx).await {
            Ok(Ok(())) => Ok(connection),
            Ok(Err(_)) => Err(Error::StreamClosed),
            Err(_) => Err(Error::ChannelUnavailable {
                channel: C::NAME.to_owned(),
            }),
        }
    }

    /// ストリーム全体のイベントの受け口を作る。生成された `broadcast()` が、バージョンセットの型を付けて包む。
    /// 受け口はいくつでも作れ、イベントは全部の受け口に届く。
    pub fn broadcast_receiver<T: DeserializeOwned>(&self) -> Receiver<T> {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut registry = self.shared.inner.lock();
        // 閉じた後なら登録しない。tx が捨てられるので、recv はすぐ None を返す。
        if !registry.closed {
            registry.broadcast.retain(|events| !events.is_closed());
            registry.broadcast.push(tx);
        }
        Receiver {
            _shared: Arc::clone(&self.shared),
            events: rx,
            _event: PhantomData,
        }
    }

    /// ノートの更新（リアクション・削除・投票）を購読する。以後、そのノートの `noteUpdated` が
    /// [`broadcast_receiver`](Self::broadcast_receiver) に届く。
    pub fn subscribe_note(&self, note_id: &str) -> Result<(), Error> {
        self.shared.send(Outgoing::Note {
            id: note_id.to_owned(),
            subscribe: true,
        })
    }

    /// [`subscribe_note`](Self::subscribe_note) をやめる。
    pub fn unsubscribe_note(&self, note_id: &str) -> Result<(), Error> {
        self.shared.send(Outgoing::Note {
            id: note_id.to_owned(),
            subscribe: false,
        })
    }

    /// `{"type": type, "body": body}` をそのまま送る。生成されていないメッセージを送るための抜け道。
    pub fn send_raw(&self, r#type: &str, body: Value) -> Result<(), Error> {
        self.shared
            .send(Outgoing::Text(envelope_text(r#type, body)))
    }

    /// 状態を見る。`changed().await` で変化を待てる。
    pub fn state(&self) -> watch::Receiver<State> {
        self.shared.inner.state.subscribe()
    }

    /// WebSocket を閉じ、再接続もやめる。以後、チャンネルと受け口の `recv` は、届いていたイベントを返し終えると
    /// `None` を返し、送信は [`Error::StreamClosed`] になる。
    /// `Stream`・`Connection`・`Receiver` を全部 Drop したときも同じように閉じる。
    pub fn close(&self) {
        self.shared.inner.lock().close();
        let _ = self.shared.outgoing.send(Outgoing::Close);
        self.shared.inner.closing.notify_one();
    }
}

impl<V> Clone for Stream<V> {
    fn clone(&self) -> Self {
        Stream {
            shared: Arc::clone(&self.shared),
            _version: PhantomData,
        }
    }
}

impl<V> fmt::Debug for Stream<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Stream")
            .field("state", &*self.shared.inner.state.borrow())
            .finish_non_exhaustive()
    }
}

impl<C: Channel> Connection<C> {
    /// この接続の id。`connect` と `ch` で送り、`channel` の振り分けに使う。
    pub fn id(&self) -> &str {
        &self.id
    }

    /// 次のイベントを受け取る。デコードできないイベントは `Err` を 1 件返し、次のイベントへ進む。
    /// ストリームが閉じたら、届いていたイベントを返し終えてから `None`。
    pub async fn recv(&mut self) -> Option<Result<C::Event, Error>> {
        let value = self.events.recv().await?;
        Some(decode(C::NAME, &value))
    }

    /// チャンネルへメッセージを送る。
    pub fn send(&self, message: C::Message) -> Result<(), Error> {
        let Value::Object(mut object) = endpoint::to_value(&message)? else {
            return Err(Error::InvalidRequest(
                "channel message must be a JSON object".into(),
            ));
        };
        object.insert("id".to_owned(), Value::String(self.id.clone()));
        self.shared
            .send(Outgoing::Text(envelope_text("ch", Value::Object(object))))
    }
}

impl<C: Channel> Drop for Connection<C> {
    fn drop(&mut self) {
        self.shared.inner.lock().channels.remove(&self.id);
        let _ = self.shared.send(Outgoing::Text(envelope_text(
            "disconnect",
            json!({"id": self.id}),
        )));
    }
}

impl<C: Channel> fmt::Debug for Connection<C> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Connection")
            .field("channel", &C::NAME)
            .field("id", &self.id)
            .finish_non_exhaustive()
    }
}

impl<T: DeserializeOwned> Receiver<T> {
    /// 次のイベントを受け取る。デコードできないイベントは `Err` を 1 件返し、次のイベントへ進む。
    /// ストリームが閉じたら、届いていたイベントを返し終えてから `None`。
    pub async fn recv(&mut self) -> Option<Result<T, Error>> {
        let value = self.events.recv().await?;
        Some(decode("broadcast", &value))
    }
}

impl<T> fmt::Debug for Receiver<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Receiver").finish_non_exhaustive()
    }
}

impl Shared {
    fn send(&self, outgoing: Outgoing) -> Result<(), Error> {
        if self.inner.lock().closed {
            return Err(Error::StreamClosed);
        }
        self.outgoing
            .send(outgoing)
            .map_err(|_| Error::StreamClosed)
    }
}

impl Drop for Shared {
    fn drop(&mut self) {
        self.task.abort();
        // 止めたタスクは最後の Closed を流さないので、ここで流す。state() の受け口だけが残っていても、閉じたと分かる。
        self.inner.state.send_replace(State::Closed);
    }
}

impl Inner {
    fn lock(&self) -> MutexGuard<'_, Registry> {
        self.registry.lock().unwrap_or_else(PoisonError::into_inner)
    }

    /// 状態を変える。ただし Closed は終わりの状態なので上書きしない。[`Shared`] の Drop は abort の直後に Closed を流すが、
    /// abort は次の await まで効かないので、別のスレッドで走っているタスクが、その後に状態を変えようとすることがある。
    fn set_state(&self, state: State) {
        self.state.send_if_modified(|current| {
            if matches!(current, State::Closed) {
                return false;
            }
            *current = state;
            true
        });
    }
}

impl Registry {
    /// 受け口を全部捨てて、`recv` が `None` を返すようにする。
    fn close(&mut self) {
        self.closed = true;
        self.channels.clear();
        self.broadcast.clear();
    }
}

async fn handshake(http: &reqwest::Client, url: reqwest::Url) -> Result<WebSocket, Error> {
    let upgrade = async {
        let response = http.get(url).upgrade().send().await?;
        response.into_websocket().await
    };
    match tokio::time::timeout(HANDSHAKE_TIMEOUT, upgrade).await {
        Ok(Ok(ws)) => Ok(ws),
        // reqwest のエラーは表示に URL を含む。クエリの `i` はアクセストークンなので、URL を外して包む。
        Ok(Err(reqwest_websocket::Error::Reqwest(error))) => {
            Err(Error::Streaming(error.without_url().into()))
        }
        Ok(Err(error)) => Err(Error::Streaming(error.into())),
        Err(_) => Err(Error::Streaming("handshake timed out".into())),
    }
}

/// WebSocket を持つタスク。切れたらつなぎ直し、[`Stream::close`] か持ち手が無くなるまで続ける。
async fn run(
    mut ws: WebSocket,
    http: reqwest::Client,
    url: reqwest::Url,
    mut outgoing: mpsc::UnboundedReceiver<Outgoing>,
    inner: Arc<Inner>,
) {
    while let Err(lost) = session(&mut ws, &mut outgoing, &inner).await {
        // つなぎ直しはサーバが止まっている間ずっと続くので、切れたソケットは先に閉じる。
        drop(ws);
        let Some(reopened) = reconnect(&http, &url, &inner, lost).await else {
            break;
        };
        ws = reopened;
        // つなぎ直しが終わるのと同時に close されていたら、Connected を流さずに閉じる。
        if inner.lock().closed {
            break;
        }
        inner.set_state(State::Connected);
    }
    inner.lock().close();
    inner.state.send_replace(State::Closed);
}

/// 1 本の WebSocket が生きている間、受信を振り分け、送信キューをソケットへ書く。
/// [`Stream::close`] されたか持ち手が無くなったら `Ok`、サーバに切られたか送受信に失敗したら `Err`。
async fn session(
    ws: &mut WebSocket,
    outgoing: &mut mpsc::UnboundedReceiver<Outgoing>,
    inner: &Inner,
) -> Result<(), Error> {
    loop {
        tokio::select! {
            incoming = ws.next() => match incoming {
                Some(Ok(Message::Text(text))) => dispatch(inner, &text),
                // ping には tungstenite が pong を返す。
                Some(Ok(Message::Binary(_) | Message::Ping(_) | Message::Pong(_))) => {}
                Some(Ok(Message::Close { code, reason })) => {
                    return Err(Error::Streaming(
                        format!("connection closed by server: {code:?} {reason}").into(),
                    ));
                }
                Some(Err(error)) => return Err(Error::Streaming(error.into())),
                None => return Err(Error::Streaming("connection closed".into())),
            },
            request = outgoing.recv() => match request {
                Some(Outgoing::Text(text)) => write(ws, text).await?,
                Some(Outgoing::Connect(id)) => send_connect(ws, inner, &id).await?,
                Some(Outgoing::Note { id, subscribe }) => send_note(ws, inner, &id, subscribe).await?,
                Some(Outgoing::Close) | None => {
                    // 閉じるフレームを送り、ソケットは run が終わるときに捨てる。
                    let _ = ws.send(Message::Close { code: CloseCode::Normal, reason: String::new() }).await;
                    return Ok(());
                }
            },
        }
    }
}

/// つなぎ直し、つないであったチャンネルと購読中のノートを登録し直す。閉じられたら `None`。
async fn reconnect(
    http: &reqwest::Client,
    url: &reqwest::Url,
    inner: &Inner,
    lost: Error,
) -> Option<WebSocket> {
    let mut attempts = 0;
    let mut delay = Duration::ZERO;
    let mut last_error = Arc::new(lost);
    loop {
        if inner.lock().closed {
            return None;
        }
        inner.set_state(State::Reconnecting {
            attempts,
            last_error: Arc::clone(&last_error),
        });
        let attempt = async {
            tokio::time::sleep(delay).await;
            let mut ws = handshake(http, url.clone()).await?;
            resend(&mut ws, inner).await?;
            Ok::<_, Error>(ws)
        };
        let error = tokio::select! {
            result = attempt => match result {
                Ok(ws) => return Some(ws),
                Err(error) => error,
            },
            () = inner.closing.notified() => return None,
        };
        attempts += 1;
        last_error = Arc::new(error);
        delay = if delay.is_zero() {
            Duration::from_secs(1)
        } else {
            (delay * 2).min(RECONNECT_MAX_DELAY)
        };
    }
}

/// つなぎ直した WebSocket へ、全チャンネルの `connect` と、購読中のノートの `subNote` を送る。
async fn resend(ws: &mut WebSocket, inner: &Inner) -> Result<(), Error> {
    let (mut ids, notes) = {
        let mut registry = inner.lock();
        for entry in registry.channels.values_mut() {
            entry.sent = false;
        }
        let ids: Vec<String> = registry.channels.keys().cloned().collect();
        let notes: Vec<String> = registry
            .subscribed_notes
            .iter()
            .flat_map(|(id, count)| std::iter::repeat_n(id.clone(), *count))
            .collect();
        (ids, notes)
    };
    // id は連番なので、つないだ順に送り直す。
    ids.sort_by_key(|id| id.parse::<u64>().unwrap_or(u64::MAX));
    for id in ids {
        send_connect(ws, inner, &id).await?;
    }
    for note in notes {
        write(ws, envelope_text("subNote", json!({"id": note}))).await?;
    }
    Ok(())
}

/// その id のチャンネルの `connect` を、いまの WebSocket でまだ送っていなければ送る。
async fn send_connect(ws: &mut WebSocket, inner: &Inner, id: &str) -> Result<(), Error> {
    let text = {
        let mut registry = inner.lock();
        let Some(entry) = registry.channels.get_mut(id) else {
            return Ok(());
        };
        if entry.sent {
            return Ok(());
        }
        entry.sent = true;
        let body = json!({"channel": entry.name, "id": id, "params": entry.params, "pong": true});
        envelope_text("connect", body)
    };
    write(ws, text).await?;
    // 書けなかったときは written を残し、つなぎ直した後の送り直しで知らせる。
    let written = inner
        .lock()
        .channels
        .get_mut(id)
        .and_then(|entry| entry.written.take());
    if let Some(written) = written {
        let _ = written.send(());
    }
    Ok(())
}

/// ノートの `subNote` か `unsubNote` を送る。購読の回数は、利用側が呼んだときでなく、ここでソケットへ書く前に数える。
/// 呼んだときに数えると、つなぎ直している間に呼んだ分が、送り直し（回数ぶんの `subNote`）とキューの送信の両方に入り、
/// サーバの回数とずれる（`unsubNote` なら、購読が残っているのにサーバでは 0 になり、`noteUpdated` が届かなくなる）。
async fn send_note(
    ws: &mut WebSocket,
    inner: &Inner,
    id: &str,
    subscribe: bool,
) -> Result<(), Error> {
    let r#type = {
        let mut registry = inner.lock();
        if subscribe {
            *registry.subscribed_notes.entry(id.to_owned()).or_insert(0) += 1;
            "subNote"
        } else {
            if let Some(count) = registry.subscribed_notes.get_mut(id) {
                *count -= 1;
                if *count == 0 {
                    registry.subscribed_notes.remove(id);
                }
            }
            "unsubNote"
        }
    };
    write(ws, envelope_text(r#type, json!({"id": id}))).await
}

async fn write(ws: &mut WebSocket, text: String) -> Result<(), Error> {
    ws.send(Message::Text(text))
        .await
        .map_err(|error| Error::Streaming(error.into()))
}

/// サーバから受け取った 1 つのメッセージを、チャンネルの受け口・`connected` の待ち・全体の受け口へ振り分ける。
fn dispatch(inner: &Inner, text: &str) {
    // JSON でないテキストは無視する。サーバも、JSON でない受信は無視する。
    let Ok(Envelope { r#type, body }) = serde_json::from_str::<Envelope>(text) else {
        return;
    };
    let mut registry = inner.lock();
    match r#type.as_str() {
        "channel" => {
            let id = body.get("id").and_then(Value::as_str);
            if let Some(entry) = id.and_then(|id| registry.channels.get(id)) {
                let _ = entry.events.send(body);
            }
        }
        "connected" => {
            let id = body.get("id").and_then(Value::as_str);
            let ack = id
                .and_then(|id| registry.channels.get_mut(id))
                .and_then(|entry| entry.ack.take());
            if let Some(ack) = ack {
                let _ = ack.send(());
            }
        }
        _ => {
            registry.broadcast.retain(|events| !events.is_closed());
            if let Some((last, rest)) = registry.broadcast.split_last() {
                let value = envelope(r#type, body);
                for events in rest {
                    let _ = events.send(value.clone());
                }
                let _ = last.send(value);
            }
        }
    }
}

/// `{type, body}` の JSON を、その型にデコードする。
fn decode<T: DeserializeOwned>(channel: &str, value: &Value) -> Result<T, Error> {
    T::deserialize(value).map_err(|source| Error::DecodeEvent {
        channel: channel.to_owned(),
        r#type: value
            .get("type")
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned(),
        source,
        body: value
            .get("body")
            .map_or_else(|| "null".to_owned(), Value::to_string),
    })
}

/// `{"type": type, "body": body}`。`json!` は本文を参照から作り直すので、動かして組む。
fn envelope(r#type: String, body: Value) -> Value {
    Value::Object(serde_json::Map::from_iter([
        ("type".to_owned(), Value::String(r#type)),
        ("body".to_owned(), body),
    ]))
}

fn envelope_text(r#type: &str, body: Value) -> String {
    envelope(r#type.to_owned(), body).to_string()
}
