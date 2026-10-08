#![doc = "`fetch-rss`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ResponseImage {
    #[serde(default)]
    pub link: ::std::option::Option<::std::string::String>,
    pub url: ::std::string::String,
    #[serde(default)]
    pub title: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ResponsePaginationLinks {
    #[serde(rename = "self", default)]
    pub self_: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub first: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub next: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub last: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub prev: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ResponseItemsItemEnclosure {
    pub url: ::std::string::String,
    #[serde(default)]
    pub length: ::std::option::Option<f64>,
    #[serde(default)]
    pub r#type: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ResponseItemsItem {
    #[serde(default)]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub guid: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub title: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pubDate", default)]
    pub pub_date: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub creator: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub content: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isoDate", default)]
    pub iso_date: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "contentSnippet", default)]
    pub content_snippet: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub enclosure: ::std::option::Option<ResponseItemsItemEnclosure>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ResponseItunesOwner {
    #[serde(default)]
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub email: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ResponseItunes {
    #[serde(default)]
    pub image: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub owner: ::std::option::Option<ResponseItunesOwner>,
    #[serde(default)]
    pub author: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub explicit: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub categories: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default)]
    pub keywords: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Response {
    #[serde(default)]
    pub image: ::std::option::Option<ResponseImage>,
    #[serde(rename = "paginationLinks", default)]
    pub pagination_links: ::std::option::Option<ResponsePaginationLinks>,
    #[serde(default)]
    pub link: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub title: ::std::option::Option<::std::string::String>,
    pub items: ::std::vec::Vec<ResponseItemsItem>,
    #[serde(rename = "feedUrl", default)]
    pub feed_url: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub itunes: ::std::option::Option<ResponseItunes>,
}
#[doc = "`fetch-rss`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    pub url: ::std::string::String,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "fetch-rss";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
#[doc = "`fetch-rss` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `url`。すべて設定すると、呼び出しを await できる。"]
pub struct Call<'a, S = (crate::builder::Unset,)> {
    client: &'a crate::generated::latest::Client,
    url: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call {
            client,
            url: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Call<'a, (S0,)> {
    pub fn url(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (crate::builder::Set,)> {
        Call {
            client: self.client,
            url: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            url: self
                .url
                .expect("required parameter is set by the type state"),
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
