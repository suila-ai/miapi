//! Misskey サーバの名前とバージョンを表示する。
//! `cargo run --example meta -- https://misskey.example.com`

use miapi::{Client, api};

#[tokio::main]
async fn main() -> Result<(), miapi::Error> {
    let origin = std::env::args()
        .nth(1)
        .expect("usage: cargo run --example meta -- <origin>");
    let client = Client::new(&origin, None)?;
    let (name, version) = match client.meta().detail(true).await? {
        api::meta::Response::MetaDetailed(meta) => (meta.name, meta.version),
        api::meta::Response::MetaLite(meta) => (meta.name, meta.version),
        api::meta::Response::Unrecognized(value) => {
            println!("unrecognized meta shape: {value}");
            return Ok(());
        }
    };
    println!(
        "{} ({origin}) runs Misskey {version}; bindings target {}",
        name.as_deref().unwrap_or("(unnamed)"),
        miapi::API_VERSION
    );
    Ok(())
}
