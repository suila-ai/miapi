//! モックサーバを相手に、生成したビルダーを通しで動かすテスト。
//! 応答のボディには、選んだバージョン用に採取した fixture を使う。
//! そのため、このテストはどのバージョンの feature でも動く。

mod common;

use common::fixture;
use common::version::{API_VERSION, Client, api};
use miapi::{Error, FilePart};
use serde_json::{Value, json};
use wiremock::matchers::{body_json, body_string_contains, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn note_json() -> Value {
    serde_json::from_str(&fixture("notes-show")).unwrap()
}

fn drive_file_json() -> Value {
    serde_json::from_str(&fixture("drive-file")).unwrap()
}

async fn server_and_client() -> (MockServer, Client) {
    let server = MockServer::start().await;
    let client = Client::new(&server.uri(), Some("tok"));
    (server, client)
}

#[test]
fn fixtures_come_from_the_selected_version() {
    let meta: Value = serde_json::from_str(&fixture("meta")).unwrap();
    assert_eq!(meta["version"], API_VERSION);
}

#[tokio::test]
async fn required_parameter_setter_then_await() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/notes/show"))
        .and(body_json(json!({"noteId": "n1", "i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(note_json()))
        .expect(1)
        .mount(&server)
        .await;
    let note = client.notes().show().note_id("n1").await.unwrap();
    assert_eq!(note.id, note_json()["id"]);
}

#[tokio::test]
async fn namespace_root_endpoint_and_optional_setters() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/notes"))
        .and(body_json(
            json!({"limit": 10, "withFiles": true, "i": "tok"}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([note_json(), note_json()])))
        .expect(1)
        .mount(&server)
        .await;
    let notes = client.notes().limit(10).with_files(true).await.unwrap();
    assert_eq!(notes.len(), 2);
}

#[tokio::test]
async fn endpoint_without_parameters() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/ping"))
        .and(body_json(json!({"i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"pong": 1234})))
        .expect(1)
        .mount(&server)
        .await;
    let pong = client.ping().await.unwrap();
    assert_eq!(pong.pong, 1234.0);
}

#[tokio::test]
async fn null_setter_sends_explicit_null_and_omits_unset_fields() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/i/update"))
        .and(body_json(
            json!({"name": "new name", "avatarId": null, "i": "tok"}),
        ))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;
    // 応答は MeDetailed で、必須のフィールドがある。このテストが検査するのはリクエストだけである。
    let result = client.i().update().name("new name").avatar_id_null().await;
    assert!(matches!(result, Err(Error::Decode { .. })));
}

#[tokio::test]
async fn maybe_setter_sends_some_and_omits_none() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/notes/create"))
        .and(body_json(json!({"text": "hi", "cw": "warn", "i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"createdNote": note_json()})))
        .expect(1)
        .mount(&server)
        .await;
    let reply_id: Option<String> = None;
    let created = client
        .notes()
        .create()
        .text("hi")
        .maybe_cw(Some("warn"))
        .reply_id("r")
        .maybe_reply_id(reply_id)
        .await
        .unwrap();
    assert_eq!(created.created_note.id, note_json()["id"]);
}

#[tokio::test]
async fn namespace_with_required_parameter_and_child_endpoint() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/notes/reactions/create"))
        .and(body_json(
            json!({"noteId": "n1", "reaction": "👍", "i": "tok"}),
        ))
        .respond_with(ResponseTemplate::new(204))
        .expect(1)
        .mount(&server)
        .await;
    Mock::given(path("/api/notes/reactions"))
        .and(body_json(json!({"noteId": "n1", "i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!([])))
        .expect(1)
        .mount(&server)
        .await;
    client
        .notes()
        .reactions()
        .create()
        .note_id("n1")
        .reaction("👍")
        .await
        .unwrap();
    let reactions = client.notes().reactions().note_id("n1").await.unwrap();
    assert!(reactions.is_empty());
}

#[tokio::test]
async fn nullable_response_endpoint_yields_none_on_204() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/federation/show-instance"))
        .and(body_json(json!({"host": "remote.example.com", "i": "tok"})))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    let instance = client
        .federation()
        .show_instance()
        .host("remote.example.com")
        .await
        .unwrap();
    assert_eq!(instance, None);
}

#[tokio::test]
async fn multipart_upload_through_builder() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/drive/files/create"))
        .and(body_string_contains("name=\"i\"\r\n\r\ntok"))
        .and(body_string_contains("name=\"isSensitive\"\r\n\r\ntrue"))
        .and(body_string_contains("name=\"file\"; filename=\"a.txt\""))
        .and(body_string_contains("hello file"))
        .respond_with(ResponseTemplate::new(200).set_body_json(drive_file_json()))
        .expect(1)
        .mount(&server)
        .await;
    let file = FilePart {
        file_name: "a.txt".into(),
        content_type: Some("text/plain".into()),
        data: b"hello file".to_vec(),
    };
    let created = client
        .drive()
        .files()
        .create()
        .file(file)
        .is_sensitive(true)
        .await
        .unwrap();
    assert_eq!(created.id, drive_file_json()["id"]);
}

#[tokio::test]
async fn request_struct_route_still_works() {
    let (server, client) = server_and_client().await;
    Mock::given(path("/api/notes/create"))
        .and(body_json(json!({"text": "hi", "cw": null, "i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"createdNote": note_json()})))
        .expect(1)
        .mount(&server)
        .await;
    let request = api::notes::create::Request {
        text: Some(Some("hi".into())),
        cw: Some(None),
        ..Default::default()
    };
    let created = client.call(request).await.unwrap();
    assert_eq!(created.created_note.id, note_json()["id"]);
    assert!(!API_VERSION.is_empty());
}
