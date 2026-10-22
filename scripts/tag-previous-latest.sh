#!/usr/bin/env bash
# main への push で latest の生成元のバージョンが上がったときに、それまでの latest のバージョンを指すタグ
# misskey-<version> を、push の直前のコミットに打って push する。
#
#   scripts/tag-previous-latest.sh <push の直前のコミット>
set -euo pipefail

if [ $# -ne 1 ]; then
    echo "使い方: $0 <push の直前のコミット>" >&2
    exit 2
fi
cd "$(dirname "$0")/.."
previous=$(git show "$1:specs/latest/api.json" | jq -r .info.version)
current=$(jq -r .info.version specs/latest/api.json)
# バージョンが上がったときだけ打つ。更新を取り消す push（バージョンが下がる）では、取り消されるバージョンをタグに残さない。
if [ "$(printf '%s\n' "$previous" "$current" | sort -V | head -n 1)" = "$current" ]; then
    echo "latest のバージョンは上がっていない（$previous → $current）"
    exit 0
fi
tag=misskey-$previous
if git ls-remote --exit-code --tags origin "refs/tags/$tag" > /dev/null; then
    echo "$tag は既にある"
    exit 0
fi
git tag "$tag" "$1"
git push origin "$tag"
