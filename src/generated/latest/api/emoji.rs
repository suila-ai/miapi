#![doc = "`emoji`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`emoji`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    pub name: ::std::string::String,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "emoji";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = crate::generated::latest::types::EmojiDetailed;
#[doc = "`emoji` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
pub struct Call<'a, S = (crate::builder::Unset,)> {
    client: &'a crate::generated::latest::Client,
    name: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call {
            client,
            name: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Call<'a, (S0,)> {
    pub fn name(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (crate::builder::Set,)> {
        Call {
            client: self.client,
            name: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            name: self
                .name
                .expect("required parameter is set by the type state"),
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
