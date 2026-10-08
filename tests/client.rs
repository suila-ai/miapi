use miapi::endpoint::{Body, json_body};
use miapi::{Endpoint, Error, FilePart};
use serde::{Deserialize, Serialize};
use serde_json::json;
use wiremock::matchers::{body_json, body_string_contains, header, header_regex, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

/// このテストは生成されたバインディングを使わないので、バージョンセットのマーカーを自前で用意する。
struct NoVersion;
type Client = miapi::client::Client<NoVersion>;

#[derive(Serialize)]
struct Show {
    #[serde(rename = "noteId")]
    note_id: String,
}

#[derive(Debug, PartialEq, Deserialize)]
struct Note {
    id: String,
    text: Option<String>,
}

impl Endpoint for Show {
    const PATH: &'static str = "notes/show";
    type Response = Note;
    fn into_body(self) -> Result<Body, Error> {
        json_body(&self)
    }
}

#[derive(Serialize)]
struct Me;

impl Endpoint for Me {
    const PATH: &'static str = "i";
    type Response = serde_json::Value;
    fn into_body(self) -> Result<Body, Error> {
        json_body(&self)
    }
}

#[derive(Serialize)]
struct Delete;

impl Endpoint for Delete {
    const PATH: &'static str = "notes/delete";
    type Response = ();
    fn into_body(self) -> Result<Body, Error> {
        json_body(&self)
    }
}

#[derive(Serialize)]
struct Translate;

impl Endpoint for Translate {
    const PATH: &'static str = "notes/translate";
    type Response = Option<Note>;
    fn into_body(self) -> Result<Body, Error> {
        json_body(&self)
    }
}

struct Upload {
    file: FilePart,
}

impl Endpoint for Upload {
    const PATH: &'static str = "drive/files/create";
    type Response = serde_json::Value;
    fn into_body(self) -> Result<Body, Error> {
        let mut fields = serde_json::Map::new();
        fields.insert("name".into(), json!("photo.png"));
        fields.insert("isSensitive".into(), json!(true));
        fields.insert("folderId".into(), json!(null));
        fields.insert("meta".into(), json!({"k": 1}));
        Ok(Body::Multipart {
            fields,
            file: ("file", self.file),
        })
    }
}

async fn server_and_client(token: Option<&str>) -> (MockServer, Client) {
    let server = MockServer::start().await;
    let client = Client::new(&format!("{}/", server.uri()), token);
    (server, client)
}

#[tokio::test]
async fn json_call_adds_token_and_decodes_response() {
    let (server, client) = server_and_client(Some("tok")).await;
    Mock::given(method("POST"))
        .and(path("/api/notes/show"))
        .and(header("content-type", "application/json"))
        .and(body_json(json!({"noteId": "n1", "i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "n1", "text": "hi"})))
        .expect(1)
        .mount(&server)
        .await;
    let note = client
        .call(Show {
            note_id: "n1".into(),
        })
        .await
        .unwrap();
    assert_eq!(
        note,
        Note {
            id: "n1".into(),
            text: Some("hi".into())
        }
    );
}

#[tokio::test]
async fn unit_request_sends_object_with_token_only() {
    let (server, client) = server_and_client(Some("tok")).await;
    Mock::given(path("/api/i"))
        .and(body_json(json!({"i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "me"})))
        .expect(1)
        .mount(&server)
        .await;
    assert_eq!(client.call(Me).await.unwrap(), json!({"id": "me"}));
}

#[tokio::test]
async fn without_token_sends_empty_object() {
    let (server, client) = server_and_client(None).await;
    Mock::given(path("/api/i"))
        .and(body_json(json!({})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({})))
        .expect(1)
        .mount(&server)
        .await;
    client.call(Me).await.unwrap();
}

#[tokio::test]
async fn no_content_decodes_into_unit_and_none() {
    let (server, client) = server_and_client(None).await;
    Mock::given(path("/api/notes/delete"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    Mock::given(path("/api/notes/translate"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&server)
        .await;
    client.call(Delete).await.unwrap();
    assert_eq!(client.call(Translate).await.unwrap(), None);
}

#[tokio::test]
async fn api_error_body_becomes_api_error() {
    let (server, client) = server_and_client(None).await;
    Mock::given(path("/api/notes/show"))
        .respond_with(ResponseTemplate::new(400).set_body_json(json!({
            "error": {
                "message": "No such note.",
                "code": "NO_SUCH_NOTE",
                "id": "24fcbfc6-2e37-42b6-8388-c29b3861a08d",
                "kind": "client"
            }
        })))
        .mount(&server)
        .await;
    let err = client
        .call(Show {
            note_id: "x".into(),
        })
        .await
        .unwrap_err();
    match err {
        Error::Api(api) => {
            assert_eq!(api.status, 400);
            assert_eq!(api.code, "NO_SUCH_NOTE");
            assert_eq!(api.kind.as_deref(), Some("client"));
            assert_eq!(api.info, None);
        }
        other => panic!("想定外のエラー: {other:?}"),
    }
}

#[tokio::test]
async fn non_json_failure_becomes_unexpected_response() {
    let (server, client) = server_and_client(None).await;
    Mock::given(path("/api/i"))
        .respond_with(ResponseTemplate::new(502).set_body_string("<html>bad gateway</html>"))
        .mount(&server)
        .await;
    match client.call(Me).await.unwrap_err() {
        Error::UnexpectedResponse { status, body } => {
            assert_eq!(status, 502);
            assert!(body.contains("bad gateway"));
        }
        other => panic!("想定外のエラー: {other:?}"),
    }
}

#[tokio::test]
async fn mismatched_body_becomes_decode_error() {
    let (server, client) = server_and_client(None).await;
    Mock::given(path("/api/notes/show"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"text": "no id"})))
        .mount(&server)
        .await;
    match client
        .call(Show {
            note_id: "x".into(),
        })
        .await
        .unwrap_err()
    {
        Error::Decode { body, .. } => assert!(body.contains("no id")),
        other => panic!("想定外のエラー: {other:?}"),
    }
}

#[tokio::test]
async fn multipart_sends_token_fields_and_file() {
    let (server, client) = server_and_client(Some("tok")).await;
    Mock::given(path("/api/drive/files/create"))
        .and(header_regex(
            "content-type",
            "^multipart/form-data; boundary=",
        ))
        .and(body_string_contains("name=\"i\"\r\n\r\ntok"))
        .and(body_string_contains("name=\"name\"\r\n\r\nphoto.png"))
        .and(body_string_contains("name=\"isSensitive\"\r\n\r\ntrue"))
        .and(body_string_contains("name=\"meta\"\r\n\r\n{\"k\":1}"))
        .and(body_string_contains(
            "name=\"file\"; filename=\"photo.png\"",
        ))
        .and(body_string_contains("Content-Type: image/png"))
        .and(body_string_contains("PNGDATA"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"id": "f1"})))
        .expect(1)
        .mount(&server)
        .await;
    let file = FilePart {
        file_name: "photo.png".into(),
        content_type: Some("image/png".into()),
        data: b"PNGDATA".to_vec(),
    };
    client.call(Upload { file }).await.unwrap();
    let body =
        String::from_utf8(server.received_requests().await.unwrap()[0].body.clone()).unwrap();
    assert!(
        !body.contains("folderId"),
        "null fields must be omitted: {body}"
    );
}

#[tokio::test]
async fn call_raw_returns_json_value() {
    let (server, client) = server_and_client(Some("tok")).await;
    Mock::given(path("/api/meta"))
        .and(body_json(json!({"detail": false, "i": "tok"})))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"name": "mi"})))
        .mount(&server)
        .await;
    let value = client
        .call_raw("meta", json!({"detail": false}))
        .await
        .unwrap();
    assert_eq!(value["name"], "mi");
}

#[test]
fn call_future_is_send() {
    fn assert_send<T: Send>(_: T) {}
    let client = Client::new("https://example.com", None);
    assert_send(client.call(Me));
}
