use serde_json::{Value, json};

use crate::corrections;
use crate::emit;
use crate::model::*;
use crate::resolve::build_tree;
use crate::spec::Spec;

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
        assert!(field(a, name).nullable, "{name} should be nullable");
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
    assert!(
        notes.contains(
            "impl<'a>::std::future::IntoFuture forCall<'a,(crate::builder::Set,)>"
                .replace(' ', "")
                .as_str()
        )
    );
    assert!(notes.contains("pubfnshow(&self)->show::Call<'a>"));
    assert!(notes.contains("pubfnlimit(mutself,value:impl::std::convert::Into<i64>)->Self"));
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
}

#[test]
fn corrections_relax_what_servers_do_not_honor() {
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
        json!({}),
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
}
