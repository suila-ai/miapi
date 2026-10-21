#!/usr/bin/env bash
# latest を更新した後、コミットする前に、固定版との互換性を調べる。
# 結果は、更新の PR に載せる箇条書きとして標準出力に出す。
#
#   scripts/propose-pins.sh
#
# 直前の latest の応答には、HEAD の tests/fixtures/latest/ を使う。
# ここで「読める」とは、その応答を fixture にして tests/deserialize.rs が通ることをいう。
# 新しい latest のバージョンセットが直前の latest の応答を読めなければ、新しい latest を固定版としても残す
# （scripts/pin-latest.sh）。既にある固定版のバージョンセットが新しい latest の応答を読めなければ、その旨を出力する。
set -euo pipefail

cd "$(dirname "$0")/.."
version=$(jq -r .info.version specs/latest/api.json)
previous=$(mktemp -d)
trap 'rm -rf "$previous"' EXIT
git archive HEAD:tests/fixtures/latest | tar -x -C "$previous"

# unreadable <feature> <fixture のディレクトリ>: その feature のバージョンセットで通らなかったテストの名前を出力する。
# すべて通れば何も出力しない。
unreadable() {
    local log
    if log=$(MIAPI_FIXTURES=$2 cargo test --no-default-features --features "$1,streaming" --test deserialize 2>&1); then
        return
    fi
    echo "$log" >&2
    # ビルドの失敗など、テストの失敗でないものは互換性の結果として扱わず、ここで止める。
    grep -oE '^test [a-z0-9_:]+ \.\.\. FAILED$' <<<"$log" | cut -d' ' -f2 | paste -sd' ' | grep .
}

failed=$(unreadable latest "$previous")
if [ -z "$failed" ]; then
    echo "- 新しいバージョンセットは直前の latest の応答も読めるので、固定版は足していない"
elif [ -e "specs/v$version" ]; then
    echo "- 新しいバージョンセットは直前の latest の応答を読めない（$failed）。v$version は既に固定版になっている"
else
    scripts/pin-latest.sh >&2
    echo "- 新しいバージョンセットは直前の latest の応答を読めない（$failed）ので、$version を固定版 v$version としても残した"
fi

shopt -s nullglob
for dir in specs/v*/; do
    name=$(basename "$dir")
    if [ "$name" = "v$version" ]; then continue; fi
    failed=$(unreadable "$name" tests/fixtures/latest)
    if [ -n "$failed" ]; then
        echo "- 固定版 $name のバージョンセットは $version の応答を読めない（$failed）"
    fi
done
