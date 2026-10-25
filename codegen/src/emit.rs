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
        (
            "types.rs".to_owned(),
            types_file(
                "エンティティの型（`api.json` の `components.schemas`）。",
                &tree.types,
                &module,
            ),
        ),
        ("api/mod.rs".to_owned(), api_mod_file(&tree.api)),
    ];
    for node in &tree.api {
        let file = format!("api/{}.rs", node.ident);
        files.push((file, api_top_file(node, &module)));
    }
    files.push((
        "streaming/mod.rs".to_owned(),
        streaming_mod_file(&tree.streaming, &module),
    ));
    files.push((
        "streaming/types.rs".to_owned(),
        types_file(
            "misskey-js の型定義（`entities.ts`・`consts.ts`・`streaming.types.ts`）にある、ストリーミングだけが使う型。",
            &tree.streaming.types,
            &module,
        ),
    ));
    for channel in &tree.streaming.channels {
        let file = format!("streaming/{}.rs", channel.ident);
        files.push((file, channel_file(channel, &module)));
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
        #![doc = "`api.json` と misskey-js の型定義から生成したバインディング。手で編集せず、`cargo codegen` を実行すること。"]
        pub mod api;
        #[cfg(feature = "streaming")]
        pub mod streaming;
        pub mod types;
        #[doc = "このバインディングの生成元である `api.json` の `info.version`。"]
        pub const API_VERSION: &str = #version;
        #[doc = "マーカー: クライアントが相手にするサーバを、このバージョンセットで読む。"]
        #[derive(Debug, Clone, Copy)]
        pub struct Version;
        #[doc = "このバージョンセットでサーバを読むクライアント。エンドポイントのビルダーはここからメソッドの連鎖で作る。"]
        pub type Client = crate::client::Client<Version>;
    }
}

fn types_file(doc: &str, items: &[Item], module: &Ident) -> TokenStream {
    let items = items.iter().map(|item| item_tokens(item, module));
    quote! {
        #![doc = #doc]
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
                let fields = crate::endpoint::multipart_fields(&self)?;
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

/// 型状態のビルダーの出力の指定。
/// REST の `Call` とストリーミングの `Connect` が、所有者と終端の呼び出しを替えて使う。
pub struct BuilderSpec<'a> {
    /// ビルダーの型名。子のビルダーも同じ名前を持つ。
    pub builder: Ident,
    /// ビルダーが参照で持つ所有者の型と、そのフィールド名。
    pub owner: TokenStream,
    pub owner_field: Ident,
    pub docs: Vec<String>,
    /// setter で集めるパラメータ。
    pub fields: &'a [Field],
    /// 子のビルダーを作るアクセサ（メソッド名, doc）。
    pub children: Vec<(Ident, String)>,
    /// await したときの動作。無ければ、await できない名前空間だけのビルダーになる。
    pub terminal: Option<Terminal>,
}

/// ビルダーを await したときに組み立てる構造体と、それを渡す所有者のメソッド。
pub struct Terminal {
    /// 組み立てる構造体の型。
    pub request: Ident,
    /// await の結果の型。
    pub output: TokenStream,
    /// 組み立てた構造体を受け取り、await すると `output` を返す、所有者のメソッド。
    pub method: Ident,
}

fn call_tokens(node: &ApiNode, module: &Ident) -> TokenStream {
    let docs = match &node.endpoint {
        Some(endpoint) => {
            let mut docs = vec![format!("`{}` のビルダー。", endpoint.path)];
            docs.extend(endpoint.docs.iter().skip(1).cloned());
            docs
        }
        None => vec![node_doc(node)],
    };
    let children = node
        .children
        .iter()
        .map(|child| (ident(&child.ident), node_doc(child)))
        .collect();
    let terminal = node.endpoint.as_ref().map(|_| Terminal {
        request: format_ident!("Request"),
        output: quote! { Response },
        method: format_ident!("call"),
    });
    let spec = BuilderSpec {
        builder: format_ident!("Call"),
        owner: quote! { crate::generated::#module::Client },
        owner_field: format_ident!("client"),
        docs,
        fields: node.endpoint.as_ref().map_or(&[], |e| &e.request.fields),
        children,
        terminal,
    };
    builder_tokens(&spec, module)
}

pub fn builder_tokens(spec: &BuilderSpec<'_>, module: &Ident) -> TokenStream {
    let kind = StructKind::Request;
    let builder = &spec.builder;
    let owner = &spec.owner;
    let owner_field = &spec.owner_field;
    let fields = spec.fields;
    let required: Vec<&Field> = fields.iter().filter(|f| f.required).collect();
    let optional: Vec<&Field> = fields.iter().filter(|f| !f.required).collect();
    let slots: Vec<Ident> = (0..required.len()).map(|i| format_ident!("S{i}")).collect();
    let unset = vec![quote! { crate::builder::Unset }; required.len()];
    let set = vec![quote! { crate::builder::Set }; required.len()];
    let typed = !required.is_empty();

    let mut docs = spec.docs.clone();
    if typed {
        let names: Vec<String> = required.iter().map(|f| format!("`{}`", f.ident)).collect();
        docs.push(format!(
            "必須パラメータ（型状態のスロットの順）: {}。すべて設定すると、呼び出しを await できる。",
            names.join("、")
        ));
    }
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
        quote! { impl<'a, #(#slots),*> #builder<'a, (#(#slots,)*)> }
    } else {
        quote! { impl<'a> #builder<'a> }
    };

    let children = spec.children.iter().map(|(name, doc)| {
        quote! {
            #[doc = #doc]
            pub fn #name(&self) -> #name::#builder<'a> {
                #name::#builder::new(self.#owner_field)
            }
        }
    });

    let optional_setters = optional.iter().flat_map(|f| {
        let name = ident(&f.ident);
        let base = type_tokens(&f.base, module);
        let doc = doc_attrs(&f.docs);
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
        let doc = doc_attrs(&f.docs);
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
            pub fn #name(self, value: impl ::std::convert::Into<#base>) -> #builder<'a, (#(#output,)*)> {
                #builder { #owner_field: self.#owner_field, #(#set_moves)* _state: ::std::marker::PhantomData }
            }
        }];
        if f.nullable {
            let null_name = format_ident!("{}", f.null_setter());
            let null_doc = format!("`{}` を `null` として送る。", f.json_name);
            let null_moves = moves(quote! { ::std::option::Option::Some(::std::option::Option::None) });
            setters.push(quote! {
                #[doc = #null_doc]
                pub fn #null_name(self) -> #builder<'a, (#(#output,)*)> {
                    #builder { #owner_field: self.#owner_field, #(#null_moves)* _state: ::std::marker::PhantomData }
                }
            });
        }
        setters
    });

    let into_future = spec.terminal.as_ref().map(|terminal| {
        let state = if typed {
            quote! { #builder<'a, (#(#set,)*)> }
        } else {
            quote! { #builder<'a> }
        };
        let inits = fields.iter().map(|f| {
            let name = ident(&f.ident);
            if f.required {
                quote! { #name: self.#name.expect("required parameter is set by the type state"), }
            } else {
                quote! { #name: self.#name, }
            }
        });
        let Terminal {
            request,
            output,
            method,
        } = terminal;
        quote! {
            impl<'a> ::std::future::IntoFuture for #state {
                type Output = ::std::result::Result<#output, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, #output>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = #request { #(#inits)* };
                    let #owner_field = self.#owner_field;
                    ::std::boxed::Box::pin(async move { #owner_field.#method(request).await })
                }
            }
        }
    });
    // 名前空間だけの Call は await できないので、エンドポイントのメソッドまでたどるよう案内する。
    let must_use = match spec.terminal {
        Some(_) => "builders do nothing unless you `.await` them",
        None => "namespace builders do nothing; call an endpoint method and `.await` it",
    };

    quote! {
        #docs
        #[must_use = #must_use]
        pub struct #builder #generics {
            #owner_field: &'a #owner,
            #(#storage)*
            #state_field
        }
        impl<'a> #builder<'a> {
            pub(crate) fn new(#owner_field: &'a #owner) -> Self {
                #builder { #owner_field, #(#init)* #state_init }
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

fn streaming_mod_file(def: &StreamingDef, module: &Ident) -> TokenStream {
    let modules = def.channels.iter().map(|channel| {
        let name = ident(&channel.ident);
        quote! {
            pub mod #name;
        }
    });
    let accessors = def.channels.iter().map(|channel| {
        let name = ident(&channel.ident);
        let doc = format!("`{}` チャンネルにつなぐビルダー。", channel.name);
        quote! {
            #[doc = #doc]
            pub fn #name(&self) -> #name::Connect<'_> {
                #name::Connect::new(self)
            }
        }
    });
    let items = def.items.iter().map(|item| item_tokens(item, module));
    quote! {
        #![doc = "ストリーミング API のチャンネル。misskey-js の `streaming.types.ts` から生成した。"]
        #![allow(clippy::large_enum_variant)]
        pub mod types;
        #(#modules)*
        #[doc = "このバージョンセットでストリーミング API を使うストリーム。チャンネルのビルダーはここからメソッドで作る。"]
        pub type Stream = crate::stream::Stream<super::Version>;
        impl Stream {
            #(#accessors)*
            #[doc = "ストリーム全体のイベント（`noteUpdated`・`emojiAdded` など）の受け口。"]
            pub fn broadcast(&self) -> crate::stream::Receiver<BroadcastEvent> {
                self.broadcast_receiver()
            }
        }
        #(#items)*
    }
}

fn channel_file(channel: &ChannelDef, module: &Ident) -> TokenStream {
    let name = &channel.name;
    let doc = format!("`{name}` チャンネル。");
    let items = channel.items.iter().map(|item| item_tokens(item, module));
    let params = struct_tokens(&channel.params, module);
    let events = envelope_tokens(&channel.events, module);
    let messages = channel
        .messages
        .as_ref()
        .map(|messages| envelope_tokens(messages, module));
    let message_ty = if channel.messages.is_some() {
        quote! { Message }
    } else {
        quote! { crate::stream::NoMessage }
    };
    let connection_doc = format!("`{name}` への接続。");
    let connect = builder_tokens(
        &BuilderSpec {
            builder: format_ident!("Connect"),
            owner: quote! { crate::generated::#module::streaming::Stream },
            owner_field: format_ident!("stream"),
            docs: vec![format!(
                "`{name}` チャンネルにつなぐビルダー。await でつなぐ。"
            )],
            fields: &channel.params.fields,
            children: Vec::new(),
            terminal: Some(Terminal {
                request: format_ident!("Params"),
                output: quote! { Connection },
                method: format_ident!("connect"),
            }),
        },
        module,
    );
    quote! {
        #![doc = #doc]
        #![allow(clippy::type_complexity, clippy::large_enum_variant)]
        #(#items)*
        #params
        impl crate::stream::Channel for Params {
            const NAME: &'static str = #name;
            type Event = Event;
            type Message = #message_ty;
        }
        #events
        #messages
        #[doc = #connection_doc]
        pub type Connection = crate::stream::Connection<Params>;
        #connect
    }
}

pub fn item_tokens(item: &Item, module: &Ident) -> TokenStream {
    match item {
        Item::Struct(def) => struct_tokens(def, module),
        Item::StrEnum(def) => str_enum_tokens(def),
        Item::Union(def) => union_tokens(def, module),
        Item::Tagged(def) => tagged_tokens(def, module),
        Item::Envelope(def) => envelope_tokens(def, module),
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
        let doc = doc_attrs(&f.docs);
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

/// `{type, body}` の enum。serde の derive には受け皿のバリアントを足せないので、
/// [`Envelope`](crate::stream::Envelope) を介して読み書きする impl を書き出す。
fn envelope_tokens(def: &EnvelopeDef, module: &Ident) -> TokenStream {
    let name = ident(&def.name);
    let docs = doc_attrs(&def.docs);
    let variants = def.variants.iter().map(|variant| {
        let docs = doc_attrs(&variant.docs);
        let ident = ident(&variant.ident);
        match &variant.payload {
            Some(ty) => {
                let ty = type_tokens(ty, module);
                quote! { #docs #ident(#ty), }
            }
            None => quote! { #docs #ident, },
        }
    });
    let catch_all = ident(CATCH_ALL);
    let catch_all_variant = def.catch_all.then(|| {
        quote! {
            #[doc = "このバージョンのバインディングが知らないイベント。"]
            #catch_all { r#type: ::std::string::String, body: ::serde_json::Value },
        }
    });
    let from_arms = def.variants.iter().map(|variant| {
        let json_name = &variant.json_name;
        let ident = ident(&variant.ident);
        match &variant.payload {
            Some(_) => quote! {
                #json_name => #name::#ident(
                    ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
                ),
            },
            None => quote! { #json_name => #name::#ident, },
        }
    });
    let from_fallback = if def.catch_all {
        quote! { _ => #name::#catch_all { r#type, body }, }
    } else {
        let known = def.variants.iter().map(|variant| &variant.json_name);
        quote! {
            _ => return ::std::result::Result::Err(::serde::de::Error::unknown_variant(&r#type, &[#(#known),*])),
        }
    };
    let body_binding = if def.catch_all || def.variants.iter().any(|v| v.payload.is_some()) {
        quote! { body }
    } else {
        quote! { body: _ }
    };
    let into_arms = def.variants.iter().map(|variant| {
        let json_name = &variant.json_name;
        let ident = ident(&variant.ident);
        match &variant.payload {
            Some(_) => quote! {
                #name::#ident(body) => (#json_name, ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?),
            },
            None => quote! { #name::#ident => (#json_name, ::serde_json::Value::Null), },
        }
    });
    let into_catch_all = def.catch_all.then(|| {
        quote! { #name::#catch_all { r#type, body } => (r#type.as_str(), body.clone()), }
    });
    quote! {
        #docs
        #[derive(Debug, Clone, PartialEq)]
        pub enum #name {
            #(#variants)*
            #catch_all_variant
        }
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
                let crate::stream::Envelope { r#type, #body_binding } = ::serde::Deserialize::deserialize(deserializer)?;
                ::std::result::Result::Ok(match r#type.as_str() {
                    #(#from_arms)*
                    #from_fallback
                })
            }
        }
        impl ::serde::Serialize for #name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
                let (r#type, body) = match self {
                    #(#into_arms)*
                    #into_catch_all
                };
                ::serde::Serialize::serialize(&crate::stream::Envelope { r#type: r#type.to_owned(), body }, serializer)
            }
        }
    }
}

/// `type` の値で変種を選ぶ object の union。変種の構造体には `type` が無いので、読むときは残りを構造体に読み、
/// 書くときは `type` を足す。
fn tagged_tokens(def: &TaggedUnionDef, module: &Ident) -> TokenStream {
    let name = ident(&def.name);
    let docs = doc_attrs(&def.docs);
    let variants = def.variants.iter().map(|variant| {
        let ident = ident(&variant.ident);
        let ty = type_tokens(&variant.ty, module);
        quote! { #ident(#ty), }
    });
    let catch_all = ident(CATCH_ALL);
    let from_arms = def.variants.iter().map(|variant| {
        let ident = ident(&variant.ident);
        let value = &variant.value;
        quote! {
            ::std::option::Option::Some(#value) => #name::#ident(
                ::serde::Deserialize::deserialize(value).map_err(::serde::de::Error::custom)?,
            ),
        }
    });
    let into_arms = def.variants.iter().map(|variant| {
        let ident = ident(&variant.ident);
        let value = &variant.value;
        quote! {
            #name::#ident(inner) => {
                let mut value = ::serde_json::to_value(inner).map_err(::serde::ser::Error::custom)?;
                if let ::serde_json::Value::Object(object) = &mut value {
                    object.insert("type".to_owned(), ::serde_json::Value::String(#value.to_owned()));
                }
                ::serde::Serialize::serialize(&value, serializer)
            }
        }
    });
    quote! {
        #docs
        #[derive(Debug, Clone, PartialEq)]
        pub enum #name {
            #(#variants)*
            #[doc = "このバージョンのバインディングが知らない形。"]
            #catch_all(::serde_json::Value),
        }
        impl<'de> ::serde::Deserialize<'de> for #name {
            fn deserialize<D: ::serde::Deserializer<'de>>(deserializer: D) -> ::std::result::Result<Self, D::Error> {
                let value = <::serde_json::Value as ::serde::Deserialize>::deserialize(deserializer)?;
                let tag = value.get("type").and_then(::serde_json::Value::as_str).map(str::to_owned);
                ::std::result::Result::Ok(match tag.as_deref() {
                    #(#from_arms)*
                    _ => #name::#catch_all(value),
                })
            }
        }
        impl ::serde::Serialize for #name {
            fn serialize<S: ::serde::Serializer>(&self, serializer: S) -> ::std::result::Result<S::Ok, S::Error> {
                match self {
                    #(#into_arms)*
                    #name::#catch_all(value) => ::serde::Serialize::serialize(value, serializer),
                }
            }
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
        RustType::Streaming(name) => {
            let name = ident(name);
            quote! { crate::generated::#module::streaming::types::#name }
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
/// code span（codegen が書く正規表現や既定値）の中はリンクにならず、`\` もそのまま出るので、置き換えない。
fn escape_doc(doc: &str) -> String {
    doc.split('`')
        .enumerate()
        .map(|(i, part)| {
            if i % 2 == 0 {
                part.replace('[', "\\[").replace(']', "\\]")
            } else {
                part.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("`")
}

/// クレートの直下に出すバージョンセットの名前。ほかは feature と同じ名前のモジュールに入れる。
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
                #[cfg(all(feature = #name, feature = "streaming"))]
                pub use #module::streaming;
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
