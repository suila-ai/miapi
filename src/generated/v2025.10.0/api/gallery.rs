#![doc = "`gallery/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`gallery/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`gallery/featured`"]
    pub fn featured(&self) -> featured::Call<'a> {
        featured::Call::new(self.client)
    }
    #[doc = "`gallery/popular`"]
    pub fn popular(&self) -> popular::Call<'a> {
        popular::Call::new(self.client)
    }
    #[doc = "`gallery/posts` と、`gallery/posts/` 以下のエンドポイント"]
    pub fn posts(&self) -> posts::Call<'a> {
        posts::Call::new(self.client)
    }
}
#[doc = "`gallery/featured`"]
pub mod featured {
    #[doc = "`gallery/featured`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "gallery/featured";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::GalleryPost>;
    #[doc = "`gallery/featured` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        until_id: ::std::option::Option<::std::string::String>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
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
                until_id: self.until_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`gallery/popular`"]
pub mod popular {
    #[doc = "`gallery/popular`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "gallery/popular";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::GalleryPost>;
    #[doc = "`gallery/popular` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
#[doc = "`gallery/posts` と、`gallery/posts/` 以下のエンドポイント"]
pub mod posts {
    #[doc = "`gallery/posts`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
        pub since_date: ::std::option::Option<i64>,
        #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
        pub until_date: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "gallery/posts";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::GalleryPost>;
    #[doc = "`gallery/posts` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`gallery/posts/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`gallery/posts/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`gallery/posts/like`"]
        pub fn like(&self) -> like::Call<'a> {
            like::Call::new(self.client)
        }
        #[doc = "`gallery/posts/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`gallery/posts/unlike`"]
        pub fn unlike(&self) -> unlike::Call<'a> {
            unlike::Call::new(self.client)
        }
        #[doc = "`gallery/posts/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
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
        pub fn since_date(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_since_date(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sinceDate` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_since_date(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.since_date = value.map(|value| value.into());
            self
        }
        pub fn until_date(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_until_date(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `untilDate` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_until_date(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.until_date = value.map(|value| value.into());
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
                since_date: self.since_date,
                until_date: self.until_date,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
    #[doc = "`gallery/posts/create`"]
    pub mod create {
        #[doc = "`gallery/posts/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "文字数は 1 以上。"]
            pub title: ::std::string::String,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[doc = "要素数は 1 以上 32 以下。要素は重複させない。"]
            #[serde(rename = "fileIds")]
            pub file_ids: ::std::vec::Vec<::std::string::String>,
            #[doc = "省略時は `false`。"]
            #[serde(
                rename = "isSensitive",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_sensitive: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "gallery/posts/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::GalleryPost;
        #[doc = "`gallery/posts/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `title`、`file_ids`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            title: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::option::Option<::std::string::String>>,
            file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            is_sensitive: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    title: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    file_ids: ::std::option::Option::None,
                    is_sensitive: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn description(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_description(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `description` を設定し、`None` なら設定を消して送らない。`null` を送るには [`description_null`](Self::description_null) を使う。"]
            pub fn maybe_description(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.description = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`description` を `null` として送る。"]
            pub fn description_null(mut self) -> Self {
                self.description = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            #[doc = "省略時は `false`。"]
            pub fn is_sensitive(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_is_sensitive(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `isSensitive` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_is_sensitive(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.is_sensitive = value.map(|value| value.into());
                self
            }
            #[doc = "文字数は 1 以上。"]
            pub fn title(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    title: ::std::option::Option::Some(value.into()),
                    description: self.description,
                    file_ids: self.file_ids,
                    is_sensitive: self.is_sensitive,
                    _state: ::std::marker::PhantomData,
                }
            }
            #[doc = "要素数は 1 以上 32 以下。要素は重複させない。"]
            pub fn file_ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    title: self.title,
                    description: self.description,
                    file_ids: ::std::option::Option::Some(value.into()),
                    is_sensitive: self.is_sensitive,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    title: self
                        .title
                        .expect("required parameter is set by the type state"),
                    description: self.description,
                    file_ids: self
                        .file_ids
                        .expect("required parameter is set by the type state"),
                    is_sensitive: self.is_sensitive,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`gallery/posts/delete`"]
    pub mod delete {
        #[doc = "`gallery/posts/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "postId")]
            pub post_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "gallery/posts/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`gallery/posts/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `post_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            post_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    post_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn post_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    post_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    post_id: self
                        .post_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`gallery/posts/like`"]
    pub mod like {
        #[doc = "`gallery/posts/like`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery-likes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "postId")]
            pub post_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "gallery/posts/like";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`gallery/posts/like` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery-likes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `post_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            post_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    post_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn post_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    post_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    post_id: self
                        .post_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`gallery/posts/show`"]
    pub mod show {
        #[doc = "`gallery/posts/show`"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "postId")]
            pub post_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "gallery/posts/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::GalleryPost;
        #[doc = "`gallery/posts/show` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *No*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `post_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            post_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    post_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn post_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    post_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    post_id: self
                        .post_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`gallery/posts/unlike`"]
    pub mod unlike {
        #[doc = "`gallery/posts/unlike`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery-likes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "postId")]
            pub post_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "gallery/posts/unlike";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`gallery/posts/unlike` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery-likes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `post_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            post_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    post_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn post_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    post_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    post_id: self
                        .post_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`gallery/posts/update`"]
    pub mod update {
        #[doc = "`gallery/posts/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "postId")]
            pub post_id: ::std::string::String,
            #[doc = "文字数は 1 以上。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub title: ::std::option::Option<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[doc = "要素数は 1 以上 32 以下。要素は重複させない。"]
            #[serde(rename = "fileIds", default, skip_serializing_if = "Option::is_none")]
            pub file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[doc = "省略時は `false`。"]
            #[serde(
                rename = "isSensitive",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_sensitive: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "gallery/posts/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::GalleryPost;
        #[doc = "`gallery/posts/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:gallery*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `post_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            post_id: ::std::option::Option<::std::string::String>,
            title: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::option::Option<::std::string::String>>,
            file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            is_sensitive: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    post_id: ::std::option::Option::None,
                    title: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    file_ids: ::std::option::Option::None,
                    is_sensitive: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "文字数は 1 以上。"]
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
            pub fn description(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_description(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `description` を設定し、`None` なら設定を消して送らない。`null` を送るには [`description_null`](Self::description_null) を使う。"]
            pub fn maybe_description(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.description = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`description` を `null` として送る。"]
            pub fn description_null(mut self) -> Self {
                self.description = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            #[doc = "要素数は 1 以上 32 以下。要素は重複させない。"]
            pub fn file_ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.maybe_file_ids(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `fileIds` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_file_ids(
                mut self,
                value: ::std::option::Option<
                    impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
                >,
            ) -> Self {
                self.file_ids = value.map(|value| value.into());
                self
            }
            #[doc = "省略時は `false`。"]
            pub fn is_sensitive(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_is_sensitive(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `isSensitive` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_is_sensitive(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.is_sensitive = value.map(|value| value.into());
                self
            }
            pub fn post_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    post_id: ::std::option::Option::Some(value.into()),
                    title: self.title,
                    description: self.description,
                    file_ids: self.file_ids,
                    is_sensitive: self.is_sensitive,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    post_id: self
                        .post_id
                        .expect("required parameter is set by the type state"),
                    title: self.title,
                    description: self.description,
                    file_ids: self.file_ids,
                    is_sensitive: self.is_sensitive,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
