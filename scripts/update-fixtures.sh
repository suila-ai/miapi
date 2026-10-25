#!/usr/bin/env bash
# 公式の Docker イメージから起動した Misskey サーバを使って、テスト用の fixture を作り直す。
#
#   scripts/update-fixtures.sh --tag 2026.10.0 [--pin]
#
# 指定したリリースを、そのリリースの Docker 用の例にある設定とデータベースのイメージで起動する。
# 続けて API でダミーのデータを入れ、テストが読む応答を保存する。
# ストリーミング API のイベントも、データを入れる間 WebSocket でつないで（scripts/capture-stream.ts）
# チャンネルごとに保存する。
# --pin が無ければ結果で tests/fixtures/latest/ を置き換え、
# --pin があれば tests/fixtures/v<version>/ に書き出す。
# ID・時刻・ファイルの URL は採取のたびに変わる。それらを除いて前回と同じ応答なら、前回のファイルを残す。
# docker・curl・jq と、Node.js 26 が要る。
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
cleanup() {
    local status=$?
    # 失敗したときは、ストリーミングの採取（scripts/capture-stream.ts）のエラー出力も表示する。
    if [ "$status" != 0 ] && [ -s "$tmp/stream.err" ]; then cat "$tmp/stream.err" >&2; fi
    "${compose[@]}" down -v > /dev/null 2>&1 || true
    rm -rf "$tmp"
}
trap cleanup EXIT

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
admin_json=$(api "" admin/accounts/create '{"username":"admin","password":"password"}')
admin=$(jq -r .token <<<"$admin_json")
admin_id=$(jq -r .id <<<"$admin_json")
alice_json=$(api "$admin" admin/accounts/create '{"username":"alice","password":"password"}')
alice=$(jq -r .token <<<"$alice_json")
alice_id=$(jq -r .id <<<"$alice_json")
bob_json=$(api "$admin" admin/accounts/create '{"username":"bob","password":"password"}')
bob=$(jq -r .token <<<"$bob_json")
bob_id=$(jq -r .id <<<"$bob_json")
# signup が `host` を受け取るのは NODE_ENV=test のときだけ。
# こうして作ったユーザーは、リモートのユーザーに見える。
# 連合はしないので、連合で取得する情報（連合先の名前やソフト、リモートのノートの `uri` など）は応答に入らない。
carol_json=$(api "" signup '{"username":"carol","password":"password","host":"remote.example.com"}')
carol=$(jq -r .token <<<"$carol_json")
carol_id=$(jq -r .id <<<"$carol_json")

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
emoji1=$(api "$admin" admin/emoji/add "$(jq -nc --arg f "$image_id" '{name: "emoji1", fileId: $f, category: "category1", aliases: ["alias1"]}')" | jq -r .id)
api "$admin" admin/emoji/add "$(jq -nc --arg f "$image_id" '{name: "emoji2", fileId: $f}')" > /dev/null

# role <name・color・iconUrl・asBadge を持つ json> [ユーザーの ID]: ロールを作り、ユーザー（省略したら alice）に付与する。
# json に無い項目は、権限の無い公開ロールの値で埋める。
role() {
    local id
    id=$(api "$admin" admin/roles/create "$(jq -c '{description: "dummy role", target: "manual", condFormula: {}, isPublic: true, isModerator: false, isAdministrator: false, canEditMembersByModerator: false, displayOrder: 0, policies: {}} + .' <<<"$1")" | jq -r .id)
    api "$admin" admin/roles/assign "$(jq -nc --arg r "$id" --arg u "${2:-$alice_id}" '{roleId: $r, userId: $u}')" > /dev/null
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

# ストリーミングのイベント。WebSocket でつなぐとユーザーの onlineStatus が変わるので、REST の fixture を保存した後につなぎ、
# ここからの操作で起きるイベントを、接続ごとに受け取って JSON 行で残す。
# つなぐ先: 未ログインの訪問者、alice、admin。チャンネルの id は "<接続の番号>-<チャンネルの番号>" になる。
# serverStats と queueStats は、NODE_ENV=test ではサーバが統計を取らずイベントが来ないので、つながない。
antenna=$(api "$alice" antennas/create '{"name":"antenna1","src":"all","keywords":[["tag1"]],"excludeKeywords":[],"users":[],"caseSensitive":false,"withReplies":false,"withFile":false,"notify":false}' | jq -r .id)
list=$(api "$alice" users/lists/create '{"name":"list1"}' | jq -r .id)
api "$alice" users/lists/push "$(jq -nc --arg l "$list" --arg u "$bob_id" '{listId: $l, userId: $u}')" > /dev/null
channel=$(api "$alice" channels/create '{"name":"channel1"}' | jq -r .id)
connections=$(jq -nc --arg alice "$alice" --arg admin "$admin" --arg antenna "$antenna" --arg list "$list" --arg channel "$channel" '[
    {token: null, channels: [{name: "localTimeline", params: {}}, {name: "globalTimeline", params: {}}, {name: "hashtag", params: {q: [["tag1"]]}}]},
    {token: $alice, channels: [{name: "main", params: {}}, {name: "drive", params: {}}, {name: "homeTimeline", params: {}}, {name: "hybridTimeline", params: {}}, {name: "antenna", params: {antennaId: $antenna}}, {name: "userList", params: {listId: $list}}, {name: "channel", params: {channelId: $channel}}]},
    {token: $admin, channels: [{name: "admin", params: {}}]}]')
mkfifo "$tmp/commands"
node "$root/scripts/capture-stream.ts" "$base" "$connections" < "$tmp/commands" > "$tmp/stream.jsonl" 2> "$tmp/stream.err" &
capture=$!
exec 3> "$tmp/commands"
for _ in $(seq 30); do
    if grep -q '"ready"' "$tmp/stream.jsonl"; then break; fi
    sleep 1
done
if ! grep -q '"ready"' "$tmp/stream.jsonl"; then
    echo "ストリーミングの接続が 30 秒以内にそろわなかった" >&2
    exit 1
fi

# send <接続の番号> <json>: その接続から json をそのまま送る。サーバに届くのを待ってから次の操作に進む。
send() {
    jq -nc --argjson c "$1" --argjson s "$2" '{connection: $c, send: $s}' >&3
    sleep 0.5
}

# イベントを起こす操作。通知は、作られてから 2 秒読まれないと unreadNotification が出るので、
# 既読にする操作を先に行い、通知を作る操作はその後にする。
api "$alice" notifications/mark-all-as-read > /dev/null
api "$admin" admin/announcements/create '{"title":"dummy announcement 2","text":"dummy text","imageUrl":null}' > /dev/null
streamed=$(note "$alice" '{"text":"dummy note 6 #tag1"}')
send 1 "$(jq -nc --arg n "$streamed" '{type: "subNote", body: {id: $n}}')"
note "$bob" "$(jq -nc --arg n "$streamed" '{text: "dummy reply 2", replyId: $n}')" > /dev/null
note "$admin" "$(jq -nc --arg n "$streamed" '{renoteId: $n}')" > /dev/null
note "$bob" '{"text":"@alice dummy mention"}' > /dev/null
api "$admin" following/create "$(jq -nc --arg u "$alice_id" '{userId: $u}')" > /dev/null
api "$alice" following/create "$(jq -nc --arg u "$bob_id" '{userId: $u}')" > /dev/null
api "$alice" following/delete "$(jq -nc --arg u "$bob_id" '{userId: $u}')" > /dev/null
api "$alice" i/update '{"description":"dummy profile 2"}' > /dev/null
api "$alice" i/registry/set '{"key":"dummy","value":{"k":1},"scope":["miapi"]}' > /dev/null
# marker を付けずに URL からアップロードすると、urlUploadFinished の marker は null になる。サーバが URL から取ってくるので、
# イベントは少し遅れて届く。取り先を上流の画像にするのは、2024.10.1 のサーバが NODE_ENV=test でも、
# プライベートアドレス（サーバ自身）からの取得を拒否するため。
api "$alice" drive/files/upload-from-url "$(jq -nc --arg u "$upstream/packages/backend/test/resources/192.jpg" '{url: $u}')" > /dev/null
# カスタム絵文字でリアクションすると、reacted の emoji に名前と URL が付く。
react "$admin" "$streamed" ':emoji1:'
api "$admin" notes/reactions/delete "$(jq -nc --arg n "$streamed" '{noteId: $n}')" > /dev/null
poll=$(note "$alice" '{"text":"dummy poll","poll":{"choices":["a","b"]}}')
send 1 "$(jq -nc --arg n "$poll" '{type: "subNote", body: {id: $n}}')"
api "$bob" notes/polls/vote "$(jq -nc --arg n "$poll" '{noteId: $n, choice: 0}')" > /dev/null
throwaway=$(note "$alice" '{"text":"dummy note to delete"}')
send 1 "$(jq -nc --arg n "$throwaway" '{type: "subNote", body: {id: $n}}')"
api "$alice" notes/delete "$(jq -nc --arg n "$throwaway" '{noteId: $n}')" > /dev/null
note "$alice" "$(jq -nc --arg c "$channel" '{text: "dummy channel note", channelId: $c}')" > /dev/null
api "$admin" admin/emoji/update "$(jq -nc --arg id "$emoji1" '{id: $id, name: "emoji1", category: "category2", aliases: ["alias1"]}')" > /dev/null
emoji3=$(api "$admin" admin/emoji/add "$(jq -nc --arg f "$image_id" '{name: "emoji3", fileId: $f}')" | jq -r .id)
api "$admin" admin/emoji/delete "$(jq -nc --arg id "$emoji3" '{id: $id}')" > /dev/null
folder=$(api "$alice" drive/folders/create '{"name":"folder1"}' | jq -r .id)
api "$alice" drive/folders/update "$(jq -nc --arg f "$folder" '{folderId: $f, name: "folder2"}')" > /dev/null
second=$(upload "$alice" "$tmp/emoji.png" second.png | jq -r .id)
api "$alice" drive/files/update "$(jq -nc --arg f "$second" '{fileId: $f, name: "renamed.png"}')" > /dev/null
api "$alice" drive/files/delete "$(jq -nc --arg f "$second" '{fileId: $f}')" > /dev/null
api "$alice" drive/folders/delete "$(jq -nc --arg f "$folder" '{folderId: $f}')" > /dev/null
# 通報はモデレーターのロールを持つユーザーの admin チャンネルに届く（root のユーザーでもロールが要る）。
role '{"name":"moderator","color":null,"iconUrl":null,"asBadge":false,"isPublic":false,"isModerator":true}' "$admin_id"
api "$bob" users/report-abuse "$(jq -nc --arg u "$carol_id" '{userId: $u, comment: "dummy report"}')" > /dev/null
# urlUploadFinished は、サーバが URL から取り終えてから届く。標準入力を閉じた後は capture-stream.ts が 3 秒しか待たないので、
# 届くまで待ってから閉じる。
for _ in $(seq 30); do
    if grep -q '"urlUploadFinished"' "$tmp/stream.jsonl"; then break; fi
    sleep 1
done
if ! grep -q '"urlUploadFinished"' "$tmp/stream.jsonl"; then
    echo "urlUploadFinished が 30 秒以内に届かなかった" >&2
    exit 1
fi
exec 3>&-
if ! wait "$capture"; then
    echo "ストリーミングの採取に失敗した" >&2
    exit 1
fi
# チャンネルごとに、type ごとの最初の 1 件を配列にして保存する。noteUpdated は本文の type（reacted・pollVoted など）でも分ける。
# チャンネルに属さないイベントは broadcast にする。
for channel in $(jq -r 'select(has("message")) | .channel // "broadcast"' "$tmp/stream.jsonl" | sort -u); do
    file=stream-$(sed -E 's/([A-Z])/-\L\1/g' <<<"$channel")
    jq -s --arg channel "$channel" 'map(select(has("message") and (.channel // "broadcast") == $channel)) | unique_by([.message.type, if .message.type == "noteUpdated" then .message.body.type else null end]) | map(.message)' \
        "$tmp/stream.jsonl" > "$new/$file.json"
done

# ID・時刻・ファイルの URL に入る UUID は、サーバがそのときの時刻と乱数から作るので、同じ手順でも採取のたびに変わる。
# それらを除いて前回と同じ応答なら、前回のファイルを残す。残さないと、応答が変わっていない fixture まで更新のたびに差分になる。
# 除くのは、項目名と値の形の両方が合うものに限る。値の形だけで決めると、同じ形の自由記述まで除いてしまう。
# ストリーミングのイベントには、本文（body）が ID そのものになるもの（drive の fileDeleted など）と、ID の配列の mentions を持つノートがある。
stable='
def id: if type == "string" and test("^[0-9a-z]{16}$") then "id" else . end;
def time: if type == "string" and test("^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9:.]+Z$") then "time" else . end;
def url: if type == "string" then gsub("[0-9a-f]{8}(-[0-9a-f]{4}){3}-[0-9a-f]{12}"; "uuid") else . end;
walk(if type != "object" then . else with_entries(
    if .key | test("^id$|Id$|^body$") then .value |= id
    elif .key | test("Ids$|^mentions$") then .value |= (if type == "array" then map(id) else . end)
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
