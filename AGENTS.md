# AGENTS.md

このリポジトリで開発するときの案内。利用者向けの説明と、spec・fixture の更新手順は README.md が正で、ここでは繰り返さない。

## コマンド

| 目的 | コマンド |
|---|---|
| 生成物を作り直す | `cargo codegen`（`cargo run -p miapi-codegen` の別名。`specs/<name>/` ごとに `src/generated/<name>/` を書き、rustfmt を当てる。Node.js 26 と、`npm ci` で入れる typescript が要る） |
| TypeScript のスクリプトを検査する | `npm run check`（`tsc --noEmit`） |
| CI と同じ検査 | `.github/workflows/ci.yml` の `run:` を上から順に実行する |
| バージョンセットを 1 つ選んで検査 | `cargo clippy -p miapi --all-targets --no-default-features --features v2024.10.1,streaming -- -D warnings`、`cargo test -p miapi --no-default-features --features v2024.10.1,streaming` |

- パッケージは `-p` で 1 つずつ選ぶ。`--workspace` にすると、codegen が有効にする serde_json の `preserve_order` が miapi のビルドにも入り、利用者が使うのとは違う構成を検査することになる。
- `src/` に単体テストは無く、テストは `tests/` の結合テストと doctest。`src/` に足すなら `Cargo.toml` の `test = false` を戻す（理由はそこのコメントにある）。
- テストは、バージョンセットの feature を 1 つだけ有効にして実行する（2 つ以上だと `tests/common/mod.rs` の `version` が重複する）。
- 全 feature を検査する `scripts/test-all-versions.sh` は、生成コードが大きいので時間がかかる。開発中はバージョンセットを 1 つ選んで検査する。
- spec と fixture の更新には docker・curl・jq が要る。手順は README の「開発」。

## 構成

### 生成器（`codegen/src/`）

`corrections.rs` の補正の根拠は、サーバ側の実装を引いたコメントに書く。

`resolve.rs` の決めごとは次のとおり。

- 名前の衝突は自動で改名せずエラーにする（例外は union のバリアントの連番と `ResponseBody`）。
- null 可の判定は、`nullable: true`、`type` の配列に `"null"`、enum に null、oneOf / anyOf / allOf のメンバーに `{"type": "null"}` のいずれか。
- 省略と null を区別するのは Request だけ。必須で null 可なら `Option<T>`、任意で null 可なら `Option<Option<T>>`（`double_option`）。応答の型では、欠けていても null でも `None`。
- Request のフィールドは、`required` に入っていても `default` があれば任意にする（サーバの ajv は `required` を検査する前に既定値を入れる）。
- 文字列の enum は、値が 2 つ以上なら `Unrecognized(String)` を付ける。ただし union の判別に使う必須の enum には付けない（untagged の union で最初のバリアントが何でも受けてしまうため）。
- oneOf / anyOf は、null 以外のメンバーが 2 つ以上なら untagged の enum にして末尾に `Unrecognized(Value)` を置く。union を指すメンバーはその中身に展開する。バリアントの並びは必須プロパティ数の降順。
- 自分自身に戻る参照を持つフィールドは `Box` で包む。
- `string` + `format: binary` は `FilePart` にし、`into_body` は multipart を作る。
- `string` + `format: date-time` は `chrono::DateTime<Utc>` にする。
- 応答が null 可か 204 も返すエンドポイントは `type Response = Option<ResponseBody>`、200 の JSON が無ければ `()`。

ストリーミングの型は、misskey-js の `streaming.types.ts`（と、それが参照する `entities.ts`・`consts.ts`）から作る。`codegen/ts-types.ts` が TypeScript の compiler API で構文木を JSON にし（名前の解決や型の意味づけはしない）、`streaming.rs` がそれを解決する。`ts-types.ts` で compiler API に触るのは `parseSourceFile` と、`SyntaxKind`・ノードの判定関数（`is〜`）・ノードの型だけで、走査はノードのプロパティ名で行う（typescript 7 の `typescript/sync` と `typescript/ast` へ差し替えやすくするため）。`streaming.rs` の決めごとは次のとおり。

- 名前は、そのファイルの宣言 → import 元 → `export *` の再輸出先の順に引く。`./autogen/models.js` に解決した名前は `api.json` のコンポーネント（無ければエラー）。それ以外のモジュール（`./autogen/types.js`、npm パッケージ）に解決したらエラー。
- `Channels` の各メンバーが 1 チャンネル。`params` は null か object、`events` は名前 → 型、`receives` は null か object。チャンネルのモジュールに `Params`（`stream::Channel` を実装）・`Event`・`Message`（`receives` があるときだけ）・`Connect`・`Connection` を生成する。`BroadcastEvents` は `streaming/mod.rs` の `BroadcastEvent`。
- `events` の値が関数型なら第 1 引数が本文（`() => void` は本文なし）、関数型でなければその型が本文。本文の型が null、または null と `Record<string, never>` だけの union なら、本文なし（`body` は null）。
- `string` → `String`、`number` → `f64`、`boolean` → `bool`、`any` / `unknown` → `Value`、`T[]` → `Vec`、`Record<string, T>` → `HashMap<String, T>`（`T` が `never` / `unknown` / `any` なら `serde_json::Map`）。文字列リテラル 1 つは値 1 つの StrEnum（受け皿なし）、真偽・数値リテラルは `bool` / `f64`。
- object literal は構造体。`events` の本文は Entity、`params` と `receives` は Request として扱う。`?` は任意。メンバーと同じ行の `//` コメントを doc にする。
- union は null を除き、残りが全部文字列リテラルなら StrEnum（2 つ以上なら `Unrecognized(String)` 付き）、1 つならその型、全部 object literal で必須の `type` が互いに違う文字列リテラルなら TaggedUnion（`type` で変種を選び、残りのフィールドの構造体に読む。末尾に `Unrecognized(Value)`）、それ以外は REST と同じ untagged の union。
- intersection は object 同士ならプロパティを併合（同名で型が違えばエラー）、object と union なら union の各メンバーに分配する。
- 添字アクセスは、添字が文字列リテラル 1 つならそのプロパティの型（X がコンポーネントなら REST で生成したそのフィールドの型。union のコンポーネントは、展開した全メンバーで同じ型でなければエラー）、`typeof arr[number]` なら定数配列の値の StrEnum、それ以外（型パラメータ、リテラルの union、`keyof`）なら `Value`。
- generic な alias は型引数を置き換えて展開し、使った側のモジュールにその名前で生成する。関数型の `<K extends C>` は `K` を `C` に置き換える。
- generic でない名前付きの型（`Channels` と `BroadcastEvents` 自体は除く）は、到達したものだけを `streaming/types.rs` に 1 回生成する。無名の型は、イベント名・メッセージ名から作った名前で、使った側のモジュールに生成する。名前の衝突は改名せずエラーにする（REST と同じ）。
- mapped type・条件型などの未対応の構文は、到達したときだけエラーにする。
- `{type, body}` の enum（`Event`・`Message`・`BroadcastEvent`）の serde は derive でなく、`stream::Envelope` を介した impl を書き出す。受け皿の `Unrecognized` は受け取る側（`Event`・`BroadcastEvent`）だけに付ける。

### バージョンセット

- 1 バージョンセットは `specs/<name>/`、`src/generated/<name>/`、`tests/fixtures/<name>/`、`Cargo.toml` の feature `<name>`、`tests/common/mod.rs` の `version` の行、README の「対応バージョン」の表の行、の 6 か所で対になる。固定版を増やすときも消すときも、全部をそろえる（`scripts/pin-latest.sh` が latest を固定版として残す側の手順）。

## 規約

### 言語

- 文言の言語は、その文言がどの環境に出力されるかで決める。
  - 日本語: 手書きのコメント、doc コメント（codegen が生成コードに出すものを含む）、`Cargo.toml` の description、workflow 名とステップ名、開発時にだけ出る文言（codegen のエラーと進捗、スクリプトの usage と echo、テストの `panic!` / `expect`）。
  - 英語のまま: 利用者の環境に出る文言（`miapi::Error` の Display、`compile_error!` のメッセージ、生成コードの panic メッセージ、`examples/` の出力）。
  - 訳さない: 識別子、上流の spec 由来の description、fixture のダミーデータ。
- 日本語の文は常体で書く。用語は README に合わせる（バージョンセット、固定版、latest）。
- コミットメッセージは、英語の conventional prefix に続けて日本語の件名を書く。

### 生成物・spec・fixture

- `src/generated/` は手で編集せず、`cargo codegen` で作り直す。CI が再生成して差分が無いことを検査する。
- `specs/` は上流の出力のまま置く。misskey-js の `streaming.types.ts`・`entities.ts`・`consts.ts` と `LICENSE`（MIT）も、`specs/<name>/misskey-js/` に原文のまま置く。spec（`api.json` と misskey-js の型定義）と実際の応答の食い違いは `codegen/src/corrections.rs` に足して、生成前に補正する。
- コミットする spec は `scripts/update-spec.sh --tag` の結果にする。`--origin` で取るサーバの `/api.json` は、自己参照（`PageBlock.section.children` など）を素の object に潰し、古いリリースでは条件付きでしか返されない項目まで `required` に入れている。
- latest の更新は、spec・生成物・fixture を 1 コミットにし、件名を `build(api): Misskey API を 2026.x.y に更新する` とする。
- fixture に実在のサーバの応答は入れない。固定版の spec と fixture は、足した後は更新しない。
- Misskey の挙動（値の形式や導出、既定値、spec と応答の食い違い）を前提にするときは、実インスタンスの応答の観測だけで決めず、上流 misskey-dev/misskey のソースで確かめる。確率（`Math.random`）・キャッシュ・待たない非同期処理で揺れる挙動がある。
