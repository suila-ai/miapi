use serde_json::{Value, json};

use crate::corrections;
use crate::emit;
use crate::model::*;
use crate::spec::Spec;
use crate::ts;

/// チャンネルの無い、最小のストリーミングの型定義。
fn empty_ts() -> ts::Files {
    streaming_ts(json!({}), json!({}), json!([]))
}

/// `Channels` と `BroadcastEvents` のメンバーと、entities.ts の型の宣言から、型定義を組み立てる。
fn streaming_ts(channels: Value, broadcast: Value, entities: Value) -> ts::Files {
    let object = |members: Value| {
        obj(members
            .as_object()
            .unwrap()
            .iter()
            .map(|(name, ty)| member(name, false, ty.clone()))
            .collect())
    };
    serde_json::from_value(json!({
        "streaming.types.ts": {
            "imports": [
                {"name": "Note", "from": "./autogen/models.js"},
                {"name": "PageEvent", "from": "./entities.js"},
                {"name": "Keys", "from": "./consts.js"},
                {"name": "Generic", "from": "./entities.js"},
            ],
            "reexports": [],
            "types": [
                {"name": "Channels", "typeParams": [], "type": object(channels)},
                {"name": "BroadcastEvents", "typeParams": [], "type": object(broadcast)},
            ],
            "consts": [],
        },
        "entities.ts": {
            "imports": [{"name": "Note", "from": "./autogen/models.js"}],
            "reexports": ["./autogen/models.js"],
            "types": entities,
            "consts": [],
        },
        "consts.ts": {
            "imports": [],
            "reexports": [],
            "types": [{"name": "Keys", "typeParams": [], "type": indexed(json!({"kind": "typeof", "name": "keys"}), kw("number"))}],
            "consts": [{"name": "keys", "values": ["a", "b"]}],
        },
    }))
    .unwrap()
}

fn build_tree(spec: &Spec) -> Result<Tree, String> {
    crate::resolve::build_tree(spec, &empty_ts())
}

fn spec(components: Value, paths: Value) -> Spec {
    serde_json::from_value(json!({
        "openapi": "3.1.0",
        "info": {"version": "test"},
        "paths": paths,
        "components": {"schemas": components}
    }))
    .unwrap()
}

fn post(request: Option<Value>, responses: Value) -> Value {
    let mut op = json!({"operationId": "x", "responses": responses});
    if let Some(schema) = request {
        op["requestBody"] =
            json!({"required": true, "content": {"application/json": {"schema": schema}}});
    }
    json!({"post": op})
}

fn ok(schema: Value) -> Value {
    json!({"200": {"description": "OK", "content": {"application/json": {"schema": schema}}}})
}

fn find_struct<'a>(items: &'a [Item], name: &str) -> &'a StructDef {
    items
        .iter()
        .find_map(|item| match item {
            Item::Struct(def) if def.name == name => Some(def),
            _ => None,
        })
        .unwrap_or_else(|| panic!("構造体 {name} が無い"))
}

fn find_union<'a>(items: &'a [Item], name: &str) -> &'a UnionDef {
    items
        .iter()
        .find_map(|item| match item {
            Item::Union(def) if def.name == name => Some(def),
            _ => None,
        })
        .unwrap_or_else(|| panic!("union {name} が無い"))
}

fn find_enum<'a>(items: &'a [Item], name: &str) -> &'a StrEnumDef {
    items
        .iter()
        .find_map(|item| match item {
            Item::StrEnum(def) if def.name == name => Some(def),
            _ => None,
        })
        .unwrap_or_else(|| panic!("enum {name} が無い"))
}

fn field<'a>(def: &'a StructDef, json_name: &str) -> &'a Field {
    def.fields
        .iter()
        .find(|f| f.json_name == json_name)
        .unwrap_or_else(|| panic!("{} にフィールド {json_name} が無い", def.name))
}

fn endpoint<'a>(tree: &'a Tree, path: &str) -> &'a EndpointDef {
    let mut nodes = &tree.api;
    let mut found = None;
    for segment in path.split('/') {
        let node = nodes.iter().find(|n| n.segment == segment).unwrap();
        found = node.endpoint.as_ref();
        nodes = &node.children;
    }
    found.unwrap()
}

fn component(name: &str) -> RustType {
    RustType::Component(name.to_owned())
}

fn tokens_without_spaces(tokens: &proc_macro2::TokenStream) -> String {
    tokens.to_string().replace(' ', "")
}

fn source_of(tree: &Tree, path: &str) -> String {
    let files = emit::tree_files(tree, "v2024.10.1");
    let (_, tokens) = files
        .iter()
        .find(|(file, _)| file == path)
        .unwrap_or_else(|| panic!("ファイル {path} が無い"));
    tokens_without_spaces(tokens)
}

fn variant_names(def: &UnionDef) -> Vec<&str> {
    def.variants.iter().map(|(name, _)| name.as_str()).collect()
}

#[test]
fn nullable_forms_become_option() {
    let tree = build_tree(&spec(
        json!({
            "B": {"type": "object", "properties": {"x": {"type": "string"}}},
            "A": {"type": "object", "properties": {
                "p1": {"type": ["string", "null"]},
                "p2": {"oneOf": [{"$ref": "#/components/schemas/B"}, {"type": "null"}]},
                "p3": {"type": ["object", "null"], "anyOf": [{"$ref": "#/components/schemas/B"}, {"type": "null"}]},
                "p4": {"type": ["object", "null"], "allOf": [{"$ref": "#/components/schemas/B"}]},
                "p5": {"anyOf": [{"type": "number"}]},
                "p6": {"type": ["string", "null"], "enum": ["a", "b", null]}
            }, "required": ["p1", "p2", "p3", "p4", "p5", "p6"]}
        }),
        json!({}),
    ))
    .unwrap();
    let a = find_struct(&tree.types, "A");
    for name in ["p1", "p2", "p3", "p4", "p6"] {
        assert!(field(a, name).nullable, "{name} が null 可になっていない");
        assert!(matches!(
            field(a, name).declared_type(StructKind::Entity),
            RustType::Option(_)
        ));
    }
    assert_eq!(field(a, "p1").base, RustType::String);
    assert_eq!(field(a, "p2").base, component("B"));
    assert_eq!(field(a, "p3").base, component("B"));
    assert_eq!(field(a, "p4").base, component("B"));
    assert!(!field(a, "p5").nullable);
    assert_eq!(field(a, "p5").base, RustType::F64);
    assert_eq!(field(a, "p6").base, component("AP6"));
    assert_eq!(find_enum(&tree.types, "AP6").variants.len(), 2);
}

#[test]
fn date_time_strings_become_chrono_date_time() {
    let tree = build_tree(&spec(
        json!({"A": {"type": "object", "properties": {
            "createdAt": {"type": "string", "format": "date-time"},
            "deletedAt": {"type": ["string", "null"], "format": "date-time"},
            "name": {"type": "string"}
        }, "required": ["createdAt", "deletedAt", "name"]}}),
        json!({"/x": post(Some(json!({"type": "object", "properties": {"since": {"type": "string", "format": "date-time"}}, "required": ["since"]})), json!({"204": {"description": "none"}}))}),
    ))
    .unwrap();
    let a = find_struct(&tree.types, "A");
    assert_eq!(field(a, "createdAt").base, RustType::DateTime);
    assert_eq!(field(a, "deletedAt").base, RustType::DateTime);
    assert!(field(a, "deletedAt").nullable);
    assert_eq!(field(a, "name").base, RustType::String);
    assert!(
        source_of(&tree, "types.rs").contains("pubcreated_at:::chrono::DateTime<::chrono::Utc>,")
    );
    // `DateTime<Utc>` は `Default`（UNIX エポック）を持つので、Request の `Default` を妨げない。
    assert!(endpoint(&tree, "x").request.derive_default);
}

#[test]
fn request_fields_distinguish_omitted_and_null() {
    let tree = build_tree(&spec(
        json!({}),
        json!({"/x": post(Some(json!({"type": "object", "properties": {
            "a": {"type": "string"}, "b": {"type": "string"},
            "c": {"type": ["string", "null"]}, "d": {"type": ["string", "null"]}
        }, "required": ["a", "d"]})), json!({"204": {"description": "OK"}}))}),
    ))
    .unwrap();
    let request = &endpoint(&tree, "x").request;
    let declared = |name: &str| field(request, name).declared_type(StructKind::Request);
    assert_eq!(declared("a"), RustType::String);
    assert_eq!(declared("b"), RustType::Option(Box::new(RustType::String)));
    assert_eq!(
        declared("c"),
        RustType::Option(Box::new(RustType::Option(Box::new(RustType::String))))
    );
    assert_eq!(declared("d"), RustType::Option(Box::new(RustType::String)));
    assert!(request.derive_default);
    let source = source_of(&tree, "api/x.rs");
    assert!(
        source.contains("pubc:::std::option::Option<::std::option::Option<::std::string::String>>")
    );
    assert!(source.contains("with=\"crate::serde_helpers::double_option\""));
    assert!(source.contains("pubtypeResponse=();"));
}

#[test]
fn all_of_merges_members_and_ref_with_properties() {
    let tree = build_tree(&spec(
        json!({
            "Lite": {"type": "object", "properties": {"id": {"type": "string"}, "flag": {"type": "boolean"}}, "required": ["id"]},
            "OnlyX": {"type": "object", "properties": {"flag": {"type": "boolean", "default": false}, "extra": {"type": "string"}}, "required": ["flag"]},
            "Full": {"type": "object", "allOf": [{"$ref": "#/components/schemas/Lite"}, {"$ref": "#/components/schemas/OnlyX"}]},
            "WithRef": {"$ref": "#/components/schemas/Lite", "type": "object", "properties": {"token": {"type": "string"}}, "required": ["token"]}
        }),
        json!({}),
    ))
    .unwrap();
    let full = find_struct(&tree.types, "Full");
    let names: Vec<&str> = full.fields.iter().map(|f| f.json_name.as_str()).collect();
    assert_eq!(names, ["id", "flag", "extra"]);
    assert!(field(full, "flag").required);
    let with_ref = find_struct(&tree.types, "WithRef");
    let names: Vec<&str> = with_ref
        .fields
        .iter()
        .map(|f| f.json_name.as_str())
        .collect();
    assert_eq!(names, ["id", "flag", "token"]);
}

#[test]
fn all_of_with_conflicting_types_is_an_error() {
    let error = build_tree(&spec(
        json!({
            "A": {"type": "object", "properties": {"v": {"type": "string"}}},
            "B": {"type": "object", "properties": {"v": {"type": "integer"}}},
            "C": {"allOf": [{"$ref": "#/components/schemas/A"}, {"$ref": "#/components/schemas/B"}]}
        }),
        json!({}),
    ))
    .unwrap_err();
    assert!(error.contains("C.v"), "{error}");
}

#[test]
fn nested_composition_in_all_of_is_an_error() {
    let nested = json!({"allOf": [
        {"anyOf": [{"type": "object", "properties": {"a": {"type": "string"}}}, {"type": "object", "properties": {"b": {"type": "string"}}}]},
        {"type": "object", "properties": {"limit": {"type": "integer"}}}
    ]});
    for components in [
        json!({"C": nested.clone()}),
        json!({"C": {"type": "object", "properties": {"inner": nested}}}),
    ] {
        let error = build_tree(&spec(components, json!({}))).unwrap_err();
        assert!(error.contains("allOf の中の"), "{error}");
    }
}

#[test]
fn recursive_references_are_boxed() {
    let tree = build_tree(&spec(
        json!({
            "Node": {"type": "object", "properties": {
                "next": {"oneOf": [{"$ref": "#/components/schemas/Node"}, {"type": "null"}]},
                "children": {"type": "array", "items": {"$ref": "#/components/schemas/Node"}},
                "owner": {"$ref": "#/components/schemas/Owner"}
            }},
            "Owner": {"type": "object", "properties": {"name": {"type": "string"}}},
            "Expr": {"oneOf": [{"$ref": "#/components/schemas/Not"}, {"$ref": "#/components/schemas/Lit"}]},
            "Not": {"type": "object", "properties": {"value": {"$ref": "#/components/schemas/Expr"}}, "required": ["value"]},
            "Lit": {"type": "object", "properties": {"v": {"type": "number"}}, "required": ["v"]}
        }),
        json!({}),
    ))
    .unwrap();
    let node = find_struct(&tree.types, "Node");
    assert_eq!(
        field(node, "next").base,
        RustType::Boxed(Box::new(component("Node")))
    );
    assert_eq!(
        field(node, "children").base,
        RustType::Vec(Box::new(component("Node")))
    );
    assert_eq!(field(node, "owner").base, component("Owner"));
    let not = find_struct(&tree.types, "Not");
    assert_eq!(
        field(not, "value").base,
        RustType::Boxed(Box::new(component("Expr")))
    );
}

#[test]
fn union_variants_are_ordered_by_transitive_required_count() {
    let tree = build_tree(&spec(
        json!({
            "UserLite": {"type": "object", "properties": {"id": {"type": "string"}, "name": {"type": "string"}}, "required": ["id", "name"]},
            "OnlyMore": {"type": "object", "properties": {"a": {"type": "string"}, "b": {"type": "string"}, "c": {"type": "string"}}, "required": ["a", "b", "c"]},
            "NotMe": {"type": "object", "allOf": [{"$ref": "#/components/schemas/UserLite"}, {"$ref": "#/components/schemas/OnlyMore"}]},
            "MeOnly": {"type": "object", "properties": {"d": {"type": "string"}, "e": {"type": "string"}}, "required": ["d", "e"]},
            "Me": {"type": "object", "allOf": [{"$ref": "#/components/schemas/UserLite"}, {"$ref": "#/components/schemas/OnlyMore"}, {"$ref": "#/components/schemas/MeOnly"}]},
            "Detailed": {"oneOf": [{"$ref": "#/components/schemas/NotMe"}, {"$ref": "#/components/schemas/Me"}]},
            "User": {"oneOf": [{"$ref": "#/components/schemas/UserLite"}, {"$ref": "#/components/schemas/Detailed"}]}
        }),
        json!({}),
    ))
    .unwrap();
    let detailed = variant_names(find_union(&tree.types, "Detailed"));
    assert_eq!(detailed, ["Me", "NotMe"]);
    let user = variant_names(find_union(&tree.types, "User"));
    // union のメンバーである union は、そのメンバーに展開する。
    assert_eq!(user, ["Me", "NotMe", "UserLite"]);
}

#[test]
fn union_variant_names_are_deduplicated_after_escape() {
    let tree = build_tree(&spec(
        json!({
            "_1": {"type": "object", "properties": {"x": {"type": "string"}}},
            "A": {"oneOf": [
                {"$ref": "#/components/schemas/_1"},
                {"type": "object", "properties": {"t": {"type": "string", "enum": ["1"]}}, "required": ["t"]}
            ]}
        }),
        json!({}),
    ))
    .unwrap();
    // 判別値 `1` のメンバーと、`_1` を指す $ref は、どちらもバリアント `_1` になる。
    // 必須プロパティの少ない $ref が後に並ぶので、そちらに連番が付く。
    let types = source_of(&tree, "types.rs");
    assert!(types.contains("_12(crate::generated::v2024_10_1::types::_1),"));
}

#[test]
fn discriminator_enums_have_no_catch_all() {
    let tree = build_tree(&spec(
        json!({
            "Cond": {"oneOf": [{"$ref": "#/components/schemas/A"}, {"$ref": "#/components/schemas/B"}]},
            "A": {"type": "object", "properties": {"id": {"type": "string"}, "type": {"type": "string", "enum": ["x", "y"]}}, "required": ["id", "type"]},
            "B": {"type": "object", "properties": {"id": {"type": "string"}, "type": {"type": "string", "enum": ["p", "q", "r"]}}, "required": ["id", "type"]},
            "C": {"type": "object", "properties": {"status": {"type": "string", "enum": ["on", "off"]}}, "required": ["status"]},
            "Cfg": {"type": "object", "properties": {"app": {"oneOf": [
                {"type": "object", "properties": {"type": {"type": "string", "enum": ["all", "following"]}}, "required": ["type"]},
                {"type": "object", "properties": {"type": {"type": "string", "enum": ["list"]}, "userListId": {"type": "string"}}, "required": ["type", "userListId"]}
            ]}}, "required": ["app"]}
        }),
        json!({}),
    ))
    .unwrap();
    assert!(!find_enum(&tree.types, "AType").other);
    assert!(!find_enum(&tree.types, "BType").other);
    assert!(find_enum(&tree.types, "CStatus").other);
    let app = variant_names(find_union(&tree.types, "CfgApp"));
    assert_eq!(app, ["List", "Object"]);
    assert!(!find_enum(&tree.types, "CfgAppObjectType").other);
    assert!(!find_enum(&tree.types, "CfgAppListType").other);
}

#[test]
fn response_shapes() {
    let inline =
        json!({"type": "object", "properties": {"v": {"type": "string"}}, "required": ["v"]});
    let tree = build_tree(&spec(
        json!({"Note": {"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]}}),
        json!({
            "/a": post(None, ok(inline.clone())),
            "/b": post(None, json!({"200": ok(inline.clone())["200"], "204": {"description": "none"}})),
            "/c": post(None, json!({"204": {"description": "none"}})),
            "/d": post(None, ok(json!({"$ref": "#/components/schemas/Note"}))),
            "/e": post(None, ok(json!({"type": "array", "items": inline}))),
            "/f": post(None, ok(json!({"oneOf": [{"$ref": "#/components/schemas/Note"}, {"type": "null"}]})))
        }),
    ))
    .unwrap();
    assert!(matches!(endpoint(&tree, "a").response, ResponseDef::Struct));
    let b = endpoint(&tree, "b");
    assert!(
        matches!(&b.response, ResponseDef::Alias(RustType::Option(inner)) if **inner == RustType::Local("ResponseBody".into()))
    );
    assert!(b.items.iter().any(|item| item.name() == "ResponseBody"));
    assert!(matches!(endpoint(&tree, "c").response, ResponseDef::Unit));
    assert!(
        matches!(&endpoint(&tree, "d").response, ResponseDef::Alias(ty) if *ty == component("Note"))
    );
    assert!(
        matches!(&endpoint(&tree, "e").response, ResponseDef::Alias(RustType::Vec(inner)) if **inner == RustType::Local("ResponseItem".into()))
    );
    assert!(
        matches!(&endpoint(&tree, "f").response, ResponseDef::Alias(RustType::Option(inner)) if **inner == component("Note"))
    );
    assert!(
        source_of(&tree, "api/d.rs")
            .contains("pubtypeResponse=crate::generated::v2024_10_1::types::Note;")
    );
}

#[test]
fn builder_tokens_use_type_state_for_required_parameters() {
    let tree = build_tree(&spec(
        json!({}),
        json!({
            "/notes/show": post(Some(json!({"type": "object", "properties": {
                "noteId": {"type": "string"}, "cw": {"type": ["string", "null"]}
            }, "required": ["noteId"]})), json!({"204": {"description": "none"}})),
            "/notes": post(Some(json!({"type": "object", "properties": {"limit": {"type": "integer"}}})), json!({"204": {"description": "none"}}))
        }),
    ))
    .unwrap();
    let notes = source_of(&tree, "api/notes.rs");
    assert!(notes.contains("pubstructCall<'a,S=(crate::builder::Unset,)>"));
    assert!(notes.contains("pubfnnote_id(self,value:impl::std::convert::Into<::std::string::String>)->Call<'a,(crate::builder::Set,)>"));
    assert!(notes.contains("pubfncw_null(mutself)->Self"));
    assert!(notes.contains(
        "pubfnmaybe_cw(mutself,value:::std::option::Option<impl::std::convert::Into<::std::string::String>>)->Self"
    ));
    assert!(notes.contains("`null`を送るには[`cw_null`](Self::cw_null)を使う。"));
    assert!(notes.contains("`Some`なら`limit`を設定し、`None`なら設定を消して送らない。\"]"));
    assert!(notes.contains("self.cw=value.map(|value|::std::option::Option::Some(value.into()));"));
    assert!(notes.contains("self.limit=value.map(|value|value.into());"));
    assert!(notes.contains("self.maybe_limit(::std::option::Option::Some(value))"));
    assert!(!notes.contains("maybe_note_id"));
    assert!(
        notes.contains(
            "impl<'a>::std::future::IntoFuture forCall<'a,(crate::builder::Set,)>"
                .replace(' ', "")
                .as_str()
        )
    );
    assert!(notes.contains("pubfnshow(&self)->show::Call<'a>"));
    assert!(notes.contains("pubfnlimit(self,value:impl::std::convert::Into<i64>)->Self"));
    assert!(notes.contains("client:&'acrate::generated::v2024_10_1::Client,"));
    let api_mod = source_of(&tree, "api/mod.rs");
    assert!(api_mod.contains("implsuper::Client{"));
    assert!(api_mod.contains("pubfnnotes(&self)->notes::Call<'_>"));
    let tree_mod = source_of(&tree, "mod.rs");
    assert!(tree_mod.contains("pubtypeClient=crate::client::Client<Version>;"));
}

#[test]
fn request_alternatives_are_documented() {
    let tree = build_tree(&spec(
        json!({}),
        json!({"/users/show": post(Some(json!({"type": "object", "properties": {
            "userId": {"type": "string"}, "userIds": {"type": "array", "items": {"type": "string"}}, "username": {"type": "string"}, "host": {"type": ["string", "null"]}
        }, "anyOf": [{"required": ["userId"]}, {"required": ["userIds"]}, {"required": ["username"]}]})), json!({"204": {"description": "none"}}))}),
    ))
    .unwrap();
    let request = &endpoint(&tree, "users/show").request;
    assert!(request.fields.iter().all(|f| !f.required));
    assert!(
        request
            .docs
            .iter()
            .any(|d| d == "必須: `userId` / `userIds` / `username` のいずれか。")
    );
}

#[test]
fn request_fields_with_defaults_are_optional() {
    let scope = json!({"type": "array", "items": {"type": "string"}, "default": []});
    let tree = build_tree(&spec(
        json!({}),
        json!({"/i/registry/get": post(Some(json!({"type": "object", "properties": {
            "key": {"type": "string"},
            "scope": scope.clone(),
            "reply": {"type": ["boolean", "null"], "default": null},
            "options": {"type": "object", "properties": {"limit": {"type": "integer", "default": 10}}, "required": ["limit"]}
        }, "required": ["key", "scope", "reply"], "allOf": [
            {"type": "object", "properties": {"detail": {"type": "boolean", "default": true}}, "required": ["detail"]}
        ]})), ok(json!({"type": "object", "properties": {"scope": scope}, "required": ["scope"]})))}),
    ))
    .unwrap();
    let registry = endpoint(&tree, "i/registry/get");
    assert!(field(&registry.request, "key").required);
    assert!(!field(&registry.request, "scope").required);
    assert!(!field(&registry.request, "reply").required);
    assert!(!field(&registry.request, "detail").required);
    assert!(!field(find_struct(&registry.items, "RequestOptions"), "limit").required);
    // 必須から外したパラメータにも、既定値を書く。
    assert_eq!(field(&registry.request, "scope").docs, ["省略時は `[]`。"]);
    // 応答の型では必須のまま。
    assert!(field(find_struct(&registry.items, "Response"), "scope").required);
}

#[test]
fn request_constraints_are_documented() {
    let limit = json!({"type": "integer", "minimum": 1, "maximum": 100, "default": 10});
    let tree = build_tree(&spec(
        json!({"Page": {"type": "object", "properties": {"limit": limit.clone()}}}),
        json!({"/notes": post(Some(json!({"type": "object", "properties": {
            "limit": limit,
            "text": {"type": ["string", "null"], "description": "本文", "minLength": 1, "maxLength": 3000},
            "choices": {"type": "array", "items": {"type": "string", "minLength": 1, "maxLength": 50}, "minItems": 2},
            "fileIds": {"type": "array", "items": {"type": "string"}, "minItems": 0, "maxItems": 16},
            "offset": {"type": "integer", "minimum": 0},
            "scores": {"type": "array", "items": {"type": "number", "minimum": 0, "maximum": 1}},
            "color": {"type": "string", "description": "[任意] 色", "pattern": "^#[0-9a-f]{6}$"},
            "tags": {"type": "array", "items": {"type": "string", "pattern": "^\\w+$"}, "uniqueItems": true, "default": ["a"]},
            "cw": {"type": ["string", "null"], "minLength": 0, "default": null},
            "visibility": {"type": "string", "default": "public"}
        }})), json!({"204": {"description": "none"}}))}),
    ))
    .unwrap();
    let request = &endpoint(&tree, "notes").request;
    assert_eq!(
        field(request, "limit").docs,
        ["1 以上 100 以下。省略時は `10`。"]
    );
    assert_eq!(
        field(request, "text").docs,
        ["本文", "文字数は 1 以上 3000 以下。"]
    );
    assert_eq!(
        field(request, "choices").docs,
        ["要素数は 2 以上。各要素の文字数は 1 以上 50 以下。"]
    );
    // 文字数と要素数の下限の 0 は書かない。
    assert_eq!(field(request, "fileIds").docs, ["要素数は 16 以下。"]);
    // 数値の下限の 0 は書く。
    assert_eq!(field(request, "offset").docs, ["0 以上。"]);
    assert_eq!(
        field(request, "scores").docs,
        ["各要素の値は 0 以上 1 以下。"]
    );
    assert_eq!(
        field(request, "color").docs,
        ["[任意] 色", "形式は正規表現 `^#[0-9a-f]{6}$` に合うもの。"]
    );
    assert_eq!(
        field(request, "tags").docs,
        ["要素は重複させない。各要素の形式は正規表現 `^\\w+$` に合うもの。省略時は `[\"a\"]`。"]
    );
    assert!(field(request, "cw").docs.is_empty());
    assert_eq!(
        field(request, "visibility").docs,
        ["省略時は `\"public\"`。"]
    );
    // 応答やエンティティの型には出さない。
    assert!(
        field(find_struct(&tree.types, "Page"), "limit")
            .docs
            .is_empty()
    );
    let notes = source_of(&tree, "api/notes.rs");
    assert!(notes.contains("#[doc=\"1以上100以下。省略時は`10`。\"]pubfnlimit("));
    // code span の外の `[` `]` だけをエスケープする。
    assert!(notes.contains(
        r#"#[doc="\\[任意\\]色"]#[doc="形式は正規表現`^#[0-9a-f]{6}$`に合うもの。"]pubfncolor("#
    ));
}

#[test]
fn multipart_file_field_is_skipped_by_serde() {
    let tree = build_tree(&spec(
        json!({}),
        json!({"/drive/files/create": {"post": {"operationId": "x", "requestBody": {"content": {"multipart/form-data": {"schema": {
            "type": "object", "properties": {"name": {"type": ["string", "null"]}, "file": {"type": "string", "format": "binary"}}, "required": ["file"]
        }}}}, "responses": {"204": {"description": "none"}}}}}),
    ))
    .unwrap();
    let request = &endpoint(&tree, "drive/files/create").request;
    assert_eq!(field(request, "file").base, RustType::FilePart);
    assert!(!request.derive_default);
    let source = source_of(&tree, "api/drive.rs");
    assert!(source.contains("#[serde(skip)]pubfile:crate::FilePart"));
    assert!(source.contains("crate::Body::Multipart{fields,file:(\"file\",self.file)"));
}

#[test]
fn selector_file_puts_latest_at_the_root_and_the_rest_in_modules() {
    let source = tokens_without_spaces(&emit::selector_file(&[
        "latest".to_owned(),
        "v2024.10.1".to_owned(),
    ]));
    assert!(source.contains("#[cfg(feature=\"latest\")]#[path=\"latest/mod.rs\"]modlatest;"));
    assert!(
        source.contains("#[cfg(feature=\"latest\")]pubuselatest::{API_VERSION,Client,api,types};")
    );
    assert!(
        source.contains(
            "#[cfg(all(feature=\"latest\",feature=\"streaming\"))]pubuselatest::streaming;"
        )
    );
    assert!(
        source.contains(
            "#[cfg(feature=\"v2024.10.1\")]#[path=\"v2024.10.1/mod.rs\"]pubmodv2024_10_1;"
        )
    );
    assert!(!source.contains("compile_error"));
}

#[test]
fn name_collisions_are_rejected() {
    let error = build_tree(&spec(
        json!({}),
        json!({"/call": post(None, json!({"204": {"description": "none"}}))}),
    ))
    .unwrap_err();
    assert!(error.contains("Client のメソッド"), "{error}");
    let error = build_tree(&spec(
        json!({}),
        json!({"/x": post(Some(json!({"type": "object", "properties": {"client": {"type": "string"}}})), json!({"204": {"description": "none"}}))}),
    ))
    .unwrap_err();
    assert!(error.contains("ビルダーのフィールド"), "{error}");
    // 入れ子の構造体にはビルダーが無いので、同じ名前のプロパティがあってもよい。
    build_tree(&spec(
        json!({}),
        json!({"/x": post(Some(json!({"type": "object", "properties": {"poll": {"type": "object", "properties": {"client": {"type": "string"}}}}})), json!({"204": {"description": "none"}}))}),
    ))
    .unwrap();
    let error = build_tree(&spec(
        json!({}),
        json!({
            "/x": post(Some(json!({"type": "object", "properties": {"y": {"type": "string"}}})), json!({"204": {"description": "none"}})),
            "/x/y": post(None, json!({"204": {"description": "none"}}))
        }),
    ))
    .unwrap_err();
    assert!(
        error.contains("ビルダーのほかのメソッドと衝突する"),
        "{error}"
    );
    // 任意のプロパティ `limit` に足す `maybe_limit` が、プロパティ `maybeLimit` の setter と衝突する。
    let error = build_tree(&spec(
        json!({}),
        json!({"/x": post(Some(json!({"type": "object", "properties": {"limit": {"type": "integer"}, "maybeLimit": {"type": "integer"}}})), json!({"204": {"description": "none"}}))}),
    ))
    .unwrap_err();
    assert!(
        error.contains("ビルダーのほかのメソッドと衝突する"),
        "{error}"
    );
    // null 可の応答の本体を `ResponseBody` に改名すると、プロパティ `body` の入れ子の型と同じ名前になる。
    let error = build_tree(&spec(
        json!({}),
        json!({"/x": post(None, ok(json!({"type": ["object", "null"], "properties": {
            "body": {"type": "object", "properties": {"a": {"type": "string"}}, "required": ["a"]}
        }, "required": ["body"]})))}),
    ))
    .unwrap_err();
    assert!(error.contains("型名 `ResponseBody`"), "{error}");
    // component の `Self` と `Self_` は、ident を通すとどちらも型名 `Self_` になる。
    let error = build_tree(&spec(
        json!({
            "Self": {"type": "object", "properties": {}},
            "Self_": {"type": "object", "properties": {}}
        }),
        json!({}),
    ))
    .unwrap_err();
    assert!(error.contains("型名 `Self_`"), "{error}");
}

#[test]
fn escape_is_applied_only_to_whole_identifiers() {
    let tree = build_tree(&spec(
        json!({"A": {"type": "object", "properties": {
            "kind": {"type": "string", "enum": ["self", "1"]},
            "u": {"oneOf": [
                {"type": "object", "properties": {"t": {"type": "string", "enum": ["1"]}}, "required": ["t"]},
                {"type": "object", "properties": {"t": {"type": "string", "enum": ["self"]}}, "required": ["t"]}
            ]},
            "self": {"type": "array", "items": {"type": "object", "properties": {"x": {"type": "string"}}}},
            "2fa": {"type": "object", "properties": {"x": {"type": "string"}}}
        }}}),
        json!({"/x": post(Some(json!({"type": "object", "properties": {
            "self": {"type": ["string", "null"]},
            "2fa": {"type": ["string", "null"]}
        }})), json!({"204": {"description": "none"}}))}),
    ))
    .unwrap();
    // 型名の途中には `_` を足さない。`ASelf_Item` や `A_2fa` は non_camel_case_types の警告になる。
    find_struct(&tree.types, "ASelfItem");
    find_struct(&tree.types, "A2fa");
    let types = source_of(&tree, "types.rs");
    assert!(types.contains("_1(crate::generated::v2024_10_1::types::AU1),"));
    assert!(types.contains("Self_(crate::generated::v2024_10_1::types::AUSelf),"));
    assert!(types.contains("#[serde(rename=\"self\")]Self_,"));
    assert!(types.contains("#[serde(rename=\"1\")]_1,"));
    // setter の名前でも、つないだところに `_` を重ねない。`self__null` や `maybe__2fa` は non_snake_case の警告になる。
    // `self_` の末尾の `_` も、つないだ後は要らないので落とす（`maybe_self`）。
    let x = source_of(&tree, "api/x.rs");
    assert!(x.contains("pubfnself_null(mutself)"));
    assert!(x.contains("pubfn_2fa_null(mutself)"));
    assert!(x.contains("pubfnmaybe_self(mutself"));
    assert!(x.contains("pubfnmaybe_2fa(mutself"));
}

#[test]
fn corrections_match_what_servers_return() {
    let mut spec = spec(
        json!({
            "RolePolicies": {"type": "object", "properties": {"rateLimitFactor": {"type": "integer"}, "pinLimit": {"type": "integer"}}, "required": ["rateLimitFactor", "pinLimit"]},
            "MetaDetailedOnly": {"type": "object", "properties": {"features": {"type": "object", "properties": {
                "localTimeline": {"type": "boolean"}, "globalTimeline": {"type": "boolean"}, "miauth": {"type": "boolean"}
            }, "required": ["localTimeline", "globalTimeline", "miauth"]}}},
            "MetaClientOptions": {"type": "object", "properties": {
                "entrancePageStyle": {"type": "string"}, "showTimelineForVisitor": {"type": "boolean"}, "showActivitiesForVisitor": {"type": "boolean"}
            }, "required": ["entrancePageStyle", "showTimelineForVisitor", "showActivitiesForVisitor"]},
            "Announcement": {"type": "object", "properties": {"id": {"type": "string"}, "forYou": {"type": "boolean"}}, "required": ["id", "forYou"]},
            "QueueJob": {"type": "object", "properties": {
                "id": {"type": "string"}, "progress": {"type": "object"}, "returnValue": {"type": "object"}, "failedReason": {"type": "string"}
            }, "required": ["id", "progress", "returnValue", "failedReason"]},
            "RoleLite": {"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]},
            "Role": {"type": "object", "allOf": [{"$ref": "#/components/schemas/RoleLite"}, {"type": "object", "properties": {
                "policies": {"type": "object", "additionalProperties": {"anyOf": [{"type": "object", "properties": {
                    "value": {"oneOf": [{"type": "integer"}, {"type": "boolean"}]}, "priority": {"type": "integer"}, "useDefault": {"type": "boolean"}
                }}]}}
            }, "required": ["policies"]}]}
        }),
        json!({
            "/i/registry/get": post(None, ok(json!({"type": "object"}))),
            "/i/registry/get-all": post(None, ok(json!({"type": "object"}))),
            "/i/2fa/remove-key": post(None, json!({"204": {"description": "none"}})),
            "/i/2fa/update-key": post(None, ok(json!({"type": "object"})))
        }),
    );
    corrections::apply(&mut spec);
    let tree = build_tree(&spec).unwrap();
    let policies = find_struct(&tree.types, "RolePolicies");
    assert_eq!(field(policies, "rateLimitFactor").base, RustType::F64);
    assert_eq!(field(policies, "pinLimit").base, RustType::I64);
    let features = find_struct(&tree.types, "MetaDetailedOnlyFeatures");
    assert!(!field(features, "localTimeline").required);
    assert!(!field(features, "globalTimeline").required);
    assert!(field(features, "miauth").required);
    let options = find_struct(&tree.types, "MetaClientOptions");
    assert!(options.fields.iter().all(|f| !f.required));
    let announcement = find_struct(&tree.types, "Announcement");
    assert!(field(announcement, "id").required);
    assert!(!field(announcement, "forYou").required);
    let job = find_struct(&tree.types, "QueueJob");
    for name in ["progress", "returnValue"] {
        assert_eq!(field(job, name).base, RustType::Value);
        assert!(field(job, name).required);
    }
    assert!(field(job, "id").required);
    assert!(!field(job, "failedReason").required);
    let policy = find_struct(&tree.types, "RolePoliciesValue");
    assert_eq!(field(policy, "value").base, RustType::Value);
    assert_eq!(field(policy, "priority").base, RustType::I64);
    assert!(matches!(
        &endpoint(&tree, "i/registry/get").response,
        ResponseDef::Alias(RustType::Value)
    ));
    assert!(matches!(
        &endpoint(&tree, "i/registry/get-all").response,
        ResponseDef::Alias(RustType::JsonMap)
    ));
    assert!(matches!(
        &endpoint(&tree, "i/2fa/remove-key").response,
        ResponseDef::Struct
    ));
    // 上流が 200 を載せたら、補正せずにそのスキーマを使う。
    assert!(matches!(
        &endpoint(&tree, "i/2fa/update-key").response,
        ResponseDef::Alias(RustType::JsonMap)
    ));
}

// ストリーミングの型定義（ts-types.ts の JSON）の解決。

fn kw(name: &str) -> Value {
    json!({"kind": "keyword", "name": name})
}

fn lit(value: &str) -> Value {
    json!({"kind": "literal", "value": value})
}

fn refn(name: &str) -> Value {
    json!({"kind": "ref", "name": name, "args": []})
}

fn member(name: &str, optional: bool, ty: Value) -> Value {
    json!({"name": name, "optional": optional, "type": ty, "doc": null})
}

fn obj(members: Vec<Value>) -> Value {
    json!({"kind": "object", "members": members})
}

fn union(types: Vec<Value>) -> Value {
    json!({"kind": "union", "types": types})
}

/// `(payload: T) => void`。`None` なら `() => void`。
fn func(payload: Option<Value>) -> Value {
    let params: Vec<Value> = payload.into_iter().collect();
    json!({"kind": "function", "typeParams": [], "params": params})
}

fn indexed(object: Value, index: Value) -> Value {
    json!({"kind": "indexed", "object": object, "index": index})
}

fn channel(params: Value, events: Vec<Value>, receives: Value) -> Value {
    obj(vec![
        member("params", false, params),
        member("events", false, obj(events)),
        member("receives", false, receives),
    ])
}

fn note_spec() -> Spec {
    spec(
        json!({"Note": {"type": "object", "properties": {"id": {"type": "string"}, "text": {"type": ["string", "null"]}}, "required": ["id", "text"]}}),
        json!({}),
    )
}

fn streaming_tree(channels: Value, broadcast: Value, entities: Value) -> Result<Tree, String> {
    crate::resolve::build_tree(&note_spec(), &streaming_ts(channels, broadcast, entities))
}

fn find_tagged<'a>(items: &'a [Item], name: &str) -> &'a TaggedUnionDef {
    items
        .iter()
        .find_map(|item| match item {
            Item::Tagged(def) if def.name == name => Some(def),
            _ => None,
        })
        .unwrap_or_else(|| panic!("tagged union {name} が無い"))
}

fn payloads(def: &EnvelopeDef) -> Vec<(&str, Option<&RustType>)> {
    def.variants
        .iter()
        .map(|v| (v.ident.as_str(), v.payload.as_ref()))
        .collect()
}

#[test]
fn streaming_channels_have_params_events_and_messages() {
    let tree = streaming_tree(
        json!({
            "tl": channel(
                obj(vec![
                    member("withFiles", true, kw("boolean")),
                    json!({"name": "listId", "optional": false, "type": kw("string"), "doc": "リスト"}),
                ]),
                vec![
                    member("note", false, func(Some(refn("Note")))),
                    member("cleared", false, func(None)),
                    member("deleted", false, func(Some(indexed(refn("Note"), lit("id"))))),
                    member("report", false, obj(vec![member("id", false, kw("string"))])),
                ],
                obj(vec![
                    member("read", false, obj(vec![member("id", false, kw("string"))])),
                    member("cancel", false, union(vec![kw("null"), json!({"kind": "ref", "name": "Record", "args": [kw("string"), kw("never")]})])),
                ]),
            ),
            "main": channel(kw("null"), vec![], kw("null")),
        }),
        json!({}),
        json!([]),
    )
    .unwrap();
    let [tl, main] = tree.streaming.channels.as_slice() else {
        panic!("チャンネルが 2 つのはず");
    };
    assert_eq!((tl.name.as_str(), tl.ident.as_str()), ("tl", "tl"));
    assert_eq!(tl.params.kind, StructKind::Request);
    assert!(!field(&tl.params, "withFiles").required);
    let list_id = field(&tl.params, "listId");
    assert!(list_id.required);
    assert_eq!(list_id.docs, vec!["リスト"]);
    assert_eq!(
        payloads(&tl.events),
        vec![
            ("Note", Some(&component("Note"))),
            ("Cleared", None),
            ("Deleted", Some(&RustType::String)),
            ("Report", Some(&RustType::Local("Report".into()))),
        ]
    );
    assert!(tl.events.catch_all);
    let messages = tl.messages.as_ref().unwrap();
    assert_eq!(
        payloads(messages),
        vec![
            ("Read", Some(&RustType::Local("Read".into()))),
            ("Cancel", None)
        ]
    );
    assert!(!messages.catch_all);
    assert_eq!(find_struct(&tl.items, "Report").kind, StructKind::Entity);
    let read = find_struct(&tl.items, "Read");
    assert_eq!(read.kind, StructKind::Request);
    assert!(read.derive_default);
    assert!(main.params.fields.is_empty());
    assert!(main.params.derive_default);
    assert!(main.messages.is_none());
    assert!(tree.streaming.types.is_empty());
}

#[test]
fn streaming_named_types_are_generated_once_in_streaming_types() {
    let page_event = json!({"name": "PageEvent", "typeParams": [], "type": obj(vec![
        member("pageId", false, indexed(refn("Note"), lit("id"))),
        member("var", false, kw("any")),
        member("note", false, refn("Note")),
    ])});
    let tree = streaming_tree(
        json!({
            "a": channel(kw("null"), vec![member("page", false, func(Some(refn("PageEvent")))), member("key", false, func(Some(refn("Keys"))))], kw("null")),
            "b": channel(kw("null"), vec![member("page", false, func(Some(refn("PageEvent"))))], kw("null")),
        }),
        json!({"page": func(Some(refn("PageEvent")))}),
        json!([page_event]),
    )
    .unwrap();
    let names: Vec<&str> = tree.streaming.types.iter().map(Item::name).collect();
    assert_eq!(names, vec!["PageEvent", "Keys"]);
    let page = find_struct(&tree.streaming.types, "PageEvent");
    assert_eq!(field(page, "pageId").base, RustType::String);
    assert_eq!(field(page, "var").base, RustType::Value);
    assert_eq!(field(page, "note").base, component("Note"));
    let keys = find_enum(&tree.streaming.types, "Keys");
    assert_eq!(
        keys.variants,
        vec![("A".into(), "a".into()), ("B".into(), "b".into())]
    );
    assert!(keys.other);
    for channel in &tree.streaming.channels {
        assert_eq!(
            channel.events.variants[0].payload,
            Some(RustType::Streaming("PageEvent".into()))
        );
    }
    let broadcast = tree
        .streaming
        .items
        .iter()
        .find_map(|item| match item {
            Item::Envelope(def) => Some(def),
            _ => None,
        })
        .unwrap();
    assert_eq!(broadcast.name, "BroadcastEvent");
    assert_eq!(
        payloads(broadcast),
        vec![("Page", Some(&RustType::Streaming("PageEvent".into())))]
    );
}

#[test]
fn streaming_unions_become_enums_unions_and_tagged_unions() {
    let tree = streaming_tree(
        json!({
            "c": channel(kw("null"), vec![
                member("mode", false, func(Some(union(vec![lit("x"), lit("y")])))),
                member("maybe", false, func(Some(union(vec![kw("null"), kw("string")])))),
                member("either", false, func(Some(union(vec![refn("Note"), obj(vec![member("a", false, kw("string")), member("b", false, kw("number")), member("c", false, kw("boolean"))])])))),
                member("tagged", false, func(Some(union(vec![
                    obj(vec![member("type", false, lit("x")), member("body", false, obj(vec![member("a", false, kw("string"))]))]),
                    obj(vec![member("type", false, lit("y"))]),
                ])))),
                member("spread", false, func(Some(json!({"kind": "intersection", "types": [
                    obj(vec![member("id", false, kw("string"))]),
                    union(vec![obj(vec![member("type", false, lit("x"))]), obj(vec![member("type", false, lit("y"))])]),
                ]})))),
            ], kw("null")),
        }),
        json!({}),
        json!([]),
    )
    .unwrap();
    let c = &tree.streaming.channels[0];
    let mode = find_enum(&c.items, "Mode");
    assert_eq!(mode.variants.len(), 2);
    assert!(mode.other);
    assert_eq!(
        payloads(&c.events)[1],
        ("Maybe", Some(&RustType::Option(Box::new(RustType::String))))
    );
    // object literal のほうが必須プロパティが多いので先になる。
    let either = find_union(&c.items, "Either");
    assert_eq!(variant_names(either), vec!["Object", "Note"]);
    assert_eq!(either.variants[0].1, RustType::Local("EitherObject".into()));
    let tagged = find_tagged(&c.items, "Tagged");
    let variants: Vec<(&str, &str)> = tagged
        .variants
        .iter()
        .map(|v| (v.ident.as_str(), v.value.as_str()))
        .collect();
    assert_eq!(variants, vec![("X", "x"), ("Y", "y")]);
    assert_eq!(tagged.variants[0].ty, RustType::Local("TaggedX".into()));
    assert_eq!(
        field(find_struct(&c.items, "TaggedX"), "body").base,
        RustType::Local("TaggedXBody".into())
    );
    assert!(find_struct(&c.items, "TaggedY").fields.is_empty());
    let spread = find_tagged(&c.items, "Spread");
    assert_eq!(spread.variants.len(), 2);
    assert_eq!(
        field(find_struct(&c.items, "SpreadX"), "id").base,
        RustType::String
    );
}

#[test]
fn streaming_unions_expand_union_components_into_their_members() {
    // `UserDetailed | UserLite` のような union は、union のコンポーネントを中身に展開してから、
    // 必須プロパティの多い順に並べる。展開しないと、UserDetailed の受け皿が UserLite の値まで受けてしまう。
    let spec = spec(
        json!({
            "Lite": {"type": "object", "properties": {"id": {"type": "string"}}, "required": ["id"]},
            "NotMe": {"type": "object", "properties": {"id": {"type": "string"}, "a": {"type": "string"}, "b": {"type": "string"}}, "required": ["id", "a", "b"]},
            "Me": {"type": "object", "properties": {"id": {"type": "string"}, "a": {"type": "string"}}, "required": ["id", "a"]},
            "Detailed": {"oneOf": [{"$ref": "#/components/schemas/NotMe"}, {"$ref": "#/components/schemas/Me"}]},
        }),
        json!({}),
    );
    let ts = serde_json::from_value(json!({
        "streaming.types.ts": {
            "imports": [{"name": "Lite", "from": "./autogen/models.js"}, {"name": "Detailed", "from": "./autogen/models.js"}],
            "reexports": [],
            "types": [
                {"name": "Channels", "typeParams": [], "type": obj(vec![member("c", false, channel(kw("null"), vec![member("followed", false, func(Some(union(vec![refn("Lite"), refn("Detailed")]))))], kw("null")))])},
                {"name": "BroadcastEvents", "typeParams": [], "type": obj(vec![])},
            ],
            "consts": [],
        },
    }))
    .unwrap();
    let tree = crate::resolve::build_tree(&spec, &ts).unwrap();
    let followed = find_union(&tree.streaming.channels[0].items, "Followed");
    assert_eq!(variant_names(followed), vec!["NotMe", "Me", "Lite"]);
    assert_eq!(followed.variants[2].1, component("Lite"));
}

#[test]
fn streaming_generics_substitute_type_arguments_and_constraints() {
    let generic = json!({"name": "Generic", "typeParams": [{"name": "K", "constraint": null}], "type": obj(vec![
        member("key", false, refn("K")),
        member("value", false, indexed(refn("Note"), refn("K"))),
    ])});
    let constrained = json!({"kind": "function", "typeParams": [{"name": "K", "constraint": refn("Keys")}], "params": [obj(vec![member("key", false, refn("K"))])]});
    let tree = streaming_tree(
        json!({
            "g": channel(kw("null"), vec![
                member("settings", false, func(Some(json!({"kind": "ref", "name": "Generic", "args": [refn("Keys")]})))),
                member("update", false, constrained),
            ], kw("null")),
        }),
        json!({}),
        json!([generic]),
    )
    .unwrap();
    let g = &tree.streaming.channels[0];
    let settings = find_struct(&g.items, "Generic");
    assert_eq!(
        field(settings, "key").base,
        RustType::Streaming("Keys".into())
    );
    assert_eq!(field(settings, "value").base, RustType::Value);
    assert_eq!(
        field(find_struct(&g.items, "Update"), "key").base,
        RustType::Streaming("Keys".into())
    );
    assert_eq!(
        g.events.variants[0].payload,
        Some(RustType::Local("Generic".into()))
    );
}

#[test]
fn streaming_definitions_are_validated() {
    let cases: Vec<(Value, Value, Value, &str)> = vec![
        (
            json!({"c": channel(kw("null"), vec![member("e", false, func(Some(refn("Nope"))))], kw("null"))}),
            json!({}),
            json!([]),
            "名前 `Nope` が無い",
        ),
        (
            json!({"c": channel(kw("null"), vec![member("e", false, func(Some(json!({"kind": "unsupported", "syntax": "MappedType", "text": "{ [K in T]: 1 }"}))))], kw("null"))}),
            json!({}),
            json!([]),
            "対応していない構文 MappedType",
        ),
        (
            json!({"connect": channel(kw("null"), vec![], kw("null"))}),
            json!({}),
            json!([]),
            "Stream のメソッドと衝突する",
        ),
        (
            json!({"c": channel(kw("null"), vec![member("params", false, func(Some(obj(vec![]))))], kw("null"))}),
            json!({}),
            json!([]),
            "固定のアイテムと衝突する",
        ),
        (
            json!({"c": channel(kw("null"), vec![member("unrecognized", false, func(None))], kw("null"))}),
            json!({}),
            json!([]),
            "受け皿の `Unrecognized` と衝突する",
        ),
        (
            json!({"c": channel(kw("string"), vec![], kw("null"))}),
            json!({}),
            json!([]),
            "params は null か object",
        ),
        (
            json!({"c": channel(kw("null"), vec![member("e", false, func(Some(indexed(refn("Note"), lit("nope")))))], kw("null"))}),
            json!({}),
            json!([]),
            "プロパティ `nope` が無い",
        ),
        (
            json!({"c": channel(kw("null"), vec![member("e", false, func(Some(refn("PageEvent"))))], kw("null"))}),
            json!({}),
            json!([{"name": "PageEvent", "typeParams": [], "type": union(vec![obj(vec![member("a", false, kw("string"))]), kw("null")])}]),
            "null を含む型を、同じ名前の構造体・enum にすることには対応していない",
        ),
        (
            json!({"c": channel(kw("null"), vec![member("key", false, func(Some(refn("Keys")))), member("page", false, func(Some(refn("PageEvent"))))], kw("null"))}),
            json!({}),
            json!([
                {"name": "Keys", "typeParams": [], "type": obj(vec![member("a", false, kw("string"))])},
                {"name": "PageEvent", "typeParams": [], "type": obj(vec![member("key", false, refn("Keys"))])},
            ]),
            "型名 `Keys` が consts.ts と entities.ts の両方で宣言されている",
        ),
    ];
    for (channels, broadcast, entities, expected) in cases {
        let error = streaming_tree(channels, broadcast, entities).unwrap_err();
        assert!(error.contains(expected), "{error}");
    }
}

#[test]
fn streaming_tokens_implement_channel_and_envelope_codecs() {
    let tree = streaming_tree(
        json!({
            "tl": channel(
                obj(vec![member("listId", false, kw("string"))]),
                vec![member("note", false, func(Some(refn("Note")))), member("cleared", false, func(None))],
                obj(vec![member("read", false, obj(vec![member("id", false, kw("string"))]))]),
            ),
        }),
        json!({"updated": func(Some(union(vec![
            obj(vec![member("type", false, lit("x")), member("id", false, kw("string"))]),
            obj(vec![member("type", false, lit("y")), member("id", false, kw("string"))]),
        ])))}),
        json!([]),
    )
    .unwrap();
    let module = source_of(&tree, "mod.rs");
    assert!(
        module.contains("#[cfg(feature=\"streaming\")]pubmodstreaming;"),
        "{module}"
    );
    let streaming = source_of(&tree, "streaming/mod.rs");
    assert!(
        streaming.contains("pubtypeStream=crate::stream::Stream<super::Version>;"),
        "{streaming}"
    );
    assert!(
        streaming.contains("pubfntl(&self)->tl::Connect<'_>{tl::Connect::new(self)}"),
        "{streaming}"
    );
    assert!(
        streaming.contains("pubfnbroadcast(&self)->crate::stream::Receiver<BroadcastEvent>"),
        "{streaming}"
    );
    assert!(
        streaming.contains(
            "object.insert(\"type\".to_owned(),::serde_json::Value::String(\"x\".to_owned()));"
        ),
        "{streaming}"
    );
    assert!(
        streaming.contains("_=>Updated::Unrecognized(value),"),
        "{streaming}"
    );
    let tl = source_of(&tree, "streaming/tl.rs");
    assert!(tl.contains("implcrate::stream::ChannelforParams{constNAME:&'staticstr=\"tl\";typeEvent=Event;typeMessage=Message;}"), "{tl}");
    assert!(tl.contains("pubenumEvent{Note(crate::generated::v2024_10_1::types::Note),Cleared,#[doc=\"このバージョンのバインディングが知らないイベント。\"]Unrecognized{r#type:::std::string::String,body:::serde_json::Value},}"), "{tl}");
    assert!(tl.contains("\"cleared\"=>Event::Cleared,"), "{tl}");
    assert!(tl.contains("_=>Event::Unrecognized{r#type,body},"), "{tl}");
    assert!(
        tl.contains("Event::Cleared=>(\"cleared\",::serde_json::Value::Null),"),
        "{tl}"
    );
    assert!(
        tl.contains("::serde::de::Error::unknown_variant(&r#type,&[\"read\"])"),
        "{tl}"
    );
    assert!(
        tl.contains("pubtypeConnection=crate::stream::Connection<Params>;"),
        "{tl}"
    );
    assert!(tl.contains("pubstructConnect<'a,S=(crate::builder::Unset,)>{stream:&'acrate::generated::v2024_10_1::streaming::Stream,"), "{tl}");
    assert!(tl.contains("letrequest=Params{list_id:self.list_id.expect(\"requiredparameterissetbythetypestate\"),};letstream=self.stream;::std::boxed::Box::pin(asyncmove{stream.connect(request).await})"), "{tl}");
    let no_message = streaming_tree(
        json!({"m": channel(kw("null"), vec![], kw("null"))}),
        json!({}),
        json!([]),
    )
    .unwrap();
    assert!(
        source_of(&no_message, "streaming/m.rs").contains("typeMessage=crate::stream::NoMessage;")
    );
}

/// `NoteUpdatedEvent` の reacted の `emoji` と、`main` の `urlUploadFinished` の `marker` を、与えた型にした型定義。
fn reaction_and_upload_ts(emoji: Value, marker: Value) -> ts::Files {
    let reacted = obj(vec![
        member("type", false, lit("reacted")),
        member(
            "body",
            false,
            obj(vec![
                member("reaction", false, kw("string")),
                member("emoji", false, emoji),
            ]),
        ),
    ]);
    let unreacted = obj(vec![
        member("type", false, lit("unreacted")),
        member(
            "body",
            false,
            obj(vec![member("reaction", false, kw("string"))]),
        ),
    ]);
    let note_updated = json!({"kind": "intersection", "types": [obj(vec![member("id", false, kw("string"))]), union(vec![reacted, unreacted])]});
    let upload = func(Some(obj(vec![
        member("marker", false, marker),
        member("file", false, refn("Note")),
    ])));
    streaming_ts(
        json!({"main": channel(kw("null"), vec![member("urlUploadFinished", false, upload)], kw("null"))}),
        json!({}),
        json!([{"name": "NoteUpdatedEvent", "typeParams": [], "type": note_updated}]),
    )
}

fn declared_type<'a>(files: &'a ts::Files, file: &str, name: &str) -> &'a ts::TypeNode {
    &files[file]
        .types
        .iter()
        .find(|decl| decl.name == name)
        .unwrap()
        .ty
}

#[test]
fn streaming_corrections_match_what_servers_send() {
    let string_or_null = || union(vec![kw("string"), kw("null")]);
    // 2025.10.0 までの misskey-js の形。
    let mut files = reaction_and_upload_ts(string_or_null(), kw("string"));
    corrections::apply_streaming(&mut files);
    let emoji = union(vec![
        obj(vec![
            member("name", false, kw("string")),
            member("url", false, kw("string")),
        ]),
        kw("null"),
    ]);
    let expected = reaction_and_upload_ts(emoji, string_or_null());
    for (file, name) in [
        ("entities.ts", "NoteUpdatedEvent"),
        ("streaming.types.ts", "Channels"),
    ] {
        assert_eq!(
            declared_type(&files, file, name),
            declared_type(&expected, file, name)
        );
    }
}
