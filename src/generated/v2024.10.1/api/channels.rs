#![doc = "`channels/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`channels/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2024_10_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`channels/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`channels/featured`"]
    pub fn featured(&self) -> featured::Call<'a> {
        featured::Call::new(self.client)
    }
    #[doc = "`channels/follow`"]
    pub fn follow(&self) -> follow::Call<'a> {
        follow::Call::new(self.client)
    }
    #[doc = "`channels/followed`"]
    pub fn followed(&self) -> followed::Call<'a> {
        followed::Call::new(self.client)
    }
    #[doc = "`channels/owned`"]
    pub fn owned(&self) -> owned::Call<'a> {
        owned::Call::new(self.client)
    }
    #[doc = "`channels/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`channels/timeline`"]
    pub fn timeline(&self) -> timeline::Call<'a> {
        timeline::Call::new(self.client)
    }
    #[doc = "`channels/unfollow`"]
    pub fn unfollow(&self) -> unfollow::Call<'a> {
        unfollow::Call::new(self.client)
    }
    #[doc = "`channels/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
    }
    #[doc = "`channels/favorite`"]
    pub fn favorite(&self) -> favorite::Call<'a> {
        favorite::Call::new(self.client)
    }
    #[doc = "`channels/unfavorite`"]
    pub fn unfavorite(&self) -> unfavorite::Call<'a> {
        unfavorite::Call::new(self.client)
    }
    #[doc = "`channels/my-favorites`"]
    pub fn my_favorites(&self) -> my_favorites::Call<'a> {
        my_favorites::Call::new(self.client)
    }
    #[doc = "`channels/search`"]
    pub fn search(&self) -> search::Call<'a> {
        search::Call::new(self.client)
    }
}
#[doc = "`channels/create`"]
pub mod create {
    #[doc = "`channels/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub name: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "bannerId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub banner_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "isSensitive",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub is_sensitive: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            rename = "allowRenoteToExternal",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub allow_renote_to_external: ::std::option::Option<::std::option::Option<bool>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Channel;
    #[doc = "`channels/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        name: ::std::option::Option<::std::string::String>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        banner_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        color: ::std::option::Option<::std::string::String>,
        is_sensitive: ::std::option::Option<::std::option::Option<bool>>,
        allow_renote_to_external: ::std::option::Option<::std::option::Option<bool>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                name: ::std::option::Option::None,
                description: ::std::option::Option::None,
                banner_id: ::std::option::Option::None,
                color: ::std::option::Option::None,
                is_sensitive: ::std::option::Option::None,
                allow_renote_to_external: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn banner_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_banner_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `bannerId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`banner_id_null`](Self::banner_id_null) を使う。"]
        pub fn maybe_banner_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.banner_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`bannerId` を `null` として送る。"]
        pub fn banner_id_null(mut self) -> Self {
            self.banner_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn color(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_color(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `color` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_color(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.color = value.map(|value| value.into());
            self
        }
        pub fn is_sensitive(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_sensitive(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isSensitive` を設定し、`None` なら設定を消して送らない。`null` を送るには [`is_sensitive_null`](Self::is_sensitive_null) を使う。"]
        pub fn maybe_is_sensitive(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_sensitive = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`isSensitive` を `null` として送る。"]
        pub fn is_sensitive_null(mut self) -> Self {
            self.is_sensitive = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn allow_renote_to_external(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_allow_renote_to_external(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `allowRenoteToExternal` を設定し、`None` なら設定を消して送らない。`null` を送るには [`allow_renote_to_external_null`](Self::allow_renote_to_external_null) を使う。"]
        pub fn maybe_allow_renote_to_external(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.allow_renote_to_external =
                value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`allowRenoteToExternal` を `null` として送る。"]
        pub fn allow_renote_to_external_null(mut self) -> Self {
            self.allow_renote_to_external =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn name(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                name: ::std::option::Option::Some(value.into()),
                description: self.description,
                banner_id: self.banner_id,
                color: self.color,
                is_sensitive: self.is_sensitive,
                allow_renote_to_external: self.allow_renote_to_external,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                name: self
                    .name
                    .expect("required parameter is set by the type state"),
                description: self.description,
                banner_id: self.banner_id,
                color: self.color,
                is_sensitive: self.is_sensitive,
                allow_renote_to_external: self.allow_renote_to_external,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/featured`"]
pub mod featured {
    #[doc = "`channels/featured`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/featured";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Channel>;
    #[doc = "`channels/featured` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
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
#[doc = "`channels/follow`"]
pub mod follow {
    #[doc = "`channels/follow`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/follow";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`channels/follow` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                channel_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn channel_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                channel_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                channel_id: self
                    .channel_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/followed`"]
pub mod followed {
    #[doc = "`channels/followed`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/followed";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Channel>;
    #[doc = "`channels/followed` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
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
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                since_id: self.since_id,
                until_id: self.until_id,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/owned`"]
pub mod owned {
    #[doc = "`channels/owned`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/owned";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Channel>;
    #[doc = "`channels/owned` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
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
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                since_id: self.since_id,
                until_id: self.until_id,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/show`"]
pub mod show {
    #[doc = "`channels/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Channel;
    #[doc = "`channels/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                channel_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn channel_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                channel_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                channel_id: self
                    .channel_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/timeline`"]
pub mod timeline {
    #[doc = "`channels/timeline`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
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
        #[serde(
            rename = "allowPartial",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_partial: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/timeline";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Note>;
    #[doc = "`channels/timeline` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        allow_partial: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                channel_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                allow_partial: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn allow_partial(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_allow_partial(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `allowPartial` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_allow_partial(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.allow_partial = value.map(|value| value.into());
            self
        }
        pub fn channel_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                channel_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                allow_partial: self.allow_partial,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                channel_id: self
                    .channel_id
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                allow_partial: self.allow_partial,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/unfollow`"]
pub mod unfollow {
    #[doc = "`channels/unfollow`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/unfollow";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`channels/unfollow` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                channel_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn channel_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                channel_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                channel_id: self
                    .channel_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/update`"]
pub mod update {
    #[doc = "`channels/update`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "bannerId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub banner_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "isArchived",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub is_archived: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            rename = "pinnedNoteIds",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub pinned_note_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub color: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "isSensitive",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub is_sensitive: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            rename = "allowRenoteToExternal",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub allow_renote_to_external: ::std::option::Option<::std::option::Option<bool>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/update";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Channel;
    #[doc = "`channels/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        name: ::std::option::Option<::std::string::String>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        banner_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        is_archived: ::std::option::Option<::std::option::Option<bool>>,
        pinned_note_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        color: ::std::option::Option<::std::string::String>,
        is_sensitive: ::std::option::Option<::std::option::Option<bool>>,
        allow_renote_to_external: ::std::option::Option<::std::option::Option<bool>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                channel_id: ::std::option::Option::None,
                name: ::std::option::Option::None,
                description: ::std::option::Option::None,
                banner_id: ::std::option::Option::None,
                is_archived: ::std::option::Option::None,
                pinned_note_ids: ::std::option::Option::None,
                color: ::std::option::Option::None,
                is_sensitive: ::std::option::Option::None,
                allow_renote_to_external: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn banner_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_banner_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `bannerId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`banner_id_null`](Self::banner_id_null) を使う。"]
        pub fn maybe_banner_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.banner_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`bannerId` を `null` として送る。"]
        pub fn banner_id_null(mut self) -> Self {
            self.banner_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn is_archived(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_archived(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isArchived` を設定し、`None` なら設定を消して送らない。`null` を送るには [`is_archived_null`](Self::is_archived_null) を使う。"]
        pub fn maybe_is_archived(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_archived = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`isArchived` を `null` として送る。"]
        pub fn is_archived_null(mut self) -> Self {
            self.is_archived = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn pinned_note_ids(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.maybe_pinned_note_ids(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `pinnedNoteIds` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_pinned_note_ids(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.pinned_note_ids = value.map(|value| value.into());
            self
        }
        pub fn color(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_color(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `color` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_color(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.color = value.map(|value| value.into());
            self
        }
        pub fn is_sensitive(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_sensitive(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isSensitive` を設定し、`None` なら設定を消して送らない。`null` を送るには [`is_sensitive_null`](Self::is_sensitive_null) を使う。"]
        pub fn maybe_is_sensitive(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_sensitive = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`isSensitive` を `null` として送る。"]
        pub fn is_sensitive_null(mut self) -> Self {
            self.is_sensitive = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn allow_renote_to_external(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_allow_renote_to_external(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `allowRenoteToExternal` を設定し、`None` なら設定を消して送らない。`null` を送るには [`allow_renote_to_external_null`](Self::allow_renote_to_external_null) を使う。"]
        pub fn maybe_allow_renote_to_external(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.allow_renote_to_external =
                value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`allowRenoteToExternal` を `null` として送る。"]
        pub fn allow_renote_to_external_null(mut self) -> Self {
            self.allow_renote_to_external =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn channel_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                channel_id: ::std::option::Option::Some(value.into()),
                name: self.name,
                description: self.description,
                banner_id: self.banner_id,
                is_archived: self.is_archived,
                pinned_note_ids: self.pinned_note_ids,
                color: self.color,
                is_sensitive: self.is_sensitive,
                allow_renote_to_external: self.allow_renote_to_external,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                channel_id: self
                    .channel_id
                    .expect("required parameter is set by the type state"),
                name: self.name,
                description: self.description,
                banner_id: self.banner_id,
                is_archived: self.is_archived,
                pinned_note_ids: self.pinned_note_ids,
                color: self.color,
                is_sensitive: self.is_sensitive,
                allow_renote_to_external: self.allow_renote_to_external,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/favorite`"]
pub mod favorite {
    #[doc = "`channels/favorite`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/favorite";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`channels/favorite` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                channel_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn channel_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                channel_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                channel_id: self
                    .channel_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/unfavorite`"]
pub mod unfavorite {
    #[doc = "`channels/unfavorite`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "channelId")]
        pub channel_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/unfavorite";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`channels/unfavorite` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                channel_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn channel_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                channel_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                channel_id: self
                    .channel_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/my-favorites`"]
pub mod my_favorites {
    #[doc = "`channels/my-favorites`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/my-favorites";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Channel>;
    #[doc = "`channels/my-favorites` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
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
#[doc = "`channels/search`"]
pub mod search {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestType {
        #[serde(rename = "nameAndDescription")]
        NameAndDescription,
        #[serde(rename = "nameOnly")]
        NameOnly,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`channels/search`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub query: ::std::string::String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub r#type: ::std::option::Option<RequestType>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "channels/search";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Channel>;
    #[doc = "`channels/search` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `query`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        query: ::std::option::Option<::std::string::String>,
        r#type: ::std::option::Option<RequestType>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                query: ::std::option::Option::None,
                r#type: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn r#type(self, value: impl ::std::convert::Into<RequestType>) -> Self {
            self.maybe_type(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `type` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_type(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestType>>,
        ) -> Self {
            self.r#type = value.map(|value| value.into());
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
        pub fn query(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                query: ::std::option::Option::Some(value.into()),
                r#type: self.r#type,
                since_id: self.since_id,
                until_id: self.until_id,
                limit: self.limit,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                query: self
                    .query
                    .expect("required parameter is set by the type state"),
                r#type: self.r#type,
                since_id: self.since_id,
                until_id: self.until_id,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
