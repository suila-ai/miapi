# AGENTS.md

このリポジトリで開発するときの案内。利用者向けの説明と、spec・fixture の更新手順は README.md が正で、ここでは繰り返さない。

## コマンド

| 目的 | コマンド |
|---|---|
| 生成物を作り直す | `cargo codegen`（`cargo run -p miapi-codegen` の別名。`specs/*.json` ごとに `src/generated/<name>/` を書き、rustfmt を当てる） |
| CI と同じ検査 | `.github/workflows/ci.yml` の `run:` を上から順に実行する |
| feature を 1 つ選んで検査 | `cargo clippy -p miapi --all-targets --no-default-features --features v2024.10.1 -- -D warnings`、`cargo test -p miapi --no-default-features --features v2024.10.1` |

- パッケージは `-p` で 1 つずつ選ぶ。`--workspace` にすると、codegen が有効にする serde_json の `preserve_order` が miapi のビルドにも入り、利用者が使うのとは違う構成を検査することになる。
- `src/` に単体テストは無く、テストは `tests/` の結合テストと doctest。`src/` に足すなら `Cargo.toml` の `test = false` を戻す（理由はそこのコメントにある）。
- テストは feature を 1 つだけ有効にして実行する（2 つ以上だと `tests/common/mod.rs` の `version` が重複する）。
- 全 feature を検査する `scripts/test-all-versions.sh` は、生成コードが大きいので時間がかかる。開発中は feature を 1 つ選んで検査する。
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

### バージョンセット

- 1 バージョンセットは `specs/<name>.json`、`src/generated/<name>/`、`tests/fixtures/<name>/`、`Cargo.toml` の feature `<name>`、`tests/common/mod.rs` の `version` の行、README の「対応バージョン」の表の行、の 6 か所で対になる。固定版を増やすときも消すときも、全部をそろえる（`scripts/pin-latest.sh` が latest を固定版として残す側の手順）。

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
- `specs/` は上流の出力のまま置く。spec と実際の応答の食い違いは `codegen/src/corrections.rs` に足して、生成前に補正する。
- コミットする spec は `scripts/update-spec.sh --tag` の結果にする。`--origin` で取るサーバの `/api.json` は、自己参照（`PageBlock.section.children` など）を素の object に潰し、古いリリースでは条件付きでしか返されない項目まで `required` に入れている。
- latest の更新は、spec・生成物・fixture を 1 コミットにし、件名を `build(api): Misskey API を 2026.x.y に更新する` とする。
- fixture に実在のサーバの応答は入れない。固定版の spec と fixture は、足した後は更新しない。
- Misskey の挙動（値の形式や導出、既定値、spec と応答の食い違い）を前提にするときは、実インスタンスの応答の観測だけで決めず、上流 misskey-dev/misskey のソースで確かめる。確率（`Math.random`）・キャッシュ・待たない非同期処理で揺れる挙動がある。
