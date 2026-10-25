use std::fmt;
use std::marker::PhantomData;

use serde::de::DeserializeOwned;
use serde_json::Value;

use crate::endpoint::{Body, Endpoint, into_object};
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
pub struct Client<V> {
    http: reqwest::Client,
    origin: reqwest::Url,
    token: Option<String>,
    _version: PhantomData<fn() -> V>,
}

#[derive(serde::Deserialize)]
struct ErrorEnvelope {
    error: ApiError,
}

impl<V> Client<V> {
    /// `origin` はサーバのベース URL で、`https://misskey.example.com` のように書く。
    /// 前後の空白と末尾のスラッシュは無視する。
    /// http か https の URL として読めない `origin` と、クエリかフラグメントが付いた `origin` には、
    /// [`Error::InvalidOrigin`] を返す。
    /// `token` はアクセストークンで、認証の要らないエンドポイントを呼ぶなら `None` でよい。
    ///
    /// HTTP クライアントは `reqwest::Client::new()` と同じ設定で作るので、タイムアウトは無い。
    /// ただし feature `streaming` が有効なら、HTTP/1.1 だけを使う。WebSocket の upgrade は HTTP/1.1 でしか成立せず、
    /// HTTP/2 で開いた接続には乗らないためである。
    /// タイムアウト、プロキシ、ユーザーエージェントなどは、設定した `reqwest::Client` を
    /// [`with_http_client`](Self::with_http_client) に渡して使う。
    pub fn new(origin: &str, token: Option<&str>) -> Result<Self, Error> {
        #[cfg(feature = "streaming")]
        let http = reqwest::Client::builder().http1_only().build()?;
        #[cfg(not(feature = "streaming"))]
        let http = reqwest::Client::new();
        Self::with_http_client(http, origin, token)
    }

    /// [`new`](Self::new) と同じだが、HTTP クライアントに `http` をそのまま使う。
    /// `http` は miapi と同じ版の reqwest で作る。`miapi::reqwest::Client::builder()` を使えば版がそろう。
    /// ストリーミングを使うなら、`http1_only()` を付けて作る（[`new`](Self::new) の説明を参照）。
    pub fn with_http_client(
        http: reqwest::Client,
        origin: &str,
        token: Option<&str>,
    ) -> Result<Self, Error> {
        let invalid = |source: Box<dyn std::error::Error + Send + Sync>| Error::InvalidOrigin {
            origin: origin.to_owned(),
            source,
        };
        let mut url = reqwest::Url::parse(origin.trim()).map_err(|error| invalid(error.into()))?;
        if !matches!(url.scheme(), "http" | "https") {
            return Err(invalid("scheme must be http or https".into()));
        }
        if url.query().is_some() || url.fragment().is_some() {
            return Err(invalid("must not have a query or fragment".into()));
        }
        // 末尾の / は、文字列でなく、読んだ後の URL のパスから除く。URL のパーサは途中の改行やタブを消すので、
        // 文字列の上で除くと、`https://h/misskey/\t/` の / が残って重なる。
        let path = url.path().trim_end_matches('/').to_owned();
        url.set_path(&path);
        Ok(Self {
            http,
            origin: url,
            token: token.map(str::to_owned),
            _version: PhantomData,
        })
    }

    /// サーバのベース URL。[`new`](Self::new) に渡した `origin` を URL として読んだ形（ホスト名は小文字にし、
    /// 既定のポートは省く）で返し、末尾のスラッシュは付けない。
    pub fn origin(&self) -> &str {
        // パスが空の URL は、`https://h/` のように / で終わる。
        self.origin.as_str().trim_end_matches('/')
    }

    /// `request` のエンドポイントを呼び、応答をその `Response` 型にデコードする。
    pub async fn call<E: Endpoint>(&self, request: E) -> Result<E::Response, Error> {
        let body = request.into_body()?;
        let (status, bytes) = self.send(E::PATH, body).await?;
        decode(E::PATH, status, &bytes)
    }

    /// 任意のエンドポイントを、パラメータの JSON オブジェクトを渡して呼び、
    /// 応答の JSON をそのまま返す（204 のときは `Value::Null`）。
    /// バインディングが生成されていないエンドポイントを呼ぶための抜け道。
    pub async fn call_raw(&self, path: &str, params: Value) -> Result<Value, Error> {
        let (status, bytes) = self.send(path, Body::Json(params)).await?;
        decode(path, status, &bytes)
    }

    /// ストリーミング API（WebSocket の `/streaming`）につなぐ。認証はクエリの `i` で送る（misskey-js と同じ）。
    /// サーバが 101 以外で応答したら（認証に失敗した 401 など）[`Error::UnexpectedResponse`] を、
    /// それ以外の理由でつなげなかったら [`Error::Streaming`] を返す。つないだ後に切れたときは、
    /// 返した [`Stream`](crate::stream::Stream) がつなぎ直す。
    /// HTTP クライアントは HTTP/1.1 だけを使うものにしておくこと（[`new`](Self::new) の説明を参照）。
    #[cfg(feature = "streaming")]
    pub async fn streaming(&self) -> Result<crate::stream::Stream<V>, Error> {
        let mut url = self.origin.clone();
        url.path_segments_mut()
            .expect("http and https URLs can be a base")
            .push("streaming");
        if let Some(token) = &self.token {
            url.query_pairs_mut().append_pair("i", token);
        }
        crate::stream::Stream::open(self.http.clone(), url).await
    }

    async fn send(&self, path: &str, body: Body) -> Result<(u16, bytes::Bytes), Error> {
        let mut url = self.origin.clone();
        url.path_segments_mut()
            .expect("http and https URLs can be a base")
            .push("api")
            .extend(path.split('/'));
        let request = self.http.post(url);
        let request = match body {
            Body::Json(value) => {
                let mut object = into_object(value)?;
                if let Some(token) = &self.token {
                    object.insert("i".to_owned(), Value::String(token.clone()));
                }
                request.json(&object)
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
                    part = part.mime_str(content_type).map_err(|error| {
                        // reqwest の builder エラーの Display は "builder error" だけなので、原因の mime のエラーを書く。
                        let reason = std::error::Error::source(&error).unwrap_or(&error);
                        Error::InvalidRequest(
                            format!("invalid content type {content_type:?}: {reason}").into(),
                        )
                    })?;
                }
                request.multipart(form.part(name, part))
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
                endpoint: path.to_owned(),
                ..envelope.error
            })),
            Err(_) => Error::UnexpectedResponse {
                endpoint: path.to_owned(),
                status,
                body: String::from_utf8_lossy(&bytes).into_owned(),
            },
        })
    }
}

fn decode<T: DeserializeOwned>(endpoint: &str, status: u16, bytes: &[u8]) -> Result<T, Error> {
    let decoded = if status == 204 {
        serde_json::from_value(Value::Null)
    } else {
        serde_json::from_slice(bytes)
    };
    decoded.map_err(|source| Error::Decode {
        endpoint: endpoint.to_owned(),
        source,
        body: String::from_utf8_lossy(bytes).into_owned(),
    })
}

impl<V> Clone for Client<V> {
    fn clone(&self) -> Self {
        Client {
            http: self.http.clone(),
            origin: self.origin.clone(),
            token: self.token.clone(),
            _version: PhantomData,
        }
    }
}

impl<V> fmt::Debug for Client<V> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("origin", &self.origin())
            .finish_non_exhaustive()
    }
}
