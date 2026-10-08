#!/usr/bin/env bash
# 公式の Docker イメージから起動した Misskey サーバを使って、テスト用の fixture を作り直す。
#
#   scripts/update-fixtures.sh --tag 2026.10.0 [--pin]
#
# 指定したリリースを、そのリリースの Docker 用の例にある設定とデータベースのイメージで起動する。
# 続けて API でダミーのデータを入れ、テストが読む応答を保存する。
# --pin が無ければ結果で tests/fixtures/latest/ を置き換え、
# --pin があれば tests/fixtures/v<version>/ に書き出す。
# ID・時刻・ファイルの URL は採取のたびに変わる。それらを除いて前回と同じ応答は、前回のファイルを残す。
# docker・curl・jq が要る。
set -euo pipefail

usage() {
    echo "使い方: $0 --tag <version> [--pin]" >&2
    exit 2
}

root=$(cd "$(dirname "$0")/.." && pwd)
tag=
pin=0
while [ $# -gt 0 ]; do
    case $1 in
        --tag) tag=$2; shift 2 ;;
        --pin) pin=1; shift ;;
        *) usage ;;
    esac
done
if [ -z "$tag" ]; then
    usage
fi

tmp=$(mktemp -d)
compose=(docker compose -p miapi-fixtures -f "$root/scripts/fixture-server/compose.yml")
trap '"${compose[@]}" down -v > /dev/null 2>&1 || true; rm -rf "$tmp"' EXIT

upstream=https://raw.githubusercontent.com/misskey-dev/misskey/$tag
mkdir "$tmp/.config"
curl -fsSL "$upstream/.config/docker_example.yml" \
    | sed -E 's#^url: .*#url: https://misskey.example.com/#' > "$tmp/.config/default.yml"
# サーバは NODE_ENV=test のとき test.yml を読む。イメージのヘルスチェックは default.yml を読む。
cp "$tmp/.config/default.yml" "$tmp/.config/test.yml"
curl -fsSL "$upstream/.config/docker_example.env" -o "$tmp/.config/docker.env"
curl -fsSL "$upstream/compose_example.yml" -o "$tmp/compose_example.yml"
curl -fsSL "$upstream/packages/backend/test/resources/192.jpg" -o "$tmp/photo.jpg"
curl -fsSL "$upstream/packages/backend/test/resources/192.png" -o "$tmp/emoji.png"
# コンテナは別のユーザーで動く。
chmod 755 "$tmp" "$tmp/.config"
chmod 644 "$tmp"/.config/*

export MISSKEY_TAG=$tag
export CONFIG_DIR=$tmp/.config
POSTGRES_IMAGE=$(grep -oE 'image: postgres:[^ ]+' "$tmp/compose_example.yml" | cut -d' ' -f2)
REDIS_IMAGE=$(grep -oE 'image: redis:[^ ]+' "$tmp/compose_example.yml" | cut -d' ' -f2)
export POSTGRES_IMAGE REDIS_IMAGE
"${compose[@]}" up -d --quiet-pull
base=http://$("${compose[@]}" port web 3000)

# api <token|""> <endpoint> [json]: 応答のボディを出力する。エラー応答なら失敗する。
api() {
    local token=$1 endpoint=$2 body=${3:-'{}'}
    if [ -n "$token" ]; then body=$(jq -c --arg i "$token" '. + {i: $i}' <<<"$body"); fi
    curl --fail-with-body -sS -m 60 -H 'Content-Type: application/json' -d "$body" "$base/api/$endpoint"
}

# upload <token> <file> <name>: 作成された DriveFile を出力する。
upload() {
    curl --fail-with-body -sS -m 60 -F "i=$1" -F "name=$3" -F "force=true" -F "file=@$2" \
        "$base/api/drive/files/create"
}

# note <token> <json>: 作成されたノートの id を出力する。
note() {
    api "$1" notes/create "$2" | jq -r .createdNote.id
}

# react <token> <note id> <reaction>
react() {
    api "$1" notes/reactions/create "$(jq -nc --arg n "$2" --arg r "$3" '{noteId: $n, reaction: $r}')" > /dev/null
}

echo "Misskey $tag の起動を待っている" >&2
for _ in $(seq 150); do
    if api "" meta > /dev/null 2>&1; then break; fi
    sleep 2
done
api "" meta > /dev/null

# ユーザー。最初のアカウントは認証なしで作れる。残りは管理者が作る。
admin=$(api "" admin/accounts/create '{"username":"admin","password":"password"}' | jq -r .token)
alice_json=$(api "$admin" admin/accounts/create '{"username":"alice","password":"password"}')
alice=$(jq -r .token <<<"$alice_json")
alice_id=$(jq -r .id <<<"$alice_json")
bob=$(api "$admin" admin/accounts/create '{"username":"bob","password":"password"}' | jq -r .token)
# signup が `host` を受け取るのは NODE_ENV=test のときだけ。
# こうして作ったユーザーは、リモートのユーザーに見える。
# 連合はしないので、連合で取得する情報（連合先の名前やソフト、リモートのノートの `uri` など）は応答に入らない。
carol=$(api "" signup '{"username":"carol","password":"password","host":"remote.example.com"}' | jq -r .token)

# サーバの設定。ベースロールのポリシーとクライアントのオプションは、
# Web クライアントの初期設定ウィザードが「お一人様サーバー」向けに保存する内容にしている。
# 多くのサーバはこの状態にあり、応答がスキーマと食い違う。
# レートリミットの係数は小数になり、meta.features には localTimeline と globalTimeline が無く、
# meta.clientOptions には 3 つあるフィールドのうち 1 つしか無い
# （codegen/src/corrections.rs を参照）。
settings='{"name":"Example Misskey","shortName":"Example","description":"A server with dummy data.","maintainerName":"admin","maintainerEmail":"admin@example.com","tosUrl":"https://misskey.example.com/tos","serverRules":["dummy rule"],"themeColor":"#86b300"}'
if api "" meta '{"detail":true}' | jq -e 'has("clientOptions")' > /dev/null; then
    settings=$(jq -c '. + {clientOptions: {entrancePageStyle: "simple"}}' <<<"$settings")
fi
api "$admin" admin/update-meta "$settings" > /dev/null
api "$admin" admin/roles/update-default-policies '{"policies":{"rateLimitFactor":0.3,"driveCapacityMb":8192,"userListLimit":100,"antennaLimit":100,"webhookLimit":100,"canImportFollowing":true,"canImportMuting":true,"canImportBlocking":true,"canImportUserLists":true,"canImportAntennas":true}}' > /dev/null
# まだ誰も読んでいないお知らせ。`i` の応答に載る。
api "$admin" admin/announcements/create '{"title":"dummy announcement","text":"dummy text","imageUrl":null}' > /dev/null

# ファイル、カスタム絵文字、ロール、アバターデコレーション。
photo=$(upload "$alice" "$tmp/photo.jpg" photo.jpg | jq -r .id)
image=$(upload "$admin" "$tmp/emoji.png" emoji.png)
image_id=$(jq -r .id <<<"$image")
image_url=$(jq -r .url <<<"$image")
api "$admin" admin/emoji/add "$(jq -nc --arg f "$image_id" '{name: "emoji1", fileId: $f, category: "category1", aliases: ["alias1"]}')" > /dev/null
api "$admin" admin/emoji/add "$(jq -nc --arg f "$image_id" '{name: "emoji2", fileId: $f}')" > /dev/null

# role <name・color・iconUrl・asBadge を持つ json>: 公開ロールを作り、alice に付与する。
role() {
    local id
    id=$(api "$admin" admin/roles/create "$(jq -c '. + {description: "dummy role", target: "manual", condFormula: {}, isPublic: true, isModerator: false, isAdministrator: false, canEditMembersByModerator: false, displayOrder: 0, policies: {}}' <<<"$1")" | jq -r .id)
    api "$admin" admin/roles/assign "$(jq -nc --arg r "$id" --arg u "$alice_id" '{roleId: $r, userId: $u}')" > /dev/null
}
role "$(jq -nc --arg u "$image_url" '{name: "role1", color: "#ff0000", iconUrl: $u, asBadge: true}')"
role '{"name":"role2","color":null,"iconUrl":null,"asBadge":false}'

api "$admin" admin/avatar-decorations/create "$(jq -nc --arg u "$image_url" '{name: "decoration1", description: "dummy decoration", url: $u, roleIdsThatCanBeUsedThisDecoration: []}')" > /dev/null
decoration=$(api "$admin" admin/avatar-decorations/list | jq -r '.[0].id')

# プロフィールとフォロー。
api "$alice" i/update "$(jq -nc --arg a "$photo" --arg d "$decoration" '{name: "Alice", description: "dummy profile", birthday: "2000-01-01", isCat: true, avatarId: $a, bannerId: $a, fields: [{name: "field1", value: "value1"}], avatarDecorations: [{id: $d, offsetX: 0.1}]}')" > /dev/null
api "$bob" i/update '{"followersVisibility":"private","followingVisibility":"private"}' > /dev/null
api "$bob" following/create "$(jq -nc --arg u "$alice_id" '{userId: $u}')" > /dev/null

# ノート、リアクション、ピン留め。
tagged=$(note "$alice" '{"text":"dummy note 1 #tag1 #tag2"}')
with_file=$(note "$alice" "$(jq -nc --arg f "$photo" '{text: "dummy note 2", fileIds: [$f]}')")
note "$admin" "$(jq -nc --arg n "$tagged" '{renoteId: $n}')" > /dev/null
note "$bob" "$(jq -nc --arg n "$tagged" '{text: "dummy reply", replyId: $n, cw: "cw"}')" > /dev/null
note "$bob" '{"text":"dummy note 5 #tag1","localOnly":true,"reactionAcceptance":"nonSensitiveOnly"}' > /dev/null
remote=$(note "$carol" '{"text":"dummy remote note #tag3"}')
renote_of_remote=$(note "$admin" "$(jq -nc --arg n "$remote" '{renoteId: $n}')")
react "$admin" "$tagged" '👍'
react "$bob" "$tagged" ':emoji1:'
react "$admin" "$with_file" ':emoji2:'
api "$alice" i/pin "$(jq -nc --arg n "$tagged" '{noteId: $n}')" > /dev/null

# 設定は Redis のメッセージを介して meta エンドポイントに反映されるので、反映を待つ。
configured() {
    api "" meta '{"detail":true}' | jq -e '.name == "Example Misskey" and .policies.rateLimitFactor == 0.3' > /dev/null
}
for _ in $(seq 30); do
    if configured; then break; fi
    sleep 1
done
if ! configured; then
    echo "サーバの設定が meta に反映されなかった" >&2
    exit 1
fi

version=$(api "" meta | jq -r .version)
name=latest
if [ "$pin" = 1 ]; then name=v$version; fi
out=$root/tests/fixtures/$name
new=$tmp/fixtures
mkdir -p "$out" "$new"

# save <fixture> <endpoint> [json]: 未ログインの訪問者が受け取る応答を、整形して保存する。
save() {
    api "" "$2" "${3:-}" | jq . > "$new/$1.json"
}
save meta meta '{"detail":true}'
save notes-local-timeline notes/local-timeline '{"limit":10}'
save notes-show notes/show "$(jq -nc --arg n "$renote_of_remote" '{noteId: $n}')"
save users users '{"limit":10}'
save users-show users/show "$(jq -nc --arg u "$alice_id" '{userId: $u}')"
save federation-instances federation/instances '{"limit":10}'
save emojis emojis
save hashtags-trend hashtags/trend
# ユーザー自身のアカウントの情報。本人だけが取得できる。
api "$alice" i | jq . > "$new/i.json"
api "" notes/show "$(jq -nc --arg n "$with_file" '{noteId: $n}')" | jq '.files[0]' > "$new/drive-file.json"
# エラー応答そのものが fixture なので、エラーでも失敗にしない。
curl -sS -m 60 -H 'Content-Type: application/json' -d '{"noteId":"nonexistent"}' "$base/api/notes/show" \
    | jq . > "$new/error-no-such-note.json"

# ID・時刻・ファイルの URL に入る UUID は、サーバがそのときの時刻と乱数から作るので、同じ手順でも採取のたびに変わる。
# それらを除いて前回と同じ応答は、前回のファイルを残す。残さないと、応答が変わっていない fixture まで更新のたびに差分になる。
# 除くのは、項目名と値の形の両方が合うものに限る。値の形だけで決めると、同じ形の自由記述まで除いてしまう。
stable='
def id: if type == "string" and test("^[0-9a-z]{16}$") then "id" else . end;
def time: if type == "string" and test("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:.]+Z$") then "time" else . end;
def url: if type == "string" then gsub("[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}"; "uuid") else . end;
walk(if type != "object" then . else with_entries(
    if .key | test("^id$|Id$") then .value |= id
    elif .key | test("Ids$") then .value |= (if type == "array" then map(id) else . end)
    elif .key | test("At$") then .value |= time
    elif .key | test("^url$|Url$") then .value |= url
    else . end) end)'
for file in "$new"/*.json; do
    previous=$out/${file##*/}
    if [ -f "$previous" ] && cmp -s <(jq "$stable" "$previous") <(jq "$stable" "$file"); then
        cp "$previous" "$file"
    fi
done
rm -f "$out"/*.json
cp "$new"/*.json "$out"/
echo "Misskey $version -> tests/fixtures/$name/"
