#![cfg(feature = "streaming")]
// accept_async のコールバックが返す Err の型は async-tungstenite が決めている。
#![allow(clippy::result_large_err)]
//! WebSocket のモックサーバを相手に、生成したストリーミングのバインディング（ビルダー・Event・Message）を
//! 通しで動かすテスト。イベントの本文には選んだバージョン用に採取した fixture を使う。

mod common;

use std::time::Duration;

use async_tungstenite::WebSocketStream;
use async_tungstenite::tokio::{TokioAdapter, accept_async};
use async_tungstenite::tungstenite::Message as WsMessage;
use common::fixture;
use common::version::Client;
use common::version::streaming::{self, BroadcastEvent, Stream, types};
use futures_util::StreamExt;
use serde_json::{Value, json};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::mpsc;
use tokio::time::timeout;

struct Session {
    ws: WebSocketStream<TokioAdapter<TcpStream>>,
}

impl Session {
    async fn recv_json(&mut self) -> Value {
        match timeout(Duration::from_secs(5), self.ws.next()).await {
            Ok(Some(Ok(WsMessage::Text(text)))) => serde_json::from_str(text.as_str()).unwrap(),
            other => panic!("テキストのメッセージのはずが {other:?} だった"),
        }
    }

    async fn send_json(&mut self, value: Value) {
        self.ws
            .send(WsMessage::Text(value.to_string().into()))
            .await
            .unwrap();
    }

    /// `connect` を受け取って channel と params を確かめ、`connected` を返す。
    async fn accept_connect(&mut self, channel: &str, params: Value) -> String {
        let message = self.recv_json().await;
        assert_eq!(message["type"], "connect");
        assert_eq!(message["body"]["channel"], channel);
        assert_eq!(message["body"]["params"], params);
        assert_eq!(message["body"]["pong"], true);
        let id = message["body"]["id"].as_str().unwrap().to_owned();
        self.send_json(json!({"type": "connected", "body": {"id": id}}))
            .await;
        id
    }

    async fn send_event(&mut self, id: &str, r#type: &str, body: Value) {
        self.send_json(
            json!({"type": "channel", "body": {"id": id, "type": r#type, "body": body}}),
        )
        .await;
    }
}

/// モックサーバを起動してストリームをつなぎ、サーバ側の接続と一緒に返す。
async fn connected() -> (Stream, Session) {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let origin = format!("http://{}", listener.local_addr().unwrap());
    let (sessions, mut rx) = mpsc::unbounded_channel();
    tokio::spawn(async move {
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            let ws = accept_async(stream).await.unwrap();
            let _ = sessions.send(Session { ws });
        }
    });
    let client = Client::new(&origin, Some("tok")).unwrap();
    let stream = client.streaming().await.unwrap();
    let session = rx.recv().await.unwrap();
    (stream, session)
}

fn note_json() -> Value {
    serde_json::from_str(&fixture("notes-show")).unwrap()
}

#[tokio::test]
async fn builder_sends_params_and_delivers_typed_events() {
    let (stream, mut session) = connected().await;
    let (timeline, id) = tokio::join!(
        stream.local_timeline().with_renotes(false),
        session.accept_connect("localTimeline", json!({"withRenotes": false})),
    );
    let mut timeline = timeline.unwrap();
    assert_eq!(timeline.id(), id);
    session.send_event(&id, "note", note_json()).await;
    match timeline.recv().await.unwrap().unwrap() {
        streaming::local_timeline::Event::Note(note) => assert_eq!(note.id, note_json()["id"]),
        other => panic!("想定外のイベント: {other:?}"),
    }
    // 必須の params は型状態で求められ、設定すると await できる。
    let (list, _) = tokio::join!(
        stream.user_list().list_id("l1").with_files(true),
        session.accept_connect("userList", json!({"listId": "l1", "withFiles": true})),
    );
    let _ = list.unwrap();
}

#[tokio::test]
async fn events_without_body_and_unknown_types() {
    let (stream, mut session) = connected().await;
    let (main, id) = tokio::join!(stream.main(), session.accept_connect("main", json!({})));
    let mut main = main.unwrap();
    session
        .send_event(&id, "readAllNotifications", Value::Null)
        .await;
    session.send_event(&id, "mention", note_json()).await;
    session.send_event(&id, "somethingNew", json!(1)).await;
    session.send_event(&id, "mention", json!({"id": 5})).await;
    assert!(matches!(
        main.recv().await.unwrap().unwrap(),
        streaming::main::Event::ReadAllNotifications
    ));
    assert!(matches!(
        main.recv().await.unwrap().unwrap(),
        streaming::main::Event::Mention(note) if note.id == note_json()["id"]
    ));
    match main.recv().await.unwrap().unwrap() {
        streaming::main::Event::Unrecognized { r#type, body } => {
            assert_eq!(r#type, "somethingNew");
            assert_eq!(body, json!(1));
        }
        other => panic!("想定外のイベント: {other:?}"),
    }
    match main.recv().await.unwrap() {
        Err(miapi::Error::DecodeEvent {
            channel, r#type, ..
        }) => assert_eq!((channel.as_str(), r#type.as_str()), ("main", "mention")),
        other => panic!("想定外の結果: {other:?}"),
    }
}

#[tokio::test]
async fn messages_are_sent_as_type_and_body() {
    let (stream, mut session) = connected().await;
    let (game, id) = tokio::join!(
        stream.reversi_game().game_id("g1"),
        session.accept_connect("reversiGame", json!({"gameId": "g1"})),
    );
    let game = game.unwrap();
    game.send(streaming::reversi_game::Message::PutStone(
        streaming::reversi_game::PutStone {
            pos: 12.0,
            id: "a".into(),
        },
    ))
    .unwrap();
    game.send(streaming::reversi_game::Message::Cancel).unwrap();
    assert_eq!(
        session.recv_json().await,
        json!({"type": "ch", "body": {"id": id, "type": "putStone", "body": {"pos": 12.0, "id": "a"}}})
    );
    assert_eq!(
        session.recv_json().await,
        json!({"type": "ch", "body": {"id": id, "type": "cancel", "body": null}})
    );
}

#[tokio::test]
async fn broadcast_decodes_note_updated_and_unknown_shapes() {
    let (stream, mut session) = connected().await;
    let mut events = stream.broadcast();
    session
        .send_json(json!({"type": "noteUpdated", "body": {"id": "n1", "type": "reacted", "body": {"reaction": "👍", "emoji": null, "userId": "u1"}}}))
        .await;
    session
        .send_json(json!({"type": "noteUpdated", "body": {"id": "n1", "type": "somethingNew", "body": {}}}))
        .await;
    session
        .send_json(json!({"type": "somethingNew", "body": {}}))
        .await;
    match events.recv().await.unwrap().unwrap() {
        BroadcastEvent::NoteUpdated(types::NoteUpdatedEvent::Reacted(reacted)) => {
            assert_eq!(reacted.id, "n1");
            assert_eq!(reacted.body.reaction, "👍");
            assert_eq!(reacted.body.user_id, "u1");
        }
        other => panic!("想定外のイベント: {other:?}"),
    }
    assert!(matches!(
        events.recv().await.unwrap().unwrap(),
        BroadcastEvent::NoteUpdated(types::NoteUpdatedEvent::Unrecognized(_))
    ));
    assert!(matches!(
        events.recv().await.unwrap().unwrap(),
        BroadcastEvent::Unrecognized { r#type, .. } if r#type == "somethingNew"
    ));
}

#[test]
fn event_codecs_round_trip() {
    let event: streaming::main::Event =
        serde_json::from_value(json!({"type": "readAllNotifications", "body": null})).unwrap();
    assert_eq!(
        serde_json::to_value(&event).unwrap(),
        json!({"type": "readAllNotifications", "body": null})
    );
    let updated: types::NoteUpdatedEvent = serde_json::from_value(
        json!({"id": "n1", "type": "deleted", "body": {"deletedAt": "2026-01-01T00:00:00.000Z"}}),
    )
    .unwrap();
    assert!(matches!(updated, types::NoteUpdatedEvent::Deleted(_)));
    assert_eq!(
        serde_json::to_value(&updated).unwrap(),
        json!({"id": "n1", "type": "deleted", "body": {"deletedAt": "2026-01-01T00:00:00.000Z"}})
    );
}
