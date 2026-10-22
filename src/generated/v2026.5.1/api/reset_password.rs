#![doc = "`reset-password`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`reset-password`"]
#[doc = "Complete the password reset that was previously requested.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    pub token: ::std::string::String,
    pub password: ::std::string::String,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "reset-password";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ();
#[doc = "`reset-password` のビルダー。"]
#[doc = "Complete the password reset that was previously requested.\n\n**Credential required**: *No*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `token`、`password`。すべて設定すると、呼び出しを await できる。"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
    client: &'a crate::generated::v2026_5_1::Client,
    token: ::std::option::Option<::std::string::String>,
    password: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
        Call {
            client,
            token: ::std::option::Option::None,
            password: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0, S1> Call<'a, (S0, S1)> {
    pub fn token(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (crate::builder::Set, S1)> {
        Call {
            client: self.client,
            token: ::std::option::Option::Some(value.into()),
            password: self.password,
            _state: ::std::marker::PhantomData,
        }
    }
    pub fn password(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (S0, crate::builder::Set)> {
        Call {
            client: self.client,
            token: self.token,
            password: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            token: self
                .token
                .expect("required parameter is set by the type state"),
            password: self
                .password
                .expect("required parameter is set by the type state"),
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
