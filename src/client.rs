use std::fmt;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::endpoint::{Body, Endpoint};
use crate::error::{ApiError, Error};

/// 1 つの Misskey サーバを相手にする HTTP クライアント。
///
/// `V` は、そのサーバを読むバージョンセットを表すマーカーである。
/// バージョンセットのモジュールごとに、`V` を決めた別名の `Client` があり、
/// エンドポイントのビルダーを作るメソッドは、その別名の側にある。
///
/// 認証は misskey-js と同じ方式にしている。
/// アクセストークンは、JSON ボディでは `i` フィールドとして、
/// multipart ボディではテキストのパートとして送る。
/// この方式は Misskey のどのバージョンでも受け付けられる。
#[derive(Clone)]
pub struct Client<V> {
    http: reqwest::Client,
    origin: String,
    token: Option<String>,
    _version: PhantomData<fn() -> V>,
}

#[derive(serde::Deserialize)]
struct ErrorEnvelope {
    error: ApiError,
}

impl<V> Client<V> {
    /// `origin` はサーバのベース URL で、`https://misskey.example.com` のように書く。
    /// 末尾のスラッシュは無視する。
    /// `token` はアクセストークンで、認証の要らないエンドポイントを呼ぶなら `None` でよい。
    pub fn new(origin: &str, token: Option<&str>) -> Self {
        Self {
            http: reqwest::Client::new(),
            origin: origin.trim_end_matches('/').to_owned(),
            token: token.map(str::to_owned),
            _version: PhantomData,
        }
    }

    /// 内部で使う HTTP クライアントを差し替える。
    /// プロキシ、タイムアウト、ユーザーエージェントなどを設定するときに使う。
    pub fn with_http_client(mut self, http: reqwest::Client) -> Self {
        self.http = http;
        self
    }

    pub fn origin(&self) -> &str {
        &self.origin
    }

    /// `request` のエンドポイントを呼び、応答をその `Response` 型にデコードする。
    pub async fn call<E: Endpoint>(&self, request: E) -> Result<E::Response, Error> {
        let body = request.into_body()?;
        let (status, bytes) = self.send(E::PATH, body).await?;
        decode(status, &bytes)
    }

    /// 任意のエンドポイントを、パラメータの JSON オブジェクトを渡して呼び、
    /// 応答の JSON をそのまま返す（204 のときは `Value::Null`）。
    /// バインディングが生成されていないエンドポイントを呼ぶための抜け道。
    pub async fn call_raw(&self, path: &str, params: Value) -> Result<Value, Error> {
        let (status, bytes) = self.send(path, Body::Json(params)).await?;
        decode(status, &bytes)
    }

    async fn send(&self, path: &str, body: Body) -> Result<(u16, bytes::Bytes), Error> {
        let url = format!("{}/api/{}", self.origin, path);
        let request = match body {
            Body::Json(value) => {
                let mut object = match value {
                    Value::Object(object) => object,
                    Value::Null => Map::new(),
                    _ => {
                        return Err(Error::Serialize(
                            <serde_json::Error as serde::ser::Error>::custom(
                                "request body must be a JSON object",
                            ),
                        ));
                    }
                };
                if let Some(token) = &self.token {
                    object.insert("i".to_owned(), Value::String(token.clone()));
                }
                self.http.post(&url).json(&object)
            }
            Body::Multipart {
                fields,
                file: (name, file),
            } => {
                let mut form = reqwest::multipart::Form::new();
                if let Some(token) = &self.token {
                    form = form.text("i", token.clone());
                }
                for (key, value) in fields {
                    // misskey-js と同じ符号化にする。
                    // null は省き、オブジェクトは JSON のテキストで送る。
                    let text = match value {
                        Value::Null => continue,
                        Value::String(text) => text,
                        other => other.to_string(),
                    };
                    form = form.text(key, text);
                }
                let mut part = reqwest::multipart::Part::bytes(file.data).file_name(file.file_name);
                if let Some(content_type) = &file.content_type {
                    part = part.mime_str(content_type)?;
                }
                self.http.post(&url).multipart(form.part(name, part))
            }
        };
        let response = request.send().await?;
        let status = response.status().as_u16();
        let bytes = response.bytes().await?;
        if (200..300).contains(&status) {
            return Ok((status, bytes));
        }
        Err(match serde_json::from_slice::<ErrorEnvelope>(&bytes) {
            Ok(envelope) => Error::Api(Box::new(ApiError {
                status,
                ..envelope.error
            })),
            Err(_) => Error::UnexpectedResponse {
                status,
                body: String::from_utf8_lossy(&bytes).into_owned(),
            },
        })
    }
}

fn decode<T: DeserializeOwned>(status: u16, bytes: &[u8]) -> Result<T, Error> {
    if status == 204 {
        return serde_json::from_value(Value::Null).map_err(|source| Error::Decode {
            source,
            body: String::new(),
        });
    }
    serde_json::from_slice(bytes).map_err(|source| Error::Decode {
        source,
        body: String::from_utf8_lossy(bytes).into_owned(),
    })
}

impl<V> fmt::Debug for Client<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("origin", &self.origin)
            .finish_non_exhaustive()
    }
}
