#![doc = "`flash/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`flash/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2024_10_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`flash/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`flash/delete`"]
    pub fn delete(&self) -> delete::Call<'a> {
        delete::Call::new(self.client)
    }
    #[doc = "`flash/featured`"]
    pub fn featured(&self) -> featured::Call<'a> {
        featured::Call::new(self.client)
    }
    #[doc = "`flash/like`"]
    pub fn like(&self) -> like::Call<'a> {
        like::Call::new(self.client)
    }
    #[doc = "`flash/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`flash/unlike`"]
    pub fn unlike(&self) -> unlike::Call<'a> {
        unlike::Call::new(self.client)
    }
    #[doc = "`flash/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
    }
    #[doc = "`flash/my`"]
    pub fn my(&self) -> my::Call<'a> {
        my::Call::new(self.client)
    }
    #[doc = "`flash/my-likes`"]
    pub fn my_likes(&self) -> my_likes::Call<'a> {
        my_likes::Call::new(self.client)
    }
}
#[doc = "`flash/create`"]
pub mod create {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestVisibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`flash/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub title: ::std::string::String,
        pub summary: ::std::string::String,
        pub script: ::std::string::String,
        pub permissions: ::std::vec::Vec<::std::string::String>,
        #[doc = "省略時は `\"public\"`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub visibility: ::std::option::Option<RequestVisibility>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Flash;
    #[doc = "`flash/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `title`、`summary`、`script`、`permissions`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<
        'a,
        S = (
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
        ),
    > {
        client: &'a crate::generated::v2024_10_1::Client,
        title: ::std::option::Option<::std::string::String>,
        summary: ::std::option::Option<::std::string::String>,
        script: ::std::option::Option<::std::string::String>,
        permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        visibility: ::std::option::Option<RequestVisibility>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                title: ::std::option::Option::None,
                summary: ::std::option::Option::None,
                script: ::std::option::Option::None,
                permissions: ::std::option::Option::None,
                visibility: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1, S2, S3> Call<'a, (S0, S1, S2, S3)> {
        #[doc = "省略時は `\"public\"`。"]
        pub fn visibility(self, value: impl ::std::convert::Into<RequestVisibility>) -> Self {
            self.maybe_visibility(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `visibility` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_visibility(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestVisibility>>,
        ) -> Self {
            self.visibility = value.map(|value| value.into());
            self
        }
        pub fn title(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1, S2, S3)> {
            Call {
                client: self.client,
                title: ::std::option::Option::Some(value.into()),
                summary: self.summary,
                script: self.script,
                permissions: self.permissions,
                visibility: self.visibility,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn summary(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set, S2, S3)> {
            Call {
                client: self.client,
                title: self.title,
                summary: ::std::option::Option::Some(value.into()),
                script: self.script,
                permissions: self.permissions,
                visibility: self.visibility,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn script(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, S1, crate::builder::Set, S3)> {
            Call {
                client: self.client,
                title: self.title,
                summary: self.summary,
                script: ::std::option::Option::Some(value.into()),
                permissions: self.permissions,
                visibility: self.visibility,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn permissions(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Call<'a, (S0, S1, S2, crate::builder::Set)> {
            Call {
                client: self.client,
                title: self.title,
                summary: self.summary,
                script: self.script,
                permissions: ::std::option::Option::Some(value.into()),
                visibility: self.visibility,
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
                crate::builder::Set,
            ),
        >
    {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                title: self
                    .title
                    .expect("required parameter is set by the type state"),
                summary: self
                    .summary
                    .expect("required parameter is set by the type state"),
                script: self
                    .script
                    .expect("required parameter is set by the type state"),
                permissions: self
                    .permissions
                    .expect("required parameter is set by the type state"),
                visibility: self.visibility,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/delete`"]
pub mod delete {
    #[doc = "`flash/delete`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "flashId")]
        pub flash_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/delete";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`flash/delete` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `flash_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        flash_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                flash_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn flash_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                flash_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                flash_id: self
                    .flash_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/featured`"]
pub mod featured {
    #[doc = "`flash/featured`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "0 以上。省略時は `0`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/featured";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Flash>;
    #[doc = "`flash/featured` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        offset: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                offset: ::std::option::Option::None,
                limit: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "0 以上。省略時は `0`。"]
        pub fn offset(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_offset(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `offset` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_offset(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.offset = value.map(|value| value.into());
            self
        }
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        pub fn limit(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_limit(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `limit` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_limit(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.limit = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                offset: self.offset,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/like`"]
pub mod like {
    #[doc = "`flash/like`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash-likes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "flashId")]
        pub flash_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/like";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`flash/like` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash-likes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `flash_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        flash_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                flash_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn flash_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                flash_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                flash_id: self
                    .flash_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/show`"]
pub mod show {
    #[doc = "`flash/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "flashId")]
        pub flash_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Flash;
    #[doc = "`flash/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `flash_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        flash_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                flash_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn flash_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                flash_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                flash_id: self
                    .flash_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/unlike`"]
pub mod unlike {
    #[doc = "`flash/unlike`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash-likes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "flashId")]
        pub flash_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/unlike";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`flash/unlike` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash-likes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `flash_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        flash_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                flash_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn flash_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                flash_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                flash_id: self
                    .flash_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/update`"]
pub mod update {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestVisibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "private")]
        Private,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`flash/update`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "flashId")]
        pub flash_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub title: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub summary: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub script: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub visibility: ::std::option::Option<RequestVisibility>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/update";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`flash/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:flash*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `flash_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        flash_id: ::std::option::Option<::std::string::String>,
        title: ::std::option::Option<::std::string::String>,
        summary: ::std::option::Option<::std::string::String>,
        script: ::std::option::Option<::std::string::String>,
        permissions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        visibility: ::std::option::Option<RequestVisibility>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                flash_id: ::std::option::Option::None,
                title: ::std::option::Option::None,
                summary: ::std::option::Option::None,
                script: ::std::option::Option::None,
                permissions: ::std::option::Option::None,
                visibility: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn title(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_title(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `title` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_title(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.title = value.map(|value| value.into());
            self
        }
        pub fn summary(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_summary(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `summary` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_summary(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.summary = value.map(|value| value.into());
            self
        }
        pub fn script(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_script(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `script` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_script(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.script = value.map(|value| value.into());
            self
        }
        pub fn permissions(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.maybe_permissions(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `permissions` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_permissions(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.permissions = value.map(|value| value.into());
            self
        }
        pub fn visibility(self, value: impl ::std::convert::Into<RequestVisibility>) -> Self {
            self.maybe_visibility(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `visibility` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_visibility(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestVisibility>>,
        ) -> Self {
            self.visibility = value.map(|value| value.into());
            self
        }
        pub fn flash_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                flash_id: ::std::option::Option::Some(value.into()),
                title: self.title,
                summary: self.summary,
                script: self.script,
                permissions: self.permissions,
                visibility: self.visibility,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                flash_id: self
                    .flash_id
                    .expect("required parameter is set by the type state"),
                title: self.title,
                summary: self.summary,
                script: self.script,
                permissions: self.permissions,
                visibility: self.visibility,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/my`"]
pub mod my {
    #[doc = "`flash/my`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:flash*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/my";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Flash>;
    #[doc = "`flash/my` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:flash*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        pub fn limit(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_limit(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `limit` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_limit(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.limit = value.map(|value| value.into());
            self
        }
        pub fn since_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_since_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sinceId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_since_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.since_id = value.map(|value| value.into());
            self
        }
        pub fn until_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_until_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `untilId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_until_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.until_id = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`flash/my-likes`"]
pub mod my_likes {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        pub flash: crate::generated::v2024_10_1::types::Flash,
    }
    #[doc = "`flash/my-likes`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:flash-likes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "flash/my-likes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`flash/my-likes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:flash-likes*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2024_10_1::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        pub fn limit(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_limit(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `limit` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_limit(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.limit = value.map(|value| value.into());
            self
        }
        pub fn since_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_since_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sinceId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_since_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.since_id = value.map(|value| value.into());
            self
        }
        pub fn until_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_until_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `untilId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_until_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.until_id = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
