#![doc = "`notifications/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`notifications/` 以下のエンドポイント"]
#[must_use = "namespace builders do nothing; call an endpoint method and `.await` it"]
pub struct Call<'a> {
    client: &'a crate::generated::v2024_10_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`notifications/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`notifications/flush`"]
    pub fn flush(&self) -> flush::Call<'a> {
        flush::Call::new(self.client)
    }
    #[doc = "`notifications/mark-all-as-read`"]
    pub fn mark_all_as_read(&self) -> mark_all_as_read::Call<'a> {
        mark_all_as_read::Call::new(self.client)
    }
    #[doc = "`notifications/test-notification`"]
    pub fn test_notification(&self) -> test_notification::Call<'a> {
        test_notification::Call::new(self.client)
    }
}
#[doc = "`notifications/create`"]
pub mod create {
    #[doc = "`notifications/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub body: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub header: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub icon: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notifications/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`notifications/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `body`。すべて設定すると、呼び出しを await できる。"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        body: ::std::option::Option<::std::string::String>,
        header: ::std::option::Option<::std::option::Option<::std::string::String>>,
        icon: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                body: ::std::option::Option::None,
                header: ::std::option::Option::None,
                icon: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn header(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_header(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `header` を設定し、`None` なら設定を消して送らない。`null` を送るには [`header_null`](Self::header_null) を使う。"]
        pub fn maybe_header(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.header = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`header` を `null` として送る。"]
        pub fn header_null(mut self) -> Self {
            self.header = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn icon(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_icon(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `icon` を設定し、`None` なら設定を消して送らない。`null` を送るには [`icon_null`](Self::icon_null) を使う。"]
        pub fn maybe_icon(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.icon = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`icon` を `null` として送る。"]
        pub fn icon_null(mut self) -> Self {
            self.icon = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn body(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                body: ::std::option::Option::Some(value.into()),
                header: self.header,
                icon: self.icon,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                body: self
                    .body
                    .expect("required parameter is set by the type state"),
                header: self.header,
                icon: self.icon,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notifications/flush`"]
pub mod flush {
    #[doc = "`notifications/flush`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notifications/flush";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`notifications/flush` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notifications/mark-all-as-read`"]
pub mod mark_all_as_read {
    #[doc = "`notifications/mark-all-as-read`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notifications/mark-all-as-read";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`notifications/mark-all-as-read` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notifications/test-notification`"]
pub mod test_notification {
    #[doc = "`notifications/test-notification`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notifications/test-notification";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`notifications/test-notification` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notifications*"]
    #[must_use = "builders do nothing unless you `.await` them"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {}
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {};
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
