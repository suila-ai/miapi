#![doc = "`pages/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`pages/` 以下のエンドポイント"]
#[must_use = "namespace builders do nothing; call an endpoint method and `.await` it"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`pages/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`pages/delete`"]
    pub fn delete(&self) -> delete::Call<'a> {
        delete::Call::new(self.client)
    }
    #[doc = "`pages/featured`"]
    pub fn featured(&self) -> featured::Call<'a> {
        featured::Call::new(self.client)
    }
    #[doc = "`pages/like`"]
    pub fn like(&self) -> like::Call<'a> {
        like::Call::new(self.client)
    }
    #[doc = "`pages/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`pages/unlike`"]
    pub fn unlike(&self) -> unlike::Call<'a> {
        unlike::Call::new(self.client)
    }
    #[doc = "`pages/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
    }
}
#[doc = "`pages/create`"]
pub mod create {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestFont {
        #[serde(rename = "serif")]
        Serif,
        #[serde(rename = "sans-serif")]
        SansSerif,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`pages/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:pages*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub title: ::std::string::String,
        #[doc = "文字数は 1 以上。形式は正規表現 `^[^\\s:\\/?#\\[\\]@!$&'()*+,;=\\\\%\\x00-\\x20]{1,256}$` に合うもの。"]
        pub name: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub summary: ::std::option::Option<::std::option::Option<::std::string::String>>,
        pub content: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub variables:
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        pub script: ::std::string::String,
        #[serde(
            rename = "eyeCatchingImageId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub eye_catching_image_id:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[doc = "省略時は `\"sans-serif\"`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub font: ::std::option::Option<RequestFont>,
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "alignCenter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub align_center: ::std::option::Option<bool>,
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "hideTitleWhenPinned",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub hide_title_when_pinned: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "pages/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::Page;
    #[doc = "`pages/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:pages*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `title`、`name`、`content`、`variables`、`script`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<
        'a,
        S = (
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
        ),
    > {
        client: &'a crate::generated::v2025_10_0::Client,
        title: ::std::option::Option<::std::string::String>,
        name: ::std::option::Option<::std::string::String>,
        summary: ::std::option::Option<::std::option::Option<::std::string::String>>,
        content: ::std::option::Option<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        >,
        variables: ::std::option::Option<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        >,
        script: ::std::option::Option<::std::string::String>,
        eye_catching_image_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        font: ::std::option::Option<RequestFont>,
        align_center: ::std::option::Option<bool>,
        hide_title_when_pinned: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                title: ::std::option::Option::None,
                name: ::std::option::Option::None,
                summary: ::std::option::Option::None,
                content: ::std::option::Option::None,
                variables: ::std::option::Option::None,
                script: ::std::option::Option::None,
                eye_catching_image_id: ::std::option::Option::None,
                font: ::std::option::Option::None,
                align_center: ::std::option::Option::None,
                hide_title_when_pinned: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1, S2, S3, S4> Call<'a, (S0, S1, S2, S3, S4)> {
        pub fn summary(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_summary(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `summary` を設定し、`None` なら設定を消して送らない。`null` を送るには [`summary_null`](Self::summary_null) を使う。"]
        pub fn maybe_summary(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.summary = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`summary` を `null` として送る。"]
        pub fn summary_null(mut self) -> Self {
            self.summary = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn eye_catching_image_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.maybe_eye_catching_image_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `eyeCatchingImageId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`eye_catching_image_id_null`](Self::eye_catching_image_id_null) を使う。"]
        pub fn maybe_eye_catching_image_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.eye_catching_image_id =
                value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`eyeCatchingImageId` を `null` として送る。"]
        pub fn eye_catching_image_id_null(mut self) -> Self {
            self.eye_catching_image_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        #[doc = "省略時は `\"sans-serif\"`。"]
        pub fn font(self, value: impl ::std::convert::Into<RequestFont>) -> Self {
            self.maybe_font(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `font` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_font(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestFont>>,
        ) -> Self {
            self.font = value.map(|value| value.into());
            self
        }
        #[doc = "省略時は `false`。"]
        pub fn align_center(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_align_center(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `alignCenter` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_align_center(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.align_center = value.map(|value| value.into());
            self
        }
        #[doc = "省略時は `false`。"]
        pub fn hide_title_when_pinned(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_hide_title_when_pinned(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `hideTitleWhenPinned` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_hide_title_when_pinned(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.hide_title_when_pinned = value.map(|value| value.into());
            self
        }
        pub fn title(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1, S2, S3, S4)> {
            Call {
                client: self.client,
                title: ::std::option::Option::Some(value.into()),
                name: self.name,
                summary: self.summary,
                content: self.content,
                variables: self.variables,
                script: self.script,
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "文字数は 1 以上。形式は正規表現 `^[^\\s:\\/?#\\[\\]@!$&'()*+,;=\\\\%\\x00-\\x20]{1,256}$` に合うもの。"]
        pub fn name(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set, S2, S3, S4)> {
            Call {
                client: self.client,
                title: self.title,
                name: ::std::option::Option::Some(value.into()),
                summary: self.summary,
                content: self.content,
                variables: self.variables,
                script: self.script,
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn content(
            self,
            value: impl ::std::convert::Into<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
        ) -> Call<'a, (S0, S1, crate::builder::Set, S3, S4)> {
            Call {
                client: self.client,
                title: self.title,
                name: self.name,
                summary: self.summary,
                content: ::std::option::Option::Some(value.into()),
                variables: self.variables,
                script: self.script,
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn variables(
            self,
            value: impl ::std::convert::Into<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
        ) -> Call<'a, (S0, S1, S2, crate::builder::Set, S4)> {
            Call {
                client: self.client,
                title: self.title,
                name: self.name,
                summary: self.summary,
                content: self.content,
                variables: ::std::option::Option::Some(value.into()),
                script: self.script,
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn script(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, S1, S2, S3, crate::builder::Set)> {
            Call {
                client: self.client,
                title: self.title,
                name: self.name,
                summary: self.summary,
                content: self.content,
                variables: self.variables,
                script: ::std::option::Option::Some(value.into()),
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture
        for Call<
            'a,
            (
                crate::builder::Set,
                crate::builder::Set,
                crate::builder::Set,
                crate::builder::Set,
                crate::builder::Set,
            ),
        >
    {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                title: self
                    .title
                    .expect("required parameter is set by the type state"),
                name: self
                    .name
                    .expect("required parameter is set by the type state"),
                summary: self.summary,
                content: self
                    .content
                    .expect("required parameter is set by the type state"),
                variables: self
                    .variables
                    .expect("required parameter is set by the type state"),
                script: self
                    .script
                    .expect("required parameter is set by the type state"),
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`pages/delete`"]
pub mod delete {
    #[doc = "`pages/delete`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:pages*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "pageId")]
        pub page_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "pages/delete";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`pages/delete` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:pages*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `page_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        page_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                page_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn page_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                page_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                page_id: self
                    .page_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`pages/featured`"]
pub mod featured {
    #[doc = "`pages/featured`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "pages/featured";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Page>;
    #[doc = "`pages/featured` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`pages/like`"]
pub mod like {
    #[doc = "`pages/like`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:page-likes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "pageId")]
        pub page_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "pages/like";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`pages/like` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:page-likes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `page_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        page_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                page_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn page_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                page_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                page_id: self
                    .page_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`pages/show`"]
pub mod show {
    #[doc = "`pages/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須: `pageId` / `name` と `username` のいずれか。"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "pageId", default, skip_serializing_if = "Option::is_none")]
        pub page_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "pages/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::Page;
    #[doc = "`pages/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        page_id: ::std::option::Option<::std::string::String>,
        name: ::std::option::Option<::std::string::String>,
        username: ::std::option::Option<::std::string::String>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                page_id: ::std::option::Option::None,
                name: ::std::option::Option::None,
                username: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn page_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_page_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `pageId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_page_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.page_id = value.map(|value| value.into());
            self
        }
        pub fn name(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_name(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `name` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_name(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.name = value.map(|value| value.into());
            self
        }
        pub fn username(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_username(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `username` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_username(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.username = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                page_id: self.page_id,
                name: self.name,
                username: self.username,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`pages/unlike`"]
pub mod unlike {
    #[doc = "`pages/unlike`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:page-likes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "pageId")]
        pub page_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "pages/unlike";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`pages/unlike` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:page-likes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `page_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        page_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                page_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn page_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                page_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                page_id: self
                    .page_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`pages/update`"]
pub mod update {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestFont {
        #[serde(rename = "serif")]
        Serif,
        #[serde(rename = "sans-serif")]
        SansSerif,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`pages/update`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:pages*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "pageId")]
        pub page_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[doc = "文字数は 1 以上。形式は正規表現 `^[^\\s:\\/?#\\[\\]@!$&'()*+,;=\\\\%\\x00-\\x20]{1,256}$` に合うもの。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub summary: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub content: ::std::option::Option<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        >,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub variables: ::std::option::Option<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        >,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub script: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "eyeCatchingImageId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub eye_catching_image_id:
            ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub font: ::std::option::Option<RequestFont>,
        #[serde(
            rename = "alignCenter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub align_center: ::std::option::Option<bool>,
        #[serde(
            rename = "hideTitleWhenPinned",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub hide_title_when_pinned: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "pages/update";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`pages/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:pages*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `page_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        page_id: ::std::option::Option<::std::string::String>,
        title: ::std::option::Option<::std::string::String>,
        name: ::std::option::Option<::std::string::String>,
        summary: ::std::option::Option<::std::option::Option<::std::string::String>>,
        content: ::std::option::Option<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        >,
        variables: ::std::option::Option<
            ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
        >,
        script: ::std::option::Option<::std::string::String>,
        eye_catching_image_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        font: ::std::option::Option<RequestFont>,
        align_center: ::std::option::Option<bool>,
        hide_title_when_pinned: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                page_id: ::std::option::Option::None,
                title: ::std::option::Option::None,
                name: ::std::option::Option::None,
                summary: ::std::option::Option::None,
                content: ::std::option::Option::None,
                variables: ::std::option::Option::None,
                script: ::std::option::Option::None,
                eye_catching_image_id: ::std::option::Option::None,
                font: ::std::option::Option::None,
                align_center: ::std::option::Option::None,
                hide_title_when_pinned: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn title(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_title(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `title` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_title(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.title = value.map(|value| value.into());
            self
        }
        #[doc = "文字数は 1 以上。形式は正規表現 `^[^\\s:\\/?#\\[\\]@!$&'()*+,;=\\\\%\\x00-\\x20]{1,256}$` に合うもの。"]
        pub fn name(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_name(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `name` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_name(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.name = value.map(|value| value.into());
            self
        }
        pub fn summary(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_summary(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `summary` を設定し、`None` なら設定を消して送らない。`null` を送るには [`summary_null`](Self::summary_null) を使う。"]
        pub fn maybe_summary(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.summary = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`summary` を `null` として送る。"]
        pub fn summary_null(mut self) -> Self {
            self.summary = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn content(
            self,
            value: impl ::std::convert::Into<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
        ) -> Self {
            self.maybe_content(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `content` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_content(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<
                    ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
                >,
            >,
        ) -> Self {
            self.content = value.map(|value| value.into());
            self
        }
        pub fn variables(
            self,
            value: impl ::std::convert::Into<
                ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
            >,
        ) -> Self {
            self.maybe_variables(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `variables` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_variables(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<
                    ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
                >,
            >,
        ) -> Self {
            self.variables = value.map(|value| value.into());
            self
        }
        pub fn script(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_script(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `script` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_script(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.script = value.map(|value| value.into());
            self
        }
        pub fn eye_catching_image_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.maybe_eye_catching_image_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `eyeCatchingImageId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`eye_catching_image_id_null`](Self::eye_catching_image_id_null) を使う。"]
        pub fn maybe_eye_catching_image_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.eye_catching_image_id =
                value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`eyeCatchingImageId` を `null` として送る。"]
        pub fn eye_catching_image_id_null(mut self) -> Self {
            self.eye_catching_image_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn font(self, value: impl ::std::convert::Into<RequestFont>) -> Self {
            self.maybe_font(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `font` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_font(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestFont>>,
        ) -> Self {
            self.font = value.map(|value| value.into());
            self
        }
        pub fn align_center(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_align_center(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `alignCenter` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_align_center(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.align_center = value.map(|value| value.into());
            self
        }
        pub fn hide_title_when_pinned(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_hide_title_when_pinned(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `hideTitleWhenPinned` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_hide_title_when_pinned(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.hide_title_when_pinned = value.map(|value| value.into());
            self
        }
        pub fn page_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                page_id: ::std::option::Option::Some(value.into()),
                title: self.title,
                name: self.name,
                summary: self.summary,
                content: self.content,
                variables: self.variables,
                script: self.script,
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                page_id: self
                    .page_id
                    .expect("required parameter is set by the type state"),
                title: self.title,
                name: self.name,
                summary: self.summary,
                content: self.content,
                variables: self.variables,
                script: self.script,
                eye_catching_image_id: self.eye_catching_image_id,
                font: self.font,
                align_center: self.align_center,
                hide_title_when_pinned: self.hide_title_when_pinned,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
