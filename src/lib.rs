//! Misskey API のクライアント。自動生成でサーバのエンドポイントと型に追従する。
//!
//! バインディングは、サーバの `api.json` から Misskey のバージョンごとに生成してあり、cargo feature で有効にする。
//! 既定の `latest` は、クレートの直下から使う。
//! エンドポイントのパスを [`Client`](client::Client) からのメソッドの連鎖として書き、できたビルダーを await する。
//!
//! ```no_run
//! # #[cfg(feature = "latest")]
//! # async fn run() -> Result<(), miapi::Error> {
//! let client = miapi::Client::new("https://misskey.example.com", Some("token"))?;
//! let note = client.notes().show().note_id("xxxxxxxxxx").await?;
//! println!("{}", note.text.unwrap_or_default());
//! let created = client.notes().create().text("hello").await?;
//! println!("{}", created.created_note.id);
//! # Ok(()) }
//! ```
//!
//! 固定版は、`miapi::v2024_10_1::Client` のように、バージョンのモジュールにある `Client` から同じように使う。
//!
//! 各エンドポイントには、[`Client::call`](client::Client::call) に渡す `Request` 構造体もある。
//! バインディングに無いエンドポイントは [`Client::call_raw`](client::Client::call_raw) で呼べる。

pub mod builder;
pub mod client;
pub mod endpoint;
mod error;
mod generated;
#[doc(hidden)]
pub mod serde_helpers;

pub use endpoint::{Body, Endpoint, FilePart};
pub use error::{ApiError, Error};
pub use generated::*;
pub use reqwest;
