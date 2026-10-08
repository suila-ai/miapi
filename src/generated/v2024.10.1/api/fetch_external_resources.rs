#![doc = "`fetch-external-resources`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Response {
    pub r#type: ::std::string::String,
    pub data: ::std::string::String,
}
#[doc = "`fetch-external-resources`"]
#[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    pub url: ::std::string::String,
    pub hash: ::std::string::String,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "fetch-external-resources";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
#[doc = "`fetch-external-resources` のビルダー。"]
#[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `url`、`hash`。すべて設定すると、呼び出しを await できる。"]
pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
    client: &'a crate::generated::v2024_10_1::Client,
    url: ::std::option::Option<::std::string::String>,
    hash: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call {
            client,
            url: ::std::option::Option::None,
            hash: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0, S1> Call<'a, (S0, S1)> {
    pub fn url(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (crate::builder::Set, S1)> {
        Call {
            client: self.client,
            url: ::std::option::Option::Some(value.into()),
            hash: self.hash,
            _state: ::std::marker::PhantomData,
        }
    }
    pub fn hash(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (S0, crate::builder::Set)> {
        Call {
            client: self.client,
            url: self.url,
            hash: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            url: self
                .url
                .expect("required parameter is set by the type state"),
            hash: self
                .hash
                .expect("required parameter is set by the type state"),
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
