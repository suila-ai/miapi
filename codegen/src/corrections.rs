//! Misskey の `api.json` と misskey-js の型定義が、サーバが実際に返す内容と食い違う箇所。
//! 補正は該当するスキーマや型を持つすべてのバージョンセットに、型を解決する前に適用する。
//! こうすることで、`specs/` のファイルを上流のまま保つ。

use indexmap::IndexMap;

use crate::spec::{AdditionalProperties, MediaType, Response, Schema, SchemaType, Spec};
use crate::ts::{self, Member, TypeNode};

pub fn apply(spec: &mut Spec) {
    let schemas = &mut spec.components.schemas;

    // この係数は倍率である。
    // Web クライアントの初期設定ウィザードは、0.3・0.7・1.25・1.5 といった値を保存する。
    if let Some(factor) = property(schemas, "RolePolicies", &["rateLimitFactor"]) {
        factor.schema_type = Some(SchemaType::One("number".to_owned()));
    }

    // 保存されているベースロールから、既定値を補わずに取り出される（`MetaEntityService`）。
    // そのため、管理者がベースロールを一度保存するまでは、どちらも欠ける。
    optional(
        property(schemas, "MetaDetailedOnly", &["features"]),
        &["localTimeline", "globalTimeline"],
    );

    // この列の初期値は `{}` で、初期設定ウィザードは `entrancePageStyle` だけを保存する。
    optional(
        property(schemas, "MetaClientOptions", &[]),
        &[
            "entrancePageStyle",
            "showTimelineForVisitor",
            "showActivitiesForVisitor",
        ],
    );

    // `i` は、未読のお知らせを保存されている形のまま返す（`UserEntityService`）。
    // ほかの箇所で `AnnouncementEntityService` が足す `forYou` は付かない。
    optional(property(schemas, "Announcement", &[]), &["forYou"]);

    // BullMQ のジョブの値をそのまま返す（`QueueService` の `packJobData`）。
    // `progress` は 0 から始まり、`returnValue` は処理が値を返すまで null で、どちらも object とは限らない。
    // `failedReason` は、ジョブが失敗するまで無い。
    for name in ["progress", "returnValue"] {
        if let Some(value) = property(schemas, "QueueJob", &[name]) {
            value.schema_type = None;
        }
    }
    optional(property(schemas, "QueueJob", &[]), &["failedReason"]);

    // ロールのポリシーの上書きは、値をそのポリシーの型のまま返す。
    // 上書きの無いポリシーも既定値で埋めるので（`RoleEntityService`）、小数（`rateLimitFactor`）、
    // 文字列（`chatAvailability`）、配列（`uploadableFileTypes`）も入り、整数か真偽値とは限らない。
    if let Some(value) = role_policy_value(schemas) {
        value.one_of = None;
    }

    // `i/registry/get` は、`i/registry/set` で保存された値をそのまま返す。`set` の `value` はどんな JSON でもよく、
    // Web クライアントも設定値を数値や文字列のまま保存する（`pizzax.ts`）。
    // 値が null なら 204 になるが（`ApiCallService`）、`Value` は null も表せるので、
    // spec に 204 を足して `Option` で包むことはしない。
    if let Some(value) = response(spec, "/i/registry/get") {
        value.schema_type = None;
    }

    // `i/2fa/remove-key` と `i/2fa/update-key` は、処理の最後に `{}` を返す。
    // spec には 204 しか書かれていないが、204 になるのは処理が null を返したときだけなので（`ApiCallService`）、
    // 実際の応答は 200 と `{}` になる。
    // 204 は残さない。残すと `Response` が `Option` になり、返ることの無い `None` の分岐を、
    // 使う側が書くことになる。
    // 上流が `res` を足して 200 が載ったら補正しない。上書きすると、そのスキーマが消える。
    for path in ["/i/2fa/remove-key", "/i/2fa/update-key"] {
        if let Some(responses) = responses(spec, path)
            && !responses.contains_key("200")
        {
            responses.shift_remove("204");
            responses.insert("200".to_owned(), empty_object());
        }
    }
}

/// misskey-js の型定義の補正。型が下の誤った形のときだけ直すので、上流が直した版では何もしない。
pub fn apply_streaming(files: &mut ts::Files) {
    // カスタム絵文字のリアクションには、絵文字の名前と URL が付く（`ReactionService`）。
    // misskey-js の型は 2025.10.0 まで、これを文字列にしていた。
    if let Some(emoji) = ts_property(files, "NoteUpdatedEvent", &["body", "emoji"])
        && *emoji == nullable(keyword("string"))
    {
        let members = ["name", "url"].map(|name| Member {
            name: name.to_owned(),
            optional: false,
            ty: keyword("string"),
            doc: None,
        });
        *emoji = nullable(TypeNode::Object {
            members: members.into(),
        });
    }

    // `drive/files/upload-from-url` の `marker` は省略でき、省略すると既定値の null のまま送る。
    if let Some(marker) = ts_property(
        files,
        "Channels",
        &["main", "events", "urlUploadFinished", "marker"],
    ) && *marker == keyword("string")
    {
        *marker = nullable(keyword("string"));
    }
}

/// `path` のエンドポイントの応答の一覧。キーはステータスコード。
fn responses<'a>(spec: &'a mut Spec, path: &str) -> Option<&'a mut IndexMap<String, Response>> {
    Some(&mut spec.paths.get_mut(path)?.post.as_mut()?.responses)
}

/// `path` のエンドポイントが 200 で返す JSON のスキーマ。
fn response<'a>(spec: &'a mut Spec, path: &str) -> Option<&'a mut Schema> {
    let content = responses(spec, path)?.get_mut("200")?.content.as_mut()?;
    Some(&mut content.get_mut("application/json")?.schema)
}

/// プロパティの無い object を返す、200 の応答。
fn empty_object() -> Response {
    let schema = Schema {
        schema_type: Some(SchemaType::One("object".to_owned())),
        properties: Some(IndexMap::new()),
        ..Schema::default()
    };
    Response {
        content: Some(IndexMap::from([(
            "application/json".to_owned(),
            MediaType { schema },
        )])),
    }
}

fn property<'a>(
    schemas: &'a mut IndexMap<String, Schema>,
    component: &str,
    path: &[&str],
) -> Option<&'a mut Schema> {
    path.iter()
        .try_fold(schemas.get_mut(component)?, |schema, name| {
            schema.properties.as_mut()?.get_mut(*name)
        })
}

/// `Role` の `policies` の各要素にある `value`。
/// `Role` は allOf で、`policies` はマップなので、[`property`] ではたどれない。
fn role_policy_value(schemas: &mut IndexMap<String, Schema>) -> Option<&mut Schema> {
    let policies = schemas
        .get_mut("Role")?
        .all_of
        .as_mut()?
        .iter_mut()
        .find_map(|member| member.properties.as_mut()?.get_mut("policies"))?;
    let Some(AdditionalProperties::Schema(entry)) = policies.additional_properties.as_mut() else {
        return None;
    };
    entry
        .any_of
        .as_mut()?
        .iter_mut()
        .find_map(|member| member.properties.as_mut()?.get_mut("value"))
}

fn optional(schema: Option<&mut Schema>, names: &[&str]) {
    if let Some(required) = schema.and_then(|schema| schema.required.as_mut()) {
        required.retain(|name| !names.contains(&name.as_str()));
    }
}

/// misskey-js の型定義で `name` と宣言された型から、`path` のメンバーをたどった先の型。
fn ts_property<'a>(
    files: &'a mut ts::Files,
    name: &str,
    path: &[&str],
) -> Option<&'a mut TypeNode> {
    let decl = files
        .values_mut()
        .flat_map(|file| &mut file.types)
        .find(|decl| decl.name == name)?;
    member_type(&mut decl.ty, path)
}

/// object literal のメンバーを名前でたどる。関数型は第 1 引数（本文）へ、union と intersection は、
/// 残りの `path` をたどれる最初のメンバーへ進む。
fn member_type<'a>(node: &'a mut TypeNode, path: &[&str]) -> Option<&'a mut TypeNode> {
    let Some((name, rest)) = path.split_first() else {
        return Some(node);
    };
    match node {
        TypeNode::Object { members } => {
            let member = members.iter_mut().find(|member| member.name == *name)?;
            member_type(&mut member.ty, rest)
        }
        TypeNode::Function { params, .. } => member_type(params.first_mut()?, path),
        TypeNode::Union { types } | TypeNode::Intersection { types } => {
            types.iter_mut().find_map(|ty| member_type(ty, path))
        }
        _ => None,
    }
}

fn keyword(name: &str) -> TypeNode {
    TypeNode::Keyword {
        name: name.to_owned(),
    }
}

/// `ty | null`。
fn nullable(ty: TypeNode) -> TypeNode {
    TypeNode::Union {
        types: vec![ty, keyword("null")],
    }
}
