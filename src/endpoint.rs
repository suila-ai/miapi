use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::Error;

/// 1 つの Misskey エンドポイントに対応するリクエスト型。
///
/// 生成される `Request` 型がこのトレイトを実装し、
/// [`Client::call`](crate::client::Client::call) がそれを送る。
pub trait Endpoint {
    /// `/api/` に続くエンドポイント名。たとえば `notes/create`。
    const PATH: &'static str;
    /// 応答のボディをデコードした結果の型。204 の応答は null としてデコードするので、
    /// 204 だけを返すエンドポイントでは `()`、
    /// 200 か 204 のどちらかを返すエンドポイントでは `Option<T>` か `serde_json::Value` にする。
    type Response: DeserializeOwned;
    /// リクエストを、送信するボディに変換する。認証情報はクライアントが足す。
    fn into_body(self) -> Result<Body, Error>;
}

/// エンドポイントを呼ぶときのリクエストボディ。認証情報を足す前の状態である。
#[derive(Debug, Clone, PartialEq)]
pub enum Body {
    /// `application/json`。`Value::Null` は、パラメータの無いエンドポイントを表す。
    Json(Value),
    /// ファイルのパートを 1 つ持つ `multipart/form-data`。
    /// ほかのフィールドはテキストのパートとして送る。
    Multipart {
        fields: Map<String, Value>,
        file: (&'static str, FilePart),
    },
}

/// `drive/files/create` のような multipart のエンドポイントに渡す、ファイルの内容。
#[derive(Debug, Clone, PartialEq, Default)]
pub struct FilePart {
    pub file_name: String,
    /// このパートに付けて送る MIME タイプ。`None` のときは、サーバが内容から判定する。
    pub content_type: Option<String>,
    pub data: Vec<u8>,
}

/// リクエストの構造体を [`Body::Json`] にシリアライズする。
pub fn json_body<T: Serialize>(request: &T) -> Result<Body, Error> {
    to_value(request).map(Body::Json)
}

/// リクエストの構造体を、[`Body::Multipart`] のファイル以外のフィールドにシリアライズする。
/// ファイルのフィールドは `#[serde(skip)]` で除いておく。
pub fn multipart_fields<T: Serialize>(request: &T) -> Result<Map<String, Value>, Error> {
    to_value(request).and_then(into_object)
}

fn to_value<T: Serialize>(request: &T) -> Result<Value, Error> {
    serde_json::to_value(request).map_err(|error| Error::InvalidRequest(error.into()))
}

/// リクエストのパラメータを JSON の object として取り出す。null は [`Body::Json`] と同じく、パラメータが無いものとして空の object にする。
pub(crate) fn into_object(params: Value) -> Result<Map<String, Value>, Error> {
    match params {
        Value::Object(object) => Ok(object),
        Value::Null => Ok(Map::new()),
        _ => Err(Error::InvalidRequest(
            "request parameters must be a JSON object".into(),
        )),
    }
}
