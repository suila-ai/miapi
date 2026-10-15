#![doc = "`meta`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum Response {
    MetaDetailed(crate::generated::v2024_10_1::types::MetaDetailed),
    MetaLite(crate::generated::v2024_10_1::types::MetaLite),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[doc = "`meta`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub detail: ::std::option::Option<bool>,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "meta";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
#[doc = "`meta` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
pub struct Call<'a> {
    client: &'a crate::generated::v2024_10_1::Client,
    detail: ::std::option::Option<bool>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call {
            client,
            detail: ::std::option::Option::None,
        }
    }
}
impl<'a> Call<'a> {
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
            detail: self.detail,
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
