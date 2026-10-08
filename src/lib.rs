//! Misskey API のクライアント。自動生成でサーバのエンドポイントと型に追従する。
//!
//! ```no_run
//! # async fn run() -> Result<(), miapi::Error> {
//! struct Version;
//! let client = miapi::client::Client::<Version>::new("https://misskey.example.com", Some("token"));
//! let raw = client.call_raw("meta", serde_json::json!({"detail": false})).await?;
//! println!("{}", raw["name"]);
//! # Ok(()) }
//! ```

pub mod builder;
pub mod client;
pub mod endpoint;
mod error;
#[doc(hidden)]
pub mod serde_helpers;

pub use endpoint::{Body, Endpoint, FilePart};
pub use error::{ApiError, Error};
