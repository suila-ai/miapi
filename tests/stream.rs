#![cfg(feature = "streaming")]
// accept_hdr_async のコールバックが返す Err の型は async-tungstenite が決めている。
#![allow(clippy::result_large_err)]
//! WebSocket のモックサーバを相手に、ストリーミングの接続・振り分け・送信・再接続を確かめる。
//! 生成したバインディングは使わず、`Channel` を手で実装する。

use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use async_tungstenite::WebSocketStream;
use async_tungstenite::tokio::{TokioAdapter, accept_hdr_async};
use async_tungstenite::tungstenite::Message;
use async_tungstenite::tungstenite::handshake::server::{Request, Response};
use futures_util::StreamExt;
use miapi::Error;
use miapi::stream::{Channel, Connection, State, Stream};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::timeout;

/// このテストは生成されたバインディングを使わないので、バージョンセットのマーカーを自前で用意する。
struct NoVersion;
type Client = miapi::client::Client<NoVersion>;

#[derive(Serialize)]
struct Params {
    a: i64,
}

impl Channel for Params {
    const NAME: &'static str = "x";
    type Event = Event;
    type Message = Msg;
}

#[derive(Debug, PartialEq, Deserialize)]
#[serde(tag = "type", content = "body")]
enum Event {
    #[serde(rename = "note")]
    Note { id: String },
    #[serde(rename = "ping")]
    Ping,
}

#[derive(Serialize)]
#[serde(tag = "type", content = "body")]
enum Msg {
    #[serde(rename = "read")]
    Read { id: String },
}

/// サーバ側から見た 1 本の接続。
struct Session {
    uri: String,
    ws: WebSocketStream<TokioAdapter<TcpStream>>,
}

impl Session {
    async fn recv_json(&mut self) -> Value {
        match timeout(Duration::from_secs(5), self.ws.next()).await {
            Ok(Some(Ok(Message::Text(text)))) => serde_json::from_str(text.as_str()).unwrap(),
            other => panic!("テキストのメッセージのはずが {other:?} だった"),
        }
    }

    async fn send_json(&mut self, value: Value) {
        self.ws
            .send(Message::Text(value.to_string().into()))
            .await
            .unwrap();
    }

    async fn ack(&mut self, id: &str) {
        self.send_json(json!({"type": "connected", "body": {"id": id}}))
            .await;
    }
}

/// WebSocket のモックサーバ。受け付けた接続を `sessions` で渡す。
/// `reject` を立てている間は、TCP の接続をハンドシェイクせずに切る。
/// `stall` を立てている間は、TCP の接続を受けたまま応答せず、受けた接続を `stalled` で渡す。
struct MockServer {
    origin: String,
    sessions: mpsc::UnboundedReceiver<Session>,
    reject: Arc<AtomicBool>,
    stall: Arc<AtomicBool>,
    stalled: mpsc::UnboundedReceiver<TcpStream>,
}

impl MockServer {
    async fn start() -> Self {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let origin = format!("http://{}", listener.local_addr().unwrap());
        let (sessions, rx) = mpsc::unbounded_channel();
        let (stalled, stalled_rx) = mpsc::unbounded_channel();
        let reject = Arc::new(AtomicBool::new(false));
        let rejecting = Arc::clone(&reject);
        let stall = Arc::new(AtomicBool::new(false));
        let stalling = Arc::clone(&stall);
        tokio::spawn(async move {
            loop {
                let (stream, _) = listener.accept().await.unwrap();
                if rejecting.load(Ordering::SeqCst) {
                    continue;
                }
                if stalling.load(Ordering::SeqCst) {
                    let _ = stalled.send(stream);
                    continue;
                }
                let sessions = sessions.clone();
                tokio::spawn(async move {
                    let mut uri = String::new();
                    let ws = accept_hdr_async(stream, |request: &Request, response: Response| {
                        uri = request.uri().to_string();
                        Ok(response)
                    })
                    .await
                    .unwrap();
                    let _ = sessions.send(Session { uri, ws });
                });
            }
        });
        MockServer {
            origin,
            sessions: rx,
            reject,
            stall,
            stalled: stalled_rx,
        }
    }

    async fn session(&mut self) -> Session {
        timeout(Duration::from_secs(5), self.sessions.recv())
            .await
            .expect("5 秒以内に接続が来るはず")
            .unwrap()
    }
}

/// サーバを起動し、トークン付きでつないだストリームと、サーバ側の接続を返す。
async fn connected() -> (Stream<NoVersion>, Session, MockServer) {
    let mut server = MockServer::start().await;
    let client = Client::new(&server.origin, Some("tok")).unwrap();
    let stream = client.streaming().await.unwrap();
    let session = server.session().await;
    (stream, session, server)
}

/// チャンネルにつなぐ。サーバ側は `connect` を受け取って確かめ、`connected` を返す。
async fn open_channel(
    stream: &Stream<NoVersion>,
    session: &mut Session,
    params: Params,
    expected_id: &str,
) -> Connection<Params> {
    let expected = connect_json(expected_id, params.a);
    let (connection, ()) = tokio::join!(stream.connect(params), async {
        assert_eq!(session.recv_json().await, expected);
        session.ack(expected_id).await;
    });
    connection.unwrap()
}

fn connect_json(id: &str, a: i64) -> Value {
    json!({"type": "connect", "body": {"channel": "x", "id": id, "params": {"a": a}, "pong": true}})
}

async fn wait_for_state(
    stream: &Stream<NoVersion>,
    predicate: impl FnMut(&State) -> bool,
) -> State {
    let mut state = stream.state();
    timeout(Duration::from_secs(5), state.wait_for(predicate))
        .await
        .expect("5 秒以内に状態が変わるはず")
        .unwrap()
        .clone()
}

#[tokio::test]
async fn handshake_uses_streaming_path_under_origin_and_token_query() {
    let mut server = MockServer::start().await;
    let client = Client::new(&format!("{}/sub/", server.origin), Some("tok")).unwrap();
    let _stream = client.streaming().await.unwrap();
    assert_eq!(server.session().await.uri, "/sub/streaming?i=tok");
    let client = Client::new(&server.origin, None).unwrap();
    let _stream = client.streaming().await.unwrap();
    assert_eq!(server.session().await.uri, "/streaming");
}

#[tokio::test]
async fn handshake_failure_is_streaming_error() {
    let server = wiremock::MockServer::start().await;
    wiremock::Mock::given(wiremock::matchers::path("/streaming"))
        .respond_with(wiremock::ResponseTemplate::new(401))
        .mount(&server)
        .await;
    let client = Client::new(&server.uri(), Some("bad")).unwrap();
    match client.streaming().await {
        Err(Error::Streaming(source)) => {
            let chain =
                std::iter::successors(Some(&*source as &dyn std::error::Error), |e| e.source())
                    .map(ToString::to_string)
                    .collect::<Vec<_>>()
                    .join(": ");
            assert!(chain.contains("401"), "{chain}");
        }
        other => panic!("想定外の結果: {other:?}"),
    }
}

#[tokio::test]
async fn handshake_error_does_not_contain_token() {
    let server = MockServer::start().await;
    server.reject.store(true, Ordering::SeqCst);
    let client = Client::new(&server.origin, Some("secret-token")).unwrap();
    let error = client.streaming().await.unwrap_err();
    assert!(matches!(error, Error::Streaming(_)), "{error:?}");
    // トークンは URL のクエリにあるので、エラーに URL を残さない。
    assert!(!format!("{error:?}").contains("secret-token"), "{error:?}");
}

#[tokio::test]
async fn connect_waits_for_ack_then_delivers_events_by_id() {
    let (stream, mut session, _server) = connected().await;
    let mut first = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    let mut second = open_channel(&stream, &mut session, Params { a: 2 }, "2").await;
    session
        .send_json(
            json!({"type": "channel", "body": {"id": "2", "type": "note", "body": {"id": "n2"}}}),
        )
        .await;
    session
        .send_json(json!({"type": "channel", "body": {"id": "1", "type": "ping", "body": null}}))
        .await;
    session
        .send_json(
            json!({"type": "channel", "body": {"id": "1", "type": "note", "body": {"id": 5}}}),
        )
        .await;
    session
        .send_json(
            json!({"type": "channel", "body": {"id": "1", "type": "note", "body": {"id": "n1"}}}),
        )
        .await;
    // 知らない id と、JSON でないテキストは無視される。
    session
        .send_json(json!({"type": "channel", "body": {"id": "9", "type": "note", "body": {}}}))
        .await;
    session.ws.send(Message::Text("ping".into())).await.unwrap();
    assert_eq!(
        second.recv().await.unwrap().unwrap(),
        Event::Note { id: "n2".into() }
    );
    assert_eq!(first.recv().await.unwrap().unwrap(), Event::Ping);
    match first.recv().await.unwrap() {
        Err(Error::DecodeEvent {
            channel,
            r#type,
            body,
            ..
        }) => {
            assert_eq!(channel, "x");
            assert_eq!(r#type, "note");
            assert_eq!(body, r#"{"id":5}"#);
        }
        other => panic!("想定外の結果: {other:?}"),
    }
    assert_eq!(
        first.recv().await.unwrap().unwrap(),
        Event::Note { id: "n1".into() }
    );
    assert_eq!(first.id(), "1");
    assert!(matches!(*stream.state().borrow(), State::Connected));
}

#[tokio::test]
async fn dropping_connection_sends_disconnect() {
    let (stream, mut session, _server) = connected().await;
    let connection = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    drop(connection);
    assert_eq!(
        session.recv_json().await,
        json!({"type": "disconnect", "body": {"id": "1"}})
    );
}

#[tokio::test]
async fn messages_to_channel_and_stream() {
    let (stream, mut session, _server) = connected().await;
    let connection = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    connection.send(Msg::Read { id: "m".into() }).unwrap();
    stream.subscribe_note("n").unwrap();
    stream.unsubscribe_note("n").unwrap();
    stream.send_raw("readNotification", Value::Null).unwrap();
    assert_eq!(
        session.recv_json().await,
        json!({"type": "ch", "body": {"id": "1", "type": "read", "body": {"id": "m"}}})
    );
    assert_eq!(
        session.recv_json().await,
        json!({"type": "subNote", "body": {"id": "n"}})
    );
    assert_eq!(
        session.recv_json().await,
        json!({"type": "unsubNote", "body": {"id": "n"}})
    );
    assert_eq!(
        session.recv_json().await,
        json!({"type": "readNotification", "body": null})
    );
}

#[tokio::test]
async fn broadcast_events_reach_every_receiver() {
    let (stream, mut session, _server) = connected().await;
    let mut first = stream.broadcast_receiver::<Value>();
    let mut second = stream.broadcast_receiver::<Value>();
    let event = json!({"type": "emojiAdded", "body": {"emoji": {"name": "e"}}});
    session.send_json(event.clone()).await;
    // `connected` と、チャンネルのイベントは、全体の受け口には届かない。
    session
        .send_json(json!({"type": "connected", "body": {"id": "9"}}))
        .await;
    session
        .send_json(json!({"type": "channel", "body": {"id": "1", "type": "note", "body": {}}}))
        .await;
    session
        .send_json(
            json!({"type": "noteUpdated", "body": {"id": "n", "type": "deleted", "body": {}}}),
        )
        .await;
    assert_eq!(first.recv().await.unwrap().unwrap(), event);
    assert_eq!(second.recv().await.unwrap().unwrap(), event);
    assert_eq!(first.recv().await.unwrap().unwrap()["type"], "noteUpdated");
    assert_eq!(second.recv().await.unwrap().unwrap()["type"], "noteUpdated");
}

#[tokio::test]
async fn connect_without_ack_is_channel_unavailable() {
    let (stream, mut session, _server) = connected().await;
    let (result, ()) = tokio::join!(stream.connect(Params { a: 1 }), async {
        assert_eq!(session.recv_json().await["type"], "connect");
    });
    match result {
        Err(Error::ChannelUnavailable { channel }) => assert_eq!(channel, "x"),
        other => panic!("想定外の結果: {other:?}"),
    }
    // 登録を消し、disconnect を送っている。
    assert_eq!(
        session.recv_json().await,
        json!({"type": "disconnect", "body": {"id": "1"}})
    );
}

#[tokio::test]
async fn close_ends_receivers_and_rejects_sends() {
    let (stream, mut session, _server) = connected().await;
    let mut connection = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    let mut events = stream.broadcast_receiver::<Value>();
    stream.close();
    assert!(connection.recv().await.is_none());
    assert!(events.recv().await.is_none());
    assert!(matches!(
        stream.send_raw("x", Value::Null),
        Err(Error::StreamClosed)
    ));
    assert!(matches!(
        connection.send(Msg::Read { id: "m".into() }),
        Err(Error::StreamClosed)
    ));
    assert!(matches!(
        stream.connect(Params { a: 2 }).await,
        Err(Error::StreamClosed)
    ));
    wait_for_state(&stream, |s| matches!(s, State::Closed)).await;
    // 閉じ終えた後に作った受け口も None を返す。
    assert!(stream.broadcast_receiver::<Value>().recv().await.is_none());
    // サーバ側には close が届く。
    assert!(matches!(
        session.ws.next().await,
        Some(Ok(Message::Close(_)))
    ));
}

#[tokio::test]
async fn reconnects_at_once_and_registers_channels_and_notes_again() {
    let (stream, mut session, mut server) = connected().await;
    let mut first = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    let _second = open_channel(&stream, &mut session, Params { a: 2 }, "2").await;
    stream.subscribe_note("n").unwrap();
    stream.subscribe_note("n").unwrap();
    stream.unsubscribe_note("n").unwrap();
    stream.subscribe_note("m").unwrap();
    stream.unsubscribe_note("m").unwrap();
    for _ in 0..5 {
        session.recv_json().await;
    }
    session.ws.close(None).await.unwrap();
    // 最初のつなぎ直しは待たない。
    let mut session = timeout(Duration::from_millis(500), server.sessions.recv())
        .await
        .expect("すぐにつなぎ直すはず")
        .unwrap();
    assert_eq!(session.uri, "/streaming?i=tok");
    assert_eq!(session.recv_json().await, connect_json("1", 1));
    assert_eq!(session.recv_json().await, connect_json("2", 2));
    assert_eq!(
        session.recv_json().await,
        json!({"type": "subNote", "body": {"id": "n"}})
    );
    wait_for_state(&stream, |s| matches!(s, State::Connected)).await;
    session
        .send_json(json!({"type": "channel", "body": {"id": "1", "type": "ping", "body": null}}))
        .await;
    assert_eq!(first.recv().await.unwrap().unwrap(), Event::Ping);
    // つなぎ直した後に送ったものも届く。
    first.send(Msg::Read { id: "m".into() }).unwrap();
    assert_eq!(session.recv_json().await["type"], "ch");
}

#[tokio::test]
async fn reconnect_backs_off_while_handshakes_fail_and_connect_waits_for_it() {
    let (stream, mut session, mut server) = connected().await;
    let _first = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    server.reject.store(true, Ordering::SeqCst);
    session.ws.close(None).await.unwrap();
    let state = wait_for_state(
        &stream,
        |s| matches!(s, State::Reconnecting { attempts, .. } if *attempts >= 1),
    )
    .await;
    let State::Reconnecting { last_error, .. } = state else {
        panic!("想定外の状態: {state:?}");
    };
    assert!(matches!(*last_error, Error::Streaming(_)), "{last_error:?}");
    // 再接続中に始めた connect は、つなぎ直してから connect を送り、そこから connected を待つ。
    let connecting = tokio::spawn({
        let stream = stream.clone();
        async move { stream.connect(Params { a: 2 }).await }
    });
    server.reject.store(false, Ordering::SeqCst);
    let mut session = server.session().await;
    assert_eq!(session.recv_json().await, connect_json("1", 1));
    assert_eq!(session.recv_json().await, connect_json("2", 2));
    session.ack("2").await;
    let second = connecting.await.unwrap().unwrap();
    assert_eq!(second.id(), "2");
    wait_for_state(&stream, |s| matches!(s, State::Connected)).await;
    // 送り直しの後に、キューに残っていた connect を重ねて送ることはない。
    second.send(Msg::Read { id: "m".into() }).unwrap();
    assert_eq!(session.recv_json().await["type"], "ch");
}

#[tokio::test]
async fn note_counts_stay_in_sync_when_changed_while_reconnecting() {
    let (stream, mut session, mut server) = connected().await;
    stream.subscribe_note("n").unwrap();
    stream.subscribe_note("n").unwrap();
    stream.subscribe_note("m").unwrap();
    for _ in 0..3 {
        session.recv_json().await;
    }
    server.reject.store(true, Ordering::SeqCst);
    session.ws.close(None).await.unwrap();
    wait_for_state(
        &stream,
        |s| matches!(s, State::Reconnecting { attempts, .. } if *attempts >= 1),
    )
    .await;
    // つなぎ直している間に、n を 1 回やめ、m をもう 1 回購読する。クライアントの回数は n が 1、m が 2 になる。
    stream.unsubscribe_note("n").unwrap();
    stream.subscribe_note("m").unwrap();
    server.reject.store(false, Ordering::SeqCst);
    let mut session = server.session().await;
    // サーバは購読を回数で数える。つなぎ直した後に届く subNote と unsubNote（送り直しの 3 件と、つなぎ直している間に
    // 呼んだ 2 件）を足し引きした回数が、クライアントの回数と一致するはず。
    let mut counts = HashMap::new();
    for _ in 0..5 {
        let message = session.recv_json().await;
        let delta = match message["type"].as_str() {
            Some("subNote") => 1,
            Some("unsubNote") => -1,
            _ => panic!("subNote か unsubNote のはずが {message} だった"),
        };
        *counts
            .entry(message["body"]["id"].as_str().unwrap().to_owned())
            .or_insert(0) += delta;
    }
    assert_eq!(
        counts,
        HashMap::from([("n".to_owned(), 1), ("m".to_owned(), 2)])
    );
}

#[tokio::test]
async fn close_stops_reconnecting() {
    let (stream, mut session, server) = connected().await;
    let mut connection = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    server.reject.store(true, Ordering::SeqCst);
    session.ws.close(None).await.unwrap();
    wait_for_state(&stream, |s| matches!(s, State::Reconnecting { .. })).await;
    stream.close();
    wait_for_state(&stream, |s| matches!(s, State::Closed)).await;
    assert!(connection.recv().await.is_none());
}

#[tokio::test]
async fn close_interrupts_a_stalled_handshake() {
    let (stream, mut session, mut server) = connected().await;
    server.stall.store(true, Ordering::SeqCst);
    session.ws.close(None).await.unwrap();
    // つなぎ直しの TCP の接続をサーバが受けたまま応答しないうちに閉じる。ハンドシェイクは HANDSHAKE_TIMEOUT（10 秒）まで
    // 終わらないので、close() がハンドシェイクを打ち切らなければ、5 秒のうちに Closed にならない。
    let _held = timeout(Duration::from_secs(5), server.stalled.recv())
        .await
        .expect("5 秒以内につなぎ直しが来るはず")
        .unwrap();
    stream.close();
    wait_for_state(&stream, |s| matches!(s, State::Closed)).await;
}

#[tokio::test]
async fn dropping_all_handles_closes_the_stream() {
    let (stream, mut session, _server) = connected().await;
    let state = stream.state();
    drop(stream);
    // 状態の受け口だけが残っていても、閉じたことが見える。
    assert!(matches!(*state.borrow(), State::Closed));
    assert!(
        session
            .ws
            .next()
            .await
            .is_none_or(|m| matches!(m, Ok(Message::Close(_)) | Err(_)))
    );
}

#[test]
fn streaming_futures_are_send() {
    fn assert_send<T: Send>(_: T) {}
    let client = Client::new("https://example.com", None).unwrap();
    assert_send(client.streaming());
}
