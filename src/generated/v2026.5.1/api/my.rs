#![doc = "`my/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`my/` 以下のエンドポイント"]
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
    #[doc = "`my/apps`"]
    pub fn apps(&self) -> apps::Call<'a> {
        apps::Call::new(self.client)
    }
}
#[doc = "`my/apps`"]
pub mod apps {
    #[doc = "`my/apps`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
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
        const PATH: &'static str = "my/apps";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2026_5_1::types::App>;
    #[doc = "`my/apps` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2026_5_1::Client,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
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
