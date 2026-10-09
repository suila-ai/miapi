#![doc = "cargo feature で有効にした、Misskey のバージョンごとの生成済みバインディング。"]
#[cfg(feature = "latest")]
#[path = "latest/mod.rs"]
mod latest;
#[cfg(feature = "latest")]
pub use latest::{API_VERSION, Client, api, types};
#[cfg(feature = "v2024.10.1")]
#[path = "v2024.10.1/mod.rs"]
pub mod v2024_10_1;
#[cfg(feature = "v2025.10.0")]
#[path = "v2025.10.0/mod.rs"]
pub mod v2025_10_0;
