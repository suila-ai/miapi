#![doc = "`ap/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`ap/` 以下のエンドポイント"]
#[must_use = "namespace builders do nothing; call an endpoint method and `.await` it"]
pub struct Call<'a> {
    client: &'a crate::generated::latest::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`ap/get`"]
    pub fn get(&self) -> get::Call<'a> {
        get::Call::new(self.client)
    }
    #[doc = "`ap/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
}
#[doc = "`ap/get`"]
pub mod get {
    #[doc = "`ap/get`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:federation*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub uri: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "ap/get";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    #[doc = "`ap/get` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:federation*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `uri`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        uri: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                uri: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn uri(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                uri: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                uri: self
                    .uri
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`ap/show`"]
pub mod show {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseUserType {
        #[serde(rename = "User")]
        User,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseUser {
        pub r#type: ResponseUserType,
        pub object: crate::generated::latest::types::UserDetailedNotMe,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum ResponseNoteType {
        #[serde(rename = "Note")]
        Note,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseNote {
        pub r#type: ResponseNoteType,
        pub object: crate::generated::latest::types::Note,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum Response {
        User(ResponseUser),
        Note(ResponseNote),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[doc = "`ap/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub uri: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "ap/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`ap/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `uri`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        uri: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                uri: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn uri(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                uri: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                uri: self
                    .uri
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
