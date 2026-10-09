#![doc = "`sw/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`sw/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`sw/register`"]
    pub fn register(&self) -> register::Call<'a> {
        register::Call::new(self.client)
    }
    #[doc = "`sw/show-registration`"]
    pub fn show_registration(&self) -> show_registration::Call<'a> {
        show_registration::Call::new(self.client)
    }
    #[doc = "`sw/unregister`"]
    pub fn unregister(&self) -> unregister::Call<'a> {
        unregister::Call::new(self.client)
    }
    #[doc = "`sw/update-registration`"]
    pub fn update_registration(&self) -> update_registration::Call<'a> {
        update_registration::Call::new(self.client)
    }
}
#[doc = "`sw/register`"]
pub mod register {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseState {
        #[serde(rename = "already-subscribed")]
        AlreadySubscribed,
        #[serde(rename = "subscribed")]
        Subscribed,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(default)]
        pub state: ::std::option::Option<ResponseState>,
        pub key: ::std::option::Option<::std::string::String>,
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        pub endpoint: ::std::string::String,
        #[serde(rename = "sendReadMessage")]
        pub send_read_message: bool,
    }
    #[doc = "`sw/register`"]
    #[doc = "Register to receive push notifications.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub endpoint: ::std::string::String,
        pub auth: ::std::string::String,
        pub publickey: ::std::string::String,
        #[serde(
            rename = "sendReadMessage",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub send_read_message: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "sw/register";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`sw/register` のビルダー。"]
    #[doc = "Register to receive push notifications.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `endpoint`、`auth`、`publickey`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<
        'a,
        S = (
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
        ),
    > {
        client: &'a crate::generated::v2025_10_0::Client,
        endpoint: ::std::option::Option<::std::string::String>,
        auth: ::std::option::Option<::std::string::String>,
        publickey: ::std::option::Option<::std::string::String>,
        send_read_message: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                endpoint: ::std::option::Option::None,
                auth: ::std::option::Option::None,
                publickey: ::std::option::Option::None,
                send_read_message: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
        pub fn send_read_message(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.send_read_message = ::std::option::Option::Some(value.into());
            self
        }
        pub fn endpoint(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1, S2)> {
            Call {
                client: self.client,
                endpoint: ::std::option::Option::Some(value.into()),
                auth: self.auth,
                publickey: self.publickey,
                send_read_message: self.send_read_message,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn auth(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set, S2)> {
            Call {
                client: self.client,
                endpoint: self.endpoint,
                auth: ::std::option::Option::Some(value.into()),
                publickey: self.publickey,
                send_read_message: self.send_read_message,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn publickey(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, S1, crate::builder::Set)> {
            Call {
                client: self.client,
                endpoint: self.endpoint,
                auth: self.auth,
                publickey: ::std::option::Option::Some(value.into()),
                send_read_message: self.send_read_message,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture
        for Call<
            'a,
            (
                crate::builder::Set,
                crate::builder::Set,
                crate::builder::Set,
            ),
        >
    {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                endpoint: self
                    .endpoint
                    .expect("required parameter is set by the type state"),
                auth: self
                    .auth
                    .expect("required parameter is set by the type state"),
                publickey: self
                    .publickey
                    .expect("required parameter is set by the type state"),
                send_read_message: self.send_read_message,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`sw/show-registration`"]
pub mod show_registration {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseBody {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        pub endpoint: ::std::string::String,
        #[serde(rename = "sendReadMessage")]
        pub send_read_message: bool,
    }
    #[doc = "`sw/show-registration`"]
    #[doc = "Check push notification registration exists.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub endpoint: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "sw/show-registration";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::option::Option<ResponseBody>;
    #[doc = "`sw/show-registration` のビルダー。"]
    #[doc = "Check push notification registration exists.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `endpoint`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        endpoint: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                endpoint: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn endpoint(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                endpoint: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                endpoint: self
                    .endpoint
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`sw/unregister`"]
pub mod unregister {
    #[doc = "`sw/unregister`"]
    #[doc = "Unregister from receiving push notifications.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub endpoint: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "sw/unregister";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`sw/unregister` のビルダー。"]
    #[doc = "Unregister from receiving push notifications.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `endpoint`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        endpoint: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                endpoint: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn endpoint(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                endpoint: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                endpoint: self
                    .endpoint
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`sw/update-registration`"]
pub mod update_registration {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "userId")]
        pub user_id: ::std::string::String,
        pub endpoint: ::std::string::String,
        #[serde(rename = "sendReadMessage")]
        pub send_read_message: bool,
    }
    #[doc = "`sw/update-registration`"]
    #[doc = "Update push notification registration.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub endpoint: ::std::string::String,
        #[serde(
            rename = "sendReadMessage",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub send_read_message: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "sw/update-registration";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`sw/update-registration` のビルダー。"]
    #[doc = "Update push notification registration.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `endpoint`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        endpoint: ::std::option::Option<::std::string::String>,
        send_read_message: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                endpoint: ::std::option::Option::None,
                send_read_message: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn send_read_message(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.send_read_message = ::std::option::Option::Some(value.into());
            self
        }
        pub fn endpoint(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                endpoint: ::std::option::Option::Some(value.into()),
                send_read_message: self.send_read_message,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                endpoint: self
                    .endpoint
                    .expect("required parameter is set by the type state"),
                send_read_message: self.send_read_message,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
