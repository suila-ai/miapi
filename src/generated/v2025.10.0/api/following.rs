#![doc = "`following/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`following/` 以下のエンドポイント"]
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
    #[doc = "`following/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`following/delete`"]
    pub fn delete(&self) -> delete::Call<'a> {
        delete::Call::new(self.client)
    }
    #[doc = "`following/invalidate`"]
    pub fn invalidate(&self) -> invalidate::Call<'a> {
        invalidate::Call::new(self.client)
    }
    #[doc = "`following/requests/` 以下のエンドポイント"]
    pub fn requests(&self) -> requests::Call<'a> {
        requests::Call::new(self.client)
    }
    #[doc = "`following/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
    }
    #[doc = "`following/update-all`"]
    pub fn update_all(&self) -> update_all::Call<'a> {
        update_all::Call::new(self.client)
    }
}
#[doc = "`following/create`"]
pub mod create {
    #[doc = "`following/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "following/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::UserLite;
    #[doc = "`following/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        user_id: ::std::option::Option<::std::string::String>,
        with_replies: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn with_replies(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_replies(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withReplies` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_replies(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_replies = value.map(|value| value.into());
            self
        }
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                with_replies: self.with_replies,
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
                with_replies: self.with_replies,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`following/delete`"]
pub mod delete {
    #[doc = "`following/delete`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "following/delete";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::UserLite;
    #[doc = "`following/delete` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
#[doc = "`following/invalidate`"]
pub mod invalidate {
    #[doc = "`following/invalidate`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "following/invalidate";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::UserLite;
    #[doc = "`following/invalidate` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
#[doc = "`following/requests/` 以下のエンドポイント"]
pub mod requests {
    #[doc = "`following/requests/` 以下のエンドポイント"]
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
        #[doc = "`following/requests/accept`"]
        pub fn accept(&self) -> accept::Call<'a> {
            accept::Call::new(self.client)
        }
        #[doc = "`following/requests/cancel`"]
        pub fn cancel(&self) -> cancel::Call<'a> {
            cancel::Call::new(self.client)
        }
        #[doc = "`following/requests/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`following/requests/reject`"]
        pub fn reject(&self) -> reject::Call<'a> {
            reject::Call::new(self.client)
        }
        #[doc = "`following/requests/sent`"]
        pub fn sent(&self) -> sent::Call<'a> {
            sent::Call::new(self.client)
        }
    }
    #[doc = "`following/requests/accept`"]
    pub mod accept {
        #[doc = "`following/requests/accept`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "following/requests/accept";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`following/requests/accept` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
    #[doc = "`following/requests/cancel`"]
    pub mod cancel {
        #[doc = "`following/requests/cancel`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "following/requests/cancel";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::UserLite;
        #[doc = "`following/requests/cancel` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
    #[doc = "`following/requests/list`"]
    pub mod list {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            pub follower: crate::generated::v2025_10_0::types::UserLite,
            pub followee: crate::generated::v2025_10_0::types::UserLite,
        }
        #[doc = "`following/requests/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:following*"]
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
            #[doc = "1 以上 100 以下。省略時は `10`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "following/requests/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`following/requests/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:following*"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            limit: ::std::option::Option<i64>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
            #[doc = "1 以上 100 以下。省略時は `10`。"]
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
                    since_date: self.since_date,
                    until_date: self.until_date,
                    limit: self.limit,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`following/requests/reject`"]
    pub mod reject {
        #[doc = "`following/requests/reject`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "following/requests/reject";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`following/requests/reject` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
    #[doc = "`following/requests/sent`"]
    pub mod sent {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            pub follower: crate::generated::v2025_10_0::types::UserLite,
            pub followee: crate::generated::v2025_10_0::types::UserLite,
        }
        #[doc = "`following/requests/sent`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:following*"]
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
            #[doc = "1 以上 100 以下。省略時は `10`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "following/requests/sent";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`following/requests/sent` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:following*"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            limit: ::std::option::Option<i64>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
            #[doc = "1 以上 100 以下。省略時は `10`。"]
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
                    since_date: self.since_date,
                    until_date: self.until_date,
                    limit: self.limit,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`following/update`"]
pub mod update {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotify {
        #[serde(rename = "normal")]
        Normal,
        #[serde(rename = "none")]
        None,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`following/update`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notify: ::std::option::Option<RequestNotify>,
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "following/update";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::UserLite;
    #[doc = "`following/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        user_id: ::std::option::Option<::std::string::String>,
        notify: ::std::option::Option<RequestNotify>,
        with_replies: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                notify: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn notify(self, value: impl ::std::convert::Into<RequestNotify>) -> Self {
            self.maybe_notify(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `notify` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_notify(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestNotify>>,
        ) -> Self {
            self.notify = value.map(|value| value.into());
            self
        }
        pub fn with_replies(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_replies(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withReplies` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_replies(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_replies = value.map(|value| value.into());
            self
        }
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                notify: self.notify,
                with_replies: self.with_replies,
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
                notify: self.notify,
                with_replies: self.with_replies,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`following/update-all`"]
pub mod update_all {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotify {
        #[serde(rename = "normal")]
        Normal,
        #[serde(rename = "none")]
        None,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`following/update-all`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub notify: ::std::option::Option<RequestNotify>,
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "following/update-all";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`following/update-all` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:following*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        notify: ::std::option::Option<RequestNotify>,
        with_replies: ::std::option::Option<bool>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                notify: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn notify(self, value: impl ::std::convert::Into<RequestNotify>) -> Self {
            self.maybe_notify(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `notify` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_notify(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestNotify>>,
        ) -> Self {
            self.notify = value.map(|value| value.into());
            self
        }
        pub fn with_replies(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_replies(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withReplies` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_replies(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_replies = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                notify: self.notify,
                with_replies: self.with_replies,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
