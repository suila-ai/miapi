#![cfg(feature = "streaming")]
// accept_hdr_async のコールバックが返す Err の型は async-tungstenite が決めている。
#![allow(clippy::result_large_err)]
//! WebSocket のモックサーバを相手に、ストリーミングの接続・振り分け・送信を確かめる。
//! 生成したバインディングは使わず、`Channel` を手で実装する。

use async_tungstenite::WebSocketStream;
use async_tungstenite::tokio::{TokioAdapter, accept_hdr_async};
use async_tungstenite::tungstenite::Message;
use async_tungstenite::tungstenite::handshake::server::{Request, Response};
use futures_util::StreamExt;
use miapi::Error;
use miapi::stream::{Channel, Connection, Stream};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;

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
        match self.ws.next().await {
            Some(Ok(Message::Text(text))) => serde_json::from_str(text.as_str()).unwrap(),
            other => panic!("テキストのメッセージのはずが {other:?} だった"),
        }
    }

    async fn send_json(&mut self, value: Value) {
        self.ws
            .send(Message::Text(value.to_string().into()))
            .await
            .unwrap();
    }
}

/// モックサーバを起動し、origin と、受け付けた接続を受け取る口を返す。
async fn server() -> (String, mpsc::UnboundedReceiver<Session>) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let (sessions, rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
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
    (origin, rx)
}

/// サーバを起動し、トークン付きでつないだストリームと、サーバ側の接続を返す。
async fn connected() -> (Stream<NoVersion>, Session, mpsc::UnboundedReceiver<Session>) {
    let (origin, mut sessions) = server().await;
    let client = Client::new(&origin, Some("tok")).unwrap();
    let stream = client.streaming().await.unwrap();
    let session = sessions.recv().await.unwrap();
    (stream, session, sessions)
}

/// チャンネルにつなぐ。サーバ側は `connect` を受け取って確かめ、`connected` を返す。
async fn open_channel(
    stream: &Stream<NoVersion>,
    session: &mut Session,
    params: Params,
    expected_id: &str,
) -> Connection<Params> {
    let expected_params = json!({"a": params.a});
    let (connection, ()) = tokio::join!(stream.connect(params), async {
        let message = session.recv_json().await;
        assert_eq!(
            message,
            json!({"type": "connect", "body": {"channel": "x", "id": expected_id, "params": expected_params, "pong": true}})
        );
        session
            .send_json(json!({"type": "connected", "body": {"id": expected_id}}))
            .await;
    });
    connection.unwrap()
}

#[tokio::test]
async fn handshake_uses_streaming_path_under_origin_and_token_query() {
    let (origin, mut sessions) = server().await;
    let client = Client::new(&format!("{origin}/sub/"), Some("tok")).unwrap();
    let _stream = client.streaming().await.unwrap();
    assert_eq!(sessions.recv().await.unwrap().uri, "/sub/streaming?i=tok");
    let client = Client::new(&origin, None).unwrap();
    let _stream = client.streaming().await.unwrap();
    assert_eq!(sessions.recv().await.unwrap().uri, "/streaming");
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
    // bind してすぐ閉じたポートには、つながらない。
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    drop(listener);
    let client = Client::new(&origin, Some("secret-token")).unwrap();
    let error = client.streaming().await.unwrap_err();
    assert!(matches!(error, Error::Streaming(_)), "{error:?}");
    // トークンは URL のクエリにあるので、エラーに URL を残さない。
    assert!(!format!("{error:?}").contains("secret-token"), "{error:?}");
}

#[tokio::test]
async fn connect_waits_for_ack_then_delivers_events_by_id() {
    let (stream, mut session, _) = connected().await;
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
}

#[tokio::test]
async fn dropping_connection_sends_disconnect() {
    let (stream, mut session, _) = connected().await;
    let connection = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    drop(connection);
    assert_eq!(
        session.recv_json().await,
        json!({"type": "disconnect", "body": {"id": "1"}})
    );
}

#[tokio::test]
async fn messages_to_channel_and_stream() {
    let (stream, mut session, _) = connected().await;
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
    let (stream, mut session, _) = connected().await;
    let mut first = stream.broadcast_receiver::<Value>();
    let mut second = stream.broadcast_receiver::<Value>();
    let event = json!({"type": "emojiAdded", "body": {"emoji": {"name": "e"}}});
    session.send_json(event.clone()).await;
    // id の無い `connected` と、チャンネルのイベントは、全体の受け口には届かない。
    session
        .send_json(json!({"type": "connected", "body": {}}))
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
    let (stream, mut session, _) = connected().await;
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
    let (stream, mut session, _) = connected().await;
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
    // サーバ側には close が届く。
    assert!(matches!(
        session.ws.next().await,
        Some(Ok(Message::Close(_)))
    ));
}

#[tokio::test]
async fn server_close_ends_receivers() {
    let (stream, mut session, _) = connected().await;
    let mut connection = open_channel(&stream, &mut session, Params { a: 1 }, "1").await;
    session.ws.close(None).await.unwrap();
    assert!(connection.recv().await.is_none());
    assert!(matches!(
        stream.send_raw("x", Value::Null),
        Err(Error::StreamClosed)
    ));
}

#[tokio::test]
async fn dropping_all_handles_closes_the_socket() {
    let (stream, mut session, _) = connected().await;
    drop(stream);
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
