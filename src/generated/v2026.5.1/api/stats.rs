#![doc = "`stats`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Response {
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "originalNotesCount")]
    pub original_notes_count: f64,
    #[serde(rename = "usersCount")]
    pub users_count: f64,
    #[serde(rename = "originalUsersCount")]
    pub original_users_count: f64,
    pub instances: f64,
    #[serde(rename = "driveUsageLocal")]
    pub drive_usage_local: f64,
    #[serde(rename = "driveUsageRemote")]
    pub drive_usage_remote: f64,
}
#[doc = "`stats`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {}
impl crate::Endpoint for Request {
    const PATH: &'static str = "stats";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
#[doc = "`stats` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Call<'a> {
    client: &'a crate::generated::v2026_5_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
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
