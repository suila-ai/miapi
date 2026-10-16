#![doc = "`miauth/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`miauth/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::latest::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`miauth/gen-token`"]
    pub fn gen_token(&self) -> gen_token::Call<'a> {
        gen_token::Call::new(self.client)
    }
}
#[doc = "`miauth/gen-token`"]
pub mod gen_token {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub token: ::std::string::String,
    }
    #[doc = "`miauth/gen-token`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub session: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "iconUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[doc = "要素は重複させない。"]
        pub permission: ::std::vec::Vec<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "miauth/gen-token";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`miauth/gen-token` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `session`、`permission`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::latest::Client,
        session: ::std::option::Option<::std::option::Option<::std::string::String>>,
        name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        icon_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        permission: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                session: ::std::option::Option::None,
                name: ::std::option::Option::None,
                description: ::std::option::Option::None,
                icon_url: ::std::option::Option::None,
                permission: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn name(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_name(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `name` を設定し、`None` なら設定を消して送らない。`null` を送るには [`name_null`](Self::name_null) を使う。"]
        pub fn maybe_name(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.name = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`name` を `null` として送る。"]
        pub fn name_null(mut self) -> Self {
            self.name = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn description(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_description(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `description` を設定し、`None` なら設定を消して送らない。`null` を送るには [`description_null`](Self::description_null) を使う。"]
        pub fn maybe_description(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.description = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`description` を `null` として送る。"]
        pub fn description_null(mut self) -> Self {
            self.description = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn icon_url(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_icon_url(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `iconUrl` を設定し、`None` なら設定を消して送らない。`null` を送るには [`icon_url_null`](Self::icon_url_null) を使う。"]
        pub fn maybe_icon_url(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.icon_url = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`iconUrl` を `null` として送る。"]
        pub fn icon_url_null(mut self) -> Self {
            self.icon_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn session(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                session: ::std::option::Option::Some(::std::option::Option::Some(value.into())),
                name: self.name,
                description: self.description,
                icon_url: self.icon_url,
                permission: self.permission,
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "`session` を `null` として送る。"]
        pub fn session_null(self) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                session: ::std::option::Option::Some(::std::option::Option::None),
                name: self.name,
                description: self.description,
                icon_url: self.icon_url,
                permission: self.permission,
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "要素は重複させない。"]
        pub fn permission(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                session: self.session,
                name: self.name,
                description: self.description,
                icon_url: self.icon_url,
                permission: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                session: self
                    .session
                    .expect("required parameter is set by the type state"),
                name: self.name,
                description: self.description,
                icon_url: self.icon_url,
                permission: self
                    .permission
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
