//! misskey-js のストリーミングの型定義（`ts-types.ts` が読んだ構文木）を中間表現に変換する。
//! 決めごとは AGENTS.md「構成」にある。

use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

use indexmap::IndexMap;
use serde_json::Value;

use crate::model::*;
use crate::naming::{ident, pascal, snake};
use crate::resolve::{
    CATCH_ALL, Ctx, Resolver, Scope, check_fields, claim_variant, derives_default,
};
use crate::ts::{ConstDecl, File, Files, Member, TypeDecl, TypeNode};

type Result<T> = std::result::Result<T, String>;

const STREAMING_FILE: &str = "streaming.types.ts";
/// misskey-js で、`api.json` の `components.schemas` から生成した型のモジュール。
const MODELS_MODULE: &str = "./autogen/models.js";
/// `Stream` のメソッド名。実装している `Clone` の `clone` も含む。
/// 生成するチャンネルのアクセサは、これらと同じ名前にできない。
const RESERVED_STREAM_METHODS: &[&str] = &[
    "connect",
    "broadcast",
    "broadcast_receiver",
    "subscribe_note",
    "unsubscribe_note",
    "send_raw",
    "state",
    "close",
    "clone",
];
/// チャンネルのモジュールで、生成するアイテムに使えない名前。
const RESERVED_CHANNEL_ITEMS: &[&str] = &["Params", "Event", "Message", "Connect", "Connection"];

/// 型を読んでいる場所。型パラメータの置換は、置換する型と、その型を読むファイルの組で持つ。
#[derive(Clone)]
struct Place {
    file: String,
    env: HashMap<String, (String, TypeNode)>,
}

impl Place {
    fn new(file: &str) -> Self {
        Place {
            file: file.to_owned(),
            env: HashMap::new(),
        }
    }
}

enum Found<'f> {
    Component(String),
    Type(&'f str, &'f TypeDecl),
    Const(&'f ConstDecl),
}

/// untagged の union のメンバー。misskey-js の型か、展開した `api.json` のスキーマ。
enum UnionMember<'n, 'a> {
    Ts(&'n TypeNode),
    Schema(&'a crate::spec::Schema),
}

struct Streaming<'r, 'a, 'f> {
    resolver: &'r mut Resolver<'a>,
    files: &'f Files,
    /// `streaming/types.rs`。
    types: Ctx,
    /// `types` に生成した（しようとしている）名前付きの型と、その宣言のファイル。
    resolved: HashMap<String, &'f str>,
}

pub(crate) fn build(resolver: &mut Resolver<'_>, files: &Files) -> Result<StreamingDef> {
    let root = file_of(files, STREAMING_FILE)?;
    let mut streaming = Streaming {
        resolver,
        files,
        types: Ctx::new(Scope::Streaming, StructKind::Entity),
        resolved: HashMap::new(),
    };
    let channels_decl = declaration(root, "Channels")?;
    let TypeNode::Object { members } = &channels_decl.ty else {
        return Err("Channels が object でない".to_owned());
    };
    let mut channels = Vec::new();
    for member in members {
        channels.push(
            streaming
                .channel(member)
                .map_err(|e| format!("チャンネル {}: {e}", member.name))?,
        );
    }
    validate_channels(&channels)?;
    let broadcast_decl = declaration(root, "BroadcastEvents")?;
    let TypeNode::Object { members } = &broadcast_decl.ty else {
        return Err("BroadcastEvents が object でない".to_owned());
    };
    let mut cx = Ctx::new(Scope::Module, StructKind::Entity);
    let docs = vec![
        "ストリーム全体のイベント（misskey-js の `BroadcastEvents`）。".to_owned(),
        "`noteUpdated` は、`Stream::subscribe_note` で購読したノートの分だけ届く。".to_owned(),
    ];
    let broadcast = streaming
        .envelope(
            &mut cx,
            &Place::new(STREAMING_FILE),
            "BroadcastEvent",
            docs,
            members,
            true,
        )
        .map_err(|e| format!("BroadcastEvents: {e}"))?;
    cx.push(Item::Envelope(broadcast))?;
    Ok(StreamingDef {
        channels,
        types: streaming.types.items,
        items: cx.items,
    })
}

fn file_of<'f>(files: &'f Files, name: &str) -> Result<(&'f str, &'f File)> {
    files
        .get_key_value(name)
        .map(|(key, file)| (key.as_str(), file))
        .ok_or_else(|| format!("{name} が読めていない"))
}

fn declaration<'f>(file: (&'f str, &'f File), name: &str) -> Result<&'f TypeDecl> {
    file.1
        .types
        .iter()
        .find(|decl| decl.name == name)
        .ok_or_else(|| format!("{} に {name} が無い", file.0))
}

fn validate_channels(channels: &[ChannelDef]) -> Result<()> {
    let mut seen = HashSet::new();
    for channel in channels {
        if !seen.insert(&channel.ident) {
            return Err(format!(
                "複数のチャンネルが同じモジュール `{}` になる",
                channel.ident
            ));
        }
        if RESERVED_STREAM_METHODS.contains(&channel.ident.as_str()) {
            return Err(format!(
                "チャンネル `{}` が Stream のメソッドと衝突する",
                channel.name
            ));
        }
    }
    Ok(())
}

/// 本文の無いイベント・メッセージの型。`null` か、`null` と空の object（`Record<string, never>`）の union。
fn no_body(node: &TypeNode) -> bool {
    match node {
        TypeNode::Union { types } => types.iter().all(|t| is_null(t) || is_empty_record(t)),
        other => is_null(other),
    }
}

fn is_empty_record(node: &TypeNode) -> bool {
    matches!(
        node,
        TypeNode::Ref { name, args }
            if name == "Record"
                && args.len() == 2
                && matches!(&args[1], TypeNode::Keyword { name } if name == "never")
    )
}

fn is_null(node: &TypeNode) -> bool {
    matches!(node, TypeNode::Keyword { name } if name == "null")
}

fn string_literal(node: &TypeNode) -> Option<&str> {
    match node {
        TypeNode::Literal {
            value: Value::String(value),
        } => Some(value),
        _ => None,
    }
}

fn wrap((ty, nullable): (RustType, bool)) -> RustType {
    if nullable { ty.option() } else { ty }
}

/// 全部の object に `type` が必須で入っていて、型が文字列リテラルで、値が object ごとに違うなら、object ごとのその値。
fn tag_values(objects: &[&[Member]]) -> Option<Vec<String>> {
    let values: Vec<String> = objects
        .iter()
        .filter_map(|object| {
            object
                .iter()
                .find(|m| m.name == "type")
                .filter(|m| !m.optional)
                .and_then(|m| string_literal(&m.ty))
                .map(str::to_owned)
        })
        .collect();
    let distinct: HashSet<&String> = values.iter().collect();
    (values.len() == objects.len() && distinct.len() == values.len()).then_some(values)
}

/// intersection の object 同士のプロパティを併合する。同名で型が違えばエラー。
fn merge_members(objects: &[&[Member]]) -> Result<Vec<Member>> {
    let mut merged: IndexMap<String, Member> = IndexMap::new();
    for object in objects {
        for member in object.iter() {
            match merged.get_mut(&member.name) {
                Some(existing) => {
                    if existing.ty != member.ty {
                        return Err(format!(
                            "intersection のメンバーの間で、プロパティ `{}` の型が食い違っている",
                            member.name
                        ));
                    }
                    existing.optional &= member.optional;
                    if existing.doc.is_none() {
                        existing.doc = member.doc.clone();
                    }
                }
                None => {
                    merged.insert(member.name.clone(), member.clone());
                }
            }
        }
    }
    Ok(merged.into_values().collect())
}

impl<'r, 'a, 'f> Streaming<'r, 'a, 'f> {
    fn channel(&mut self, member: &'f Member) -> Result<ChannelDef> {
        let name = member.name.clone();
        let TypeNode::Object { members } = &member.ty else {
            return Err("定義が object でない".to_owned());
        };
        let part = |key: &str| {
            members
                .iter()
                .find(|m| m.name == key)
                .map(|m| &m.ty)
                .ok_or_else(|| format!("`{key}` が無い"))
        };
        let (params, events, receives) = (part("params")?, part("events")?, part("receives")?);
        let place = Place::new(STREAMING_FILE);
        let mut cx = Ctx::new(Scope::Module, StructKind::Request);
        let params_members: &[Member] = match params {
            TypeNode::Object { members } => members,
            node if is_null(node) => &[],
            _ => return Err("params は null か object でなければならない".to_owned()),
        };
        let mut params = self
            .struct_def(&mut cx, &place, "Params", params_members)
            .map_err(|e| format!("params: {e}"))?;
        params.docs = vec![format!("`{name}` につなぐときの params。")];
        cx.kind = StructKind::Entity;
        let events = match events {
            TypeNode::Object { members } => self
                .envelope(
                    &mut cx,
                    &place,
                    "Event",
                    vec![format!("`{name}` から届くイベント。")],
                    members,
                    true,
                )
                .map_err(|e| format!("events: {e}"))?,
            _ => return Err("events は object でなければならない".to_owned()),
        };
        cx.kind = StructKind::Request;
        let messages = match receives {
            node if is_null(node) => None,
            TypeNode::Object { members } => Some(
                self.envelope(
                    &mut cx,
                    &place,
                    "Message",
                    vec![format!("`{name}` へ送るメッセージ。")],
                    members,
                    false,
                )
                .map_err(|e| format!("receives: {e}"))?,
            ),
            _ => return Err("receives は null か object でなければならない".to_owned()),
        };
        if let Some(item) = cx
            .items
            .iter()
            .find(|item| RESERVED_CHANNEL_ITEMS.contains(&item.name()))
        {
            return Err(format!(
                "型名 `{}` がチャンネルのモジュールの固定のアイテムと衝突する",
                item.name()
            ));
        }
        Ok(ChannelDef {
            ident: snake(&name),
            name,
            params,
            events,
            messages,
            items: cx.items,
        })
    }

    /// `{type, body}` の形の enum。`members` の名前が `type`、値が本文の型である。
    fn envelope(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        name: &str,
        docs: Vec<String>,
        members: &[Member],
        catch_all: bool,
    ) -> Result<EnvelopeDef> {
        let mut variants = Vec::new();
        let mut used = HashSet::new();
        for member in members {
            let variant = pascal(&member.name);
            claim_variant(&mut used, &variant, catch_all)
                .map_err(|reason| format!("`{}` が{reason}", member.name))?;
            // 関数型なら第 1 引数が本文。型パラメータは、その制約に置き換える。
            let mut place = place.clone();
            let payload = match &member.ty {
                TypeNode::Function {
                    type_params,
                    params,
                } => {
                    for param in type_params {
                        let constraint = param.constraint.clone().unwrap_or(TypeNode::Keyword {
                            name: "any".to_owned(),
                        });
                        place
                            .env
                            .insert(param.name.clone(), (place.file.clone(), constraint));
                    }
                    params.first()
                }
                other => Some(other),
            };
            let payload = match payload {
                Some(node) if !no_body(node) => {
                    let resolved = self
                        .resolve(cx, &place, node, &variant)
                        .map_err(|e| format!("{}: {e}", member.name))?;
                    Some(wrap(resolved))
                }
                _ => None,
            };
            variants.push(EnvelopeVariant {
                ident: variant,
                json_name: member.name.clone(),
                docs: member.doc.iter().cloned().collect(),
                payload,
            });
        }
        Ok(EnvelopeDef {
            name: name.to_owned(),
            docs,
            variants,
            catch_all,
        })
    }

    fn struct_def(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        name: &str,
        members: &[Member],
    ) -> Result<StructDef> {
        let mut fields = Vec::new();
        for member in members {
            let hint = format!("{name}{}", pascal(&member.name));
            let (base, nullable) = self
                .resolve(cx, place, &member.ty, &hint)
                .map_err(|e| format!("{name}.{}: {e}", member.name))?;
            fields.push(Field {
                ident: snake(&member.name),
                json_name: member.name.clone(),
                docs: member.doc.iter().cloned().collect(),
                required: !member.optional,
                nullable,
                base,
            });
        }
        check_fields(name, &fields)?;
        let derive_default = derives_default(cx.kind, &fields, &cx.items);
        Ok(StructDef {
            name: name.to_owned(),
            docs: Vec::new(),
            kind: cx.kind,
            fields,
            derive_default,
        })
    }

    fn make_struct(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        name: &str,
        members: &[Member],
    ) -> Result<RustType> {
        let def = self.struct_def(cx, place, name, members)?;
        cx.push(Item::Struct(def))?;
        Ok(cx.named(name))
    }

    /// 型を解決し、null も許されるかどうかを併せて返す。
    fn resolve(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        node: &TypeNode,
        hint: &str,
    ) -> Result<(RustType, bool)> {
        match node {
            TypeNode::Keyword { name } => Ok(match name.as_str() {
                "string" => (RustType::String, false),
                "number" => (RustType::F64, false),
                "boolean" => (RustType::Bool, false),
                "any" | "unknown" => (RustType::Value, false),
                "null" => (RustType::Value, true),
                other => return Err(format!("型 `{other}` には対応していない")),
            }),
            TypeNode::Literal { value } => match value {
                Value::String(value) => {
                    let ty = self.resolver.make_str_enum(
                        cx,
                        hint,
                        vec![value.clone()],
                        false,
                        Vec::new(),
                    )?;
                    Ok((ty, false))
                }
                Value::Bool(_) => Ok((RustType::Bool, false)),
                Value::Number(_) => Ok((RustType::F64, false)),
                other => Err(format!("リテラル `{other}` には対応していない")),
            },
            TypeNode::Ref { name, args } => self.resolve_ref(cx, place, name, args, hint),
            TypeNode::Object { members } => {
                Ok((self.make_struct(cx, place, hint, members)?, false))
            }
            TypeNode::Union { types } => self.resolve_union(cx, place, types, hint),
            TypeNode::Intersection { types } => self.resolve_intersection(cx, place, types, hint),
            TypeNode::Array { element } => {
                let element = self.resolve(cx, place, element, &format!("{hint}Item"))?;
                Ok((RustType::Vec(Box::new(wrap(element))), false))
            }
            TypeNode::Indexed { object, index } => {
                self.resolve_indexed(cx, place, object, index, hint)
            }
            TypeNode::Function { .. } => Err("関数型は events の定義でだけ使える".to_owned()),
            TypeNode::Typeof { .. } => {
                Err("typeof は `[number]` と組み合わせたときだけ対応している".to_owned())
            }
            TypeNode::Unsupported { syntax, text } => {
                Err(format!("対応していない構文 {syntax}: {text}"))
            }
        }
    }

    fn resolve_ref(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        name: &str,
        args: &[TypeNode],
        hint: &str,
    ) -> Result<(RustType, bool)> {
        if let Some((file, substituted)) = place.env.get(name) {
            return self.resolve(cx, &Place::new(file), substituted, hint);
        }
        if name == "Record" {
            let [key, value] = args else {
                return Err("Record の型引数は 2 つ".to_owned());
            };
            if !matches!(key, TypeNode::Keyword { name } if name == "string") {
                return Err("Record のキーは string でなければならない".to_owned());
            }
            let ty = match value {
                TypeNode::Keyword { name }
                    if matches!(name.as_str(), "never" | "unknown" | "any") =>
                {
                    RustType::JsonMap
                }
                other => {
                    let value = self.resolve(cx, place, other, &format!("{hint}Value"))?;
                    RustType::Map(Box::new(wrap(value)))
                }
            };
            return Ok((ty, false));
        }
        match self.lookup(&place.file, name)? {
            Found::Component(component) => Ok((RustType::Component(component), false)),
            Found::Const(decl) => Err(format!(
                "定数 `{}` は typeof と `[number]` を組み合わせたときだけ使える",
                decl.name
            )),
            Found::Type(file, decl) => {
                if decl.type_params.len() != args.len() {
                    return Err(format!(
                        "`{}` の型引数の数が合わない（{} に対して {}）",
                        decl.name,
                        decl.type_params.len(),
                        args.len()
                    ));
                }
                if decl.type_params.is_empty() {
                    return Ok((self.resolve_named(file, decl)?, false));
                }
                // generic な alias は、型引数を置き換えて、使った側のモジュールにその名前で生成する。
                let env = decl
                    .type_params
                    .iter()
                    .zip(args)
                    .map(|(param, arg)| (param.name.clone(), (place.file.clone(), arg.clone())))
                    .collect();
                let inner = Place {
                    file: file.to_owned(),
                    env,
                };
                self.resolve(cx, &inner, &decl.ty, &decl.name)
            }
        }
    }

    /// generic でない名前付きの型を、`streaming/types.rs` に 1 回だけ生成する。
    fn resolve_named(&mut self, file: &'f str, decl: &'f TypeDecl) -> Result<RustType> {
        let ty = RustType::Streaming(decl.name.clone());
        if let Some(&first) = self.resolved.get(&decl.name) {
            return if first == file {
                Ok(ty)
            } else {
                Err(format!(
                    "型名 `{}` が {first} と {file} の両方で宣言されている",
                    decl.name
                ))
            };
        }
        self.resolved.insert(decl.name.clone(), file);
        // self.resolve には self.types を渡せないので、別の Ctx に作ってから移す（移すときに名前の衝突を検査する）。
        // 入れ子の名前付きの型は、その間に self.types へ直に入る。
        let mut cx = Ctx::new(Scope::Streaming, StructKind::Entity);
        let (resolved, nullable) = self
            .resolve(&mut cx, &Place::new(file), &decl.ty, &decl.name)
            .map_err(|e| format!("{}: {e}", decl.name))?;
        for item in cx.items {
            self.types.push(item)?;
        }
        if resolved != ty {
            self.types.push(Item::Alias(AliasDef {
                name: decl.name.clone(),
                docs: Vec::new(),
                ty: wrap((resolved, nullable)),
            }))?;
        } else if nullable {
            return Err(format!(
                "{}: null を含む型を、同じ名前の構造体・enum にすることには対応していない",
                decl.name
            ));
        }
        Ok(ty)
    }

    fn resolve_union(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        types: &[TypeNode],
        hint: &str,
    ) -> Result<(RustType, bool)> {
        let nullable = types.iter().any(is_null);
        let members: Vec<&TypeNode> = types.iter().filter(|t| !is_null(t)).collect();
        if members.is_empty() {
            return Ok((RustType::Value, true));
        }
        let literals: Vec<String> = members
            .iter()
            .filter_map(|m| string_literal(m).map(str::to_owned))
            .collect();
        if literals.len() == members.len() {
            let other = literals.len() >= 2;
            let ty = self
                .resolver
                .make_str_enum(cx, hint, literals, other, Vec::new())?;
            return Ok((ty, nullable));
        }
        if let [member] = members.as_slice() {
            let (ty, inner) = self.resolve(cx, place, member, hint)?;
            return Ok((ty, nullable || inner));
        }
        let objects: Vec<&[Member]> = members
            .iter()
            .filter_map(|m| match m {
                TypeNode::Object { members } => Some(members.as_slice()),
                _ => None,
            })
            .collect();
        if objects.len() == members.len()
            && let Some(values) = tag_values(&objects)
        {
            let ty = self.make_tagged(cx, place, hint, &objects, values)?;
            return Ok((ty, nullable));
        }
        Ok((self.make_union(cx, place, hint, members)?, nullable))
    }

    fn make_tagged(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        name: &str,
        objects: &[&[Member]],
        values: Vec<String>,
    ) -> Result<RustType> {
        let mut variants = Vec::new();
        let mut used = HashSet::new();
        for (object, value) in objects.iter().zip(values) {
            let variant = pascal(&value);
            claim_variant(&mut used, &variant, true)
                .map_err(|reason| format!("union {name}: `type` の値 `{value}` が{reason}"))?;
            let rest: Vec<Member> = object
                .iter()
                .filter(|m| m.name != "type")
                .cloned()
                .collect();
            let ty = self.make_struct(cx, place, &format!("{name}{variant}"), &rest)?;
            variants.push(TaggedVariant {
                ident: variant,
                value,
                ty,
            });
        }
        cx.push(Item::Tagged(TaggedUnionDef {
            name: name.to_owned(),
            docs: Vec::new(),
            variants,
        }))?;
        Ok(cx.named(name))
    }

    fn make_union(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        name: &str,
        members: Vec<&TypeNode>,
    ) -> Result<RustType> {
        // union のコンポーネントを指すメンバーは、その中身に展開する（REST の union と同じ理由。
        // union は受け皿を持ってどんな値でも読めるので、バリアントにするとそれより後ろに届かない）。
        let mut flat: Vec<UnionMember<'_, 'a>> = Vec::new();
        for member in members {
            let component = match member {
                TypeNode::Ref { name, args }
                    if args.is_empty() && !place.env.contains_key(name) =>
                {
                    match self.lookup(&place.file, name)? {
                        Found::Component(component) => Some(component),
                        _ => None,
                    }
                }
                _ => None,
            };
            let nested = component
                .and_then(|component| self.resolver.component(&component).ok())
                .and_then(crate::resolve::union_members);
            match nested {
                Some(nested) => flat.extend(
                    self.resolver
                        .flatten_union(&nested, 0)
                        .into_iter()
                        .map(UnionMember::Schema),
                ),
                None => flat.push(UnionMember::Ts(member)),
            }
        }
        // 必須プロパティの多いメンバーから並べる（REST の union と同じ）。
        flat.sort_by_cached_key(|member| {
            Reverse(match member {
                UnionMember::Ts(node) => self.required_count(place, node, 0),
                UnionMember::Schema(schema) => self.resolver.required_count(schema, 0),
            })
        });
        let mut variants = Vec::new();
        let mut used = HashSet::new();
        for member in flat {
            let base_name = match &member {
                UnionMember::Ts(node) => variant_name(node),
                UnionMember::Schema(schema) => self.resolver.variant_name(schema),
            };
            let mut variant = base_name.clone();
            let mut counter = 2;
            while !used.insert(ident(&variant).to_string()) {
                variant = format!("{base_name}{counter}");
                counter += 1;
            }
            if variant == CATCH_ALL {
                return Err(format!(
                    "union {name}: バリアントが受け皿の `{CATCH_ALL}` と衝突する"
                ));
            }
            let hint = format!("{name}{variant}");
            let ty = match member {
                UnionMember::Ts(node) => self.resolve(cx, place, node, &hint)?,
                UnionMember::Schema(schema) => {
                    self.resolver.resolve_inner(cx, schema, &hint, false)?
                }
            };
            variants.push((variant, wrap(ty)));
        }
        cx.push(Item::Union(UnionDef {
            name: name.to_owned(),
            docs: Vec::new(),
            variants,
        }))?;
        Ok(cx.named(name))
    }

    fn resolve_intersection(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        types: &[TypeNode],
        hint: &str,
    ) -> Result<(RustType, bool)> {
        let mut objects: Vec<&[Member]> = Vec::new();
        let mut unions: Vec<&[TypeNode]> = Vec::new();
        for member in types {
            match member {
                TypeNode::Object { members } => objects.push(members),
                TypeNode::Union { types } => unions.push(types),
                _ => {
                    return Err(
                        "intersection のメンバーには object literal と union だけ対応している"
                            .to_owned(),
                    );
                }
            }
        }
        match unions.as_slice() {
            [] => {
                let merged = merge_members(&objects)?;
                Ok((self.make_struct(cx, place, hint, &merged)?, false))
            }
            // object と union の intersection は、union の各メンバーに分配する。
            [union] => {
                let mut distributed = Vec::new();
                for member in union.iter() {
                    match member {
                        TypeNode::Object { members } => {
                            let mut all = objects.clone();
                            all.push(members);
                            distributed.push(TypeNode::Object {
                                members: merge_members(&all)?,
                            });
                        }
                        other => {
                            return Err(format!(
                                "intersection に分配する union のメンバーには object literal だけ対応している: {other:?}"
                            ));
                        }
                    }
                }
                self.resolve_union(cx, place, &distributed, hint)
            }
            _ => Err("union 同士の intersection には対応していない".to_owned()),
        }
    }

    fn resolve_indexed(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        object: &TypeNode,
        index: &TypeNode,
        hint: &str,
    ) -> Result<(RustType, bool)> {
        match (object, index) {
            (
                _,
                TypeNode::Literal {
                    value: Value::String(key),
                },
            ) => self.property_of(cx, place, object, key, hint),
            (TypeNode::Typeof { name }, TypeNode::Keyword { name: index }) if index == "number" => {
                let Found::Const(decl) = self.lookup(&place.file, name)? else {
                    return Err(format!("typeof の対象 `{name}` が定数でない"));
                };
                let values = decl
                    .values
                    .clone()
                    .ok_or_else(|| format!("定数 `{name}` が文字列の配列でない"))?;
                let other = values.len() >= 2;
                let ty = self
                    .resolver
                    .make_str_enum(cx, hint, values, other, Vec::new())?;
                Ok((ty, false))
            }
            // 型パラメータ、リテラルの union、keyof を添字にしたものは、値の型を 1 つに決められない。
            _ => Ok((RustType::Value, false)),
        }
    }

    /// `X['key']` の型。X はコンポーネントか、名前付きの型か、object literal。
    fn property_of(
        &mut self,
        cx: &mut Ctx,
        place: &Place,
        object: &TypeNode,
        key: &str,
        hint: &str,
    ) -> Result<(RustType, bool)> {
        match object {
            TypeNode::Ref { name, args } if args.is_empty() => {
                if let Some((file, substituted)) = place.env.get(name) {
                    return self.property_of(cx, &Place::new(file), substituted, key, hint);
                }
                match self.lookup(&place.file, name)? {
                    Found::Component(component) => self.resolver.component_field(&component, key),
                    Found::Type(file, decl) if decl.type_params.is_empty() => {
                        self.property_of(cx, &Place::new(file), &decl.ty, key, hint)
                    }
                    _ => Err(format!("`{name}` は添字で引けない")),
                }
            }
            TypeNode::Object { members } => {
                let member = members
                    .iter()
                    .find(|m| m.name == key)
                    .ok_or_else(|| format!("プロパティ `{key}` が無い"))?;
                let (ty, nullable) = self.resolve(cx, place, &member.ty, hint)?;
                Ok((ty, nullable || member.optional))
            }
            _ => Err("添字アクセスの対象には対応していない".to_owned()),
        }
    }

    /// 必須プロパティの数。union のバリアントの並びに使う。
    fn required_count(&self, place: &Place, node: &TypeNode, depth: usize) -> usize {
        if depth > 16 {
            return 0;
        }
        match node {
            TypeNode::Object { members } => members.iter().filter(|m| !m.optional).count(),
            TypeNode::Ref { name, args } if args.is_empty() => {
                if let Some((file, substituted)) = place.env.get(name) {
                    return self.required_count(&Place::new(file), substituted, depth + 1);
                }
                match self.lookup(&place.file, name) {
                    Ok(Found::Component(component)) => self
                        .resolver
                        .component(&component)
                        .map_or(0, |schema| self.resolver.required_count(schema, 0)),
                    Ok(Found::Type(file, decl)) if decl.type_params.is_empty() => {
                        self.required_count(&Place::new(file), &decl.ty, depth + 1)
                    }
                    _ => 0,
                }
            }
            _ => 0,
        }
    }

    /// 名前を、そのファイルの宣言、import 元、`export *` の再輸出先の順に引く。
    fn lookup(&self, file: &str, name: &str) -> Result<Found<'f>> {
        let (key, decls) = file_of(self.files, file)?;
        if let Some(decl) = decls.types.iter().find(|d| d.name == name) {
            return Ok(Found::Type(key, decl));
        }
        if let Some(decl) = decls.consts.iter().find(|c| c.name == name) {
            return Ok(Found::Const(decl));
        }
        if let Some(import) = decls.imports.iter().find(|i| i.name == name) {
            return self.resolve_module(&import.from, name);
        }
        for module in &decls.reexports {
            if let Ok(found) = self.resolve_module(module, name) {
                return Ok(found);
            }
        }
        Err(format!("{file}: 名前 `{name}` が無い"))
    }

    fn resolve_module(&self, module: &str, name: &str) -> Result<Found<'f>> {
        if module == MODELS_MODULE {
            return if self.resolver.component(name).is_ok() {
                Ok(Found::Component(name.to_owned()))
            } else {
                Err(format!("`{name}` が api.json の components.schemas に無い"))
            };
        }
        if let Some(rest) = module.strip_prefix("./")
            && !rest.contains('/')
        {
            let file = format!("{}.ts", rest.strip_suffix(".js").unwrap_or(rest));
            return self.lookup(&file, name);
        }
        Err(format!("`{name}` の参照先 `{module}` には対応していない"))
    }
}

fn variant_name(node: &TypeNode) -> String {
    match node {
        TypeNode::Ref { name, .. } => name.rsplit('.').next().unwrap_or(name).to_owned(),
        TypeNode::Object { .. } => "Object".to_owned(),
        TypeNode::Array { .. } => "Array".to_owned(),
        TypeNode::Literal { value } => pascal(value.to_string().trim_matches('"')),
        TypeNode::Keyword { name } => match name.as_str() {
            "string" => "String",
            "number" => "Number",
            "boolean" => "Boolean",
            _ => "Value",
        }
        .to_owned(),
        _ => "Value".to_owned(),
    }
}
