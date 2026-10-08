/// Misskey API が返すエラーのボディ（`{"error": {...}}`）に、
/// その応答の HTTP ステータスを加えたもの。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ApiError {
    /// 応答の HTTP ステータスコード（400、401、403、429、500 など）。
    #[serde(skip)]
    pub status: u16,
    /// エラーの定義を識別する、変わらない UUID。
    pub id: String,
    /// `NO_SUCH_NOTE` のようなエラーコード。エンドポイントの中で一意である。
    pub code: String,
    pub message: String,
    /// `"client"` か `"server"`。サーバが返した場合にだけ入る。
    #[serde(default)]
    pub kind: Option<String>,
    /// 追加の詳細。たとえば `INVALID_PARAM` のときの検証の失敗内容。
    #[serde(default)]
    pub info: Option<serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Misskey API error {} ({}): {}", .0.code, .0.status, .0.message)]
    Api(Box<ApiError>),
    #[error("HTTP transport error")]
    Transport(#[from] reqwest::Error),
    #[error("failed to decode response body")]
    Decode {
        #[source]
        source: serde_json::Error,
        body: String,
    },
    #[error("unexpected HTTP status {status}")]
    UnexpectedResponse { status: u16, body: String },
    #[error("failed to serialize request")]
    Serialize(#[source] serde_json::Error),
}
