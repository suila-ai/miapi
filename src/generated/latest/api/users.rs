#![doc = "`users` と、`users/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
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
#[doc = "`users`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    #[doc = "1 以上 100 以下。省略時は `10`。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: ::std::option::Option<i64>,
    #[doc = "省略時は `0`。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub offset: ::std::option::Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sort: ::std::option::Option<RequestSort>,
    #[doc = "省略時は `\"all\"`。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state: ::std::option::Option<RequestState>,
    #[doc = "省略時は `\"local\"`。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin: ::std::option::Option<RequestOrigin>,
    #[doc = "The local host is represented with `null`."]
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_helpers::double_option"
    )]
    pub hostname: ::std::option::Option<::std::option::Option<::std::string::String>>,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "users";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ::std::vec::Vec<crate::generated::latest::types::UserDetailed>;
#[doc = "`users` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Call<'a> {
    client: &'a crate::generated::latest::Client,
    limit: ::std::option::Option<i64>,
    offset: ::std::option::Option<i64>,
    sort: ::std::option::Option<RequestSort>,
    state: ::std::option::Option<RequestState>,
    origin: ::std::option::Option<RequestOrigin>,
    hostname: ::std::option::Option<::std::option::Option<::std::string::String>>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call {
            client,
            limit: ::std::option::Option::None,
            offset: ::std::option::Option::None,
            sort: ::std::option::Option::None,
            state: ::std::option::Option::None,
            origin: ::std::option::Option::None,
            hostname: ::std::option::Option::None,
        }
    }
}
impl<'a> Call<'a> {
    #[doc = "`users/achievements`"]
    pub fn achievements(&self) -> achievements::Call<'a> {
        achievements::Call::new(self.client)
    }
    #[doc = "`users/clips`"]
    pub fn clips(&self) -> clips::Call<'a> {
        clips::Call::new(self.client)
    }
    #[doc = "`users/featured-notes`"]
    pub fn featured_notes(&self) -> featured_notes::Call<'a> {
        featured_notes::Call::new(self.client)
    }
    #[doc = "`users/flashs`"]
    pub fn flashs(&self) -> flashs::Call<'a> {
        flashs::Call::new(self.client)
    }
    #[doc = "`users/followers`"]
    pub fn followers(&self) -> followers::Call<'a> {
        followers::Call::new(self.client)
    }
    #[doc = "`users/following`"]
    pub fn following(&self) -> following::Call<'a> {
        following::Call::new(self.client)
    }
    #[doc = "`users/gallery/` 以下のエンドポイント"]
    pub fn gallery(&self) -> gallery::Call<'a> {
        gallery::Call::new(self.client)
    }
    #[doc = "`users/get-following-users-by-birthday`"]
    pub fn get_following_users_by_birthday(&self) -> get_following_users_by_birthday::Call<'a> {
        get_following_users_by_birthday::Call::new(self.client)
    }
    #[doc = "`users/get-frequently-replied-users`"]
    pub fn get_frequently_replied_users(&self) -> get_frequently_replied_users::Call<'a> {
        get_frequently_replied_users::Call::new(self.client)
    }
    #[doc = "`users/lists/` 以下のエンドポイント"]
    pub fn lists(&self) -> lists::Call<'a> {
        lists::Call::new(self.client)
    }
    #[doc = "`users/notes`"]
    pub fn notes(&self) -> notes::Call<'a> {
        notes::Call::new(self.client)
    }
    #[doc = "`users/pages`"]
    pub fn pages(&self) -> pages::Call<'a> {
        pages::Call::new(self.client)
    }
    #[doc = "`users/reactions`"]
    pub fn reactions(&self) -> reactions::Call<'a> {
        reactions::Call::new(self.client)
    }
    #[doc = "`users/recommendation`"]
    pub fn recommendation(&self) -> recommendation::Call<'a> {
        recommendation::Call::new(self.client)
    }
    #[doc = "`users/relation`"]
    pub fn relation(&self) -> relation::Call<'a> {
        relation::Call::new(self.client)
    }
    #[doc = "`users/report-abuse`"]
    pub fn report_abuse(&self) -> report_abuse::Call<'a> {
        report_abuse::Call::new(self.client)
    }
    #[doc = "`users/search`"]
    pub fn search(&self) -> search::Call<'a> {
        search::Call::new(self.client)
    }
    #[doc = "`users/search-by-username-and-host`"]
    pub fn search_by_username_and_host(&self) -> search_by_username_and_host::Call<'a> {
        search_by_username_and_host::Call::new(self.client)
    }
    #[doc = "`users/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`users/update-memo`"]
    pub fn update_memo(&self) -> update_memo::Call<'a> {
        update_memo::Call::new(self.client)
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
    pub fn sort(self, value: impl ::std::convert::Into<RequestSort>) -> Self {
        self.maybe_sort(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `sort` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_sort(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<RequestSort>>,
    ) -> Self {
        self.sort = value.map(|value| value.into());
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
    #[doc = "The local host is represented with `null`."]
    pub fn hostname(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.maybe_hostname(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `hostname` を設定し、`None` なら設定を消して送らない。`null` を送るには [`hostname_null`](Self::hostname_null) を使う。"]
    pub fn maybe_hostname(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
    ) -> Self {
        self.hostname = value.map(|value| ::std::option::Option::Some(value.into()));
        self
    }
    #[doc = "`hostname` を `null` として送る。"]
    pub fn hostname_null(mut self) -> Self {
        self.hostname = ::std::option::Option::Some(::std::option::Option::None);
        self
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            limit: self.limit,
            offset: self.offset,
            sort: self.sort,
            state: self.state,
            origin: self.origin,
            hostname: self.hostname,
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
#[doc = "`users/achievements`"]
pub mod achievements {
    #[doc = "`users/achievements`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/achievements";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Achievement>;
    #[doc = "`users/achievements` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
#[doc = "`users/clips`"]
pub mod clips {
    #[doc = "`users/clips`"]
    #[doc = "Show all clips this user owns.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
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
        const PATH: &'static str = "users/clips";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Clip>;
    #[doc = "`users/clips` のビルダー。"]
    #[doc = "Show all clips this user owns.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
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
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
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
#[doc = "`users/featured-notes`"]
pub mod featured_notes {
    #[doc = "`users/featured-notes`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/featured-notes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Note>;
    #[doc = "`users/featured-notes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        limit: ::std::option::Option<i64>,
        until_id: ::std::option::Option<::std::string::String>,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                user_id: ::std::option::Option::None,
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
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                limit: self.limit,
                until_id: self.until_id,
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
                limit: self.limit,
                until_id: self.until_id,
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/flashs`"]
pub mod flashs {
    #[doc = "`users/flashs`"]
    #[doc = "Show all flashs this user created.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
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
        const PATH: &'static str = "users/flashs";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Flash>;
    #[doc = "`users/flashs` のビルダー。"]
    #[doc = "Show all flashs this user created.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
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
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
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
#[doc = "`users/followers`"]
pub mod followers {
    #[doc = "`users/followers`"]
    #[doc = "Show everyone that follows this user.\n\n**Credential required**: *No*"]
    #[doc = "必須: `userId` / `username` と `host` のいずれか。"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
        #[doc = "The local host is represented with `null`."]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub host: ::std::option::Option<::std::option::Option<::std::string::String>>,
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
        const PATH: &'static str = "users/followers";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Following>;
    #[doc = "`users/followers` のビルダー。"]
    #[doc = "Show everyone that follows this user.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        username: ::std::option::Option<::std::string::String>,
        host: ::std::option::Option<::std::option::Option<::std::string::String>>,
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
                user_id: ::std::option::Option::None,
                username: ::std::option::Option::None,
                host: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn user_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_user_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `userId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_user_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.user_id = value.map(|value| value.into());
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
        #[doc = "The local host is represented with `null`."]
        pub fn host(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_host(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `host` を設定し、`None` なら設定を消して送らない。`null` を送るには [`host_null`](Self::host_null) を使う。"]
        pub fn maybe_host(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.host = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`host` を `null` として送る。"]
        pub fn host_null(mut self) -> Self {
            self.host = ::std::option::Option::Some(::std::option::Option::None);
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
                user_id: self.user_id,
                username: self.username,
                host: self.host,
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
#[doc = "`users/following`"]
pub mod following {
    #[doc = "`users/following`"]
    #[doc = "Show everyone that this user is following.\n\n**Credential required**: *No*"]
    #[doc = "必須: `userId` / `username` と `host` のいずれか。"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
        #[doc = "The local host is represented with `null`."]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub host: ::std::option::Option<::std::option::Option<::std::string::String>>,
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
        #[doc = "@deprecated use get-following-users-by-birthday instead."]
        #[doc = "形式は正規表現 `^([0-9]{4})-([0-9]{2})-([0-9]{2})$` に合うもの。"]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub birthday: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/following";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Following>;
    #[doc = "`users/following` のビルダー。"]
    #[doc = "Show everyone that this user is following.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        username: ::std::option::Option<::std::string::String>,
        host: ::std::option::Option<::std::option::Option<::std::string::String>>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
        birthday: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                username: ::std::option::Option::None,
                host: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                birthday: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn user_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_user_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `userId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_user_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.user_id = value.map(|value| value.into());
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
        #[doc = "The local host is represented with `null`."]
        pub fn host(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_host(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `host` を設定し、`None` なら設定を消して送らない。`null` を送るには [`host_null`](Self::host_null) を使う。"]
        pub fn maybe_host(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.host = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`host` を `null` として送る。"]
        pub fn host_null(mut self) -> Self {
            self.host = ::std::option::Option::Some(::std::option::Option::None);
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
        #[doc = "@deprecated use get-following-users-by-birthday instead."]
        #[doc = "形式は正規表現 `^([0-9]{4})-([0-9]{2})-([0-9]{2})$` に合うもの。"]
        pub fn birthday(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_birthday(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `birthday` を設定し、`None` なら設定を消して送らない。`null` を送るには [`birthday_null`](Self::birthday_null) を使う。"]
        pub fn maybe_birthday(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.birthday = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`birthday` を `null` として送る。"]
        pub fn birthday_null(mut self) -> Self {
            self.birthday = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self.user_id,
                username: self.username,
                host: self.host,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
                birthday: self.birthday,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/gallery/` 以下のエンドポイント"]
pub mod gallery {
    #[doc = "`users/gallery/` 以下のエンドポイント"]
    #[must_use = "namespace builders do nothing; call an endpoint method and `.await` it"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`users/gallery/posts`"]
        pub fn posts(&self) -> posts::Call<'a> {
            posts::Call::new(self.client)
        }
    }
    #[doc = "`users/gallery/posts`"]
    pub mod posts {
        #[doc = "`users/gallery/posts`"]
        #[doc = "Show all gallery posts by the given user.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
            #[doc = "1 以上 100 以下。省略時は `10`。"]
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
            const PATH: &'static str = "users/gallery/posts";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::latest::types::GalleryPost>;
        #[doc = "`users/gallery/posts` のビルダー。"]
        #[doc = "Show all gallery posts by the given user.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            user_id: ::std::option::Option<::std::string::String>,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    user_id: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    since_date: ::std::option::Option::None,
                    until_date: ::std::option::Option::None,
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
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    user_id: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
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
}
#[doc = "`users/get-following-users-by-birthday`"]
pub mod get_following_users_by_birthday {
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestBirthdayObject {
        #[doc = "1 以上 12 以下。"]
        pub month: i64,
        #[doc = "1 以上 31 以下。"]
        pub day: i64,
    }
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestBirthdayObject2Begin {
        #[doc = "1 以上 12 以下。"]
        pub month: i64,
        #[doc = "1 以上 31 以下。"]
        pub day: i64,
    }
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestBirthdayObject2End {
        #[doc = "1 以上 12 以下。"]
        pub month: i64,
        #[doc = "1 以上 31 以下。"]
        pub day: i64,
    }
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestBirthdayObject2 {
        pub begin: RequestBirthdayObject2Begin,
        pub end: RequestBirthdayObject2End,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestBirthday {
        Object(RequestBirthdayObject),
        Object2(RequestBirthdayObject2),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        pub birthday: ::std::string::String,
        pub user: crate::generated::latest::types::UserLite,
    }
    #[doc = "`users/get-following-users-by-birthday`"]
    #[doc = "Retrieve users who have a birthday on the specified range.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "省略時は `0`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        pub birthday: RequestBirthday,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/get-following-users-by-birthday";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`users/get-following-users-by-birthday` のビルダー。"]
    #[doc = "Retrieve users who have a birthday on the specified range.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `birthday`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
        birthday: ::std::option::Option<RequestBirthday>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                birthday: ::std::option::Option::None,
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
        pub fn birthday(
            self,
            value: impl ::std::convert::Into<RequestBirthday>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                limit: self.limit,
                offset: self.offset,
                birthday: ::std::option::Option::Some(value.into()),
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
                offset: self.offset,
                birthday: self
                    .birthday
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/get-frequently-replied-users`"]
pub mod get_frequently_replied_users {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub user: crate::generated::latest::types::UserDetailed,
        pub weight: f64,
    }
    #[doc = "`users/get-frequently-replied-users`"]
    #[doc = "Get a list of other users that the specified user frequently replies to.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/get-frequently-replied-users";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`users/get-frequently-replied-users` のビルダー。"]
    #[doc = "Get a list of other users that the specified user frequently replies to.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
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
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
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
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/lists/` 以下のエンドポイント"]
pub mod lists {
    #[doc = "`users/lists/` 以下のエンドポイント"]
    #[must_use = "namespace builders do nothing; call an endpoint method and `.await` it"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`users/lists/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`users/lists/create-from-public`"]
        pub fn create_from_public(&self) -> create_from_public::Call<'a> {
            create_from_public::Call::new(self.client)
        }
        #[doc = "`users/lists/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`users/lists/favorite`"]
        pub fn favorite(&self) -> favorite::Call<'a> {
            favorite::Call::new(self.client)
        }
        #[doc = "`users/lists/get-memberships`"]
        pub fn get_memberships(&self) -> get_memberships::Call<'a> {
            get_memberships::Call::new(self.client)
        }
        #[doc = "`users/lists/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`users/lists/pull`"]
        pub fn pull(&self) -> pull::Call<'a> {
            pull::Call::new(self.client)
        }
        #[doc = "`users/lists/push`"]
        pub fn push(&self) -> push::Call<'a> {
            push::Call::new(self.client)
        }
        #[doc = "`users/lists/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`users/lists/unfavorite`"]
        pub fn unfavorite(&self) -> unfavorite::Call<'a> {
            unfavorite::Call::new(self.client)
        }
        #[doc = "`users/lists/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
        #[doc = "`users/lists/update-membership`"]
        pub fn update_membership(&self) -> update_membership::Call<'a> {
            update_membership::Call::new(self.client)
        }
    }
    #[doc = "`users/lists/create`"]
    pub mod create {
        #[doc = "`users/lists/create`"]
        #[doc = "Create a new list of users.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "文字数は 1 以上 100 以下。"]
            pub name: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::latest::types::UserList;
        #[doc = "`users/lists/create` のビルダー。"]
        #[doc = "Create a new list of users.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            name: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "文字数は 1 以上 100 以下。"]
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
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
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/create-from-public`"]
    pub mod create_from_public {
        #[doc = "`users/lists/create-from-public`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "文字数は 1 以上 100 以下。"]
            pub name: ::std::string::String,
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/create-from-public";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::latest::types::UserList;
        #[doc = "`users/lists/create-from-public` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`list_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::latest::Client,
            name: ::std::option::Option<::std::string::String>,
            list_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    list_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            #[doc = "文字数は 1 以上 100 以下。"]
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    list_id: self.list_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    name: self.name,
                    list_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/delete`"]
    pub mod delete {
        #[doc = "`users/lists/delete`"]
        #[doc = "Delete an existing list of users.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`users/lists/delete` のビルダー。"]
        #[doc = "Delete an existing list of users.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/favorite`"]
    pub mod favorite {
        #[doc = "`users/lists/favorite`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/favorite";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`users/lists/favorite` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/get-memberships`"]
    pub mod get_memberships {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
            pub user: crate::generated::latest::types::UserLite,
            #[serde(rename = "withReplies")]
            pub with_replies: bool,
        }
        #[doc = "`users/lists/get-memberships`"]
        #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
            #[doc = "省略時は `false`。"]
            #[serde(rename = "forPublic", default, skip_serializing_if = "Option::is_none")]
            pub for_public: ::std::option::Option<bool>,
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
            const PATH: &'static str = "users/lists/get-memberships";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`users/lists/get-memberships` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            for_public: ::std::option::Option<bool>,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    for_public: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    since_date: ::std::option::Option::None,
                    until_date: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "省略時は `false`。"]
            pub fn for_public(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_for_public(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `forPublic` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_for_public(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.for_public = value.map(|value| value.into());
                self
            }
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
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    for_public: self.for_public,
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                    for_public: self.for_public,
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
    #[doc = "`users/lists/list`"]
    pub mod list {
        #[doc = "`users/lists/list`"]
        #[doc = "Show all lists that the authenticated user has created.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
            pub user_id: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::latest::types::UserList>;
        #[doc = "`users/lists/list` のビルダー。"]
        #[doc = "Show all lists that the authenticated user has created.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a> {
            client: &'a crate::generated::latest::Client,
            user_id: ::std::option::Option<::std::string::String>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    user_id: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn user_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_user_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `userId` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_user_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.user_id = value.map(|value| value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    user_id: self.user_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/pull`"]
    pub mod pull {
        #[doc = "`users/lists/pull`"]
        #[doc = "Remove a user from a list.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/pull";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`users/lists/pull` のビルダー。"]
        #[doc = "Remove a user from a list.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    user_id: self.user_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    list_id: self.list_id,
                    user_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/push`"]
    pub mod push {
        #[doc = "`users/lists/push`"]
        #[doc = "Add a user to an existing list.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/push";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`users/lists/push` のビルダー。"]
        #[doc = "Add a user to an existing list.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    user_id: self.user_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    list_id: self.list_id,
                    user_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/show`"]
    pub mod show {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
            #[serde(rename = "createdAt")]
            pub created_at: ::chrono::DateTime<::chrono::Utc>,
            pub name: ::std::string::String,
            #[serde(rename = "userIds", default)]
            pub user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(rename = "isPublic")]
            pub is_public: bool,
            #[serde(rename = "likedCount", default)]
            pub liked_count: ::std::option::Option<f64>,
            #[serde(rename = "isLiked", default)]
            pub is_liked: ::std::option::Option<bool>,
        }
        #[doc = "`users/lists/show`"]
        #[doc = "Show the properties of a list.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
            #[doc = "省略時は `false`。"]
            #[serde(rename = "forPublic", default, skip_serializing_if = "Option::is_none")]
            pub for_public: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`users/lists/show` のビルダー。"]
        #[doc = "Show the properties of a list.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            for_public: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    for_public: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "省略時は `false`。"]
            pub fn for_public(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_for_public(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `forPublic` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_for_public(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.for_public = value.map(|value| value.into());
                self
            }
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    for_public: self.for_public,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                    for_public: self.for_public,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/unfavorite`"]
    pub mod unfavorite {
        #[doc = "`users/lists/unfavorite`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/unfavorite";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`users/lists/unfavorite` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/update`"]
    pub mod update {
        #[doc = "`users/lists/update`"]
        #[doc = "Update the properties of a list.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
            #[doc = "文字数は 1 以上 100 以下。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(rename = "isPublic", default, skip_serializing_if = "Option::is_none")]
            pub is_public: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "users/lists/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::latest::types::UserList;
        #[doc = "`users/lists/update` のビルダー。"]
        #[doc = "Update the properties of a list.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            name: ::std::option::Option<::std::string::String>,
            is_public: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    is_public: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "文字数は 1 以上 100 以下。"]
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
            pub fn is_public(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_is_public(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `isPublic` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_is_public(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.is_public = value.map(|value| value.into());
                self
            }
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    name: self.name,
                    is_public: self.is_public,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
                    name: self.name,
                    is_public: self.is_public,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`users/lists/update-membership`"]
    pub mod update_membership {
        #[doc = "`users/lists/update-membership`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "listId")]
            pub list_id: ::std::string::String,
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
            const PATH: &'static str = "users/lists/update-membership";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`users/lists/update-membership` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::latest::Client,
            list_id: ::std::option::Option<::std::string::String>,
            user_id: ::std::option::Option<::std::string::String>,
            with_replies: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
                Call {
                    client,
                    list_id: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    with_replies: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
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
            pub fn list_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    list_id: ::std::option::Option::Some(value.into()),
                    user_id: self.user_id,
                    with_replies: self.with_replies,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    list_id: self.list_id,
                    user_id: ::std::option::Option::Some(value.into()),
                    with_replies: self.with_replies,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    list_id: self
                        .list_id
                        .expect("required parameter is set by the type state"),
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
}
#[doc = "`users/notes`"]
pub mod notes {
    #[doc = "`users/notes`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
        #[doc = "省略時は `true`。"]
        #[serde(
            rename = "withRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_renotes: ::std::option::Option<bool>,
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "withChannelNotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_channel_notes: ::std::option::Option<bool>,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
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
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "allowPartial",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_partial: ::std::option::Option<bool>,
        #[doc = "省略時は `false`。"]
        #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
        pub with_files: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/notes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Note>;
    #[doc = "`users/notes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        with_replies: ::std::option::Option<bool>,
        with_renotes: ::std::option::Option<bool>,
        with_channel_notes: ::std::option::Option<bool>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        allow_partial: ::std::option::Option<bool>,
        with_files: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
                with_renotes: ::std::option::Option::None,
                with_channel_notes: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                allow_partial: ::std::option::Option::None,
                with_files: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "省略時は `false`。"]
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
        #[doc = "省略時は `true`。"]
        pub fn with_renotes(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_renotes(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withRenotes` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_renotes(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_renotes = value.map(|value| value.into());
            self
        }
        #[doc = "省略時は `false`。"]
        pub fn with_channel_notes(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_channel_notes(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withChannelNotes` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_channel_notes(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_channel_notes = value.map(|value| value.into());
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
        #[doc = "省略時は `false`。"]
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
        #[doc = "省略時は `false`。"]
        pub fn with_files(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_files(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withFiles` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_files(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_files = value.map(|value| value.into());
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
                with_renotes: self.with_renotes,
                with_channel_notes: self.with_channel_notes,
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                allow_partial: self.allow_partial,
                with_files: self.with_files,
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
                with_renotes: self.with_renotes,
                with_channel_notes: self.with_channel_notes,
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                allow_partial: self.allow_partial,
                with_files: self.with_files,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/pages`"]
pub mod pages {
    #[doc = "`users/pages`"]
    #[doc = "Show all pages this user created.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
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
        const PATH: &'static str = "users/pages";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::Page>;
    #[doc = "`users/pages` のビルダー。"]
    #[doc = "Show all pages this user created.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
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
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
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
#[doc = "`users/reactions`"]
pub mod reactions {
    #[doc = "`users/reactions`"]
    #[doc = "Show all reactions this user made.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
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
        const PATH: &'static str = "users/reactions";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::NoteReactionWithNote>;
    #[doc = "`users/reactions` のビルダー。"]
    #[doc = "Show all reactions this user made.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
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
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
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
#[doc = "`users/recommendation`"]
pub mod recommendation {
    #[doc = "`users/recommendation`"]
    #[doc = "Show users that the authenticated user might be interested to follow.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "省略時は `0`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/recommendation";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::UserDetailed>;
    #[doc = "`users/recommendation` のビルダー。"]
    #[doc = "Show users that the authenticated user might be interested to follow.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
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
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/relation`"]
pub mod relation {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestUserId {
        String(::std::string::String),
        Array(::std::vec::Vec<::std::string::String>),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseObject {
        pub id: ::std::string::String,
        #[serde(rename = "isFollowing")]
        pub is_following: bool,
        #[serde(rename = "hasPendingFollowRequestFromYou")]
        pub has_pending_follow_request_from_you: bool,
        #[serde(rename = "hasPendingFollowRequestToYou")]
        pub has_pending_follow_request_to_you: bool,
        #[serde(rename = "isFollowed")]
        pub is_followed: bool,
        #[serde(rename = "isBlocking")]
        pub is_blocking: bool,
        #[serde(rename = "isBlocked")]
        pub is_blocked: bool,
        #[serde(rename = "isMuted")]
        pub is_muted: bool,
        #[serde(rename = "isRenoteMuted")]
        pub is_renote_muted: bool,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseArrayItem {
        pub id: ::std::string::String,
        #[serde(rename = "isFollowing")]
        pub is_following: bool,
        #[serde(rename = "hasPendingFollowRequestFromYou")]
        pub has_pending_follow_request_from_you: bool,
        #[serde(rename = "hasPendingFollowRequestToYou")]
        pub has_pending_follow_request_to_you: bool,
        #[serde(rename = "isFollowed")]
        pub is_followed: bool,
        #[serde(rename = "isBlocking")]
        pub is_blocking: bool,
        #[serde(rename = "isBlocked")]
        pub is_blocked: bool,
        #[serde(rename = "isMuted")]
        pub is_muted: bool,
        #[serde(rename = "isRenoteMuted")]
        pub is_renote_muted: bool,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum Response {
        Object(ResponseObject),
        Array(::std::vec::Vec<ResponseArrayItem>),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[doc = "`users/relation`"]
    #[doc = "Show the different kinds of relations between the authenticated user and the specified user(s).\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: RequestUserId,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/relation";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`users/relation` のビルダー。"]
    #[doc = "Show the different kinds of relations between the authenticated user and the specified user(s).\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<RequestUserId>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
            value: impl ::std::convert::Into<RequestUserId>,
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
#[doc = "`users/report-abuse`"]
pub mod report_abuse {
    #[doc = "`users/report-abuse`"]
    #[doc = "File a report.\n\n**Credential required**: *Yes* / **Permission**: *write:report-abuse*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "文字数は 1 以上 2048 以下。"]
        pub comment: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/report-abuse";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`users/report-abuse` のビルダー。"]
    #[doc = "File a report.\n\n**Credential required**: *Yes* / **Permission**: *write:report-abuse*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`、`comment`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        comment: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                comment: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                comment: self.comment,
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "文字数は 1 以上 2048 以下。"]
        pub fn comment(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                user_id: self.user_id,
                comment: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
                comment: self
                    .comment
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/search`"]
pub mod search {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestOrigin {
        #[serde(rename = "local")]
        Local,
        #[serde(rename = "remote")]
        Remote,
        #[serde(rename = "combined")]
        Combined,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`users/search`"]
    #[doc = "Search for users.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub query: ::std::string::String,
        #[doc = "省略時は `0`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "省略時は `\"combined\"`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: ::std::option::Option<RequestOrigin>,
        #[doc = "省略時は `true`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub detail: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/search";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::User>;
    #[doc = "`users/search` のビルダー。"]
    #[doc = "Search for users.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `query`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        query: ::std::option::Option<::std::string::String>,
        offset: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
        origin: ::std::option::Option<RequestOrigin>,
        detail: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                query: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                origin: ::std::option::Option::None,
                detail: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        #[doc = "省略時は `\"combined\"`。"]
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
        #[doc = "省略時は `true`。"]
        pub fn detail(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_detail(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `detail` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_detail(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.detail = value.map(|value| value.into());
            self
        }
        pub fn query(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                query: ::std::option::Option::Some(value.into()),
                offset: self.offset,
                limit: self.limit,
                origin: self.origin,
                detail: self.detail,
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
                offset: self.offset,
                limit: self.limit,
                origin: self.origin,
                detail: self.detail,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/search-by-username-and-host`"]
pub mod search_by_username_and_host {
    #[doc = "`users/search-by-username-and-host`"]
    #[doc = "Search for a user by username and/or host.\n\n**Credential required**: *No*"]
    #[doc = "必須: `username` / `host` のいずれか。"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub username: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub host: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "省略時は `true`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub detail: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/search-by-username-and-host";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::User>;
    #[doc = "`users/search-by-username-and-host` のビルダー。"]
    #[doc = "Search for a user by username and/or host.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        username: ::std::option::Option<::std::option::Option<::std::string::String>>,
        host: ::std::option::Option<::std::option::Option<::std::string::String>>,
        limit: ::std::option::Option<i64>,
        detail: ::std::option::Option<bool>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                username: ::std::option::Option::None,
                host: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                detail: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn username(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_username(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `username` を設定し、`None` なら設定を消して送らない。`null` を送るには [`username_null`](Self::username_null) を使う。"]
        pub fn maybe_username(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.username = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`username` を `null` として送る。"]
        pub fn username_null(mut self) -> Self {
            self.username = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn host(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_host(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `host` を設定し、`None` なら設定を消して送らない。`null` を送るには [`host_null`](Self::host_null) を使う。"]
        pub fn maybe_host(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.host = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`host` を `null` として送る。"]
        pub fn host_null(mut self) -> Self {
            self.host = ::std::option::Option::Some(::std::option::Option::None);
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
        #[doc = "省略時は `true`。"]
        pub fn detail(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_detail(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `detail` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_detail(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.detail = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                username: self.username,
                host: self.host,
                limit: self.limit,
                detail: self.detail,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/show`"]
pub mod show {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum Response {
        MeDetailed(crate::generated::latest::types::MeDetailed),
        UserDetailedNotMe(crate::generated::latest::types::UserDetailedNotMe),
        Array(::std::vec::Vec<crate::generated::latest::types::UserDetailed>),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[doc = "`users/show`"]
    #[doc = "Show the properties of a user.\n\n**Credential required**: *No*"]
    #[doc = "必須: `userId` / `userIds` / `username` のいずれか。"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: ::std::option::Option<::std::string::String>,
        #[doc = "要素は重複させない。"]
        #[serde(rename = "userIds", default, skip_serializing_if = "Option::is_none")]
        pub user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: ::std::option::Option<::std::string::String>,
        #[doc = "The local host is represented with `null`."]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub host: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`users/show` のビルダー。"]
    #[doc = "Show the properties of a user.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        username: ::std::option::Option<::std::string::String>,
        host: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                user_ids: ::std::option::Option::None,
                username: ::std::option::Option::None,
                host: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn user_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_user_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `userId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_user_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.user_id = value.map(|value| value.into());
            self
        }
        #[doc = "要素は重複させない。"]
        pub fn user_ids(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.maybe_user_ids(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `userIds` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_user_ids(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.user_ids = value.map(|value| value.into());
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
        #[doc = "The local host is represented with `null`."]
        pub fn host(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_host(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `host` を設定し、`None` なら設定を消して送らない。`null` を送るには [`host_null`](Self::host_null) を使う。"]
        pub fn maybe_host(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.host = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`host` を `null` として送る。"]
        pub fn host_null(mut self) -> Self {
            self.host = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self.user_id,
                user_ids: self.user_ids,
                username: self.username,
                host: self.host,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`users/update-memo`"]
pub mod update_memo {
    #[doc = "`users/update-memo`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        #[doc = "A personal memo for the target user. If null or empty, delete the memo."]
        pub memo: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "users/update-memo";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`users/update-memo` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`、`memo`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::string::String>,
        memo: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                memo: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn user_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                user_id: ::std::option::Option::Some(value.into()),
                memo: self.memo,
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "A personal memo for the target user. If null or empty, delete the memo."]
        pub fn memo(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                user_id: self.user_id,
                memo: ::std::option::Option::Some(::std::option::Option::Some(value.into())),
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "`memo` を `null` として送る。"]
        pub fn memo_null(self) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                user_id: self.user_id,
                memo: ::std::option::Option::Some(::std::option::Option::None),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self
                    .user_id
                    .expect("required parameter is set by the type state"),
                memo: self
                    .memo
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
