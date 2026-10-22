#![doc = "`announcements` と、`announcements/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`announcements`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
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
    #[doc = "省略時は `true`。"]
    #[serde(rename = "isActive", default, skip_serializing_if = "Option::is_none")]
    pub is_active: ::std::option::Option<bool>,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "announcements";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Announcement>;
#[doc = "`announcements` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
    limit: ::std::option::Option<i64>,
    since_id: ::std::option::Option<::std::string::String>,
    until_id: ::std::option::Option<::std::string::String>,
    since_date: ::std::option::Option<i64>,
    until_date: ::std::option::Option<i64>,
    is_active: ::std::option::Option<bool>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call {
            client,
            limit: ::std::option::Option::None,
            since_id: ::std::option::Option::None,
            until_id: ::std::option::Option::None,
            since_date: ::std::option::Option::None,
            until_date: ::std::option::Option::None,
            is_active: ::std::option::Option::None,
        }
    }
}
impl<'a> Call<'a> {
    #[doc = "`announcements/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
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
    #[doc = "省略時は `true`。"]
    pub fn is_active(self, value: impl ::std::convert::Into<bool>) -> Self {
        self.maybe_is_active(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `isActive` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_is_active(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<bool>>,
    ) -> Self {
        self.is_active = value.map(|value| value.into());
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
            is_active: self.is_active,
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
#[doc = "`announcements/show`"]
pub mod show {
    #[doc = "`announcements/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "announcementId")]
        pub announcement_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "announcements/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::Announcement;
    #[doc = "`announcements/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `announcement_id`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        announcement_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                announcement_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn announcement_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                announcement_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                announcement_id: self
                    .announcement_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
