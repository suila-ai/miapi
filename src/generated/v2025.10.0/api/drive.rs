#![doc = "`drive` と、`drive/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Response {
    pub capacity: f64,
    pub usage: f64,
}
#[doc = "`drive`"]
#[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {}
impl crate::Endpoint for Request {
    const PATH: &'static str = "drive";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
#[doc = "`drive` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`drive/files` と、`drive/files/` 以下のエンドポイント"]
    pub fn files(&self) -> files::Call<'a> {
        files::Call::new(self.client)
    }
    #[doc = "`drive/folders` と、`drive/folders/` 以下のエンドポイント"]
    pub fn folders(&self) -> folders::Call<'a> {
        folders::Call::new(self.client)
    }
    #[doc = "`drive/stream`"]
    pub fn stream(&self) -> stream::Call<'a> {
        stream::Call::new(self.client)
    }
}
impl<'a> ::std::future::IntoFuture for Call<'a> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {};
        let client = self.client;
        ::std::boxed::Box::pin(async move { client.call(request).await })
    }
}
#[doc = "`drive/files` と、`drive/files/` 以下のエンドポイント"]
pub mod files {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSort {
        #[serde(rename = "+createdAt")]
        PlusCreatedAt,
        #[serde(rename = "-createdAt")]
        MinusCreatedAt,
        #[serde(rename = "+name")]
        PlusName,
        #[serde(rename = "-name")]
        MinusName,
        #[serde(rename = "+size")]
        PlusSize,
        #[serde(rename = "-size")]
        MinusSize,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`drive/files`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
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
        #[serde(
            rename = "folderId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub sort: ::std::option::Option<::std::option::Option<RequestSort>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "drive/files";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::DriveFile>;
    #[doc = "`drive/files` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
        sort: ::std::option::Option<::std::option::Option<RequestSort>>,
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
                folder_id: ::std::option::Option::None,
                r#type: ::std::option::Option::None,
                sort: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`drive/files/attached-chat-messages`"]
        pub fn attached_chat_messages(&self) -> attached_chat_messages::Call<'a> {
            attached_chat_messages::Call::new(self.client)
        }
        #[doc = "`drive/files/attached-notes`"]
        pub fn attached_notes(&self) -> attached_notes::Call<'a> {
            attached_notes::Call::new(self.client)
        }
        #[doc = "`drive/files/check-existence`"]
        pub fn check_existence(&self) -> check_existence::Call<'a> {
            check_existence::Call::new(self.client)
        }
        #[doc = "`drive/files/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`drive/files/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`drive/files/find`"]
        pub fn find(&self) -> find::Call<'a> {
            find::Call::new(self.client)
        }
        #[doc = "`drive/files/find-by-hash`"]
        pub fn find_by_hash(&self) -> find_by_hash::Call<'a> {
            find_by_hash::Call::new(self.client)
        }
        #[doc = "`drive/files/move-bulk`"]
        pub fn move_bulk(&self) -> move_bulk::Call<'a> {
            move_bulk::Call::new(self.client)
        }
        #[doc = "`drive/files/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`drive/files/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
        #[doc = "`drive/files/upload-from-url`"]
        pub fn upload_from_url(&self) -> upload_from_url::Call<'a> {
            upload_from_url::Call::new(self.client)
        }
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
        pub fn folder_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_folder_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `folderId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`folder_id_null`](Self::folder_id_null) を使う。"]
        pub fn maybe_folder_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.folder_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`folderId` を `null` として送る。"]
        pub fn folder_id_null(mut self) -> Self {
            self.folder_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn r#type(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_type(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `type` を設定し、`None` なら設定を消して送らない。`null` を送るには [`type_null`](Self::type_null) を使う。"]
        pub fn maybe_type(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.r#type = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`type` を `null` として送る。"]
        pub fn type_null(mut self) -> Self {
            self.r#type = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn sort(self, value: impl ::std::convert::Into<RequestSort>) -> Self {
            self.maybe_sort(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sort` を設定し、`None` なら設定を消して送らない。`null` を送るには [`sort_null`](Self::sort_null) を使う。"]
        pub fn maybe_sort(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestSort>>,
        ) -> Self {
            self.sort = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`sort` を `null` として送る。"]
        pub fn sort_null(mut self) -> Self {
            self.sort = ::std::option::Option::Some(::std::option::Option::None);
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
                folder_id: self.folder_id,
                r#type: self.r#type,
                sort: self.sort,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
    #[doc = "`drive/files/attached-chat-messages`"]
    pub mod attached_chat_messages {
        #[doc = "`drive/files/attached-chat-messages`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
            pub since_date: ::std::option::Option<i64>,
            #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
            pub until_date: ::std::option::Option<i64>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "fileId")]
            pub file_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/attached-chat-messages";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatMessage>;
        #[doc = "`drive/files/attached-chat-messages` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `file_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            limit: ::std::option::Option<i64>,
            file_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    since_date: ::std::option::Option::None,
                    until_date: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    file_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
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
            pub fn file_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    limit: self.limit,
                    file_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    limit: self.limit,
                    file_id: self
                        .file_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/attached-notes`"]
    pub mod attached_notes {
        #[doc = "`drive/files/attached-notes`"]
        #[doc = "Find the notes to which the given file is attached.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
            pub since_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
            pub until_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
            pub since_date: ::std::option::Option<i64>,
            #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
            pub until_date: ::std::option::Option<i64>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(rename = "fileId")]
            pub file_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/attached-notes";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
        #[doc = "`drive/files/attached-notes` のビルダー。"]
        #[doc = "Find the notes to which the given file is attached.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `file_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            limit: ::std::option::Option<i64>,
            file_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    since_date: ::std::option::Option::None,
                    until_date: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    file_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
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
            pub fn file_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    limit: self.limit,
                    file_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    limit: self.limit,
                    file_id: self
                        .file_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/check-existence`"]
    pub mod check_existence {
        #[doc = "`drive/files/check-existence`"]
        #[doc = "Check if a given file exists.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub md5: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/check-existence";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = bool;
        #[doc = "`drive/files/check-existence` のビルダー。"]
        #[doc = "Check if a given file exists.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `md5`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            md5: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    md5: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn md5(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    md5: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    md5: self
                        .md5
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/create`"]
    pub mod create {
        #[doc = "`drive/files/create`"]
        #[doc = "Upload a new drive file.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(
                rename = "folderId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub name: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub comment: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "isSensitive",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_sensitive: ::std::option::Option<bool>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub force: ::std::option::Option<bool>,
            #[doc = "The file contents."]
            #[serde(skip)]
            pub file: crate::FilePart,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                let fields = match ::serde_json::to_value(&self).map_err(crate::Error::Serialize)? {
                    ::serde_json::Value::Object(fields) => fields,
                    _ => ::serde_json::Map::new(),
                };
                ::std::result::Result::Ok(crate::Body::Multipart {
                    fields,
                    file: ("file", self.file),
                })
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::DriveFile;
        #[doc = "`drive/files/create` のビルダー。"]
        #[doc = "Upload a new drive file.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `file`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            name: ::std::option::Option<::std::option::Option<::std::string::String>>,
            comment: ::std::option::Option<::std::option::Option<::std::string::String>>,
            is_sensitive: ::std::option::Option<bool>,
            force: ::std::option::Option<bool>,
            file: ::std::option::Option<crate::FilePart>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    folder_id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    comment: ::std::option::Option::None,
                    is_sensitive: ::std::option::Option::None,
                    force: ::std::option::Option::None,
                    file: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_folder_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `folderId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`folder_id_null`](Self::folder_id_null) を使う。"]
            pub fn maybe_folder_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.folder_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`folderId` を `null` として送る。"]
            pub fn folder_id_null(mut self) -> Self {
                self.folder_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn name(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_name(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `name` を設定し、`None` なら設定を消して送らない。`null` を送るには [`name_null`](Self::name_null) を使う。"]
            pub fn maybe_name(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.name = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`name` を `null` として送る。"]
            pub fn name_null(mut self) -> Self {
                self.name = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn comment(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_comment(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `comment` を設定し、`None` なら設定を消して送らない。`null` を送るには [`comment_null`](Self::comment_null) を使う。"]
            pub fn maybe_comment(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.comment = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`comment` を `null` として送る。"]
            pub fn comment_null(mut self) -> Self {
                self.comment = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
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
            pub fn force(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_force(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `force` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_force(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.force = value.map(|value| value.into());
                self
            }
            #[doc = "The file contents."]
            pub fn file(
                self,
                value: impl ::std::convert::Into<crate::FilePart>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    folder_id: self.folder_id,
                    name: self.name,
                    comment: self.comment,
                    is_sensitive: self.is_sensitive,
                    force: self.force,
                    file: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    folder_id: self.folder_id,
                    name: self.name,
                    comment: self.comment,
                    is_sensitive: self.is_sensitive,
                    force: self.force,
                    file: self
                        .file
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/delete`"]
    pub mod delete {
        #[doc = "`drive/files/delete`"]
        #[doc = "Delete an existing drive file.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "fileId")]
            pub file_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`drive/files/delete` のビルダー。"]
        #[doc = "Delete an existing drive file.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `file_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            file_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    file_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn file_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    file_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    file_id: self
                        .file_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/find`"]
    pub mod find {
        #[doc = "`drive/files/find`"]
        #[doc = "Search for a drive file by the given parameters.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub name: ::std::string::String,
            #[serde(
                rename = "folderId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/find";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::DriveFile>;
        #[doc = "`drive/files/find` のビルダー。"]
        #[doc = "Search for a drive file by the given parameters.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            name: ::std::option::Option<::std::string::String>,
            folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    folder_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_folder_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `folderId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`folder_id_null`](Self::folder_id_null) を使う。"]
            pub fn maybe_folder_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.folder_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`folderId` を `null` として送る。"]
            pub fn folder_id_null(mut self) -> Self {
                self.folder_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    folder_id: self.folder_id,
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
                    folder_id: self.folder_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/find-by-hash`"]
    pub mod find_by_hash {
        #[doc = "`drive/files/find-by-hash`"]
        #[doc = "Search for a drive file by a hash of the contents.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub md5: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/find-by-hash";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::DriveFile>;
        #[doc = "`drive/files/find-by-hash` のビルダー。"]
        #[doc = "Search for a drive file by a hash of the contents.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `md5`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            md5: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    md5: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn md5(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    md5: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    md5: self
                        .md5
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/move-bulk`"]
    pub mod move_bulk {
        #[doc = "`drive/files/move-bulk`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "fileIds")]
            pub file_ids: ::std::vec::Vec<::std::string::String>,
            #[serde(
                rename = "folderId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/move-bulk";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`drive/files/move-bulk` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `file_ids`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    file_ids: ::std::option::Option::None,
                    folder_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_folder_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `folderId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`folder_id_null`](Self::folder_id_null) を使う。"]
            pub fn maybe_folder_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.folder_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`folderId` を `null` として送る。"]
            pub fn folder_id_null(mut self) -> Self {
                self.folder_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn file_ids(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    file_ids: ::std::option::Option::Some(value.into()),
                    folder_id: self.folder_id,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    file_ids: self
                        .file_ids
                        .expect("required parameter is set by the type state"),
                    folder_id: self.folder_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/show`"]
    pub mod show {
        #[doc = "`drive/files/show`"]
        #[doc = "Show the properties of a drive file.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須: `fileId` / `url` のいずれか。"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
            pub file_id: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub url: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::DriveFile;
        #[doc = "`drive/files/show` のビルダー。"]
        #[doc = "Show the properties of a drive file.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
            file_id: ::std::option::Option<::std::string::String>,
            url: ::std::option::Option<::std::string::String>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    file_id: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn file_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_file_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `fileId` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_file_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.file_id = value.map(|value| value.into());
                self
            }
            pub fn url(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_url(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `url` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_url(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.url = value.map(|value| value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    file_id: self.file_id,
                    url: self.url,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/update`"]
    pub mod update {
        #[doc = "`drive/files/update`"]
        #[doc = "Update the properties of a drive file.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "fileId")]
            pub file_id: ::std::string::String,
            #[serde(
                rename = "folderId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "isSensitive",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_sensitive: ::std::option::Option<bool>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub comment: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::DriveFile;
        #[doc = "`drive/files/update` のビルダー。"]
        #[doc = "Update the properties of a drive file.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `file_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            file_id: ::std::option::Option<::std::string::String>,
            folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            name: ::std::option::Option<::std::string::String>,
            is_sensitive: ::std::option::Option<bool>,
            comment: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    file_id: ::std::option::Option::None,
                    folder_id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    is_sensitive: ::std::option::Option::None,
                    comment: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_folder_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `folderId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`folder_id_null`](Self::folder_id_null) を使う。"]
            pub fn maybe_folder_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.folder_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`folderId` を `null` として送る。"]
            pub fn folder_id_null(mut self) -> Self {
                self.folder_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn name(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_name(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `name` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_name(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.name = value.map(|value| value.into());
                self
            }
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
            pub fn comment(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_comment(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `comment` を設定し、`None` なら設定を消して送らない。`null` を送るには [`comment_null`](Self::comment_null) を使う。"]
            pub fn maybe_comment(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.comment = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`comment` を `null` として送る。"]
            pub fn comment_null(mut self) -> Self {
                self.comment = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn file_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    file_id: ::std::option::Option::Some(value.into()),
                    folder_id: self.folder_id,
                    name: self.name,
                    is_sensitive: self.is_sensitive,
                    comment: self.comment,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    file_id: self
                        .file_id
                        .expect("required parameter is set by the type state"),
                    folder_id: self.folder_id,
                    name: self.name,
                    is_sensitive: self.is_sensitive,
                    comment: self.comment,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/files/upload-from-url`"]
    pub mod upload_from_url {
        #[doc = "`drive/files/upload-from-url`"]
        #[doc = "Request the server to download a new drive file from the specified URL.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub url: ::std::string::String,
            #[serde(
                rename = "folderId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "isSensitive",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_sensitive: ::std::option::Option<bool>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub comment: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub marker: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub force: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/files/upload-from-url";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`drive/files/upload-from-url` のビルダー。"]
        #[doc = "Request the server to download a new drive file from the specified URL.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `url`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            url: ::std::option::Option<::std::string::String>,
            folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            is_sensitive: ::std::option::Option<bool>,
            comment: ::std::option::Option<::std::option::Option<::std::string::String>>,
            marker: ::std::option::Option<::std::option::Option<::std::string::String>>,
            force: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    url: ::std::option::Option::None,
                    folder_id: ::std::option::Option::None,
                    is_sensitive: ::std::option::Option::None,
                    comment: ::std::option::Option::None,
                    marker: ::std::option::Option::None,
                    force: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_folder_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `folderId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`folder_id_null`](Self::folder_id_null) を使う。"]
            pub fn maybe_folder_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.folder_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`folderId` を `null` として送る。"]
            pub fn folder_id_null(mut self) -> Self {
                self.folder_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
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
            pub fn comment(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_comment(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `comment` を設定し、`None` なら設定を消して送らない。`null` を送るには [`comment_null`](Self::comment_null) を使う。"]
            pub fn maybe_comment(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.comment = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`comment` を `null` として送る。"]
            pub fn comment_null(mut self) -> Self {
                self.comment = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn marker(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_marker(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `marker` を設定し、`None` なら設定を消して送らない。`null` を送るには [`marker_null`](Self::marker_null) を使う。"]
            pub fn maybe_marker(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.marker = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`marker` を `null` として送る。"]
            pub fn marker_null(mut self) -> Self {
                self.marker = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn force(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_force(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `force` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_force(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.force = value.map(|value| value.into());
                self
            }
            pub fn url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    url: ::std::option::Option::Some(value.into()),
                    folder_id: self.folder_id,
                    is_sensitive: self.is_sensitive,
                    comment: self.comment,
                    marker: self.marker,
                    force: self.force,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    url: self
                        .url
                        .expect("required parameter is set by the type state"),
                    folder_id: self.folder_id,
                    is_sensitive: self.is_sensitive,
                    comment: self.comment,
                    marker: self.marker,
                    force: self.force,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`drive/folders` と、`drive/folders/` 以下のエンドポイント"]
pub mod folders {
    #[doc = "`drive/folders`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
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
        #[serde(
            rename = "folderId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "drive/folders";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::DriveFolder>;
    #[doc = "`drive/folders` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        folder_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
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
                folder_id: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`drive/folders/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`drive/folders/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`drive/folders/find`"]
        pub fn find(&self) -> find::Call<'a> {
            find::Call::new(self.client)
        }
        #[doc = "`drive/folders/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`drive/folders/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
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
        pub fn folder_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_folder_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `folderId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`folder_id_null`](Self::folder_id_null) を使う。"]
        pub fn maybe_folder_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.folder_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`folderId` を `null` として送る。"]
        pub fn folder_id_null(mut self) -> Self {
            self.folder_id = ::std::option::Option::Some(::std::option::Option::None);
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
                folder_id: self.folder_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
    #[doc = "`drive/folders/create`"]
    pub mod create {
        #[doc = "`drive/folders/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "parentId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub parent_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/folders/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::DriveFolder;
        #[doc = "`drive/folders/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
            name: ::std::option::Option<::std::string::String>,
            parent_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    parent_id: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn name(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_name(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `name` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_name(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.name = value.map(|value| value.into());
                self
            }
            pub fn parent_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_parent_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `parentId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`parent_id_null`](Self::parent_id_null) を使う。"]
            pub fn maybe_parent_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.parent_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`parentId` を `null` として送る。"]
            pub fn parent_id_null(mut self) -> Self {
                self.parent_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    name: self.name,
                    parent_id: self.parent_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/folders/delete`"]
    pub mod delete {
        #[doc = "`drive/folders/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "folderId")]
            pub folder_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/folders/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`drive/folders/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `folder_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            folder_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    folder_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    folder_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    folder_id: self
                        .folder_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/folders/find`"]
    pub mod find {
        #[doc = "`drive/folders/find`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub name: ::std::string::String,
            #[serde(
                rename = "parentId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub parent_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/folders/find";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::DriveFolder>;
        #[doc = "`drive/folders/find` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            name: ::std::option::Option<::std::string::String>,
            parent_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    parent_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn parent_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_parent_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `parentId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`parent_id_null`](Self::parent_id_null) を使う。"]
            pub fn maybe_parent_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.parent_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`parentId` を `null` として送る。"]
            pub fn parent_id_null(mut self) -> Self {
                self.parent_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    parent_id: self.parent_id,
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
                    parent_id: self.parent_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/folders/show`"]
    pub mod show {
        #[doc = "`drive/folders/show`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "folderId")]
            pub folder_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/folders/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::DriveFolder;
        #[doc = "`drive/folders/show` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `folder_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            folder_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    folder_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    folder_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    folder_id: self
                        .folder_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`drive/folders/update`"]
    pub mod update {
        #[doc = "`drive/folders/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "folderId")]
            pub folder_id: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(
                rename = "parentId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub parent_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "drive/folders/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::DriveFolder;
        #[doc = "`drive/folders/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:drive*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `folder_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            folder_id: ::std::option::Option<::std::string::String>,
            name: ::std::option::Option<::std::string::String>,
            parent_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    folder_id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    parent_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn name(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_name(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `name` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_name(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.name = value.map(|value| value.into());
                self
            }
            pub fn parent_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_parent_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `parentId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`parent_id_null`](Self::parent_id_null) を使う。"]
            pub fn maybe_parent_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.parent_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`parentId` を `null` として送る。"]
            pub fn parent_id_null(mut self) -> Self {
                self.parent_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn folder_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    folder_id: ::std::option::Option::Some(value.into()),
                    name: self.name,
                    parent_id: self.parent_id,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    folder_id: self
                        .folder_id
                        .expect("required parameter is set by the type state"),
                    name: self.name,
                    parent_id: self.parent_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`drive/stream`"]
pub mod stream {
    #[doc = "`drive/stream`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub r#type: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "drive/stream";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::DriveFile>;
    #[doc = "`drive/stream` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:drive*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        r#type: ::std::option::Option<::std::string::String>,
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
                r#type: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
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
        pub fn r#type(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_type(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `type` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_type(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.r#type = value.map(|value| value.into());
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
                r#type: self.r#type,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
