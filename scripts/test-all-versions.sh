#!/usr/bin/env bash
# codegen の clippy とテストを実行する。
# 続けて specs/ の spec ごとに、同じ名前の cargo feature と streaming を選んで、miapi の clippy とテストを 1 回ずつ実行する。
# 最後に、streaming を外した構成を clippy に掛け、すべての feature を同時に有効にしてもコンパイルできることを確かめる。
set -euo pipefail

cd "$(dirname "$0")/.."
# パッケージは 1 つずつ選ぶ。--workspace でまとめると、codegen が有効にする serde_json の preserve_order が
# miapi のビルドにも入り、利用者が使うのとは違う構成をテストすることになる。
cargo clippy -p miapi-codegen --all-targets -- -D warnings
cargo test -p miapi-codegen
for dir in specs/*/; do
    name=$(basename "$dir")
    echo "== $name"
    cargo clippy -p miapi --all-targets --no-default-features --features "$name,streaming" -- -D warnings
    cargo test -p miapi --no-default-features --features "$name,streaming"
done
echo "== streaming なし"
cargo clippy -p miapi --no-default-features --features latest -- -D warnings
echo "== すべての feature"
cargo check -p miapi --all-features
