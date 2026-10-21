//! misskey-js の TypeScript の型定義を、`codegen/ts-types.ts`（Node の typescript）で読んだ構文木の JSON。
//! 知らないキーは無視する。

use std::path::{Path, PathBuf};
use std::process::Command;

use indexmap::IndexMap;
use serde::Deserialize;
use serde_json::Value;

/// ファイル名（`streaming.types.ts` など）ごとの宣言。
pub type Files = IndexMap<String, File>;

#[derive(Debug, Deserialize)]
pub struct File {
    pub imports: Vec<Import>,
    pub reexports: Vec<String>,
    pub types: Vec<TypeDecl>,
    pub consts: Vec<ConstDecl>,
}

#[derive(Debug, Deserialize)]
pub struct Import {
    pub name: String,
    pub from: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TypeDecl {
    pub name: String,
    pub type_params: Vec<TypeParam>,
    #[serde(rename = "type")]
    pub ty: TypeNode,
}

/// `export const x = ['a', 'b'] as const;`。文字列の配列でない定数は `values` が `None`。
#[derive(Debug, Deserialize)]
pub struct ConstDecl {
    pub name: String,
    pub values: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct TypeParam {
    pub name: String,
    pub constraint: Option<TypeNode>,
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase"
)]
pub enum TypeNode {
    Keyword {
        name: String,
    },
    Literal {
        value: Value,
    },
    Ref {
        name: String,
        args: Vec<TypeNode>,
    },
    Object {
        members: Vec<Member>,
    },
    Union {
        types: Vec<TypeNode>,
    },
    Intersection {
        types: Vec<TypeNode>,
    },
    Array {
        element: Box<TypeNode>,
    },
    Indexed {
        object: Box<TypeNode>,
        index: Box<TypeNode>,
    },
    Function {
        type_params: Vec<TypeParam>,
        params: Vec<TypeNode>,
    },
    Typeof {
        name: String,
    },
    /// `ts-types.ts` が写さない構文。到達したときだけエラーにする。
    Unsupported {
        syntax: String,
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct Member {
    pub name: String,
    pub optional: bool,
    #[serde(rename = "type")]
    pub ty: TypeNode,
    /// メンバーと同じ行の `//` コメント。
    pub doc: Option<String>,
}

/// `script`（`ts-types.ts`）を Node で実行し、`paths` の構文木を読む。
pub fn parse(script: &Path, paths: &[PathBuf]) -> Result<Files, String> {
    let output = Command::new("node")
        .arg(script)
        .args(paths)
        .output()
        .map_err(|e| {
            format!("TypeScript の型定義を読むには Node.js 26 と、`npm ci` で入れる typescript が必要（node の実行に失敗）: {e}")
        })?;
    if !output.status.success() {
        return Err(format!(
            "{} が失敗した: {}",
            script.display(),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| format!("{} の出力: {e}", script.display()))
}
