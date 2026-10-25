# miapi

Misskey の API を呼ぶ Rust ライブラリです。エンドポイントと型は、サーバが公開する `api.json`（OpenAPI 3.1）から生成します。Misskey のバージョンは cargo feature で選び、そのバージョンのサーバが返す形をそのまま型で表します。バージョンの違うサーバを同時に相手にするときは、feature を複数有効にします。

```rust
use miapi::Client;

let client = Client::new("https://misskey.example.com", Some("access token"))?;
let note = client.notes().show().note_id("xxxxxxxxxx").await?;      // types::Note
let created = client.notes().create().text("hello").await?;          // api::notes::create::Response
let timeline = client.notes().local_timeline().limit(20).await?;     // Vec<types::Note>
client.i().update().name("new name").avatar_id_null().await?;        // null で解除する項目は {field}_null()
client.notes().create().text("hi").maybe_reply_id(reply_id).await?;  // Option を渡すなら maybe_{field}()。Some のときだけ送る
```

## 使い方

当面は git 経由で配布します。

```toml
[dependencies]
miapi = { git = "https://github.com/suila-ai/miapi" }                                                                  # latest（最新のリリース）
miapi = { git = "https://github.com/suila-ai/miapi", features = ["v2024.10.1"] }                                        # latest と固定版
miapi = { git = "https://github.com/suila-ai/miapi", default-features = false, features = ["v2024.10.1", "streaming"] } # 固定版だけ
miapi = { git = "https://github.com/suila-ai/miapi", default-features = false, features = ["latest"] }                  # ストリーミング API なし
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

固定版は feature と同じ名前のモジュール（`.` は `_`）から使います。

```rust
let old = miapi::v2024_10_1::Client::new("https://old.example.com", Some("access token"))?;
let notes = old.notes().local_timeline().limit(20).await?;           // Vec<miapi::v2024_10_1::types::Note>
```

- エンドポイントのパスがそのままメソッドの連鎖になります。`/notes/create` は `client.notes().create()`、`/i/2fa/done` は `client.i()._2fa().done()`、`/i/move` は `client.i().r#move()` です。`/notes` のように名前空間でもあるエンドポイントは、`client.notes()` 自体がそのビルダーです（`client.notes().limit(10).await?`）。
- 必須パラメータは型状態で表します。`client.notes().show()` は `note_id` を設定するまで `.await` できず、コンパイルエラーになります。任意パラメータは setter で設定し、`Option` を持っているなら `maybe_{field}()` に渡すと `Some` のときだけ送ります。nullable な項目には `{field}_null()` があり、省略と `null` を区別して送ります。
- 各エンドポイントには `Request` 構造体もあり、`client.call(request).await?` で送れます（設定ファイルから組み立てる、共通処理で包む等）。生成されていないエンドポイントは `client.call_raw("path", json!({...})).await?` で呼べます。
- レスポンスの型は `miapi::types`（エンティティ）と各エンドポイントの `Response` です。生成元の `api.json` に無い enum 値や union の形は `Unrecognized` に入り、エラーになりません。日時（`format: date-time`）は `chrono::DateTime<Utc>` です。
- エラーは `miapi::Error` です。サーバの `{"error": {...}}` は `Error::Api(ApiError)` に `code`・`id`・`message`・HTTP ステータス付きで入ります。応答を受け取った後のエラー（`Api`・`Decode`・`UnexpectedResponse`）には、呼んだエンドポイント名も入ります。
- `Client` はバージョンセットごとに別の型で、そのバージョンにあるエンドポイントのビルダーだけが出ます。クレートの直下の `Client`・`api`・`types` が latest のもので、固定版のものは `miapi::v2024_10_1` のように、バージョンのモジュールにあります。
- `miapi::API_VERSION`（固定版は `miapi::v2024_10_1::API_VERSION`）で、生成元のバージョンが分かります。

認証はアクセストークンをボディの `i` で送ります（misskey-js と同じ）。

### ストリーミング API

```rust
use miapi::streaming;

let stream = client.streaming().await?;                                   // WebSocket を 1 本つなぐ。切れたら自動でつなぎ直す
let mut timeline = stream.local_timeline().with_renotes(false).await?;    // チャンネルにつなぐ。params は setter
while let Some(event) = timeline.recv().await {
    if let streaming::local_timeline::Event::Note(note) = event? { println!("{}", note.id); }
}
let mut chat = stream.chat_user().other_id("xxxxxxxxxx").await?;
chat.send(streaming::chat_user::Message::Read(streaming::chat_user::Read { id: "yyyyyyyyyy".into() }))?;  // チャンネルへ送る
let mut events = stream.broadcast();                                       // noteUpdated・emojiAdded など → streaming::BroadcastEvent
stream.subscribe_note("xxxxxxxxxx")?;                                     // 以後、このノートの noteUpdated が届く
```

- チャンネルは misskey-js の `streaming.types.ts` から生成してあり、`Stream` のメソッドになります（`localTimeline` は `stream.local_timeline()`）。params は setter で設定し、必須の params（`userList` の `list_id` など）を設定するまで `.await` できません。await すると `connect` を送り、サーバが受け付けたという応答を 5 秒まで待ちます。来なければ `Error::ChannelUnavailable` です。サーバは受け付けない接続（権限の無いチャンネル、存在しないリストなど）に応答しません。どんな接続を受け付けないかは、`Error::ChannelUnavailable` の doc にあります。
- 届くイベントは各チャンネルの `Event` で、知らない `type` は `Unrecognized` に入ります。`recv()` はデコードできないイベントを `Err` で 1 件返して続きます。送るメッセージのあるチャンネル（`serverStats` の `requestLog`、`chatUser` の `read` など）には `Message` があり、`send()` で送ります。
- `broadcast()` はストリーム全体のイベント（`noteUpdated`・`emojiAdded`・`announcementCreated` など）の受け口で、`subscribe_note()` したノートの `noteUpdated` もここに届きます。
- 切れたらつなぎ直して、つないであったチャンネルと購読中のノートを登録し直します。切れていた間のイベントは届かないので、つなぎ直した後に REST で取り直してください。つなぎ直したことを知る方法は、`Stream::state` の doc にあります。`stream.close()` で閉じます。
- `with_http_client` に渡す `reqwest::Client` は `http1_only()` で作ってください。WebSocket の upgrade は HTTP/1.1 でしか成立しません。`Client::new` が作るクライアントも、feature `streaming` が有効なら HTTP/1.1 だけを使います。feature `streaming` を切ると REST だけになり、HTTP/2 も使います。

## 対応バージョン

feature ごとに、Misskey の 1 バージョン分のエンドポイントと型の一式（バージョンセット）があります。

| feature | サーバの下限 | サーバの上限 |
|---|---|---|
| `latest`（既定） | 最新のリリース | — |
| `v2026.5.1` | 2026.5.1 | — |
| `v2025.10.0` | 2025.10.0 | 2026.5.0 |
| `v2024.10.1` | 2024.10.1 | 2025.9.0 |

feature は、相手にするサーバのバージョンが下限から上限までに入るものを選びます。最新のリリースのサーバは `latest` と一番新しい固定版の両方に入ります。最新のリリースで増えた API を使うなら `latest` を、リリースのたびに API が変わるのを避けるなら固定版を選びます。複数のバージョンのサーバを相手にするなら、サーバごとに合う feature を有効にし、それぞれのバージョンの `Client` でつなぎます。1 つのバージョンセットで済ませるなら、最も古いサーバに合わせます。

- 上限より新しいサーバの応答も読めます（知らないフィールドは無視します）。ただし、後のリリースで削除されたエンドポイントと、必須パラメータが増えたエンドポイントは呼べません。`call_raw` を使ってください（例: 2025.4.0 で削除された `i/read-all-unread-notes`、2026.10.0 で `auth` と `publickey` が必須になった `sw/unregister`）。
- 下限より古いサーバの応答は読めないことがあります。Misskey は年に数回、必須フィールド（ロールのポリシーなど）を足します。それより古いサーバでは、`i` の deserialize に失敗し、`meta` は `Unrecognized` に入ります。

feature を有効にした数だけ、コンパイルは長くなります。

固定版は、新しい latest が直前の latest の応答を読めなくなる更新のときに、その新しいバージョンを残して増やします。latest のバージョンが上がるたびに、それまでの latest には `misskey-<バージョン>` のタグが付きます。タグが指すのは、そのバージョンが latest だった最後のコミットです。過去の latest のバージョンセットそのものが要るときは、このタグを指定してください（`tag = "misskey-2026.10.0"`）。

## 開発

latest の更新は、GitHub Actions（`update-spec.yml`）が週次で実行して PR を作ります。手で行うときも、同じ順に実行します（docker・curl・jq と、Node.js 26 で `npm ci` した typescript が要ります）。

```sh
scripts/update-spec.sh --tag 2026.10.0
scripts/update-fixtures.sh --tag 2026.10.0
cargo codegen
scripts/propose-pins.sh        # コミットの前に実行する
```

- 固定版を足すときは、`update-spec.sh` と `update-fixtures.sh` に `--pin` を付け、`Cargo.toml` の feature、`tests/common/mod.rs` の `version` の行、「対応バージョン」の表の行を足してから `cargo codegen` を実行します（表は 1 つ古い行の上限も直します）。いまの latest を固定版にするだけなら `scripts/pin-latest.sh` で済みます。
- 固定版は 2024.10.0 以降から選びます。それより前のバージョンセットは 2024.10.0 以降のサーバで `users/followers` などを読めません（他人の二要素認証の状態が返らなくなったためです）。
- Misskey のフォークに合わせるときは、このリポジトリを fork し、`scripts/update-spec.sh --origin https://fork.example.com --pin --repo owner/name` で spec を取って固定版を作り、git 依存で使ってください（`--repo` はストリーミングの型定義を取る GitHub のリポジトリで、`api.json` のバージョンと同じ名前のタグが要ります）。fixture は `scripts/fixture-server/compose.yml` のイメージ名と `scripts/update-fixtures.sh` の取得元をフォークのものに変えて作ります。
