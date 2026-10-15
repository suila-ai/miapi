#![doc = "`page-push`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`page-push`"]
#[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    #[serde(rename = "pageId")]
    pub page_id: ::std::string::String,
    pub event: ::std::string::String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub var: ::std::option::Option<::serde_json::Value>,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "page-push";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ();
#[doc = "`page-push` のビルダー。"]
#[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `page_id`、`event`。すべて設定すると、呼び出しを await できる。"]
pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
    client: &'a crate::generated::latest::Client,
    page_id: ::std::option::Option<::std::string::String>,
    event: ::std::option::Option<::std::string::String>,
    var: ::std::option::Option<::serde_json::Value>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call {
            client,
            page_id: ::std::option::Option::None,
            event: ::std::option::Option::None,
            var: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0, S1> Call<'a, (S0, S1)> {
    pub fn var(self, value: impl ::std::convert::Into<::serde_json::Value>) -> Self {
        self.maybe_var(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `var` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_var(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<::serde_json::Value>>,
    ) -> Self {
        self.var = value.map(|value| value.into());
        self
    }
    pub fn page_id(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (crate::builder::Set, S1)> {
        Call {
            client: self.client,
            page_id: ::std::option::Option::Some(value.into()),
            event: self.event,
            var: self.var,
            _state: ::std::marker::PhantomData,
        }
    }
    pub fn event(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Call<'a, (S0, crate::builder::Set)> {
        Call {
            client: self.client,
            page_id: self.page_id,
            event: ::std::option::Option::Some(value.into()),
            var: self.var,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            page_id: self
                .page_id
                .expect("required parameter is set by the type state"),
            event: self
                .event
                .expect("required parameter is set by the type state"),
            var: self.var,
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
