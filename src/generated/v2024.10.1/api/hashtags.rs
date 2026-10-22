#![doc = "`hashtags/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`hashtags/` 以下のエンドポイント"]
#[must_use = "namespace builders do nothing; call an endpoint method and `.await` it"]
pub struct Call<'a> {
    client: &'a crate::generated::v2024_10_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`hashtags/list`"]
    pub fn list(&self) -> list::Call<'a> {
        list::Call::new(self.client)
    }
    #[doc = "`hashtags/search`"]
    pub fn search(&self) -> search::Call<'a> {
        search::Call::new(self.client)
    }
    #[doc = "`hashtags/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`hashtags/trend`"]
    pub fn trend(&self) -> trend::Call<'a> {
        trend::Call::new(self.client)
    }
    #[doc = "`hashtags/users`"]
    pub fn users(&self) -> users::Call<'a> {
        users::Call::new(self.client)
    }
}
#[doc = "`hashtags/list`"]
pub mod list {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSort {
        #[serde(rename = "+mentionedUsers")]
        PlusMentionedUsers,
        #[serde(rename = "-mentionedUsers")]
        MinusMentionedUsers,
        #[serde(rename = "+mentionedLocalUsers")]
        PlusMentionedLocalUsers,
        #[serde(rename = "-mentionedLocalUsers")]
        MinusMentionedLocalUsers,
        #[serde(rename = "+mentionedRemoteUsers")]
        PlusMentionedRemoteUsers,
        #[serde(rename = "-mentionedRemoteUsers")]
        MinusMentionedRemoteUsers,
        #[serde(rename = "+attachedUsers")]
        PlusAttachedUsers,
        #[serde(rename = "-attachedUsers")]
        MinusAttachedUsers,
        #[serde(rename = "+attachedLocalUsers")]
        PlusAttachedLocalUsers,
        #[serde(rename = "-attachedLocalUsers")]
        MinusAttachedLocalUsers,
        #[serde(rename = "+attachedRemoteUsers")]
        PlusAttachedRemoteUsers,
        #[serde(rename = "-attachedRemoteUsers")]
        MinusAttachedRemoteUsers,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`hashtags/list`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "attachedToUserOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attached_to_user_only: ::std::option::Option<bool>,
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "attachedToLocalUserOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attached_to_local_user_only: ::std::option::Option<bool>,
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "attachedToRemoteUserOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attached_to_remote_user_only: ::std::option::Option<bool>,
        pub sort: RequestSort,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "hashtags/list";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Hashtag>;
    #[doc = "`hashtags/list` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `sort`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
        attached_to_user_only: ::std::option::Option<bool>,
        attached_to_local_user_only: ::std::option::Option<bool>,
        attached_to_remote_user_only: ::std::option::Option<bool>,
        sort: ::std::option::Option<RequestSort>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                attached_to_user_only: ::std::option::Option::None,
                attached_to_local_user_only: ::std::option::Option::None,
                attached_to_remote_user_only: ::std::option::Option::None,
                sort: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        #[doc = "省略時は `false`。"]
        pub fn attached_to_user_only(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_attached_to_user_only(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `attachedToUserOnly` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_attached_to_user_only(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.attached_to_user_only = value.map(|value| value.into());
            self
        }
        #[doc = "省略時は `false`。"]
        pub fn attached_to_local_user_only(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_attached_to_local_user_only(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `attachedToLocalUserOnly` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_attached_to_local_user_only(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.attached_to_local_user_only = value.map(|value| value.into());
            self
        }
        #[doc = "省略時は `false`。"]
        pub fn attached_to_remote_user_only(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_attached_to_remote_user_only(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `attachedToRemoteUserOnly` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_attached_to_remote_user_only(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.attached_to_remote_user_only = value.map(|value| value.into());
            self
        }
        pub fn sort(
            self,
            value: impl ::std::convert::Into<RequestSort>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                limit: self.limit,
                attached_to_user_only: self.attached_to_user_only,
                attached_to_local_user_only: self.attached_to_local_user_only,
                attached_to_remote_user_only: self.attached_to_remote_user_only,
                sort: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                attached_to_user_only: self.attached_to_user_only,
                attached_to_local_user_only: self.attached_to_local_user_only,
                attached_to_remote_user_only: self.attached_to_remote_user_only,
                sort: self
                    .sort
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`hashtags/search`"]
pub mod search {
    #[doc = "`hashtags/search`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        pub query: ::std::string::String,
        #[doc = "省略時は `0`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "hashtags/search";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<::std::string::String>;
    #[doc = "`hashtags/search` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `query`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
        query: ::std::option::Option<::std::string::String>,
        offset: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                query: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        #[doc = "省略時は `0`。"]
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| value.into());
            self
        }
        pub fn query(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                limit: self.limit,
                query: ::std::option::Option::Some(value.into()),
                offset: self.offset,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                query: self
                    .query
                    .expect("required parameter is set by the type state"),
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`hashtags/show`"]
pub mod show {
    #[doc = "`hashtags/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub tag: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "hashtags/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Hashtag;
    #[doc = "`hashtags/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `tag`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        tag: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                tag: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn tag(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                tag: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                tag: self
                    .tag
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`hashtags/trend`"]
pub mod trend {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub tag: ::std::string::String,
        pub chart: ::std::vec::Vec<f64>,
        #[serde(rename = "usersCount")]
        pub users_count: f64,
    }
    #[doc = "`hashtags/trend`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "hashtags/trend";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`hashtags/trend` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
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
#[doc = "`hashtags/users`"]
pub mod users {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSort {
        #[serde(rename = "+follower")]
        PlusFollower,
        #[serde(rename = "-follower")]
        MinusFollower,
        #[serde(rename = "+createdAt")]
        PlusCreatedAt,
        #[serde(rename = "-createdAt")]
        MinusCreatedAt,
        #[serde(rename = "+updatedAt")]
        PlusUpdatedAt,
        #[serde(rename = "-updatedAt")]
        MinusUpdatedAt,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestState {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "alive")]
        Alive,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestOrigin {
        #[serde(rename = "combined")]
        Combined,
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "remote")]
        Remote,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`hashtags/users`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub tag: ::std::string::String,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        pub sort: RequestSort,
        #[doc = "省略時は `\"all\"`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: ::std::option::Option<RequestState>,
        #[doc = "省略時は `\"local\"`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: ::std::option::Option<RequestOrigin>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "hashtags/users";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::UserDetailed>;
    #[doc = "`hashtags/users` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `tag`、`sort`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::v2024_10_1::Client,
        tag: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        sort: ::std::option::Option<RequestSort>,
        state: ::std::option::Option<RequestState>,
        origin: ::std::option::Option<RequestOrigin>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                tag: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                sort: ::std::option::Option::None,
                state: ::std::option::Option::None,
                origin: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
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
        #[doc = "省略時は `\"all\"`。"]
        pub fn state(self, value: impl ::std::convert::Into<RequestState>) -> Self {
            self.maybe_state(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `state` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_state(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestState>>,
        ) -> Self {
            self.state = value.map(|value| value.into());
            self
        }
        #[doc = "省略時は `\"local\"`。"]
        pub fn origin(self, value: impl ::std::convert::Into<RequestOrigin>) -> Self {
            self.maybe_origin(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `origin` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_origin(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestOrigin>>,
        ) -> Self {
            self.origin = value.map(|value| value.into());
            self
        }
        pub fn tag(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                tag: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                sort: self.sort,
                state: self.state,
                origin: self.origin,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn sort(
            self,
            value: impl ::std::convert::Into<RequestSort>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                tag: self.tag,
                limit: self.limit,
                sort: ::std::option::Option::Some(value.into()),
                state: self.state,
                origin: self.origin,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                tag: self
                    .tag
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                sort: self
                    .sort
                    .expect("required parameter is set by the type state"),
                state: self.state,
                origin: self.origin,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
