//! spec を中間表現に変換する。
//! エンティティの型、エンドポイントのモジュール、リクエストと応答の型を作り、
//! それぞれの名前も決める。

use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};

use indexmap::IndexMap;
use serde_json::Value;

use crate::model::*;
use crate::naming::{pascal, snake};
use crate::spec::{AdditionalProperties, Operation, Schema, SchemaType, Spec};

pub type Error = String;
type Result<T> = std::result::Result<T, Error>;

/// 文字列 enum と union の末尾に足す、受け皿のバリアントの名前。
pub const CATCH_ALL: &str = "Unrecognized";

/// `Client` のメソッド名。derive している `Clone` の `clone` も含む。
/// 生成するアクセサは、これらと同じ名前にできない。
const RESERVED_CLIENT_METHODS: &[&str] = &[
    "new",
    "with_http_client",
    "origin",
    "call",
    "call_raw",
    "send",
    "clone",
];
/// 生成する `Call` ビルダー自身が使うフィールド名。
const RESERVED_CALL_FIELDS: &[&str] = &["client", "_state"];

#[derive(Clone, Copy, PartialEq)]
enum Scope {
    Types,
    Module,
}

struct Ctx {
    scope: Scope,
    kind: StructKind,
    items: Vec<Item>,
}

impl Ctx {
    fn new(scope: Scope, kind: StructKind) -> Self {
        Ctx {
            scope,
            kind,
            items: Vec::new(),
        }
    }

    fn named(&self, name: &str) -> RustType {
        match self.scope {
            Scope::Types => RustType::Component(name.to_owned()),
            Scope::Module => RustType::Local(name.to_owned()),
        }
    }

    fn push(&mut self, item: Item) -> Result<()> {
        self.ensure_unused(item.name())?;
        self.items.push(item);
        Ok(())
    }

    fn rename(&mut self, from: &str, to: &str) -> Result<()> {
        self.ensure_unused(to)?;
        let item = self
            .items
            .iter_mut()
            .find(|item| item.name() == from)
            .ok_or_else(|| format!("改名する型 `{from}` が無い"))?;
        item.set_name(to);
        Ok(())
    }

    fn ensure_unused(&self, name: &str) -> Result<()> {
        if self.items.iter().any(|item| item.name() == name) {
            return Err(format!("型名 `{name}` が同じモジュールで 2 回生成される"));
        }
        Ok(())
    }
}

enum Member {
    Ref(String),
    Inline(Box<Schema>),
}

pub struct Resolver<'a> {
    spec: &'a Spec,
    resolved: HashSet<String>,
    component_fields: HashMap<String, Vec<Field>>,
    /// （コンポーネント名, プロパティ名）の組。
    /// そのプロパティの enum が union のバリアントを判別する。
    strict_props: HashSet<(String, String)>,
}

pub fn build_tree(spec: &Spec) -> Result<Tree> {
    let mut resolver = Resolver {
        spec,
        resolved: HashSet::new(),
        component_fields: HashMap::new(),
        strict_props: HashSet::new(),
    };
    resolver.collect_strict_props();
    let mut types = Ctx::new(Scope::Types, StructKind::Entity);
    for component in spec.components.schemas.keys() {
        if component == "Error" || resolver.resolved.contains(component) {
            continue;
        }
        resolver.resolve_component(&mut types, component)?;
    }
    box_recursive_fields(&mut types.items);
    let mut api = Vec::new();
    for (path, item) in &spec.paths {
        let Some(op) = &item.post else { continue };
        let endpoint = resolver.build_endpoint(path, op)?;
        insert_endpoint(&mut api, path.trim_start_matches('/'), endpoint);
    }
    validate_api(&api, true)?;
    Ok(Tree {
        version: spec.info.version.clone(),
        types: types.items,
        api,
    })
}

impl<'a> Resolver<'a> {
    fn component(&self, name: &str) -> Result<&'a Schema> {
        self.spec
            .components
            .schemas
            .get(name)
            .ok_or_else(|| format!("不明なコンポーネント `{name}`"))
    }

    fn resolve_component(&mut self, cx: &mut Ctx, name: &str) -> Result<()> {
        self.resolved.insert(name.to_owned());
        let schema = self.component(name)?;
        let (ty, nullable) = self
            .resolve_inner(cx, schema, name, false)
            .map_err(|e| format!("{name}: {e}"))?;
        // 構造体・enum・union は、`resolve_inner` が `name` の名前で生成している。それ以外は別名にする。
        if ty != cx.named(name) {
            cx.push(Item::Alias(AliasDef {
                name: name.to_owned(),
                docs: docs_of(schema),
                ty: if nullable { ty.option() } else { ty },
            }))?;
        }
        Ok(())
    }

    fn component_fields(&mut self, cx: &mut Ctx, name: &str) -> Result<Vec<Field>> {
        if cx.scope == Scope::Types && !self.resolved.contains(name) {
            self.resolve_component(cx, name)?;
        }
        self.component_fields
            .get(name)
            .cloned()
            .ok_or_else(|| format!("allOf のメンバー `{name}` が object のスキーマでない"))
    }

    /// スキーマを型に解決する。null 可なら `Option` で包む。
    fn resolve(&mut self, cx: &mut Ctx, schema: &Schema, hint: &str) -> Result<RustType> {
        let (ty, nullable) = self.resolve_inner(cx, schema, hint, false)?;
        Ok(if nullable { ty.option() } else { ty })
    }

    /// スキーマを null 以外の型に解決し、`null` も許されるかどうかを併せて返す。
    fn resolve_inner(
        &mut self,
        cx: &mut Ctx,
        schema: &Schema,
        hint: &str,
        strict_enum: bool,
    ) -> Result<(RustType, bool)> {
        let mut nullable = schema.nullable == Some(true)
            || matches!(&schema.schema_type, Some(SchemaType::Many(names)) if names.iter().any(|n| n == "null"))
            || schema
                .enumeration
                .as_ref()
                .is_some_and(|values| values.iter().any(Value::is_null));

        if let Some(target) = schema.ref_name() {
            if schema.properties.is_some() {
                let members = vec![
                    Member::Ref(target.to_owned()),
                    Member::Inline(Box::new(schema.clone())),
                ];
                let ty = self.make_struct(cx, hint, members, docs_of(schema), &HashSet::new())?;
                return Ok((ty, nullable));
            }
            if target == "Error" {
                return Ok((RustType::Value, nullable));
            }
            return Ok((RustType::Component(target.to_owned()), nullable));
        }

        if let Some(all) = &schema.all_of {
            let members: Vec<&Schema> = all.iter().filter(|m| !m.is_null_type()).collect();
            nullable |= members.len() != all.len();
            if members.len() == 1 && schema.properties.is_none() {
                let (ty, inner_nullable) = self.resolve_inner(cx, members[0], hint, strict_enum)?;
                return Ok((ty, nullable || inner_nullable));
            }
            let mut struct_members = all_of_members(&members)?;
            if schema.properties.is_some() {
                struct_members.push(Member::Inline(Box::new(schema.clone())));
            }
            let ty =
                self.make_struct(cx, hint, struct_members, docs_of(schema), &HashSet::new())?;
            return Ok((ty, nullable));
        }

        if let Some(list) = schema.one_of.as_ref().or(schema.any_of.as_ref()) {
            let members: Vec<&Schema> = list.iter().filter(|m| !m.is_null_type()).collect();
            nullable |= members.len() != list.len();
            return match members.len() {
                0 => Ok((RustType::Value, true)),
                1 => {
                    let (ty, inner_nullable) =
                        self.resolve_inner(cx, members[0], hint, strict_enum)?;
                    Ok((ty, nullable || inner_nullable))
                }
                _ => {
                    let ty = self.make_union(cx, hint, &members, docs_of(schema))?;
                    Ok((ty, nullable))
                }
            };
        }

        let type_names = schema.type_names();
        if schema.properties.is_some() {
            let ty = self.make_struct(
                cx,
                hint,
                vec![Member::Inline(Box::new(schema.clone()))],
                docs_of(schema),
                &HashSet::new(),
            )?;
            return Ok((ty, nullable));
        }
        if type_names.contains(&"object") {
            let ty = match &schema.additional_properties {
                Some(AdditionalProperties::Schema(value)) => RustType::Map(Box::new(
                    self.resolve(cx, value, &format!("{hint}Value"))?,
                )),
                _ => RustType::JsonMap,
            };
            return Ok((ty, nullable));
        }
        if type_names.contains(&"array") || schema.items.is_some() || schema.prefix_items.is_some()
        {
            let ty = if let Some(prefix) = &schema.prefix_items {
                let mut elements = Vec::new();
                for (index, element) in prefix.iter().enumerate() {
                    elements.push(self.resolve(cx, element, &format!("{hint}{index}"))?);
                }
                RustType::Tuple(elements)
            } else if let Some(items) = &schema.items {
                RustType::Vec(Box::new(self.resolve(cx, items, &format!("{hint}Item"))?))
            } else {
                RustType::Vec(Box::new(RustType::Value))
            };
            return Ok((ty, nullable));
        }
        let string_values: Vec<String> = schema
            .enumeration
            .iter()
            .flatten()
            .filter_map(|v| v.as_str().map(str::to_owned))
            .collect();
        if !string_values.is_empty() && (type_names.is_empty() || type_names.contains(&"string")) {
            let other = !strict_enum && string_values.len() >= 2;
            let ty = self.make_str_enum(cx, hint, string_values, other, docs_of(schema))?;
            return Ok((ty, nullable));
        }
        let ty = match type_names.first() {
            Some(&"string") => match schema.format.as_deref() {
                Some("binary") => RustType::FilePart,
                Some("date-time") => RustType::DateTime,
                _ => RustType::String,
            },
            Some(&"integer") => RustType::I64,
            Some(&"number") => RustType::F64,
            Some(&"boolean") => RustType::Bool,
            _ => RustType::Value,
        };
        Ok((ty, nullable))
    }

    fn make_field(
        &mut self,
        cx: &mut Ctx,
        owner: &str,
        json_name: &str,
        schema: &Schema,
        required: bool,
        strict_props: &HashSet<String>,
    ) -> Result<Field> {
        // サーバの ajv は `required` を検査する前に既定値を入れるので、既定値があれば省略できる。
        let required = required && !(cx.kind == StructKind::Request && schema.default.is_some());
        let strict_enum = required
            && (strict_props.contains(json_name)
                || self
                    .strict_props
                    .contains(&(owner.to_owned(), json_name.to_owned())));
        let hint = format!("{owner}{}", pascal(json_name));
        let (base, nullable) = self
            .resolve_inner(cx, schema, &hint, strict_enum)
            .map_err(|e| format!("{owner}.{json_name}: {e}"))?;
        let mut docs = docs_of(schema);
        if cx.kind == StructKind::Request {
            docs.extend(constraint_doc(schema));
        }
        Ok(Field {
            ident: snake(json_name),
            json_name: json_name.to_owned(),
            docs,
            required,
            nullable,
            base,
        })
    }

    fn make_struct(
        &mut self,
        cx: &mut Ctx,
        name: &str,
        members: Vec<Member>,
        docs: Vec<String>,
        strict_props: &HashSet<String>,
    ) -> Result<RustType> {
        let mut fields: IndexMap<String, Field> = IndexMap::new();
        for member in members {
            let member_fields = match member {
                Member::Ref(target) => self.component_fields(cx, &target)?,
                Member::Inline(schema) => {
                    let mut inline = Vec::new();
                    for (json_name, property) in schema.properties.iter().flatten() {
                        let required = schema.is_required(json_name);
                        inline.push(self.make_field(
                            cx,
                            name,
                            json_name,
                            property,
                            required,
                            strict_props,
                        )?);
                    }
                    inline
                }
            };
            for field in member_fields {
                match fields.get_mut(&field.json_name) {
                    Some(existing) => {
                        if existing.base != field.base || existing.nullable != field.nullable {
                            return Err(format!(
                                "{name}.{}: allOf のメンバーの間で、プロパティの型が食い違っている",
                                field.json_name
                            ));
                        }
                        existing.required |= field.required;
                        if existing.docs.is_empty() {
                            existing.docs = field.docs;
                        }
                    }
                    None => {
                        fields.insert(field.json_name.clone(), field);
                    }
                }
            }
        }
        let fields: Vec<Field> = fields.into_values().collect();
        check_fields(name, &fields)?;
        let derive_default = cx.kind == StructKind::Request
            && fields
                .iter()
                .filter(|f| f.required)
                .all(|f| defaultable(&f.base, &cx.items));
        cx.push(Item::Struct(StructDef {
            name: name.to_owned(),
            docs,
            kind: cx.kind,
            fields: fields.clone(),
            derive_default,
        }))?;
        if cx.scope == Scope::Types {
            self.component_fields.insert(name.to_owned(), fields);
        }
        Ok(cx.named(name))
    }

    fn make_union(
        &mut self,
        cx: &mut Ctx,
        name: &str,
        members: &[&Schema],
        docs: Vec<String>,
    ) -> Result<RustType> {
        let mut members = self.flatten_union(members, 0);
        let discriminators = self.discriminators(&members);
        // 必須プロパティの多いメンバーから並べる。同数のメンバーは、安定ソートなので spec の順のままになる。
        members.sort_by_cached_key(|member| Reverse(self.required_count(member, 0)));
        let mut variants = Vec::new();
        let mut used: HashSet<String> = HashSet::new();
        for member in members {
            let base_name = self.variant_name(member);
            let mut variant = base_name.clone();
            let mut counter = 2;
            while !used.insert(variant.clone()) {
                variant = format!("{base_name}{counter}");
                counter += 1;
            }
            if variant == CATCH_ALL {
                return Err(format!(
                    "union {name}: バリアントが受け皿の `{CATCH_ALL}` と衝突する"
                ));
            }
            let hint = format!("{name}{variant}");
            let ty = if member.ref_name().is_none() && member.properties.is_some() {
                self.make_struct(
                    cx,
                    &hint,
                    vec![Member::Inline(Box::new((*member).clone()))],
                    docs_of(member),
                    &discriminators,
                )?
            } else {
                self.resolve(cx, member, &hint)?
            };
            variants.push((variant, ty));
        }
        cx.push(Item::Union(UnionDef {
            name: name.to_owned(),
            docs,
            variants,
        }))?;
        Ok(cx.named(name))
    }

    fn make_str_enum(
        &mut self,
        cx: &mut Ctx,
        name: &str,
        values: Vec<String>,
        other: bool,
        docs: Vec<String>,
    ) -> Result<RustType> {
        let mut variants = Vec::new();
        let mut seen = HashSet::new();
        for value in values {
            let ident = pascal(&value);
            if ident == CATCH_ALL {
                return Err(format!(
                    "enum {name}: 値 `{value}` が受け皿の `{CATCH_ALL}` と衝突する"
                ));
            }
            if !seen.insert(ident.clone()) {
                return Err(format!(
                    "enum {name}: 値 `{value}` がほかのバリアントと衝突する"
                ));
            }
            variants.push((ident, value));
        }
        cx.push(Item::StrEnum(StrEnumDef {
            name: name.to_owned(),
            docs,
            variants,
            other,
        }))?;
        Ok(cx.named(name))
    }

    fn variant_name(&self, member: &Schema) -> String {
        if let Some(target) = member.ref_name() {
            return target.to_owned();
        }
        if let Some(properties) = &member.properties {
            let tags: Vec<&str> = properties
                .iter()
                .filter(|(json_name, _)| member.is_required(json_name))
                .filter_map(|(_, property)| match property.enumeration.as_deref() {
                    Some([Value::String(value)]) => Some(value.as_str()),
                    _ => None,
                })
                .collect();
            if let [tag] = tags.as_slice() {
                return pascal(tag);
            }
            return "Object".to_owned();
        }
        match member.type_names().first() {
            Some(&"string") => "String",
            Some(&"integer") => "Integer",
            Some(&"number") => "Number",
            Some(&"boolean") => "Boolean",
            Some(&"array") => "Array",
            Some(&"object") => "Object",
            _ => "Value",
        }
        .to_owned()
    }

    /// 必須プロパティの数。
    /// `$ref` をたどり、`allOf` を併合し、`oneOf` / `anyOf` では最も少ない候補を採る。
    fn required_count(&self, schema: &Schema, depth: usize) -> usize {
        if depth > 16 {
            return 0;
        }
        if let Some(target) = schema.ref_name() {
            return self
                .spec
                .components
                .schemas
                .get(target)
                .map_or(0, |t| self.required_count(t, depth + 1));
        }
        for list in [&schema.one_of, &schema.any_of].into_iter().flatten() {
            let counts: Vec<usize> = list
                .iter()
                .filter(|m| !m.is_null_type())
                .map(|m| self.required_count(m, depth + 1))
                .collect();
            if let Some(min) = counts.into_iter().min() {
                return min + schema.required.as_ref().map_or(0, Vec::len);
            }
        }
        let mut names = BTreeSet::new();
        self.required_names(schema, &mut names, depth);
        names.len()
    }

    fn required_names(&self, schema: &Schema, acc: &mut BTreeSet<String>, depth: usize) {
        if depth > 16 {
            return;
        }
        if let Some(target) = schema.ref_name() {
            if let Some(t) = self.spec.components.schemas.get(target) {
                self.required_names(t, acc, depth + 1);
            }
            return;
        }
        acc.extend(schema.required.iter().flatten().cloned());
        for member in schema.all_of.iter().flatten() {
            self.required_names(member, acc, depth + 1);
        }
    }

    /// スキーマの必須プロパティのうち、文字列 enum であるもの。`$ref` と `allOf` をたどる。
    fn enum_props(
        &self,
        schema: &Schema,
        acc: &mut BTreeMap<String, BTreeSet<String>>,
        depth: usize,
    ) {
        if depth > 16 {
            return;
        }
        if let Some(target) = schema.ref_name() {
            if let Some(t) = self.spec.components.schemas.get(target) {
                self.enum_props(t, acc, depth + 1);
            }
            return;
        }
        for member in schema.all_of.iter().flatten() {
            self.enum_props(member, acc, depth + 1);
        }
        for (json_name, property) in schema.properties.iter().flatten() {
            if !schema.is_required(json_name) {
                continue;
            }
            let values: BTreeSet<String> = property
                .enumeration
                .iter()
                .flatten()
                .filter_map(|v| v.as_str().map(str::to_owned))
                .collect();
            if !values.is_empty() {
                acc.insert(json_name.clone(), values);
            }
        }
    }

    /// union のメンバーのうち、union のコンポーネントを指すものを、そのコンポーネントのメンバーに置き換える。
    /// union は受け皿のバリアントを持ち、どんな値でも読める。
    /// バリアントの型にすると、それより後ろのバリアントに届かなくなる。
    fn flatten_union<'s>(&self, members: &[&'s Schema], depth: usize) -> Vec<&'s Schema>
    where
        'a: 's,
    {
        let mut flat = Vec::new();
        for &member in members {
            let nested = member
                .ref_name()
                .filter(|_| member.properties.is_none() && depth < 16)
                .and_then(|target| self.spec.components.schemas.get(target))
                .and_then(union_members);
            match nested {
                Some(nested) => flat.extend(self.flatten_union(&nested, depth + 1)),
                None => flat.push(member),
            }
        }
        flat
    }

    /// enum の値で union のバリアントを見分けられるプロパティの名前。
    fn discriminators(&self, members: &[&Schema]) -> HashSet<String> {
        let maps: Vec<BTreeMap<String, BTreeSet<String>>> = members
            .iter()
            .map(|m| {
                let mut acc = BTreeMap::new();
                self.enum_props(m, &mut acc, 0);
                acc
            })
            .collect();
        let mut names: BTreeSet<&String> = BTreeSet::new();
        for map in &maps {
            names.extend(map.keys());
        }
        names
            .into_iter()
            .filter(|name| {
                let sets: Vec<&BTreeSet<String>> =
                    maps.iter().filter_map(|m| m.get(*name)).collect();
                sets.len() >= 2 && sets.iter().any(|s| *s != sets[0])
            })
            .cloned()
            .collect()
    }

    /// バリアントが `$ref` である union のそれぞれについて、
    /// コンポーネントのどのプロパティが判別に使われるかを記録する。
    /// 記録したプロパティの enum は、受け皿のバリアントなしで生成する。
    fn collect_strict_props(&mut self) {
        let mut found = Vec::new();
        for schema in self.spec.components.schemas.values() {
            self.walk_unions(schema, &mut found, 0);
        }
        for item in self.spec.paths.values() {
            let Some(op) = &item.post else { continue };
            for media in op.request_body.iter().flat_map(|b| b.content.values()) {
                self.walk_unions(&media.schema, &mut found, 0);
            }
            for media in op
                .responses
                .values()
                .flat_map(|r| r.content.iter().flat_map(|c| c.values()))
            {
                self.walk_unions(&media.schema, &mut found, 0);
            }
        }
        self.strict_props.extend(found);
    }

    fn walk_unions(&self, schema: &Schema, found: &mut Vec<(String, String)>, depth: usize) {
        if depth > 32 {
            return;
        }
        if let Some(members) = union_members(schema) {
            let members = self.flatten_union(&members, 0);
            let discriminators = self.discriminators(&members);
            for member in &members {
                if let Some(target) = member.ref_name() {
                    for name in &discriminators {
                        found.push((target.to_owned(), name.clone()));
                    }
                }
            }
        }
        let nested = schema
            .properties
            .iter()
            .flat_map(|p| p.values())
            .chain(schema.items.iter().map(|b| b.as_ref()))
            .chain(schema.prefix_items.iter().flatten())
            .chain(schema.all_of.iter().flatten())
            .chain(schema.one_of.iter().flatten())
            .chain(schema.any_of.iter().flatten());
        for child in nested {
            self.walk_unions(child, found, depth + 1);
        }
        if let Some(AdditionalProperties::Schema(value)) = &schema.additional_properties {
            self.walk_unions(value, found, depth + 1);
        }
    }

    fn build_endpoint(&mut self, path: &str, op: &Operation) -> Result<EndpointDef> {
        let name = path.trim_start_matches('/');
        let mut docs = vec![format!("`{name}`")];
        if let Some(description) = &op.description {
            docs.push(description.clone());
        }
        let mut cx = Ctx::new(Scope::Module, StructKind::Request);
        let request = self
            .build_request(&mut cx, op, &docs)
            .map_err(|e| format!("{name} のリクエスト: {e}"))?;
        cx.kind = StructKind::Entity;
        let response = self
            .build_response(&mut cx, op)
            .map_err(|e| format!("{name} の応答: {e}"))?;
        Ok(EndpointDef {
            path: name.to_owned(),
            docs,
            request,
            response,
            items: cx.items,
        })
    }

    fn build_request(
        &mut self,
        cx: &mut Ctx,
        op: &Operation,
        docs: &[String],
    ) -> Result<StructDef> {
        let mut props: IndexMap<String, (Schema, bool)> = IndexMap::new();
        let mut alternatives: Vec<Vec<String>> = Vec::new();
        // リクエストボディの無いエンドポイントは、フィールドの無い構造体になる。
        if let Some(body) = &op.request_body {
            let (_, content) = body
                .content
                .get_key_value("application/json")
                .or_else(|| body.content.iter().next())
                .ok_or("リクエストボディに content が無い")?;
            collect_request_props(&content.schema, true, &mut props, &mut alternatives);
        }
        let mut fields = Vec::new();
        for (json_name, (schema, required)) in &props {
            fields.push(self.make_field(
                cx,
                "Request",
                json_name,
                schema,
                *required,
                &HashSet::new(),
            )?);
        }
        check_fields("Request", &fields)?;
        if let Some(field) = fields
            .iter()
            .find(|f| RESERVED_CALL_FIELDS.contains(&f.ident.as_str()))
        {
            return Err(format!(
                "Request: プロパティ `{}` がビルダーのフィールドと衝突する",
                field.json_name
            ));
        }
        let mut docs = docs.to_vec();
        if !alternatives.is_empty() {
            let sets: Vec<String> = alternatives
                .iter()
                .map(|set| {
                    set.iter()
                        .map(|name| format!("`{name}`"))
                        .collect::<Vec<_>>()
                        .join(" と ")
                })
                .collect();
            docs.push(format!("必須: {} のいずれか。", sets.join(" / ")));
        }
        let derive_default = fields
            .iter()
            .filter(|f| f.required)
            .all(|f| defaultable(&f.base, &cx.items));
        Ok(StructDef {
            name: "Request".to_owned(),
            docs,
            kind: StructKind::Request,
            fields,
            derive_default,
        })
    }

    fn build_response(&mut self, cx: &mut Ctx, op: &Operation) -> Result<ResponseDef> {
        let schema = op
            .responses
            .get("200")
            .and_then(|r| r.content.as_ref())
            .and_then(|c| c.get("application/json"))
            .map(|m| &m.schema);
        let Some(schema) = schema else {
            return Ok(ResponseDef::Unit);
        };
        let (mut ty, nullable) = self.resolve_inner(cx, schema, "Response", false)?;
        let optional = nullable || op.responses.contains_key("204");
        if ty == RustType::Local("Response".to_owned()) {
            if !optional {
                return Ok(ResponseDef::Struct);
            }
            // `Response` は `Option<ResponseBody>` の別名にするので、本体のアイテムの名前を変える。
            // 入れ子の型の名前には必ず接尾辞が付くので、`Response` という名前のアイテムは本体だけで、
            // ほかのアイテムから参照されることもない。
            cx.rename("Response", "ResponseBody")?;
            ty = RustType::Local("ResponseBody".to_owned());
        }
        Ok(ResponseDef::Alias(if optional { ty.option() } else { ty }))
    }
}

fn docs_of(schema: &Schema) -> Vec<String> {
    schema.description.iter().cloned().collect()
}

/// リクエストのパラメータについて、サーバが検査する条件と既定値を、doc の 1 行にする。
/// 既定値の `null` は書かない。省略しても `null` を送っても同じになるので、書く必要が無い。
fn constraint_doc(schema: &Schema) -> Option<String> {
    let mut doc = validation_doc(schema, "");
    if let Some(value) = &schema.default
        && !value.is_null()
    {
        doc.push_str(&format!("省略時は `{value}`。"));
    }
    (!doc.is_empty()).then_some(doc)
}

/// 範囲・文字数・要素数・形式・重複の文。配列は、要素の制約も `of` に「各要素の」を重ねて続ける。
/// 文字列はサーバ（ajv）がコードポイントで数えるので、`str::len()` のバイト数と紛れる「長さ」と書かない。
/// 文字数と要素数は 0 を下回らないので、下限の 0 は書かない。
fn validation_doc(schema: &Schema, of: &str) -> String {
    fn range<T: std::fmt::Display>(
        of: &str,
        subject: &str,
        min: Option<T>,
        max: Option<T>,
    ) -> Option<String> {
        match (min, max) {
            (Some(min), Some(max)) => Some(format!("{of}{subject}{min} 以上 {max} 以下。")),
            (Some(min), None) => Some(format!("{of}{subject}{min} 以上。")),
            (None, Some(max)) => Some(format!("{of}{subject}{max} 以下。")),
            (None, None) => None,
        }
    }
    let positive = |n: Option<u64>| n.filter(|&n| n > 0);
    let value = if of.is_empty() { "" } else { "値は " };
    let mut doc: String = [
        range(of, value, schema.minimum.as_ref(), schema.maximum.as_ref()),
        range(
            of,
            "文字数は ",
            positive(schema.min_length),
            schema.max_length,
        ),
        range(
            of,
            "要素数は ",
            positive(schema.min_items),
            schema.max_items,
        ),
        schema
            .pattern
            .as_ref()
            .map(|pattern| format!("{of}形式は正規表現 `{pattern}` に合うもの。")),
        (schema.unique_items == Some(true)).then(|| format!("{of}要素は重複させない。")),
    ]
    .into_iter()
    .flatten()
    .collect();
    if let Some(items) = &schema.items {
        doc.push_str(&validation_doc(items, &format!("{of}各要素の")));
    }
    doc
}

/// スキーマが union になるときの、null 以外のメンバー。
fn union_members(schema: &Schema) -> Option<Vec<&Schema>> {
    if schema.ref_name().is_some() || schema.all_of.is_some() {
        return None;
    }
    let list = schema.one_of.as_ref().or(schema.any_of.as_ref())?;
    let members: Vec<&Schema> = list.iter().filter(|m| !m.is_null_type()).collect();
    (members.len() >= 2).then_some(members)
}

/// allOf のメンバーを、構造体のメンバーにする。
fn all_of_members(members: &[&Schema]) -> Result<Vec<Member>> {
    members
        .iter()
        .map(|member| {
            if member.one_of.is_some() || member.any_of.is_some() || member.all_of.is_some() {
                return Err("allOf の中の allOf / oneOf / anyOf には対応していない".to_owned());
            }
            Ok(match member.ref_name() {
                Some(target) if member.properties.is_none() => Member::Ref(target.to_owned()),
                _ => Member::Inline(Box::new((*member).clone())),
            })
        })
        .collect()
}

fn collect_request_props(
    schema: &Schema,
    allow_required: bool,
    props: &mut IndexMap<String, (Schema, bool)>,
    alternatives: &mut Vec<Vec<String>>,
) {
    for (json_name, property) in schema.properties.iter().flatten() {
        let entry = props
            .entry(json_name.clone())
            .or_insert_with(|| (property.clone(), false));
        if allow_required && schema.is_required(json_name) {
            entry.1 = true;
        }
    }
    for member in schema.all_of.iter().flatten() {
        collect_request_props(member, allow_required, props, alternatives);
    }
    for list in [&schema.one_of, &schema.any_of].into_iter().flatten() {
        for member in list {
            collect_request_props(member, false, props, alternatives);
            if let Some(required) = &member.required
                && !required.is_empty()
            {
                alternatives.push(required.clone());
            }
        }
    }
}

fn check_fields(owner: &str, fields: &[Field]) -> Result<()> {
    let mut seen = HashSet::new();
    for field in fields {
        if !seen.insert(&field.ident) {
            return Err(format!(
                "{owner}: 複数のプロパティが同じフィールド `{}` になる",
                field.ident
            ));
        }
    }
    Ok(())
}

fn defaultable(ty: &RustType, items: &[Item]) -> bool {
    match ty {
        RustType::String
        | RustType::I64
        | RustType::F64
        | RustType::Bool
        | RustType::Value
        | RustType::JsonMap
        | RustType::Map(_)
        | RustType::Vec(_)
        | RustType::Option(_)
        | RustType::DateTime => true,
        RustType::Tuple(elements) => elements.iter().all(|e| defaultable(e, items)),
        RustType::Boxed(inner) => defaultable(inner, items),
        RustType::Local(name) => items.iter().any(
            |item| matches!(item, Item::Struct(def) if &def.name == name && def.derive_default),
        ),
        RustType::Component(_) | RustType::FilePart => false,
    }
}

/// そのままだと型のサイズが無限になる構造体のフィールドを、`Box` で包む。
fn box_recursive_fields(items: &mut [Item]) {
    let mut edges: HashMap<String, Vec<String>> = HashMap::new();
    for item in items.iter() {
        let targets: Vec<String> = match item {
            Item::Struct(def) => def
                .fields
                .iter()
                .filter_map(|f| match &f.base {
                    RustType::Component(name) => Some(name.clone()),
                    _ => None,
                })
                .collect(),
            Item::Union(def) => def
                .variants
                .iter()
                .filter_map(|(_, ty)| match ty {
                    RustType::Component(name) => Some(name.clone()),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        edges.insert(item.name().to_owned(), targets);
    }
    fn reaches(
        edges: &HashMap<String, Vec<String>>,
        from: &str,
        to: &str,
        visited: &mut HashSet<String>,
    ) -> bool {
        if from == to {
            return true;
        }
        if !visited.insert(from.to_owned()) {
            return false;
        }
        edges
            .get(from)
            .is_some_and(|targets| targets.iter().any(|t| reaches(edges, t, to, visited)))
    }
    for item in items.iter_mut() {
        let Item::Struct(def) = item else { continue };
        let owner = def.name.clone();
        for field in &mut def.fields {
            if let RustType::Component(target) = &field.base
                && reaches(&edges, target, &owner, &mut HashSet::new())
            {
                field.base = RustType::Boxed(Box::new(field.base.clone()));
            }
        }
    }
}

fn insert_endpoint(nodes: &mut Vec<ApiNode>, path: &str, endpoint: EndpointDef) {
    let (head, rest) = match path.split_once('/') {
        Some((head, rest)) => (head, Some(rest)),
        None => (path, None),
    };
    let position = match nodes.iter().position(|n| n.segment == head) {
        Some(position) => position,
        None => {
            nodes.push(ApiNode {
                segment: head.to_owned(),
                ident: snake(head),
                endpoint: None,
                children: Vec::new(),
            });
            nodes.len() - 1
        }
    };
    match rest {
        Some(rest) => insert_endpoint(&mut nodes[position].children, rest, endpoint),
        None => nodes[position].endpoint = Some(endpoint),
    }
}

fn validate_api(nodes: &[ApiNode], top: bool) -> Result<()> {
    let mut seen = HashSet::new();
    for node in nodes {
        if !seen.insert(&node.ident) {
            return Err(format!(
                "複数のパスのセグメントが同じモジュール `{}` になる",
                node.ident
            ));
        }
        if top && RESERVED_CLIENT_METHODS.contains(&node.ident.as_str()) {
            return Err(format!(
                "エンドポイント `{}` が Client のメソッドと衝突する",
                node.segment
            ));
        }
        let mut methods: HashSet<String> = HashSet::from(["new".to_owned()]);
        for child in &node.children {
            methods.insert(child.ident.clone());
        }
        if let Some(endpoint) = &node.endpoint {
            for field in &endpoint.request.fields {
                let mut names = vec![field.ident.clone()];
                if field.nullable {
                    names.push(field.null_setter());
                }
                if !field.required {
                    names.push(field.maybe_setter());
                }
                for name in names {
                    if !methods.insert(name.clone()) {
                        return Err(format!(
                            "{}: setter `{name}` がビルダーのほかのメソッドと衝突する",
                            endpoint.path
                        ));
                    }
                }
            }
        }
        validate_api(&node.children, false)?;
    }
    Ok(())
}
