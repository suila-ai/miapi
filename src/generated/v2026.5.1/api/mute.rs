#![doc = "`mute/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`mute/` 以下のエンドポイント"]
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
    #[doc = "`mute/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`mute/delete`"]
    pub fn delete(&self) -> delete::Call<'a> {
        delete::Call::new(self.client)
    }
    #[doc = "`mute/list`"]
    pub fn list(&self) -> list::Call<'a> {
        list::Call::new(self.client)
    }
}
#[doc = "`mute/create`"]
pub mod create {
    #[doc = "`mute/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:mutes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "A Unix Epoch timestamp that must lie in the future. `null` means an indefinite mute."]
        #[serde(
            rename = "expiresAt",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub expires_at: ::std::option::Option<::std::option::Option<i64>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "mute/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`mute/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:mutes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        expires_at: ::std::option::Option<::std::option::Option<i64>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                expires_at: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "A Unix Epoch timestamp that must lie in the future. `null` means an indefinite mute."]
        pub fn expires_at(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_expires_at(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `expiresAt` を設定し、`None` なら設定を消して送らない。`null` を送るには [`expires_at_null`](Self::expires_at_null) を使う。"]
        pub fn maybe_expires_at(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.expires_at = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`expiresAt` を `null` として送る。"]
        pub fn expires_at_null(mut self) -> Self {
            self.expires_at = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                expires_at: self.expires_at,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
                expires_at: self.expires_at,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`mute/delete`"]
pub mod delete {
    #[doc = "`mute/delete`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:mutes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "mute/delete";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`mute/delete` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:mutes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`mute/list`"]
pub mod list {
    #[doc = "`mute/list`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:mutes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
        pub since_date: ::std::option::Option<i64>,
        #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
        pub until_date: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "mute/list";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2026_5_1::types::Muting>;
    #[doc = "`mute/list` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:mutes*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2026_5_1::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "1 以上 100 以下。省略時は `30`。"]
        pub fn limit(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_limit(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `limit` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_limit(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.limit = value.map(|value| value.into());
            self
        }
        pub fn since_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_since_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sinceId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_since_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.since_id = value.map(|value| value.into());
            self
        }
        pub fn until_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_until_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `untilId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_until_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.until_id = value.map(|value| value.into());
            self
        }
        pub fn since_date(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_since_date(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sinceDate` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_since_date(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.since_date = value.map(|value| value.into());
            self
        }
        pub fn until_date(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_until_date(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `untilDate` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_until_date(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.until_date = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
