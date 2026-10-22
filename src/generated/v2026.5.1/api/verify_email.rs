#![doc = "`verify-email`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`verify-email`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    pub code: ::std::string::String,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "verify-email";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ();
#[doc = "`verify-email` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `code`。すべて設定すると、呼び出しを await できる。"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Call<'a, S = (crate::builder::Unset,)> {
    client: &'a crate::generated::v2026_5_1::Client,
    code: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
        Call {
            client,
            code: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Call<'a, (S0,)> {
    pub fn code(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (crate::builder::Set,)> {
        Call {
            client: self.client,
            code: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            code: self
                .code
                .expect("required parameter is set by the type state"),
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
