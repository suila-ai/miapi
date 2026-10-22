#![doc = "`app/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`app/` 以下のエンドポイント"]
#[must_use = "namespace builders do nothing; call an endpoint method and `.await` it"]
pub struct Call<'a> {
    client: &'a crate::generated::v2026_5_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`app/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`app/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
}
#[doc = "`app/create`"]
pub mod create {
    #[doc = "`app/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub name: ::std::string::String,
        pub description: ::std::string::String,
        #[doc = "要素は重複させない。"]
        pub permission: ::std::vec::Vec<::std::string::String>,
        #[serde(
            rename = "callbackUrl",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub callback_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "app/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2026_5_1::types::App;
    #[doc = "`app/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`description`、`permission`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<
        'a,
        S = (
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
        ),
    > {
        client: &'a crate::generated::v2026_5_1::Client,
        name: ::std::option::Option<::std::string::String>,
        description: ::std::option::Option<::std::string::String>,
        permission: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        callback_url: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                name: ::std::option::Option::None,
                description: ::std::option::Option::None,
                permission: ::std::option::Option::None,
                callback_url: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
        pub fn callback_url(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_callback_url(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `callbackUrl` を設定し、`None` なら設定を消して送らない。`null` を送るには [`callback_url_null`](Self::callback_url_null) を使う。"]
        pub fn maybe_callback_url(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.callback_url = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`callbackUrl` を `null` として送る。"]
        pub fn callback_url_null(mut self) -> Self {
            self.callback_url = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn name(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1, S2)> {
            Call {
                client: self.client,
                name: ::std::option::Option::Some(value.into()),
                description: self.description,
                permission: self.permission,
                callback_url: self.callback_url,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn description(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set, S2)> {
            Call {
                client: self.client,
                name: self.name,
                description: ::std::option::Option::Some(value.into()),
                permission: self.permission,
                callback_url: self.callback_url,
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "要素は重複させない。"]
        pub fn permission(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Call<'a, (S0, S1, crate::builder::Set)> {
            Call {
                client: self.client,
                name: self.name,
                description: self.description,
                permission: ::std::option::Option::Some(value.into()),
                callback_url: self.callback_url,
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
            ),
        >
    {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                name: self
                    .name
                    .expect("required parameter is set by the type state"),
                description: self
                    .description
                    .expect("required parameter is set by the type state"),
                permission: self
                    .permission
                    .expect("required parameter is set by the type state"),
                callback_url: self.callback_url,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`app/show`"]
pub mod show {
    #[doc = "`app/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "appId")]
        pub app_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "app/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2026_5_1::types::App;
    #[doc = "`app/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `app_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        app_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                app_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn app_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                app_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                app_id: self
                    .app_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
