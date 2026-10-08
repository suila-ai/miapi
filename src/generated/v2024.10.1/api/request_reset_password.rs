#![doc = "`request-reset-password`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`request-reset-password`"]
#[doc = "Request a users password to be reset.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    pub username: ::std::string::String,
    pub email: ::std::string::String,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "request-reset-password";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ();
#[doc = "`request-reset-password` のビルダー。"]
#[doc = "Request a users password to be reset.\n\n**Credential required**: *No*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `username`、`email`。すべて設定すると、呼び出しを await できる。"]
pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
    client: &'a crate::generated::v2024_10_1::Client,
    username: ::std::option::Option<::std::string::String>,
    email: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call {
            client,
            username: ::std::option::Option::None,
            email: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0, S1> Call<'a, (S0, S1)> {
    pub fn username(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (crate::builder::Set, S1)> {
        Call {
            client: self.client,
            username: ::std::option::Option::Some(value.into()),
            email: self.email,
            _state: ::std::marker::PhantomData,
        }
    }
    pub fn email(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (S0, crate::builder::Set)> {
        Call {
            client: self.client,
            username: self.username,
            email: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            username: self
                .username
                .expect("required parameter is set by the type state"),
            email: self
                .email
                .expect("required parameter is set by the type state"),
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
