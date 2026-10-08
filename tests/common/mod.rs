//! テストが読む fixture（`scripts/update-fixtures.sh` が保存したサーバの応答）と、テストするバージョンセット。

use std::path::PathBuf;

// テストは、feature を 1 つだけ有効にして実行する。そのバージョンセットを `version` として使う。
#[cfg(feature = "latest")]
pub use miapi as version;

/// 選んだバージョンの fixture `tests/fixtures/<バージョン>/<name>.json` を読む。
///
/// 環境変数 `MIAPI_FIXTURES` にディレクトリを指定すると、代わりにそこから読む。
/// 別のバージョンのサーバの応答を、選んだバージョンセットで読めるかを調べるときに使う
/// （`scripts/propose-pins.sh`）。
pub fn fixture(name: &str) -> String {
    let dir = match std::env::var_os("MIAPI_FIXTURES") {
        Some(dir) => PathBuf::from(dir),
        None => {
            let version = if cfg!(feature = "latest") {
                "latest".to_owned()
            } else {
                format!("v{}", version::API_VERSION)
            };
            PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures")
                .join(version)
        }
    };
    let path = dir.join(format!("{name}.json"));
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}
