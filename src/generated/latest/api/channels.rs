#![doc = "`channels/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`channels/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::latest::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`channels/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`channels/favorite`"]
    pub fn favorite(&self) -> favorite::Call<'a> {
        favorite::Call::new(self.client)
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
    #[doc = "`channels/mute/` 以下のエンドポイント"]
    pub fn mute(&self) -> mute::Call<'a> {
        mute::Call::new(self.client)
    }
    #[doc = "`channels/my-favorites`"]
    pub fn my_favorites(&self) -> my_favorites::Call<'a> {
        my_favorites::Call::new(self.client)
    }
    #[doc = "`channels/owned`"]
    pub fn owned(&self) -> owned::Call<'a> {
        owned::Call::new(self.client)
    }
    #[doc = "`channels/search`"]
    pub fn search(&self) -> search::Call<'a> {
        search::Call::new(self.client)
    }
    #[doc = "`channels/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`channels/timeline`"]
    pub fn timeline(&self) -> timeline::Call<'a> {
        timeline::Call::new(self.client)
    }
    #[doc = "`channels/unfavorite`"]
    pub fn unfavorite(&self) -> unfavorite::Call<'a> {
        unfavorite::Call::new(self.client)
    }
    #[doc = "`channels/unfollow`"]
    pub fn unfollow(&self) -> unfollow::Call<'a> {
        unfollow::Call::new(self.client)
    }
    #[doc = "`channels/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
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
    pub type Response = crate::generated::latest::types::Channel;
    #[doc = "`channels/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        name: ::std::option::Option<::std::string::String>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        banner_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        color: ::std::option::Option<::std::string::String>,
        is_sensitive: ::std::option::Option<::std::option::Option<bool>>,
        allow_renote_to_external: ::std::option::Option<::std::option::Option<bool>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        pub fn description(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.description =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`description` を `null` として送る。"]
        pub fn description_null(mut self) -> Self {
            self.description = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn banner_id(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.banner_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`bannerId` を `null` として送る。"]
        pub fn banner_id_null(mut self) -> Self {
            self.banner_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn color(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.color = ::std::option::Option::Some(value.into());
            self
        }
        pub fn is_sensitive(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.is_sensitive =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`isSensitive` を `null` として送る。"]
        pub fn is_sensitive_null(mut self) -> Self {
            self.is_sensitive = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn allow_renote_to_external(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.allow_renote_to_external =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
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
        client: &'a crate::generated::latest::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Channel>;
    #[doc = "`channels/featured` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        client: &'a crate::generated::latest::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
        pub since_date: ::std::option::Option<i64>,
        #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
        pub until_date: ::std::option::Option<i64>,
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Channel>;
    #[doc = "`channels/followed` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn since_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.since_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.since_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.until_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
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
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`channels/mute/` 以下のエンドポイント"]
pub mod mute {
    #[doc = "`channels/mute/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`channels/mute/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`channels/mute/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`channels/mute/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
    }
    #[doc = "`channels/mute/create`"]
    pub mod create {
        #[doc = "`channels/mute/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "channelId")]
            pub channel_id: ::std::string::String,
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
            const PATH: &'static str = "channels/mute/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`channels/mute/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            channel_id: ::std::option::Option<::std::string::String>,
            expires_at: ::std::option::Option<::std::option::Option<i64>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    channel_id: ::std::option::Option::None,
                    expires_at: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "A Unix Epoch timestamp that must lie in the future. `null` means an indefinite mute."]
            pub fn expires_at(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.expires_at =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`expiresAt` を `null` として送る。"]
            pub fn expires_at_null(mut self) -> Self {
                self.expires_at = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn channel_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    channel_id: ::std::option::Option::Some(value.into()),
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
                    channel_id: self
                        .channel_id
                        .expect("required parameter is set by the type state"),
                    expires_at: self.expires_at,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`channels/mute/delete`"]
    pub mod delete {
        #[doc = "`channels/mute/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "channelId")]
            pub channel_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "channels/mute/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`channels/mute/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            channel_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
    #[doc = "`channels/mute/list`"]
    pub mod list {
        #[doc = "`channels/mute/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "channels/mute/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::latest::types::Channel>;
        #[doc = "`channels/mute/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
        pub struct Call<'a> {
            client: &'a crate::generated::latest::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Channel>;
    #[doc = "`channels/my-favorites` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
        pub since_date: ::std::option::Option<i64>,
        #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
        pub until_date: ::std::option::Option<i64>,
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Channel>;
    #[doc = "`channels/owned` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:channels*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn since_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.since_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.since_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.until_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
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
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
            };
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
        #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
        pub since_date: ::std::option::Option<i64>,
        #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
        pub until_date: ::std::option::Option<i64>,
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Channel>;
    #[doc = "`channels/search` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `query`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        query: ::std::option::Option<::std::string::String>,
        r#type: ::std::option::Option<RequestType>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                query: ::std::option::Option::None,
                r#type: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn r#type(mut self, value: impl ::std::convert::Into<RequestType>) -> Self {
            self.r#type = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.since_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.since_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.until_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
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
                since_date: self.since_date,
                until_date: self.until_date,
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
                since_date: self.since_date,
                until_date: self.until_date,
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
    pub type Response = crate::generated::latest::types::Channel;
    #[doc = "`channels/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Note>;
    #[doc = "`channels/timeline` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
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
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.since_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.since_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.until_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn allow_partial(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.allow_partial = ::std::option::Option::Some(value.into());
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
        client: &'a crate::generated::latest::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        client: &'a crate::generated::latest::Client,
        channel_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
    pub type Response = crate::generated::latest::types::Channel;
    #[doc = "`channels/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:channels*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `channel_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
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
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        pub fn name(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.name = ::std::option::Option::Some(value.into());
            self
        }
        pub fn description(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.description =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`description` を `null` として送る。"]
        pub fn description_null(mut self) -> Self {
            self.description = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn banner_id(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.banner_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`bannerId` を `null` として送る。"]
        pub fn banner_id_null(mut self) -> Self {
            self.banner_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn is_archived(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.is_archived =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`isArchived` を `null` として送る。"]
        pub fn is_archived_null(mut self) -> Self {
            self.is_archived = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn pinned_note_ids(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.pinned_note_ids = ::std::option::Option::Some(value.into());
            self
        }
        pub fn color(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.color = ::std::option::Option::Some(value.into());
            self
        }
        pub fn is_sensitive(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.is_sensitive =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`isSensitive` を `null` として送る。"]
        pub fn is_sensitive_null(mut self) -> Self {
            self.is_sensitive = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn allow_renote_to_external(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.allow_renote_to_external =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
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
