#![doc = "`federation/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`federation/` 以下のエンドポイント"]
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
    #[doc = "`federation/followers`"]
    pub fn followers(&self) -> followers::Call<'a> {
        followers::Call::new(self.client)
    }
    #[doc = "`federation/following`"]
    pub fn following(&self) -> following::Call<'a> {
        following::Call::new(self.client)
    }
    #[doc = "`federation/instances`"]
    pub fn instances(&self) -> instances::Call<'a> {
        instances::Call::new(self.client)
    }
    #[doc = "`federation/show-instance`"]
    pub fn show_instance(&self) -> show_instance::Call<'a> {
        show_instance::Call::new(self.client)
    }
    #[doc = "`federation/update-remote-user`"]
    pub fn update_remote_user(&self) -> update_remote_user::Call<'a> {
        update_remote_user::Call::new(self.client)
    }
    #[doc = "`federation/users`"]
    pub fn users(&self) -> users::Call<'a> {
        users::Call::new(self.client)
    }
    #[doc = "`federation/stats`"]
    pub fn stats(&self) -> stats::Call<'a> {
        stats::Call::new(self.client)
    }
}
#[doc = "`federation/followers`"]
pub mod followers {
    #[doc = "`federation/followers`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub host: ::std::string::String,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "federation/followers";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Following>;
    #[doc = "`federation/followers` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        host: ::std::option::Option<::std::string::String>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                host: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn host(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                host: ::std::option::Option::Some(value.into()),
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
                host: self
                    .host
                    .expect("required parameter is set by the type state"),
                since_id: self.since_id,
                until_id: self.until_id,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`federation/following`"]
pub mod following {
    #[doc = "`federation/following`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub host: ::std::string::String,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "federation/following";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Following>;
    #[doc = "`federation/following` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        host: ::std::option::Option<::std::string::String>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                host: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn host(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                host: ::std::option::Option::Some(value.into()),
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
                host: self
                    .host
                    .expect("required parameter is set by the type state"),
                since_id: self.since_id,
                until_id: self.until_id,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`federation/instances`"]
pub mod instances {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSort {
        #[serde(rename = "+pubSub")]
        PlusPubSub,
        #[serde(rename = "-pubSub")]
        MinusPubSub,
        #[serde(rename = "+notes")]
        PlusNotes,
        #[serde(rename = "-notes")]
        MinusNotes,
        #[serde(rename = "+users")]
        PlusUsers,
        #[serde(rename = "-users")]
        MinusUsers,
        #[serde(rename = "+following")]
        PlusFollowing,
        #[serde(rename = "-following")]
        MinusFollowing,
        #[serde(rename = "+followers")]
        PlusFollowers,
        #[serde(rename = "-followers")]
        MinusFollowers,
        #[serde(rename = "+firstRetrievedAt")]
        PlusFirstRetrievedAt,
        #[serde(rename = "-firstRetrievedAt")]
        MinusFirstRetrievedAt,
        #[serde(rename = "+latestRequestReceivedAt")]
        PlusLatestRequestReceivedAt,
        #[serde(rename = "-latestRequestReceivedAt")]
        MinusLatestRequestReceivedAt,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`federation/instances`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "Omit or use `null` to not filter by host."]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub host: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub blocked: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            rename = "notResponding",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub not_responding: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub suspended: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub silenced: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub federating: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub subscribing: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub publishing: ::std::option::Option<::std::option::Option<bool>>,
        #[doc = "1 以上 100 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "省略時は `0`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub sort: ::std::option::Option<::std::option::Option<RequestSort>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "federation/instances";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::FederationInstance>;
    #[doc = "`federation/instances` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        host: ::std::option::Option<::std::option::Option<::std::string::String>>,
        blocked: ::std::option::Option<::std::option::Option<bool>>,
        not_responding: ::std::option::Option<::std::option::Option<bool>>,
        suspended: ::std::option::Option<::std::option::Option<bool>>,
        silenced: ::std::option::Option<::std::option::Option<bool>>,
        federating: ::std::option::Option<::std::option::Option<bool>>,
        subscribing: ::std::option::Option<::std::option::Option<bool>>,
        publishing: ::std::option::Option<::std::option::Option<bool>>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
        sort: ::std::option::Option<::std::option::Option<RequestSort>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                host: ::std::option::Option::None,
                blocked: ::std::option::Option::None,
                not_responding: ::std::option::Option::None,
                suspended: ::std::option::Option::None,
                silenced: ::std::option::Option::None,
                federating: ::std::option::Option::None,
                subscribing: ::std::option::Option::None,
                publishing: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                sort: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "Omit or use `null` to not filter by host."]
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
        pub fn blocked(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_blocked(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `blocked` を設定し、`None` なら設定を消して送らない。`null` を送るには [`blocked_null`](Self::blocked_null) を使う。"]
        pub fn maybe_blocked(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.blocked = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`blocked` を `null` として送る。"]
        pub fn blocked_null(mut self) -> Self {
            self.blocked = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn not_responding(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_not_responding(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `notResponding` を設定し、`None` なら設定を消して送らない。`null` を送るには [`not_responding_null`](Self::not_responding_null) を使う。"]
        pub fn maybe_not_responding(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.not_responding = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`notResponding` を `null` として送る。"]
        pub fn not_responding_null(mut self) -> Self {
            self.not_responding = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn suspended(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_suspended(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `suspended` を設定し、`None` なら設定を消して送らない。`null` を送るには [`suspended_null`](Self::suspended_null) を使う。"]
        pub fn maybe_suspended(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.suspended = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`suspended` を `null` として送る。"]
        pub fn suspended_null(mut self) -> Self {
            self.suspended = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn silenced(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_silenced(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `silenced` を設定し、`None` なら設定を消して送らない。`null` を送るには [`silenced_null`](Self::silenced_null) を使う。"]
        pub fn maybe_silenced(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.silenced = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`silenced` を `null` として送る。"]
        pub fn silenced_null(mut self) -> Self {
            self.silenced = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn federating(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_federating(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `federating` を設定し、`None` なら設定を消して送らない。`null` を送るには [`federating_null`](Self::federating_null) を使う。"]
        pub fn maybe_federating(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.federating = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`federating` を `null` として送る。"]
        pub fn federating_null(mut self) -> Self {
            self.federating = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn subscribing(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_subscribing(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `subscribing` を設定し、`None` なら設定を消して送らない。`null` を送るには [`subscribing_null`](Self::subscribing_null) を使う。"]
        pub fn maybe_subscribing(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.subscribing = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`subscribing` を `null` として送る。"]
        pub fn subscribing_null(mut self) -> Self {
            self.subscribing = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn publishing(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_publishing(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `publishing` を設定し、`None` なら設定を消して送らない。`null` を送るには [`publishing_null`](Self::publishing_null) を使う。"]
        pub fn maybe_publishing(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.publishing = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`publishing` を `null` として送る。"]
        pub fn publishing_null(mut self) -> Self {
            self.publishing = ::std::option::Option::Some(::std::option::Option::None);
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
        #[doc = "`Some` なら `sort` を設定し、`None` なら設定を消して送らない。`null` を送るには [`sort_null`](Self::sort_null) を使う。"]
        pub fn maybe_sort(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestSort>>,
        ) -> Self {
            self.sort = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`sort` を `null` として送る。"]
        pub fn sort_null(mut self) -> Self {
            self.sort = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                host: self.host,
                blocked: self.blocked,
                not_responding: self.not_responding,
                suspended: self.suspended,
                silenced: self.silenced,
                federating: self.federating,
                subscribing: self.subscribing,
                publishing: self.publishing,
                limit: self.limit,
                offset: self.offset,
                sort: self.sort,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`federation/show-instance`"]
pub mod show_instance {
    #[doc = "`federation/show-instance`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub host: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "federation/show-instance";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response =
        ::std::option::Option<crate::generated::v2024_10_1::types::FederationInstance>;
    #[doc = "`federation/show-instance` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        host: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                host: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn host(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                host: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                host: self
                    .host
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`federation/update-remote-user`"]
pub mod update_remote_user {
    #[doc = "`federation/update-remote-user`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "federation/update-remote-user";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`federation/update-remote-user` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        user_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
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
#[doc = "`federation/users`"]
pub mod users {
    #[doc = "`federation/users`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub host: ::std::string::String,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "federation/users";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::UserDetailedNotMe>;
    #[doc = "`federation/users` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `host`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        host: ::std::option::Option<::std::string::String>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                host: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn host(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                host: ::std::option::Option::Some(value.into()),
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
                host: self
                    .host
                    .expect("required parameter is set by the type state"),
                since_id: self.since_id,
                until_id: self.until_id,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`federation/stats`"]
pub mod stats {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "topSubInstances")]
        pub top_sub_instances:
            ::std::vec::Vec<crate::generated::v2024_10_1::types::FederationInstance>,
        #[serde(rename = "otherFollowersCount")]
        pub other_followers_count: f64,
        #[serde(rename = "topPubInstances")]
        pub top_pub_instances:
            ::std::vec::Vec<crate::generated::v2024_10_1::types::FederationInstance>,
        #[serde(rename = "otherFollowingCount")]
        pub other_following_count: f64,
    }
    #[doc = "`federation/stats`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "federation/stats";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`federation/stats` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
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
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request { limit: self.limit };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
