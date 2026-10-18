#![doc = "`test`"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Response {
    #[serde(default)]
    pub id: ::std::option::Option<::std::string::String>,
    pub required: bool,
    #[serde(default)]
    pub string: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub default: ::std::option::Option<::std::string::String>,
    #[serde(rename = "nullableDefault", default)]
    pub nullable_default: ::std::option::Option<::std::string::String>,
}
#[doc = "`test`"]
#[doc = "Endpoint for testing input validation.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    pub required: bool,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub string: ::std::option::Option<::std::string::String>,
    #[doc = "省略時は `\"hello\"`。"]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default: ::std::option::Option<::std::string::String>,
    #[doc = "省略時は `\"hello\"`。"]
    #[serde(
        rename = "nullableDefault",
        default,
        skip_serializing_if = "Option::is_none",
        with = "crate::serde_helpers::double_option"
    )]
    pub nullable_default: ::std::option::Option<::std::option::Option<::std::string::String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: ::std::option::Option<::std::string::String>,
}
impl crate::Endpoint for Request {
    const PATH: &'static str = "test";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
#[doc = "`test` のビルダー。"]
#[doc = "Endpoint for testing input validation.\n\n**Credential required**: *No*"]
#[doc = "必須パラメータ（型状態のスロットの順）: `required`。すべて設定すると、呼び出しを await できる。"]
pub struct Call<'a, S = (crate::builder::Unset,)> {
    client: &'a crate::generated::latest::Client,
    required: ::std::option::Option<bool>,
    string: ::std::option::Option<::std::string::String>,
    default: ::std::option::Option<::std::string::String>,
    nullable_default: ::std::option::Option<::std::option::Option<::std::string::String>>,
    id: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call {
            client,
            required: ::std::option::Option::None,
            string: ::std::option::Option::None,
            default: ::std::option::Option::None,
            nullable_default: ::std::option::Option::None,
            id: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Call<'a, (S0,)> {
    pub fn string(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.maybe_string(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `string` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_string(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
    ) -> Self {
        self.string = value.map(|value| value.into());
        self
    }
    #[doc = "省略時は `\"hello\"`。"]
    pub fn default(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.maybe_default(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `default` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_default(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
    ) -> Self {
        self.default = value.map(|value| value.into());
        self
    }
    #[doc = "省略時は `\"hello\"`。"]
    pub fn nullable_default(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.maybe_nullable_default(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `nullableDefault` を設定し、`None` なら設定を消して送らない。`null` を送るには [`nullable_default_null`](Self::nullable_default_null) を使う。"]
    pub fn maybe_nullable_default(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
    ) -> Self {
        self.nullable_default = value.map(|value| ::std::option::Option::Some(value.into()));
        self
    }
    #[doc = "`nullableDefault` を `null` として送る。"]
    pub fn nullable_default_null(mut self) -> Self {
        self.nullable_default = ::std::option::Option::Some(::std::option::Option::None);
        self
    }
    pub fn id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
        self.maybe_id(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `id` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_id(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
    ) -> Self {
        self.id = value.map(|value| value.into());
        self
    }
    pub fn required(
        self,
        value: impl ::std::convert::Into<bool>,
    ) -> Call<'a, (crate::builder::Set,)> {
        Call {
            client: self.client,
            required: ::std::option::Option::Some(value.into()),
            string: self.string,
            default: self.default,
            nullable_default: self.nullable_default,
            id: self.id,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            required: self
                .required
                .expect("required parameter is set by the type state"),
            string: self.string,
            default: self.default,
            nullable_default: self.nullable_default,
            id: self.id,
        };
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
