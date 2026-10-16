//! OpenAPI 3.1 のうち、Misskey の `api.json` が使う部分。知らないキーは無視する。

use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct Spec {
    pub info: Info,
    pub paths: IndexMap<String, PathItem>,
    pub components: Components,
}

#[derive(Debug, Deserialize)]
pub struct Info {
    pub version: String,
}

#[derive(Debug, Deserialize)]
pub struct PathItem {
    pub post: Option<Operation>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    pub description: Option<String>,
    pub request_body: Option<RequestBody>,
    #[serde(default)]
    pub responses: IndexMap<String, Response>,
}

#[derive(Debug, Deserialize)]
pub struct RequestBody {
    pub content: IndexMap<String, MediaType>,
}

#[derive(Debug, Deserialize)]
pub struct MediaType {
    pub schema: Schema,
}

#[derive(Debug, Deserialize)]
pub struct Response {
    pub content: Option<IndexMap<String, MediaType>>,
}

#[derive(Debug, Deserialize)]
pub struct Components {
    pub schemas: IndexMap<String, Schema>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schema {
    #[serde(rename = "$ref")]
    pub reference: Option<String>,
    #[serde(rename = "type")]
    pub schema_type: Option<SchemaType>,
    pub properties: Option<IndexMap<String, Schema>>,
    pub required: Option<Vec<String>>,
    pub items: Option<Box<Schema>>,
    pub prefix_items: Option<Vec<Schema>>,
    pub additional_properties: Option<AdditionalProperties>,
    #[serde(rename = "enum")]
    pub enumeration: Option<Vec<Value>>,
    pub format: Option<String>,
    pub one_of: Option<Vec<Schema>>,
    pub any_of: Option<Vec<Schema>>,
    pub all_of: Option<Vec<Schema>>,
    pub description: Option<String>,
    pub nullable: Option<bool>,
    pub minimum: Option<Value>,
    pub maximum: Option<Value>,
    pub min_length: Option<u64>,
    pub max_length: Option<u64>,
    pub pattern: Option<String>,
    pub min_items: Option<u64>,
    pub max_items: Option<u64>,
    pub unique_items: Option<bool>,
    pub default: Option<Value>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum SchemaType {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
pub enum AdditionalProperties {
    Bool(#[allow(dead_code)] bool),
    Schema(Box<Schema>),
}

impl Schema {
    /// `type` に並んだ型名から `"null"` を除いたもの。
    pub fn type_names(&self) -> Vec<&str> {
        match &self.schema_type {
            Some(SchemaType::One(name)) => vec![name.as_str()],
            Some(SchemaType::Many(names)) => names
                .iter()
                .map(String::as_str)
                .filter(|name| *name != "null")
                .collect(),
            None => Vec::new(),
        }
    }

    pub fn is_null_type(&self) -> bool {
        matches!(&self.schema_type, Some(SchemaType::One(name)) if name == "null")
            && self.reference.is_none()
            && self.properties.is_none()
    }

    /// `$ref` が指すコンポーネントの名前。
    pub fn ref_name(&self) -> Option<&str> {
        self.reference
            .as_deref()
            .and_then(|reference| reference.strip_prefix("#/components/schemas/"))
    }

    pub fn is_required(&self, name: &str) -> bool {
        self.required
            .as_ref()
            .is_some_and(|required| required.iter().any(|r| r == name))
    }
}
