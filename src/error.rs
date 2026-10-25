/// Misskey API が返すエラーのボディ（`{"error": {...}}`）に、
/// その応答の HTTP ステータスと、呼んだエンドポイント名を加えたもの。
#[derive(Debug, Clone, PartialEq, serde::Deserialize)]
pub struct ApiError {
    /// 応答の HTTP ステータスコード（400、401、403、429、500 など）。
    #[serde(skip)]
    pub status: u16,
    /// 呼んだエンドポイント名。`notes/show` のように、`/api/` に続く部分を指す。
    #[serde(skip)]
    pub endpoint: String,
    /// エラーの定義を識別する、変わらない UUID。
    pub id: String,
    /// `NO_SUCH_NOTE` のようなエラーコード。エンドポイントの中で一意である。
    pub code: String,
    pub message: String,
    /// `"client"`・`"server"`・`"permission"` のどれか。サーバが返した場合にだけ入る。
    #[serde(default)]
    pub kind: Option<String>,
    /// 追加の詳細。たとえば `INVALID_PARAM` のときの検証の失敗内容。
    #[serde(default)]
    pub info: Option<serde_json::Value>,
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Misskey API error {} ({}) from {}: {}", .0.code, .0.status, .0.endpoint, .0.message)]
    Api(Box<ApiError>),
    #[error("HTTP transport error")]
    Transport(#[from] reqwest::Error),
    #[error("failed to decode response body from {endpoint}")]
    Decode {
        /// 呼んだエンドポイント名。`notes/show` のように、`/api/` に続く部分を指す。
        endpoint: String,
        #[source]
        source: serde_json::Error,
        body: String,
    },
    #[error("unexpected HTTP status {status} from {endpoint}")]
    UnexpectedResponse {
        /// 呼んだエンドポイント名。`notes/show` のように、`/api/` に続く部分を指す。
        /// ストリーミングのハンドシェイクでは `streaming` が入る。
        endpoint: String,
        status: u16,
        /// 応答の本文。ストリーミングのハンドシェイクでは、サーバやプロキシが本文に URL を写すと
        /// クエリのアクセストークンが入るので、読まずに空にする。
        body: String,
    },
    /// 送る前に分かったリクエストの誤り。サーバには送っていない。
    #[error("invalid request")]
    InvalidRequest(#[source] Box<dyn std::error::Error + Send + Sync>),
    /// [`Client`](crate::client::Client) を作るときに渡した origin が、http か https の URL として読めないか、
    /// クエリかフラグメントが付いている。
    #[error("invalid origin {origin:?}")]
    InvalidOrigin {
        /// 渡したままの origin。
        origin: String,
        #[source]
        source: Box<dyn std::error::Error + Send + Sync>,
    },
    // ストリーミングのバリアントは feature streaming を切っても残す。Error は #[non_exhaustive] でないので、feature で
    // 増減させると、別のクレートが streaming を有効にしたときに、網羅的な match がコンパイルできなくなる。
    /// ストリーミング（WebSocket）の接続か送受信の失敗。ただし、サーバがハンドシェイクに 101 以外で応答したときは
    /// [`Error::UnexpectedResponse`] になる。
    #[error("streaming transport error")]
    Streaming(#[source] Box<dyn std::error::Error + Send + Sync>),
    /// ストリーミングで受け取ったイベントを、そのイベントの型にデコードできなかった。
    #[error("failed to decode streaming event {} from {channel}", .r#type)]
    DecodeEvent {
        /// `localTimeline` のようなチャンネル名。ストリーム全体のイベント（`noteUpdated` など）では `broadcast`。
        channel: String,
        /// イベントの `type`。
        r#type: String,
        #[source]
        source: serde_json::Error,
        /// イベントの `body` の JSON。
        body: String,
    },
    /// チャンネルにつないだが、サーバが受け付けたという応答（`connected`）が
    /// [`CONNECT_ACK_TIMEOUT`](crate::stream::CONNECT_ACK_TIMEOUT) のうちに来なかった。
    /// サーバは次の接続に応答しない。
    ///
    /// - 認証の無いストリームからの、要認証のチャンネルへの接続
    /// - 権限の無いチャンネルと、知らないチャンネル名への接続
    /// - params の無いチャンネルへの、同じストリームからの 2 回目の接続
    /// - 1 つのストリームで 32 本を超える接続
    /// - params が指すリストやアンテナなどが、無いか使えない接続
    ///   （2025.10.0 までのサーバは受け付けて、イベントを送らない）
    #[error("channel {channel} did not become connected")]
    ChannelUnavailable { channel: String },
    /// ストリームが閉じた後に送受信しようとした。
    #[error("streaming connection is closed")]
    StreamClosed,
}
