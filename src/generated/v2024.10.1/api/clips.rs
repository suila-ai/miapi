#![doc = "`clips/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`clips/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2024_10_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`clips/add-note`"]
    pub fn add_note(&self) -> add_note::Call<'a> {
        add_note::Call::new(self.client)
    }
    #[doc = "`clips/remove-note`"]
    pub fn remove_note(&self) -> remove_note::Call<'a> {
        remove_note::Call::new(self.client)
    }
    #[doc = "`clips/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`clips/delete`"]
    pub fn delete(&self) -> delete::Call<'a> {
        delete::Call::new(self.client)
    }
    #[doc = "`clips/list`"]
    pub fn list(&self) -> list::Call<'a> {
        list::Call::new(self.client)
    }
    #[doc = "`clips/notes`"]
    pub fn notes(&self) -> notes::Call<'a> {
        notes::Call::new(self.client)
    }
    #[doc = "`clips/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`clips/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
    }
    #[doc = "`clips/favorite`"]
    pub fn favorite(&self) -> favorite::Call<'a> {
        favorite::Call::new(self.client)
    }
    #[doc = "`clips/unfavorite`"]
    pub fn unfavorite(&self) -> unfavorite::Call<'a> {
        unfavorite::Call::new(self.client)
    }
    #[doc = "`clips/my-favorites`"]
    pub fn my_favorites(&self) -> my_favorites::Call<'a> {
        my_favorites::Call::new(self.client)
    }
}
#[doc = "`clips/add-note`"]
pub mod add_note {
    #[doc = "`clips/add-note`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/add-note";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`clips/add-note` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`、`note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        note_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                note_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                note_id: self.note_id,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                clip_id: self.clip_id,
                note_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
                note_id: self
                    .note_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/remove-note`"]
pub mod remove_note {
    #[doc = "`clips/remove-note`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/remove-note";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`clips/remove-note` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`、`note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        note_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                note_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                note_id: self.note_id,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                clip_id: self.clip_id,
                note_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
                note_id: self
                    .note_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/create`"]
pub mod create {
    #[doc = "`clips/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "文字数は 1 以上 100 以下。"]
        pub name: ::std::string::String,
        #[doc = "省略時は `false`。"]
        #[serde(rename = "isPublic", default, skip_serializing_if = "Option::is_none")]
        pub is_public: ::std::option::Option<bool>,
        #[doc = "文字数は 1 以上 2048 以下。"]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Clip;
    #[doc = "`clips/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        name: ::std::option::Option<::std::string::String>,
        is_public: ::std::option::Option<bool>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                name: ::std::option::Option::None,
                is_public: ::std::option::Option::None,
                description: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "省略時は `false`。"]
        pub fn is_public(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_public(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isPublic` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_is_public(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_public = value.map(|value| value.into());
            self
        }
        #[doc = "文字数は 1 以上 2048 以下。"]
        pub fn description(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
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
        #[doc = "文字数は 1 以上 100 以下。"]
        pub fn name(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                name: ::std::option::Option::Some(value.into()),
                is_public: self.is_public,
                description: self.description,
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
                is_public: self.is_public,
                description: self.description,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/delete`"]
pub mod delete {
    #[doc = "`clips/delete`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/delete";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`clips/delete` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/list`"]
pub mod list {
    #[doc = "`clips/list`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/list";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Clip>;
    #[doc = "`clips/list` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
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
#[doc = "`clips/notes`"]
pub mod notes {
    #[doc = "`clips/notes`"]
    #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/notes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Note>;
    #[doc = "`clips/notes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/show`"]
pub mod show {
    #[doc = "`clips/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Clip;
    #[doc = "`clips/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/update`"]
pub mod update {
    #[doc = "`clips/update`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
        #[doc = "文字数は 1 以上 100 以下。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(rename = "isPublic", default, skip_serializing_if = "Option::is_none")]
        pub is_public: ::std::option::Option<bool>,
        #[doc = "文字数は 1 以上 2048 以下。"]
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/update";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2024_10_1::types::Clip;
    #[doc = "`clips/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        name: ::std::option::Option<::std::string::String>,
        is_public: ::std::option::Option<bool>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                name: ::std::option::Option::None,
                is_public: ::std::option::Option::None,
                description: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        #[doc = "文字数は 1 以上 100 以下。"]
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
        pub fn is_public(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_public(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isPublic` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_is_public(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_public = value.map(|value| value.into());
            self
        }
        #[doc = "文字数は 1 以上 2048 以下。"]
        pub fn description(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
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
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                name: self.name,
                is_public: self.is_public,
                description: self.description,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
                name: self.name,
                is_public: self.is_public,
                description: self.description,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/favorite`"]
pub mod favorite {
    #[doc = "`clips/favorite`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:clip-favorite*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/favorite";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`clips/favorite` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:clip-favorite*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/unfavorite`"]
pub mod unfavorite {
    #[doc = "`clips/unfavorite`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:clip-favorite*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "clipId")]
        pub clip_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/unfavorite";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`clips/unfavorite` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:clip-favorite*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `clip_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2024_10_1::Client,
        clip_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2024_10_1::Client) -> Self {
            Call {
                client,
                clip_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn clip_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                clip_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                clip_id: self
                    .clip_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`clips/my-favorites`"]
pub mod my_favorites {
    #[doc = "`clips/my-favorites`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:clip-favorite*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "clips/my-favorites";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2024_10_1::types::Clip>;
    #[doc = "`clips/my-favorites` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:clip-favorite*"]
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
