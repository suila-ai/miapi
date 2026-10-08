#![doc = "`hashtags/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`hashtags/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::latest::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            rename = "attachedToUserOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attached_to_user_only: ::std::option::Option<bool>,
        #[serde(
            rename = "attachedToLocalUserOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub attached_to_local_user_only: ::std::option::Option<bool>,
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Hashtag>;
    #[doc = "`hashtags/list` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `sort`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        limit: ::std::option::Option<i64>,
        attached_to_user_only: ::std::option::Option<bool>,
        attached_to_local_user_only: ::std::option::Option<bool>,
        attached_to_remote_user_only: ::std::option::Option<bool>,
        sort: ::std::option::Option<RequestSort>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn attached_to_user_only(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.attached_to_user_only = ::std::option::Option::Some(value.into());
            self
        }
        pub fn attached_to_local_user_only(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.attached_to_local_user_only = ::std::option::Option::Some(value.into());
            self
        }
        pub fn attached_to_remote_user_only(
            mut self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.attached_to_remote_user_only = ::std::option::Option::Some(value.into());
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        pub query: ::std::string::String,
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
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        limit: ::std::option::Option<i64>,
        query: ::std::option::Option<::std::string::String>,
        offset: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn offset(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.offset = ::std::option::Option::Some(value.into());
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
    pub type Response = crate::generated::latest::types::Hashtag;
    #[doc = "`hashtags/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `tag`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        tag: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        pub sort: RequestSort,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub state: ::std::option::Option<RequestState>,
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
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::UserDetailed>;
    #[doc = "`hashtags/users` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `tag`、`sort`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::latest::Client,
        tag: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
        sort: ::std::option::Option<RequestSort>,
        state: ::std::option::Option<RequestState>,
        origin: ::std::option::Option<RequestOrigin>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                tag: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                sort: ::std::option::Option::None,
                state: ::std::option::Option::None,
                origin: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn offset(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.offset = ::std::option::Option::Some(value.into());
            self
        }
        pub fn state(mut self, value: impl ::std::convert::Into<RequestState>) -> Self {
            self.state = ::std::option::Option::Some(value.into());
            self
        }
        pub fn origin(mut self, value: impl ::std::convert::Into<RequestOrigin>) -> Self {
            self.origin = ::std::option::Option::Some(value.into());
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
                offset: self.offset,
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
                offset: self.offset,
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
                offset: self.offset,
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
