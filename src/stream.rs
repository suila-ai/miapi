//! ストリーミング API（WebSocket の `/streaming`）の接続。
//!
//! チャンネルに依らない部分だけをここに置く。チャンネルごとの型（`Params`・`Event`・`Message`）と、
//! それらでつなぐビルダーは、バージョンセットの `streaming` モジュールに生成してある。
//!
//! プロトコルは misskey-js と同じで、やり取りはすべて `{"type": …, "body": …}` の JSON である。
//! チャンネルへは `connect`（`pong: true` を付け、サーバの `connected` を待つ）・`disconnect`・`ch` を送り、
//! サーバからは `channel`（`body.id` で振り分ける）・`connected`・それ以外
//! （`noteUpdated` や `emojiAdded` などのストリーム全体のイベント）を受け取る。

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
use tokio::sync::{mpsc, oneshot};

use crate::{Error, endpoint};

/// チャンネルにつないでから、サーバが受け付けた応答（`connected`）を待つ上限。
/// 典型的には、往復の時間とチャンネルの初期化（DB の問い合わせ）で 1 秒に収まる。
pub const CONNECT_ACK_TIMEOUT: Duration = Duration::from_secs(5);
/// WebSocket のハンドシェイクを待つ上限。
pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

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

/// 1 本の WebSocket。[`Client::streaming`](crate::client::Client::streaming) で開く。
///
/// `V` はバージョンセットのマーカーで、チャンネルのビルダーは、バージョンセットの `streaming` モジュールにある
/// 別名の `Stream` の側にある。clone はこの WebSocket を共有する。
pub struct Stream<V> {
    shared: Arc<Shared>,
    _version: PhantomData<fn() -> V>,
}

/// チャンネルへの 1 つの接続。[`Stream::connect`] で作る。
///
/// Drop すると、サーバへ `disconnect` を送る。
pub struct Connection<C: Channel> {
    id: String,
    shared: Arc<Shared>,
    events: mpsc::UnboundedReceiver<Value>,
    _channel: PhantomData<fn() -> C>,
}

/// ストリーム全体のイベント（`noteUpdated` や `emojiAdded` など、チャンネルに属さないもの）の受け口。
pub struct Receiver<T> {
    /// 受け口が残っている間は WebSocket を保つために持つ。
    _shared: Arc<Shared>,
    events: mpsc::UnboundedReceiver<Value>,
    _event: PhantomData<fn() -> T>,
}

/// WebSocket を持つタスクと、利用側の `Stream`・`Connection`・`Receiver` が共有するもの。
/// 全部の持ち手が無くなったらタスクを止める。
struct Shared {
    outgoing: mpsc::UnboundedSender<Outgoing>,
    registry: Arc<Mutex<Registry>>,
    task: tokio::task::JoinHandle<()>,
}

/// チャンネルの id ごとの受け口と、`connected` を待っている接続。
#[derive(Default)]
struct Registry {
    next_id: u64,
    channels: HashMap<String, mpsc::UnboundedSender<Value>>,
    pending: HashMap<String, oneshot::Sender<()>>,
    broadcast: Vec<mpsc::UnboundedSender<Value>>,
    closed: bool,
}

enum Outgoing {
    Text {
        text: String,
        /// ソケットへ書けたことを知らせる。[`Stream::connect`] が `connected` を待ち始める起点にする。
        written: Option<oneshot::Sender<()>>,
    },
    Close,
}

impl<V> Stream<V> {
    pub(crate) async fn open(http: reqwest::Client, url: reqwest::Url) -> Result<Self, Error> {
        let ws = handshake(&http, url).await?;
        let (outgoing, rx) = mpsc::unbounded_channel();
        let registry = Arc::new(Mutex::new(Registry::default()));
        let task = tokio::spawn(run(ws, rx, Arc::clone(&registry)));
        Ok(Stream {
            shared: Arc::new(Shared {
                outgoing,
                registry,
                task,
            }),
            _version: PhantomData,
        })
    }

    /// チャンネルにつなぐ。生成されたビルダーを await したときに呼ばれる。
    ///
    /// `connect` を送り、サーバが受け付けた応答（`connected`）を [`CONNECT_ACK_TIMEOUT`] まで待つ。
    /// 来なければ [`Error::ChannelUnavailable`]。
    pub async fn connect<C: Channel>(&self, params: C) -> Result<Connection<C>, Error> {
        let params = endpoint::to_value(&params)?;
        let (events_tx, events_rx) = mpsc::unbounded_channel();
        let (ack_tx, ack_rx) = oneshot::channel();
        let (written_tx, written_rx) = oneshot::channel();
        let id = {
            let mut registry = self.shared.lock();
            if registry.closed {
                return Err(Error::StreamClosed);
            }
            registry.next_id += 1;
            let id = registry.next_id.to_string();
            registry.channels.insert(id.clone(), events_tx);
            registry.pending.insert(id.clone(), ack_tx);
            id
        };
        // 先に Connection にしておくと、失敗したときも Drop が登録を消して disconnect を送る。
        let connection = Connection {
            id: id.clone(),
            shared: Arc::clone(&self.shared),
            events: events_rx,
            _channel: PhantomData,
        };
        let body = json!({"channel": C::NAME, "id": id, "params": params, "pong": true});
        self.shared.send(Outgoing::Text {
            text: envelope_text("connect", body),
            written: Some(written_tx),
        })?;
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
        let mut registry = self.shared.lock();
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
        self.send_raw("subNote", json!({"id": note_id}))
    }

    /// [`subscribe_note`](Self::subscribe_note) をやめる。
    pub fn unsubscribe_note(&self, note_id: &str) -> Result<(), Error> {
        self.send_raw("unsubNote", json!({"id": note_id}))
    }

    /// `{"type": type, "body": body}` をそのまま送る。生成されていないメッセージを送るための抜け道。
    pub fn send_raw(&self, r#type: &str, body: Value) -> Result<(), Error> {
        self.shared.send(Outgoing::Text {
            text: envelope_text(r#type, body),
            written: None,
        })
    }

    /// WebSocket を閉じる。以後、チャンネルと受け口の `recv` は、届いていたイベントを返し終えると `None` を返し、
    /// 送信は [`Error::StreamClosed`] になる。
    /// `Stream`・`Connection`・`Receiver` を全部 Drop したときも同じように閉じる。
    pub fn close(&self) {
        self.shared.lock().close();
        let _ = self.shared.outgoing.send(Outgoing::Close);
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
        f.debug_struct("Stream").finish_non_exhaustive()
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
        self.shared.send(Outgoing::Text {
            text: envelope_text("ch", Value::Object(object)),
            written: None,
        })
    }
}

impl<C: Channel> Drop for Connection<C> {
    fn drop(&mut self) {
        {
            let mut registry = self.shared.lock();
            registry.channels.remove(&self.id);
            registry.pending.remove(&self.id);
        }
        let _ = self.shared.send(Outgoing::Text {
            text: envelope_text("disconnect", json!({"id": self.id})),
            written: None,
        });
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
    fn lock(&self) -> MutexGuard<'_, Registry> {
        self.registry.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn send(&self, outgoing: Outgoing) -> Result<(), Error> {
        if self.lock().closed {
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
    }
}

impl Registry {
    /// 受け口を全部捨てて、`recv` が `None` を返すようにする。
    fn close(&mut self) {
        self.closed = true;
        self.channels.clear();
        self.pending.clear();
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

/// WebSocket を持つタスク。受信したメッセージを振り分け、送信キューをソケットへ書く。
async fn run(
    mut ws: WebSocket,
    mut outgoing: mpsc::UnboundedReceiver<Outgoing>,
    registry: Arc<Mutex<Registry>>,
) {
    loop {
        tokio::select! {
            incoming = ws.next() => match incoming {
                Some(Ok(Message::Text(text))) => dispatch(&registry, &text),
                // ping には tungstenite が pong を返す。
                Some(Ok(Message::Binary(_) | Message::Ping(_) | Message::Pong(_))) => {}
                Some(Ok(Message::Close { .. })) | Some(Err(_)) | None => break,
            },
            request = outgoing.recv() => match request {
                Some(Outgoing::Text { text, written }) => {
                    if ws.send(Message::Text(text)).await.is_err() {
                        break;
                    }
                    if let Some(written) = written {
                        let _ = written.send(());
                    }
                }
                Some(Outgoing::Close) | None => {
                    let _ = ws.close(CloseCode::Normal, None).await;
                    break;
                }
            },
        }
    }
    registry
        .lock()
        .unwrap_or_else(PoisonError::into_inner)
        .close();
}

/// サーバから受け取った 1 つのメッセージを、チャンネルの受け口・`connected` の待ち・全体の受け口へ振り分ける。
fn dispatch(registry: &Mutex<Registry>, text: &str) {
    // JSON でないテキストは無視する。サーバも、JSON でない受信は無視する。
    let Ok(Envelope { r#type, body }) = serde_json::from_str::<Envelope>(text) else {
        return;
    };
    let mut registry = registry.lock().unwrap_or_else(PoisonError::into_inner);
    match r#type.as_str() {
        "channel" => {
            let id = body.get("id").and_then(Value::as_str);
            if let Some(events) = id.and_then(|id| registry.channels.get(id)) {
                let _ = events.send(body);
            }
        }
        "connected" => {
            let id = body.get("id").and_then(Value::as_str);
            if let Some(ack) = id.and_then(|id| registry.pending.remove(id)) {
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
