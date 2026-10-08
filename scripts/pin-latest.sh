#!/usr/bin/env bash
# いまの latest を、固定版 v<version> としても残す。
#
#   scripts/pin-latest.sh
#
# specs/latest.json と tests/fixtures/latest/ を複製し、Cargo.toml の feature、テストが使う version の行、
# README の表の行を足して、バージョンセットを生成する。残した固定版は、latest が先へ進んでも変わらない。
set -euo pipefail

cd "$(dirname "$0")/.."
name=v$(jq -r .info.version specs/latest.json)
if [ -e "specs/$name.json" ]; then
    echo "$name は既に固定版になっている" >&2
    exit 1
fi
# README の表で 1 つ前の固定版の上限にする、このバージョンの直前のリリース。上流のタグの一覧から求める。
previous=$(git ls-remote --tags https://github.com/misskey-dev/misskey.git |
    grep -oE '[0-9]+\.[0-9]+\.[0-9]+$' | sort -V | grep -B1 -xF "${name#v}" | sed -n 1p)
cp specs/latest.json "specs/$name.json"
cp -r tests/fixtures/latest "tests/fixtures/$name"
sed -i "s/^latest = \[\]\$/&\n\"$name\" = []/" Cargo.toml
# 固定版にするのは、いつもいちばん新しいバージョンである。version の行の末尾に足せば、rustfmt が求める昇順のままになる。
last=$(grep -n ' as version;$' tests/common/mod.rs | tail -n 1 | cut -d: -f1)
sed -i "${last}a #[cfg(feature = \"$name\")]\npub use miapi::${name//./_} as version;" tests/common/mod.rs
sed -i "/^| \`v/s/ — |\$/ $previous |/" README.md
sed -i "s/^| \`latest\`.*\$/&\n| \`$name\` | ${name#v} | — |/" README.md
cargo codegen
echo "Misskey ${name#v} -> specs/$name.json、tests/fixtures/$name/"
