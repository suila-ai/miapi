#![doc = "`retention`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ResponseItem {
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub users: f64,
    pub data: ::std::collections::HashMap<::std::string::String, f64>,
}
#[doc = "`retention`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {}
impl crate::Endpoint for Request {
    const PATH: &'static str = "retention";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ::std::vec::Vec<ResponseItem>;
#[doc = "`retention` のビルダー。"]
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
