#!/usr/bin/env bash
# Misskey の api.json を取得して specs/ に保存する。
#
#   scripts/update-spec.sh --origin https://misskey.example.com [--pin]
#   scripts/update-spec.sh --tag 2026.10.0 [--pin]
#
# --origin は、動いているサーバから /api.json をダウンロードする。
# --tag は、そのリリースの公式 Docker イメージの中で api.json を生成する（docker が要る）。
# --pin が無ければ結果で specs/latest/api.json を置き換え、
# --pin があれば specs/v<version>/api.json に書き出す。
set -euo pipefail

usage() {
    echo "使い方: $0 (--origin <url> | --tag <version>) [--pin]" >&2
    exit 2
}

root=$(cd "$(dirname "$0")/.." && pwd)
origin=
tag=
pin=0
while [ $# -gt 0 ]; do
    case $1 in
        --origin) origin=$2; shift 2 ;;
        --tag) tag=$2; shift 2 ;;
        --pin) pin=1; shift ;;
        *) usage ;;
    esac
done
if [ -z "$origin" ] && [ -z "$tag" ]; then
    usage
fi

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT

if [ -n "$origin" ]; then
    curl -fsSL "${origin%/}/api.json" -o "$tmp/api.json"
else
    # イメージにはビルド済みのバックエンドが入っているが、.config/ は入っていない。
    # そこで、そのリリースの設定例をマウントする。
    # スクリプトは node で直接実行する。pnpm を経由すると、古いイメージでは失敗するためである
    # （古いイメージの corepack は、npm レジストリの署名鍵を受け付けなくなっている）。
    # compile_config.js は新しいリリースにしか無い。
    mkdir -p "$tmp/.config"
    curl -fsSL "https://raw.githubusercontent.com/misskey-dev/misskey/$tag/.config/example.yml" \
        -o "$tmp/.config/default.yml"
    chmod 755 "$tmp" "$tmp/.config"
    chmod 644 "$tmp/.config/default.yml"
    docker run --rm -v "$tmp/.config:/misskey/.config:ro" "misskey/misskey:$tag" sh -c '
        cd packages/backend
        if [ -f scripts/compile_config.js ]; then node scripts/compile_config.js >&2; fi
        node scripts/generate_api_json.js --no-build >&2
        cat built/api.json
    ' > "$tmp/api.json"
fi

version=$(jq -r '.info.version' "$tmp/api.json")
if [ "$pin" = 1 ]; then
    dir="$root/specs/v$version"
else
    dir="$root/specs/latest"
fi
mkdir -p "$dir"
# spec を更新したときの差分が読みやすくなるよう、サーバの URL を消して整形する。
jq 'del(.servers)' "$tmp/api.json" > "$dir/api.json"
echo "Misskey $version -> ${dir#"$root"/}/api.json"
