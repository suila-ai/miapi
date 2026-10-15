//! `resolve` が作り、`emit` が読む中間表現。

#[derive(Debug, Clone, PartialEq)]
pub enum RustType {
    String,
    /// `format: date-time` の文字列。`chrono::DateTime<chrono::Utc>` になる。
    DateTime,
    I64,
    F64,
    Bool,
    Value,
    JsonMap,
    Map(Box<RustType>),
    Vec(Box<RustType>),
    Tuple(Vec<RustType>),
    Option(Box<RustType>),
    Boxed(Box<RustType>),
    /// `crate::types` にあるアイテム。
    Component(String),
    /// 生成中のエンドポイントのモジュールにあるアイテム。
    Local(String),
    /// multipart のファイルのパート。この型のフィールドは serde の対象から外し、別に送る。
    FilePart,
}

impl RustType {
    pub fn option(self) -> RustType {
        match self {
            RustType::Option(_) => self,
            other => RustType::Option(Box::new(other)),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Item {
    Struct(StructDef),
    StrEnum(StrEnumDef),
    Union(UnionDef),
    Alias(AliasDef),
}

impl Item {
    pub fn name(&self) -> &str {
        match self {
            Item::Struct(def) => &def.name,
            Item::StrEnum(def) => &def.name,
            Item::Union(def) => &def.name,
            Item::Alias(def) => &def.name,
        }
    }

    pub fn set_name(&mut self, name: &str) {
        match self {
            Item::Struct(def) => def.name = name.to_owned(),
            Item::StrEnum(def) => def.name = name.to_owned(),
            Item::Union(def) => def.name = name.to_owned(),
            Item::Alias(def) => def.name = name.to_owned(),
        }
    }
}

/// 構造体のフィールドを、どうシリアライズ・デシリアライズするか。
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum StructKind {
    /// エンティティと応答のオブジェクト。任意のフィールドが欠けていれば `None` になる。
    Entity,
    /// リクエストのパラメータ。任意のフィールドは `None` のとき省略する。
    /// 任意で null 可のフィールドは、「省略」と「null」を区別する。
    Request,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub docs: Vec<String>,
    pub kind: StructKind,
    pub fields: Vec<Field>,
    pub derive_default: bool,
}

#[derive(Debug, Clone)]
pub struct Field {
    /// Rust の識別子。キーワードのこともあり、トークンにするときに生識別子になる。
    pub ident: String,
    pub json_name: String,
    pub docs: Option<String>,
    pub required: bool,
    pub nullable: bool,
    /// `required` と `nullable` に応じて足す `Option` の層を除いた型。
    pub base: RustType,
}

impl Field {
    /// 構造体で宣言するときのフィールドの型。
    pub fn declared_type(&self, kind: StructKind) -> RustType {
        match (kind, self.required, self.nullable) {
            (_, true, false) => self.base.clone(),
            (StructKind::Entity, _, _)
            | (StructKind::Request, true, true)
            | (StructKind::Request, false, false) => self.base.clone().option(),
            (StructKind::Request, false, true) => {
                RustType::Option(Box::new(self.base.clone().option()))
            }
        }
    }

    /// null 可のフィールドに足す、`null` を送る setter の名前。
    pub fn null_setter(&self) -> String {
        format!("{}_null", self.ident)
    }
}

#[derive(Debug, Clone)]
pub struct StrEnumDef {
    pub name: String,
    pub docs: Vec<String>,
    /// （バリアントの識別子, JSON の値）
    pub variants: Vec<(String, String)>,
    /// 受け皿の `Unrecognized(String)` を末尾に足すかどうか。
    pub other: bool,
}

#[derive(Debug, Clone)]
pub struct UnionDef {
    pub name: String,
    pub docs: Vec<String>,
    /// （バリアントの識別子, 中身の型）。
    /// 受け皿の `Unrecognized(serde_json::Value)` は常に末尾に足す。
    pub variants: Vec<(String, RustType)>,
}

#[derive(Debug, Clone)]
pub struct AliasDef {
    pub name: String,
    pub docs: Vec<String>,
    pub ty: RustType,
}

#[derive(Debug)]
pub struct Tree {
    pub version: String,
    pub types: Vec<Item>,
    pub api: Vec<ApiNode>,
}

#[derive(Debug)]
pub struct ApiNode {
    pub segment: String,
    pub ident: String,
    pub endpoint: Option<EndpointDef>,
    pub children: Vec<ApiNode>,
}

#[derive(Debug)]
pub struct EndpointDef {
    pub path: String,
    pub docs: Vec<String>,
    /// `pub struct Request` の定義。パラメータの無いエンドポイントでは、フィールドが無い。
    pub request: StructDef,
    pub response: ResponseDef,
    pub items: Vec<Item>,
}

#[derive(Debug)]
pub enum ResponseDef {
    /// 204 だけを返す。`type Response = ();` になる。
    Unit,
    /// エンドポイントのアイテムの中に `pub struct Response` がある。
    Struct,
    Alias(RustType),
}
