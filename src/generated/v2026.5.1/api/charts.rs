#![doc = "`charts/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`charts/` 以下のエンドポイント"]
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
    #[doc = "`charts/active-users`"]
    pub fn active_users(&self) -> active_users::Call<'a> {
        active_users::Call::new(self.client)
    }
    #[doc = "`charts/ap-request`"]
    pub fn ap_request(&self) -> ap_request::Call<'a> {
        ap_request::Call::new(self.client)
    }
    #[doc = "`charts/drive`"]
    pub fn drive(&self) -> drive::Call<'a> {
        drive::Call::new(self.client)
    }
    #[doc = "`charts/federation`"]
    pub fn federation(&self) -> federation::Call<'a> {
        federation::Call::new(self.client)
    }
    #[doc = "`charts/instance`"]
    pub fn instance(&self) -> instance::Call<'a> {
        instance::Call::new(self.client)
    }
    #[doc = "`charts/notes`"]
    pub fn notes(&self) -> notes::Call<'a> {
        notes::Call::new(self.client)
    }
    #[doc = "`charts/user/` 以下のエンドポイント"]
    pub fn user(&self) -> user::Call<'a> {
        user::Call::new(self.client)
    }
    #[doc = "`charts/users`"]
    pub fn users(&self) -> users::Call<'a> {
        users::Call::new(self.client)
    }
}
#[doc = "`charts/active-users`"]
pub mod active_users {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSpan {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "hour")]
        Hour,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "readWrite")]
        pub read_write: ::std::vec::Vec<f64>,
        pub read: ::std::vec::Vec<f64>,
        pub write: ::std::vec::Vec<f64>,
        #[serde(rename = "registeredWithinWeek")]
        pub registered_within_week: ::std::vec::Vec<f64>,
        #[serde(rename = "registeredWithinMonth")]
        pub registered_within_month: ::std::vec::Vec<f64>,
        #[serde(rename = "registeredWithinYear")]
        pub registered_within_year: ::std::vec::Vec<f64>,
        #[serde(rename = "registeredOutsideWeek")]
        pub registered_outside_week: ::std::vec::Vec<f64>,
        #[serde(rename = "registeredOutsideMonth")]
        pub registered_outside_month: ::std::vec::Vec<f64>,
        #[serde(rename = "registeredOutsideYear")]
        pub registered_outside_year: ::std::vec::Vec<f64>,
    }
    #[doc = "`charts/active-users`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub span: RequestSpan,
        #[doc = "1 以上 500 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset: ::std::option::Option<::std::option::Option<i64>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "charts/active-users";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`charts/active-users` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `span`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        span: ::std::option::Option<RequestSpan>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<::std::option::Option<i64>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                span: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "1 以上 500 以下。省略時は `30`。"]
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
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`offset` を `null` として送る。"]
        pub fn offset_null(mut self) -> Self {
            self.offset = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn span(
            self,
            value: impl ::std::convert::Into<RequestSpan>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                span: ::std::option::Option::Some(value.into()),
                limit: self.limit,
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
                span: self
                    .span
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`charts/ap-request`"]
pub mod ap_request {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSpan {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "hour")]
        Hour,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "deliverFailed")]
        pub deliver_failed: ::std::vec::Vec<f64>,
        #[serde(rename = "deliverSucceeded")]
        pub deliver_succeeded: ::std::vec::Vec<f64>,
        #[serde(rename = "inboxReceived")]
        pub inbox_received: ::std::vec::Vec<f64>,
    }
    #[doc = "`charts/ap-request`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub span: RequestSpan,
        #[doc = "1 以上 500 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset: ::std::option::Option<::std::option::Option<i64>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "charts/ap-request";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`charts/ap-request` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `span`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        span: ::std::option::Option<RequestSpan>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<::std::option::Option<i64>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                span: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "1 以上 500 以下。省略時は `30`。"]
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
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`offset` を `null` として送る。"]
        pub fn offset_null(mut self) -> Self {
            self.offset = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn span(
            self,
            value: impl ::std::convert::Into<RequestSpan>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                span: ::std::option::Option::Some(value.into()),
                limit: self.limit,
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
                span: self
                    .span
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`charts/drive`"]
pub mod drive {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSpan {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "hour")]
        Hour,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseLocal {
        #[serde(rename = "incCount")]
        pub inc_count: ::std::vec::Vec<f64>,
        #[serde(rename = "incSize")]
        pub inc_size: ::std::vec::Vec<f64>,
        #[serde(rename = "decCount")]
        pub dec_count: ::std::vec::Vec<f64>,
        #[serde(rename = "decSize")]
        pub dec_size: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseRemote {
        #[serde(rename = "incCount")]
        pub inc_count: ::std::vec::Vec<f64>,
        #[serde(rename = "incSize")]
        pub inc_size: ::std::vec::Vec<f64>,
        #[serde(rename = "decCount")]
        pub dec_count: ::std::vec::Vec<f64>,
        #[serde(rename = "decSize")]
        pub dec_size: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub local: ResponseLocal,
        pub remote: ResponseRemote,
    }
    #[doc = "`charts/drive`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub span: RequestSpan,
        #[doc = "1 以上 500 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset: ::std::option::Option<::std::option::Option<i64>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "charts/drive";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`charts/drive` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `span`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        span: ::std::option::Option<RequestSpan>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<::std::option::Option<i64>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                span: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "1 以上 500 以下。省略時は `30`。"]
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
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`offset` を `null` として送る。"]
        pub fn offset_null(mut self) -> Self {
            self.offset = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn span(
            self,
            value: impl ::std::convert::Into<RequestSpan>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                span: ::std::option::Option::Some(value.into()),
                limit: self.limit,
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
                span: self
                    .span
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`charts/federation`"]
pub mod federation {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSpan {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "hour")]
        Hour,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "deliveredInstances")]
        pub delivered_instances: ::std::vec::Vec<f64>,
        #[serde(rename = "inboxInstances")]
        pub inbox_instances: ::std::vec::Vec<f64>,
        pub stalled: ::std::vec::Vec<f64>,
        pub sub: ::std::vec::Vec<f64>,
        pub r#pub: ::std::vec::Vec<f64>,
        pub pubsub: ::std::vec::Vec<f64>,
        #[serde(rename = "subActive")]
        pub sub_active: ::std::vec::Vec<f64>,
        #[serde(rename = "pubActive")]
        pub pub_active: ::std::vec::Vec<f64>,
    }
    #[doc = "`charts/federation`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub span: RequestSpan,
        #[doc = "1 以上 500 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset: ::std::option::Option<::std::option::Option<i64>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "charts/federation";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`charts/federation` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `span`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        span: ::std::option::Option<RequestSpan>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<::std::option::Option<i64>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                span: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "1 以上 500 以下。省略時は `30`。"]
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
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`offset` を `null` として送る。"]
        pub fn offset_null(mut self) -> Self {
            self.offset = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn span(
            self,
            value: impl ::std::convert::Into<RequestSpan>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                span: ::std::option::Option::Some(value.into()),
                limit: self.limit,
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
                span: self
                    .span
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`charts/instance`"]
pub mod instance {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSpan {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "hour")]
        Hour,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseRequests {
        pub failed: ::std::vec::Vec<f64>,
        pub succeeded: ::std::vec::Vec<f64>,
        pub received: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotesDiffs {
        pub normal: ::std::vec::Vec<f64>,
        pub reply: ::std::vec::Vec<f64>,
        pub renote: ::std::vec::Vec<f64>,
        #[serde(rename = "withFile")]
        pub with_file: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNotes {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
        pub diffs: ResponseNotesDiffs,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseUsers {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseFollowing {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseFollowers {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseDrive {
        #[serde(rename = "totalFiles")]
        pub total_files: ::std::vec::Vec<f64>,
        #[serde(rename = "incFiles")]
        pub inc_files: ::std::vec::Vec<f64>,
        #[serde(rename = "decFiles")]
        pub dec_files: ::std::vec::Vec<f64>,
        #[serde(rename = "incUsage")]
        pub inc_usage: ::std::vec::Vec<f64>,
        #[serde(rename = "decUsage")]
        pub dec_usage: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub requests: ResponseRequests,
        pub notes: ResponseNotes,
        pub users: ResponseUsers,
        pub following: ResponseFollowing,
        pub followers: ResponseFollowers,
        pub drive: ResponseDrive,
    }
    #[doc = "`charts/instance`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub span: RequestSpan,
        #[doc = "1 以上 500 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset: ::std::option::Option<::std::option::Option<i64>>,
        pub host: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "charts/instance";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`charts/instance` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `span`、`host`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::v2026_5_1::Client,
        span: ::std::option::Option<RequestSpan>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<::std::option::Option<i64>>,
        host: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                span: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                host: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        #[doc = "1 以上 500 以下。省略時は `30`。"]
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
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`offset` を `null` として送る。"]
        pub fn offset_null(mut self) -> Self {
            self.offset = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn span(
            self,
            value: impl ::std::convert::Into<RequestSpan>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                span: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                offset: self.offset,
                host: self.host,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn host(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                span: self.span,
                limit: self.limit,
                offset: self.offset,
                host: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                span: self
                    .span
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                offset: self.offset,
                host: self
                    .host
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`charts/notes`"]
pub mod notes {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSpan {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "hour")]
        Hour,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseLocalDiffs {
        pub normal: ::std::vec::Vec<f64>,
        pub reply: ::std::vec::Vec<f64>,
        pub renote: ::std::vec::Vec<f64>,
        #[serde(rename = "withFile")]
        pub with_file: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseLocal {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
        pub diffs: ResponseLocalDiffs,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseRemoteDiffs {
        pub normal: ::std::vec::Vec<f64>,
        pub reply: ::std::vec::Vec<f64>,
        pub renote: ::std::vec::Vec<f64>,
        #[serde(rename = "withFile")]
        pub with_file: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseRemote {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
        pub diffs: ResponseRemoteDiffs,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub local: ResponseLocal,
        pub remote: ResponseRemote,
    }
    #[doc = "`charts/notes`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub span: RequestSpan,
        #[doc = "1 以上 500 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset: ::std::option::Option<::std::option::Option<i64>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "charts/notes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`charts/notes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `span`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        span: ::std::option::Option<RequestSpan>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<::std::option::Option<i64>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                span: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "1 以上 500 以下。省略時は `30`。"]
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
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`offset` を `null` として送る。"]
        pub fn offset_null(mut self) -> Self {
            self.offset = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn span(
            self,
            value: impl ::std::convert::Into<RequestSpan>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                span: ::std::option::Option::Some(value.into()),
                limit: self.limit,
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
                span: self
                    .span
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`charts/user/` 以下のエンドポイント"]
pub mod user {
    #[doc = "`charts/user/` 以下のエンドポイント"]
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
        #[doc = "`charts/user/drive`"]
        pub fn drive(&self) -> drive::Call<'a> {
            drive::Call::new(self.client)
        }
        #[doc = "`charts/user/following`"]
        pub fn following(&self) -> following::Call<'a> {
            following::Call::new(self.client)
        }
        #[doc = "`charts/user/notes`"]
        pub fn notes(&self) -> notes::Call<'a> {
            notes::Call::new(self.client)
        }
        #[doc = "`charts/user/pv`"]
        pub fn pv(&self) -> pv::Call<'a> {
            pv::Call::new(self.client)
        }
        #[doc = "`charts/user/reactions`"]
        pub fn reactions(&self) -> reactions::Call<'a> {
            reactions::Call::new(self.client)
        }
    }
    #[doc = "`charts/user/drive`"]
    pub mod drive {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestSpan {
            #[serde(rename = "day")]
            Day,
            #[serde(rename = "hour")]
            Hour,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            #[serde(rename = "totalCount")]
            pub total_count: ::std::vec::Vec<f64>,
            #[serde(rename = "totalSize")]
            pub total_size: ::std::vec::Vec<f64>,
            #[serde(rename = "incCount")]
            pub inc_count: ::std::vec::Vec<f64>,
            #[serde(rename = "incSize")]
            pub inc_size: ::std::vec::Vec<f64>,
            #[serde(rename = "decCount")]
            pub dec_count: ::std::vec::Vec<f64>,
            #[serde(rename = "decSize")]
            pub dec_size: ::std::vec::Vec<f64>,
        }
        #[doc = "`charts/user/drive`"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub span: RequestSpan,
            #[doc = "1 以上 500 以下。省略時は `30`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub offset: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "charts/user/drive";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`charts/user/drive` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `span`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2026_5_1::Client,
            span: ::std::option::Option<RequestSpan>,
            limit: ::std::option::Option<i64>,
            offset: ::std::option::Option<::std::option::Option<i64>>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
                Call {
                    client,
                    span: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            #[doc = "1 以上 500 以下。省略時は `30`。"]
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
            pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
                self.maybe_offset(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
            pub fn maybe_offset(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<i64>>,
            ) -> Self {
                self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`offset` を `null` として送る。"]
            pub fn offset_null(mut self) -> Self {
                self.offset = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn span(
                self,
                value: impl ::std::convert::Into<RequestSpan>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    span: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self.span,
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self
                        .span
                        .expect("required parameter is set by the type state"),
                    limit: self.limit,
                    offset: self.offset,
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`charts/user/following`"]
    pub mod following {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestSpan {
            #[serde(rename = "day")]
            Day,
            #[serde(rename = "hour")]
            Hour,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseLocalFollowings {
            pub total: ::std::vec::Vec<f64>,
            pub inc: ::std::vec::Vec<f64>,
            pub dec: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseLocalFollowers {
            pub total: ::std::vec::Vec<f64>,
            pub inc: ::std::vec::Vec<f64>,
            pub dec: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseLocal {
            pub followings: ResponseLocalFollowings,
            pub followers: ResponseLocalFollowers,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseRemoteFollowings {
            pub total: ::std::vec::Vec<f64>,
            pub inc: ::std::vec::Vec<f64>,
            pub dec: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseRemoteFollowers {
            pub total: ::std::vec::Vec<f64>,
            pub inc: ::std::vec::Vec<f64>,
            pub dec: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseRemote {
            pub followings: ResponseRemoteFollowings,
            pub followers: ResponseRemoteFollowers,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub local: ResponseLocal,
            pub remote: ResponseRemote,
        }
        #[doc = "`charts/user/following`"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub span: RequestSpan,
            #[doc = "1 以上 500 以下。省略時は `30`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub offset: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "charts/user/following";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`charts/user/following` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `span`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2026_5_1::Client,
            span: ::std::option::Option<RequestSpan>,
            limit: ::std::option::Option<i64>,
            offset: ::std::option::Option<::std::option::Option<i64>>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
                Call {
                    client,
                    span: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            #[doc = "1 以上 500 以下。省略時は `30`。"]
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
            pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
                self.maybe_offset(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
            pub fn maybe_offset(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<i64>>,
            ) -> Self {
                self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`offset` を `null` として送る。"]
            pub fn offset_null(mut self) -> Self {
                self.offset = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn span(
                self,
                value: impl ::std::convert::Into<RequestSpan>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    span: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self.span,
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self
                        .span
                        .expect("required parameter is set by the type state"),
                    limit: self.limit,
                    offset: self.offset,
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`charts/user/notes`"]
    pub mod notes {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestSpan {
            #[serde(rename = "day")]
            Day,
            #[serde(rename = "hour")]
            Hour,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseDiffs {
            pub normal: ::std::vec::Vec<f64>,
            pub reply: ::std::vec::Vec<f64>,
            pub renote: ::std::vec::Vec<f64>,
            #[serde(rename = "withFile")]
            pub with_file: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub total: ::std::vec::Vec<f64>,
            pub inc: ::std::vec::Vec<f64>,
            pub dec: ::std::vec::Vec<f64>,
            pub diffs: ResponseDiffs,
        }
        #[doc = "`charts/user/notes`"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub span: RequestSpan,
            #[doc = "1 以上 500 以下。省略時は `30`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub offset: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "charts/user/notes";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`charts/user/notes` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `span`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2026_5_1::Client,
            span: ::std::option::Option<RequestSpan>,
            limit: ::std::option::Option<i64>,
            offset: ::std::option::Option<::std::option::Option<i64>>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
                Call {
                    client,
                    span: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            #[doc = "1 以上 500 以下。省略時は `30`。"]
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
            pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
                self.maybe_offset(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
            pub fn maybe_offset(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<i64>>,
            ) -> Self {
                self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`offset` を `null` として送る。"]
            pub fn offset_null(mut self) -> Self {
                self.offset = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn span(
                self,
                value: impl ::std::convert::Into<RequestSpan>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    span: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self.span,
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self
                        .span
                        .expect("required parameter is set by the type state"),
                    limit: self.limit,
                    offset: self.offset,
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`charts/user/pv`"]
    pub mod pv {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestSpan {
            #[serde(rename = "day")]
            Day,
            #[serde(rename = "hour")]
            Hour,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseUpv {
            pub user: ::std::vec::Vec<f64>,
            pub visitor: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponsePv {
            pub user: ::std::vec::Vec<f64>,
            pub visitor: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub upv: ResponseUpv,
            pub pv: ResponsePv,
        }
        #[doc = "`charts/user/pv`"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub span: RequestSpan,
            #[doc = "1 以上 500 以下。省略時は `30`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub offset: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "charts/user/pv";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`charts/user/pv` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `span`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2026_5_1::Client,
            span: ::std::option::Option<RequestSpan>,
            limit: ::std::option::Option<i64>,
            offset: ::std::option::Option<::std::option::Option<i64>>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
                Call {
                    client,
                    span: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            #[doc = "1 以上 500 以下。省略時は `30`。"]
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
            pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
                self.maybe_offset(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
            pub fn maybe_offset(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<i64>>,
            ) -> Self {
                self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`offset` を `null` として送る。"]
            pub fn offset_null(mut self) -> Self {
                self.offset = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn span(
                self,
                value: impl ::std::convert::Into<RequestSpan>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    span: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self.span,
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self
                        .span
                        .expect("required parameter is set by the type state"),
                    limit: self.limit,
                    offset: self.offset,
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`charts/user/reactions`"]
    pub mod reactions {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestSpan {
            #[serde(rename = "day")]
            Day,
            #[serde(rename = "hour")]
            Hour,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseLocal {
            pub count: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseRemote {
            pub count: ::std::vec::Vec<f64>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub local: ResponseLocal,
            pub remote: ResponseRemote,
        }
        #[doc = "`charts/user/reactions`"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub span: RequestSpan,
            #[doc = "1 以上 500 以下。省略時は `30`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub offset: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "charts/user/reactions";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`charts/user/reactions` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `span`、`user_id`。すべて設定すると、呼び出しを await できる。"]
        #[must_use = "builders do nothing unless you `.await` them"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2026_5_1::Client,
            span: ::std::option::Option<RequestSpan>,
            limit: ::std::option::Option<i64>,
            offset: ::std::option::Option<::std::option::Option<i64>>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
                Call {
                    client,
                    span: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            #[doc = "1 以上 500 以下。省略時は `30`。"]
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
            pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
                self.maybe_offset(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
            pub fn maybe_offset(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<i64>>,
            ) -> Self {
                self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`offset` を `null` として送る。"]
            pub fn offset_null(mut self) -> Self {
                self.offset = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn span(
                self,
                value: impl ::std::convert::Into<RequestSpan>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    span: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self.span,
                    limit: self.limit,
                    offset: self.offset,
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
                    span: self
                        .span
                        .expect("required parameter is set by the type state"),
                    limit: self.limit,
                    offset: self.offset,
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`charts/users`"]
pub mod users {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSpan {
        #[serde(rename = "day")]
        Day,
        #[serde(rename = "hour")]
        Hour,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseLocal {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseRemote {
        pub total: ::std::vec::Vec<f64>,
        pub inc: ::std::vec::Vec<f64>,
        pub dec: ::std::vec::Vec<f64>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub local: ResponseLocal,
        pub remote: ResponseRemote,
    }
    #[doc = "`charts/users`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub span: RequestSpan,
        #[doc = "1 以上 500 以下。省略時は `30`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset: ::std::option::Option<::std::option::Option<i64>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "charts/users";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`charts/users` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `span`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2026_5_1::Client,
        span: ::std::option::Option<RequestSpan>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<::std::option::Option<i64>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call {
                client,
                span: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "1 以上 500 以下。省略時は `30`。"]
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
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。`null` を送るには [`offset_null`](Self::offset_null) を使う。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`offset` を `null` として送る。"]
        pub fn offset_null(mut self) -> Self {
            self.offset = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn span(
            self,
            value: impl ::std::convert::Into<RequestSpan>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                span: ::std::option::Option::Some(value.into()),
                limit: self.limit,
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
                span: self
                    .span
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
