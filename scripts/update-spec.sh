#!/usr/bin/env bash
# Misskey の api.json と、misskey-js のストリーミングの型定義を取得して specs/ に保存する。
#
#   scripts/update-spec.sh --origin https://misskey.example.com [--pin] [--repo owner/name]
#   scripts/update-spec.sh --tag 2026.10.0 [--pin] [--repo owner/name]
#
# --origin は動いているサーバから /api.json をダウンロードする。
# --tag はそのリリースの公式 Docker イメージの中で api.json を生成する（docker が要る）。
# どちらの場合も、ストリーミングの型定義（packages/misskey-js/src/ の streaming.types.ts と、それが参照する
# entities.ts・consts.ts）と misskey-js の LICENSE は、GitHub の --repo（既定は misskey-dev/misskey）の、
# api.json の info.version と同じ名前のタグから原文のまま取る。
# --pin が無ければ結果で specs/latest/ を置き換え、
# --pin があれば specs/v<version>/ に書き出す。
set -euo pipefail

usage() {
    echo "使い方: $0 (--origin <url> | --tag <version>) [--pin] [--repo <owner/name>]" >&2
    exit 2
}

root=$(cd "$(dirname "$0")/.." && pwd)
origin=
tag=
pin=0
repo=misskey-dev/misskey
while [ $# -gt 0 ]; do
    case $1 in
        --origin) origin=$2; shift 2 ;;
        --tag) tag=$2; shift 2 ;;
        --pin) pin=1; shift ;;
        --repo) repo=$2; shift 2 ;;
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
# 型定義と LICENSE を全部取得できてから書き出す。途中で取得に失敗したときに、spec だけが新しくなって残らないようにする。
mkdir "$tmp/ts"
for file in streaming.types.ts entities.ts consts.ts; do
    curl -fsSL "https://raw.githubusercontent.com/$repo/$version/packages/misskey-js/src/$file" -o "$tmp/ts/$file"
done
curl -fsSL "https://raw.githubusercontent.com/$repo/$version/packages/misskey-js/LICENSE" -o "$tmp/ts/LICENSE"
name=latest
if [ "$pin" = 1 ]; then name=v$version; fi
dir=$root/specs/$name
mkdir -p "$dir/misskey-js"
# spec を更新したときの差分が読みやすくなるよう、サーバの URL を消して整形する。
jq 'del(.servers)' "$tmp/api.json" > "$dir/api.json"
cp "$tmp"/ts/* "$dir/misskey-js"/
echo "Misskey $version -> ${dir#"$root"/}/"
