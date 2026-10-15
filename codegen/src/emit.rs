//! 中間表現から Rust のソースを出力する。

use proc_macro2::{Ident, TokenStream};
use quote::{format_ident, quote};

use crate::model::*;
use crate::naming::{ident, snake};
use crate::resolve::CATCH_ALL;

/// 生成する 1 つのツリーに含まれるファイル。パスはツリーのディレクトリからの相対パスで表す。
/// `name` は spec の名前で、ツリーは `crate::generated` の下の、その名前のモジュール（[`module_ident`]）に置かれる。
pub fn tree_files(tree: &Tree, name: &str) -> Vec<(String, TokenStream)> {
    let module = module_ident(name);
    let mut files = vec![
        ("mod.rs".to_owned(), tree_mod_file(&tree.version)),
        ("types.rs".to_owned(), types_file(&tree.types, &module)),
        ("api/mod.rs".to_owned(), api_mod_file(&tree.api)),
    ];
    for node in &tree.api {
        let file = format!("api/{}.rs", node.ident);
        files.push((file, api_top_file(node, &module)));
    }
    files
}

/// spec の名前（cargo feature の名前でもある）から、ツリーを置くモジュールの名前を作る
/// （`v2024.10.1` -> `v2024_10_1`）。
fn module_ident(name: &str) -> Ident {
    ident(&snake(name))
}

fn tree_mod_file(version: &str) -> TokenStream {
    quote! {
        #![doc = "`api.json` から生成したバインディング。手で編集せず、`cargo codegen` を実行すること。"]
        pub mod api;
        pub mod types;
        #[doc = "このバインディングの生成元である `api.json` の `info.version`。"]
        pub const API_VERSION: &str = #version;
        #[doc = "マーカー: クライアントが相手にするサーバを、このバージョンセットで読む。"]
        #[derive(Debug, Clone, Copy)]
        pub struct Version;
        #[doc = "このバージョンセットでサーバを読むクライアント。エンドポイントのビルダーは、ここからメソッドの連鎖で作る。"]
        pub type Client = crate::client::Client<Version>;
    }
}

fn types_file(items: &[Item], module: &Ident) -> TokenStream {
    let items = items.iter().map(|item| item_tokens(item, module));
    quote! {
        #![doc = "エンティティの型（`api.json` の `components.schemas`）。"]
        #![allow(clippy::large_enum_variant)]
        #(#items)*
    }
}

fn api_mod_file(nodes: &[ApiNode]) -> TokenStream {
    let modules = nodes.iter().map(|node| {
        let name = ident(&node.ident);
        quote! {
            pub mod #name;
        }
    });
    let accessors = nodes.iter().map(|node| {
        let name = ident(&node.ident);
        let doc = node_doc(node);
        quote! {
            #[doc = #doc]
            pub fn #name(&self) -> #name::Call<'_> {
                #name::Call::new(self)
            }
        }
    });
    quote! {
        #![doc = "エンドポイントのビルダー。パスのセグメントごとに 1 つのモジュールがある。"]
        #(#modules)*
        impl super::Client {
            #(#accessors)*
        }
    }
}

fn api_top_file(node: &ApiNode, module: &Ident) -> TokenStream {
    let doc = node_doc(node);
    let body = node_body(node, module);
    quote! {
        #![doc = #doc]
        #![allow(clippy::type_complexity, clippy::large_enum_variant)]
        #body
    }
}

fn node_doc(node: &ApiNode) -> String {
    match &node.endpoint {
        Some(endpoint) if node.children.is_empty() => format!("`{}`", endpoint.path),
        Some(endpoint) => format!(
            "`{}` と、`{}/` 以下のエンドポイント",
            endpoint.path, endpoint.path
        ),
        None => format!("`{}/` 以下のエンドポイント", node_path(node)),
    }
}

fn node_path(node: &ApiNode) -> String {
    node.children
        .first()
        .and_then(|child| {
            child
                .endpoint
                .as_ref()
                .map(|e| e.path.clone())
                .or_else(|| Some(node_path(child)))
        })
        .and_then(|path| path.rsplit_once('/').map(|(head, _)| head.to_owned()))
        .unwrap_or_else(|| node.segment.clone())
}

fn node_body(node: &ApiNode, module: &Ident) -> TokenStream {
    let items = node
        .endpoint
        .iter()
        .flat_map(|endpoint| endpoint.items.iter())
        .map(|item| item_tokens(item, module));
    let endpoint = node
        .endpoint
        .as_ref()
        .map(|endpoint| endpoint_tokens(endpoint, module));
    let call = call_tokens(node, module);
    let children = node.children.iter().map(|child| {
        let name = ident(&child.ident);
        let doc = node_doc(child);
        let body = node_body(child, module);
        quote! {
            #[doc = #doc]
            pub mod #name {
                #body
            }
        }
    });
    quote! {
        #(#items)*
        #endpoint
        #call
        #(#children)*
    }
}

fn endpoint_tokens(endpoint: &EndpointDef, module: &Ident) -> TokenStream {
    let path = &endpoint.path;
    let request = struct_tokens(&endpoint.request, module);
    let fields = &endpoint.request.fields;
    let into_body = match fields.iter().find(|f| f.base == RustType::FilePart) {
        Some(file) => {
            let json_name = &file.json_name;
            let name = ident(&file.ident);
            quote! {
                let fields = match ::serde_json::to_value(&self).map_err(crate::Error::Serialize)? {
                    ::serde_json::Value::Object(fields) => fields,
                    _ => ::serde_json::Map::new(),
                };
                ::std::result::Result::Ok(crate::Body::Multipart {
                    fields,
                    file: (#json_name, self.#name),
                })
            }
        }
        None => quote! { crate::endpoint::json_body(&self) },
    };
    let response = match &endpoint.response {
        ResponseDef::Unit => quote! { pub type Response = (); },
        ResponseDef::Struct => quote! {},
        ResponseDef::Alias(ty) => {
            let ty = type_tokens(ty, module);
            quote! { pub type Response = #ty; }
        }
    };
    quote! {
        #request
        impl crate::Endpoint for Request {
            const PATH: &'static str = #path;
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                #into_body
            }
        }
        #response
    }
}

fn call_tokens(node: &ApiNode, module: &Ident) -> TokenStream {
    let kind = StructKind::Request;
    let fields: &[Field] = node.endpoint.as_ref().map_or(&[], |e| &e.request.fields);
    let required: Vec<&Field> = fields.iter().filter(|f| f.required).collect();
    let optional: Vec<&Field> = fields.iter().filter(|f| !f.required).collect();
    let slots: Vec<Ident> = (0..required.len()).map(|i| format_ident!("S{i}")).collect();
    let unset = vec![quote! { crate::builder::Unset }; required.len()];
    let set = vec![quote! { crate::builder::Set }; required.len()];
    let typed = !required.is_empty();

    let docs = match &node.endpoint {
        Some(endpoint) => {
            let mut docs = vec![format!("`{}` のビルダー。", endpoint.path)];
            docs.extend(endpoint.docs.iter().skip(1).cloned());
            if typed {
                let names: Vec<String> =
                    required.iter().map(|f| format!("`{}`", f.ident)).collect();
                docs.push(format!(
                    "必須パラメータ（型状態のスロットの順）: {}。すべて設定すると、呼び出しを await できる。",
                    names.join("、")
                ));
            }
            docs
        }
        None => vec![node_doc(node)],
    };
    let docs = doc_attrs(&docs);

    let storage = fields.iter().map(|f| {
        let name = ident(&f.ident);
        let declared = f.declared_type(kind);
        let stored = if f.required {
            RustType::Option(Box::new(declared))
        } else {
            declared
        };
        let ty = type_tokens(&stored, module);
        quote! { #name: #ty, }
    });
    let init = fields.iter().map(|f| {
        let name = ident(&f.ident);
        quote! { #name: ::std::option::Option::None, }
    });
    let (generics, state_field, state_init) = if typed {
        (
            quote! { <'a, S = (#(#unset,)*)> },
            quote! { _state: ::std::marker::PhantomData<S>, },
            quote! { _state: ::std::marker::PhantomData, },
        )
    } else {
        (quote! { <'a> }, quote! {}, quote! {})
    };
    let setters_impl = if typed {
        quote! { impl<'a, #(#slots),*> Call<'a, (#(#slots,)*)> }
    } else {
        quote! { impl<'a> Call<'a> }
    };

    let children = node.children.iter().map(|child| {
        let name = ident(&child.ident);
        let doc = node_doc(child);
        quote! {
            #[doc = #doc]
            pub fn #name(&self) -> #name::Call<'a> {
                #name::Call::new(self.client)
            }
        }
    });

    let optional_setters = optional.iter().flat_map(|f| {
        let name = ident(&f.ident);
        let base = type_tokens(&f.base, module);
        let doc = doc_attrs(f.docs.as_slice());
        let stored = stored_value(f);
        let maybe_name = format_ident!("{}", f.maybe_setter());
        let mut maybe_doc = format!("`Some` なら `{}` を設定し、`None` なら設定を消して送らない。", f.json_name);
        if f.nullable {
            maybe_doc.push_str(&format!("`null` を送るには [`{0}`](Self::{0}) を使う。", f.null_setter()));
        }
        let mut setters = vec![
            quote! {
                #doc
                pub fn #name(self, value: impl ::std::convert::Into<#base>) -> Self {
                    self.#maybe_name(::std::option::Option::Some(value))
                }
            },
            quote! {
                #[doc = #maybe_doc]
                pub fn #maybe_name(mut self, value: ::std::option::Option<impl ::std::convert::Into<#base>>) -> Self {
                    self.#name = value.map(|value| #stored);
                    self
                }
            },
        ];
        if f.nullable {
            let null_name = format_ident!("{}", f.null_setter());
            let null_doc = format!("`{}` を `null` として送る。", f.json_name);
            setters.push(quote! {
                #[doc = #null_doc]
                pub fn #null_name(mut self) -> Self {
                    self.#name = ::std::option::Option::Some(::std::option::Option::None);
                    self
                }
            });
        }
        setters
    });

    let required_setters = required.iter().enumerate().flat_map(|(index, f)| {
        let name = ident(&f.ident);
        let base = type_tokens(&f.base, module);
        let doc = doc_attrs(f.docs.as_slice());
        let output: Vec<TokenStream> = slots
            .iter()
            .enumerate()
            .map(|(i, slot)| if i == index { quote! { crate::builder::Set } } else { quote! { #slot } })
            .collect();
        let moves = |assigned: TokenStream| {
            fields
                .iter()
                .map(|g| {
                    let gname = ident(&g.ident);
                    if g.ident == f.ident {
                        quote! { #gname: #assigned, }
                    } else {
                        quote! { #gname: self.#gname, }
                    }
                })
                .collect::<Vec<_>>()
        };
        let stored = stored_value(f);
        let set_moves = moves(quote! { ::std::option::Option::Some(#stored) });
        let mut setters = vec![quote! {
            #doc
            pub fn #name(self, value: impl ::std::convert::Into<#base>) -> Call<'a, (#(#output,)*)> {
                Call { client: self.client, #(#set_moves)* _state: ::std::marker::PhantomData }
            }
        }];
        if f.nullable {
            let null_name = format_ident!("{}", f.null_setter());
            let null_doc = format!("`{}` を `null` として送る。", f.json_name);
            let null_moves = moves(quote! { ::std::option::Option::Some(::std::option::Option::None) });
            setters.push(quote! {
                #[doc = #null_doc]
                pub fn #null_name(self) -> Call<'a, (#(#output,)*)> {
                    Call { client: self.client, #(#null_moves)* _state: ::std::marker::PhantomData }
                }
            });
        }
        setters
    });

    let into_future = node.endpoint.is_some().then(|| {
        let state = if typed {
            quote! { Call<'a, (#(#set,)*)> }
        } else {
            quote! { Call<'a> }
        };
        let inits = fields.iter().map(|f| {
            let name = ident(&f.ident);
            if f.required {
                quote! { #name: self.#name.expect("required parameter is set by the type state"), }
            } else {
                quote! { #name: self.#name, }
            }
        });
        quote! {
            impl<'a> ::std::future::IntoFuture for #state {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request { #(#inits)* };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
    });

    quote! {
        #docs
        pub struct Call #generics {
            client: &'a crate::generated::#module::Client,
            #(#storage)*
            #state_field
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::#module::Client) -> Self {
                Call { client, #(#init)* #state_init }
            }
        }
        #setters_impl {
            #(#children)*
            #(#optional_setters)*
            #(#required_setters)*
        }
        #into_future
    }
}

/// setter の引数 `value` から、ビルダーがそのフィールドの `Option` に入れる値を作る。
fn stored_value(field: &Field) -> TokenStream {
    if field.nullable {
        quote! { ::std::option::Option::Some(value.into()) }
    } else {
        quote! { value.into() }
    }
}

pub fn item_tokens(item: &Item, module: &Ident) -> TokenStream {
    match item {
        Item::Struct(def) => struct_tokens(def, module),
        Item::StrEnum(def) => str_enum_tokens(def),
        Item::Union(def) => union_tokens(def, module),
        Item::Alias(def) => {
            let name = ident(&def.name);
            let docs = doc_attrs(&def.docs);
            let ty = type_tokens(&def.ty, module);
            quote! {
                #docs
                pub type #name = #ty;
            }
        }
    }
}

fn struct_tokens(def: &StructDef, module: &Ident) -> TokenStream {
    let name = ident(&def.name);
    let docs = doc_attrs(&def.docs);
    let default = def.derive_default.then(|| quote! { Default, });
    let fields = def.fields.iter().map(|f| {
        let fname = ident(&f.ident);
        let ty = type_tokens(&f.declared_type(def.kind), module);
        let doc = doc_attrs(f.docs.as_slice());
        let mut attrs: Vec<TokenStream> = Vec::new();
        if f.base == RustType::FilePart {
            attrs.push(quote! { skip });
        } else {
            if f.ident != f.json_name {
                let json_name = &f.json_name;
                attrs.push(quote! { rename = #json_name });
            }
            match (def.kind, f.required, f.nullable) {
                (StructKind::Entity, false, _) => attrs.push(quote! { default }),
                (StructKind::Request, false, false) => {
                    attrs.push(quote! { default, skip_serializing_if = "Option::is_none" });
                }
                (StructKind::Request, false, true) => {
                    attrs.push(quote! {
                        default,
                        skip_serializing_if = "Option::is_none",
                        with = "crate::serde_helpers::double_option"
                    });
                }
                _ => {}
            }
        }
        let serde = (!attrs.is_empty()).then(|| quote! { #[serde(#(#attrs),*)] });
        quote! {
            #doc
            #serde
            pub #fname: #ty,
        }
    });
    quote! {
        #docs
        #[derive(Debug, Clone, PartialEq, #default serde::Serialize, serde::Deserialize)]
        pub struct #name {
            #(#fields)*
        }
    }
}

fn str_enum_tokens(def: &StrEnumDef) -> TokenStream {
    let name = ident(&def.name);
    let docs = doc_attrs(&def.docs);
    let variants = def.variants.iter().map(|(variant, value)| {
        let variant = ident(variant);
        quote! {
            #[serde(rename = #value)]
            #variant,
        }
    });
    let catch_all = def.other.then(|| {
        let variant = ident(CATCH_ALL);
        quote! {
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            #variant(::std::string::String),
        }
    });
    quote! {
        #docs
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        pub enum #name {
            #(#variants)*
            #catch_all
        }
    }
}

fn union_tokens(def: &UnionDef, module: &Ident) -> TokenStream {
    let name = ident(&def.name);
    let docs = doc_attrs(&def.docs);
    let variants = def.variants.iter().map(|(variant, ty)| {
        let variant = ident(variant);
        let ty = type_tokens(ty, module);
        quote! { #variant(#ty), }
    });
    let catch_all = ident(CATCH_ALL);
    quote! {
        #docs
        #[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
        #[serde(untagged)]
        pub enum #name {
            #(#variants)*
            #[doc = "このバージョンのバインディングが知らない形。"]
            #catch_all(::serde_json::Value),
        }
    }
}

pub fn type_tokens(ty: &RustType, module: &Ident) -> TokenStream {
    match ty {
        RustType::String => quote! { ::std::string::String },
        RustType::DateTime => quote! { ::chrono::DateTime<::chrono::Utc> },
        RustType::I64 => quote! { i64 },
        RustType::F64 => quote! { f64 },
        RustType::Bool => quote! { bool },
        RustType::Value => quote! { ::serde_json::Value },
        RustType::JsonMap => {
            quote! { ::serde_json::Map<::std::string::String, ::serde_json::Value> }
        }
        RustType::Map(inner) => {
            let inner = type_tokens(inner, module);
            quote! { ::std::collections::HashMap<::std::string::String, #inner> }
        }
        RustType::Vec(inner) => {
            let inner = type_tokens(inner, module);
            quote! { ::std::vec::Vec<#inner> }
        }
        RustType::Tuple(elements) => {
            let elements = elements.iter().map(|element| type_tokens(element, module));
            quote! { (#(#elements,)*) }
        }
        RustType::Option(inner) => {
            let inner = type_tokens(inner, module);
            quote! { ::std::option::Option<#inner> }
        }
        RustType::Boxed(inner) => {
            let inner = type_tokens(inner, module);
            quote! { ::std::boxed::Box<#inner> }
        }
        RustType::Component(name) => {
            let name = ident(name);
            quote! { crate::generated::#module::types::#name }
        }
        RustType::Local(name) => {
            let name = ident(name);
            quote! { #name }
        }
        RustType::FilePart => quote! { crate::FilePart },
    }
}

fn doc_attrs(docs: &[String]) -> TokenStream {
    let lines = docs.iter().map(|doc| {
        let doc = escape_doc(doc);
        quote! { #[doc = #doc] }
    });
    quote! { #(#lines)* }
}

/// spec の description にある `[...]` が、rustdoc のリンクとして解釈されないようにする。
fn escape_doc(doc: &str) -> String {
    doc.replace('[', "\\[").replace(']', "\\]")
}

/// クレートの直下に出すバージョンセットの名前。ほかは、feature と同じ名前のモジュールに入れる。
const ROOT: &str = "latest";

/// `src/generated/mod.rs`。cargo feature で有効にしたツリーを並べる。
pub fn selector_file(names: &[String]) -> TokenStream {
    let modules = names.iter().map(|name| {
        let module = module_ident(name);
        let path = format!("{name}/mod.rs");
        if name == ROOT {
            quote! {
                #[cfg(feature = #name)]
                #[path = #path]
                mod #module;
                #[cfg(feature = #name)]
                pub use #module::{API_VERSION, Client, api, types};
            }
        } else {
            quote! {
                #[cfg(feature = #name)]
                #[path = #path]
                pub mod #module;
            }
        }
    });
    quote! {
        #![doc = "cargo feature で有効にした、Misskey のバージョンごとの生成済みバインディング。"]
        #(#modules)*
    }
}
