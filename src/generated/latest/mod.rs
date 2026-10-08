#![doc = "`api.json` から生成したバインディング。手で編集せず、`cargo codegen` を実行すること。"]
pub mod api;
pub mod types;
#[doc = "このバインディングの生成元である `api.json` の `info.version`。"]
pub const API_VERSION: &str = "2026.10.0";
#[doc = "マーカー: クライアントが相手にするサーバを、このバージョンセットで読む。"]
#[derive(Debug, Clone, Copy)]
pub struct Version;
#[doc = "このバージョンセットでサーバを読むクライアント。エンドポイントのビルダーは、ここからメソッドの連鎖で作る。"]
pub type Client = crate::client::Client<Version>;
