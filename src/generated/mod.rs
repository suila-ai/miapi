#![doc = "cargo feature で有効にした、Misskey のバージョンごとの生成済みバインディング。"]
#[cfg(feature = "latest")]
#[path = "latest/mod.rs"]
mod latest;
#[cfg(feature = "latest")]
pub use latest::{API_VERSION, Client, api, types};
