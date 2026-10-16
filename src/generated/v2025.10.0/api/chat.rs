#![doc = "`chat/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`chat/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`chat/history`"]
    pub fn history(&self) -> history::Call<'a> {
        history::Call::new(self.client)
    }
    #[doc = "`chat/messages/` 以下のエンドポイント"]
    pub fn messages(&self) -> messages::Call<'a> {
        messages::Call::new(self.client)
    }
    #[doc = "`chat/read-all`"]
    pub fn read_all(&self) -> read_all::Call<'a> {
        read_all::Call::new(self.client)
    }
    #[doc = "`chat/rooms/` 以下のエンドポイント"]
    pub fn rooms(&self) -> rooms::Call<'a> {
        rooms::Call::new(self.client)
    }
}
#[doc = "`chat/history`"]
pub mod history {
    #[doc = "`chat/history`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "1 以上 100 以下。省略時は `10`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[doc = "省略時は `false`。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub room: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "chat/history";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatMessage>;
    #[doc = "`chat/history` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        room: ::std::option::Option<bool>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                room: ::std::option::Option::None,
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
        #[doc = "省略時は `false`。"]
        pub fn room(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_room(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `room` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_room(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.room = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                room: self.room,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`chat/messages/` 以下のエンドポイント"]
pub mod messages {
    #[doc = "`chat/messages/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`chat/messages/create-to-room`"]
        pub fn create_to_room(&self) -> create_to_room::Call<'a> {
            create_to_room::Call::new(self.client)
        }
        #[doc = "`chat/messages/create-to-user`"]
        pub fn create_to_user(&self) -> create_to_user::Call<'a> {
            create_to_user::Call::new(self.client)
        }
        #[doc = "`chat/messages/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`chat/messages/react`"]
        pub fn react(&self) -> react::Call<'a> {
            react::Call::new(self.client)
        }
        #[doc = "`chat/messages/room-timeline`"]
        pub fn room_timeline(&self) -> room_timeline::Call<'a> {
            room_timeline::Call::new(self.client)
        }
        #[doc = "`chat/messages/search`"]
        pub fn search(&self) -> search::Call<'a> {
            search::Call::new(self.client)
        }
        #[doc = "`chat/messages/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`chat/messages/unreact`"]
        pub fn unreact(&self) -> unreact::Call<'a> {
            unreact::Call::new(self.client)
        }
        #[doc = "`chat/messages/user-timeline`"]
        pub fn user_timeline(&self) -> user_timeline::Call<'a> {
            user_timeline::Call::new(self.client)
        }
    }
    #[doc = "`chat/messages/create-to-room`"]
    pub mod create_to_room {
        #[doc = "`chat/messages/create-to-room`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "文字数は 2000 以下。"]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
            pub file_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "toRoomId")]
            pub to_room_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/create-to-room";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::ChatMessageLiteForRoom;
        #[doc = "`chat/messages/create-to-room` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `to_room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            file_id: ::std::option::Option<::std::string::String>,
            to_room_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    text: ::std::option::Option::None,
                    file_id: ::std::option::Option::None,
                    to_room_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "文字数は 2000 以下。"]
            pub fn text(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_text(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `text` を設定し、`None` なら設定を消して送らない。`null` を送るには [`text_null`](Self::text_null) を使う。"]
            pub fn maybe_text(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.text = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`text` を `null` として送る。"]
            pub fn text_null(mut self) -> Self {
                self.text = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
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
            pub fn to_room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    text: self.text,
                    file_id: self.file_id,
                    to_room_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    text: self.text,
                    file_id: self.file_id,
                    to_room_id: self
                        .to_room_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/create-to-user`"]
    pub mod create_to_user {
        #[doc = "`chat/messages/create-to-user`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "文字数は 2000 以下。"]
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(rename = "fileId", default, skip_serializing_if = "Option::is_none")]
            pub file_id: ::std::option::Option<::std::string::String>,
            #[serde(rename = "toUserId")]
            pub to_user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/create-to-user";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::ChatMessageLiteFor1on1;
        #[doc = "`chat/messages/create-to-user` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `to_user_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            file_id: ::std::option::Option<::std::string::String>,
            to_user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    text: ::std::option::Option::None,
                    file_id: ::std::option::Option::None,
                    to_user_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "文字数は 2000 以下。"]
            pub fn text(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_text(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `text` を設定し、`None` なら設定を消して送らない。`null` を送るには [`text_null`](Self::text_null) を使う。"]
            pub fn maybe_text(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.text = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`text` を `null` として送る。"]
            pub fn text_null(mut self) -> Self {
                self.text = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
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
            pub fn to_user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    text: self.text,
                    file_id: self.file_id,
                    to_user_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    text: self.text,
                    file_id: self.file_id,
                    to_user_id: self
                        .to_user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/delete`"]
    pub mod delete {
        #[doc = "`chat/messages/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "messageId")]
            pub message_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`chat/messages/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `message_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            message_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    message_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn message_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    message_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    message_id: self
                        .message_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/react`"]
    pub mod react {
        #[doc = "`chat/messages/react`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "messageId")]
            pub message_id: ::std::string::String,
            pub reaction: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/react";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`chat/messages/react` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `message_id`、`reaction`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            message_id: ::std::option::Option<::std::string::String>,
            reaction: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    message_id: ::std::option::Option::None,
                    reaction: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn message_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    message_id: ::std::option::Option::Some(value.into()),
                    reaction: self.reaction,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn reaction(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    message_id: self.message_id,
                    reaction: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    message_id: self
                        .message_id
                        .expect("required parameter is set by the type state"),
                    reaction: self
                        .reaction
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/room-timeline`"]
    pub mod room_timeline {
        #[doc = "`chat/messages/room-timeline`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
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
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/room-timeline";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response =
            ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatMessageLiteForRoom>;
        #[doc = "`chat/messages/room-timeline` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            room_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
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
                    room_id: ::std::option::Option::None,
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
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    room_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/search`"]
    pub mod search {
        #[doc = "`chat/messages/search`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "文字数は 1 以上 256 以下。"]
            pub query: ::std::string::String,
            #[doc = "1 以上 100 以下。省略時は `10`。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(
                rename = "userId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "roomId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub room_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/search";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatMessage>;
        #[doc = "`chat/messages/search` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `query`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            query: ::std::option::Option<::std::string::String>,
            limit: ::std::option::Option<i64>,
            user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            room_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    query: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    user_id: ::std::option::Option::None,
                    room_id: ::std::option::Option::None,
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
            pub fn user_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_user_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `userId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`user_id_null`](Self::user_id_null) を使う。"]
            pub fn maybe_user_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.user_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`userId` を `null` として送る。"]
            pub fn user_id_null(mut self) -> Self {
                self.user_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn room_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_room_id(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `roomId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`room_id_null`](Self::room_id_null) を使う。"]
            pub fn maybe_room_id(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.room_id = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`roomId` を `null` として送る。"]
            pub fn room_id_null(mut self) -> Self {
                self.room_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            #[doc = "文字数は 1 以上 256 以下。"]
            pub fn query(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    query: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    user_id: self.user_id,
                    room_id: self.room_id,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    query: self
                        .query
                        .expect("required parameter is set by the type state"),
                    limit: self.limit,
                    user_id: self.user_id,
                    room_id: self.room_id,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/show`"]
    pub mod show {
        #[doc = "`chat/messages/show`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "messageId")]
            pub message_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::ChatMessage;
        #[doc = "`chat/messages/show` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `message_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            message_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    message_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn message_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    message_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    message_id: self
                        .message_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/unreact`"]
    pub mod unreact {
        #[doc = "`chat/messages/unreact`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "messageId")]
            pub message_id: ::std::string::String,
            pub reaction: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/unreact";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`chat/messages/unreact` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `message_id`、`reaction`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            message_id: ::std::option::Option<::std::string::String>,
            reaction: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    message_id: ::std::option::Option::None,
                    reaction: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn message_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    message_id: ::std::option::Option::Some(value.into()),
                    reaction: self.reaction,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn reaction(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    message_id: self.message_id,
                    reaction: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    message_id: self
                        .message_id
                        .expect("required parameter is set by the type state"),
                    reaction: self
                        .reaction
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/messages/user-timeline`"]
    pub mod user_timeline {
        #[doc = "`chat/messages/user-timeline`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
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
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/messages/user-timeline";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response =
            ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatMessageLiteFor1on1>;
        #[doc = "`chat/messages/user-timeline` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `user_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            user_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
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
                    user_id: ::std::option::Option::None,
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
            pub fn user_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    user_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    user_id: self
                        .user_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`chat/read-all`"]
pub mod read_all {
    #[doc = "`chat/read-all`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "chat/read-all";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`chat/read-all` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
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
#[doc = "`chat/rooms/` 以下のエンドポイント"]
pub mod rooms {
    #[doc = "`chat/rooms/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`chat/rooms/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`chat/rooms/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`chat/rooms/invitations/` 以下のエンドポイント"]
        pub fn invitations(&self) -> invitations::Call<'a> {
            invitations::Call::new(self.client)
        }
        #[doc = "`chat/rooms/join`"]
        pub fn join(&self) -> join::Call<'a> {
            join::Call::new(self.client)
        }
        #[doc = "`chat/rooms/joining`"]
        pub fn joining(&self) -> joining::Call<'a> {
            joining::Call::new(self.client)
        }
        #[doc = "`chat/rooms/leave`"]
        pub fn leave(&self) -> leave::Call<'a> {
            leave::Call::new(self.client)
        }
        #[doc = "`chat/rooms/members`"]
        pub fn members(&self) -> members::Call<'a> {
            members::Call::new(self.client)
        }
        #[doc = "`chat/rooms/mute`"]
        pub fn mute(&self) -> mute::Call<'a> {
            mute::Call::new(self.client)
        }
        #[doc = "`chat/rooms/owned`"]
        pub fn owned(&self) -> owned::Call<'a> {
            owned::Call::new(self.client)
        }
        #[doc = "`chat/rooms/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`chat/rooms/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
    }
    #[doc = "`chat/rooms/create`"]
    pub mod create {
        #[doc = "`chat/rooms/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "文字数は 256 以下。"]
            pub name: ::std::string::String,
            #[doc = "文字数は 1024 以下。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/rooms/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::ChatRoom;
        #[doc = "`chat/rooms/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            name: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "文字数は 1024 以下。"]
            pub fn description(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_description(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `description` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_description(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.description = value.map(|value| value.into());
                self
            }
            #[doc = "文字数は 256 以下。"]
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
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
                    description: self.description,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/rooms/delete`"]
    pub mod delete {
        #[doc = "`chat/rooms/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/rooms/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`chat/rooms/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            room_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    room_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    room_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/rooms/invitations/` 以下のエンドポイント"]
    pub mod invitations {
        #[doc = "`chat/rooms/invitations/` 以下のエンドポイント"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {
            #[doc = "`chat/rooms/invitations/create`"]
            pub fn create(&self) -> create::Call<'a> {
                create::Call::new(self.client)
            }
            #[doc = "`chat/rooms/invitations/ignore`"]
            pub fn ignore(&self) -> ignore::Call<'a> {
                ignore::Call::new(self.client)
            }
            #[doc = "`chat/rooms/invitations/inbox`"]
            pub fn inbox(&self) -> inbox::Call<'a> {
                inbox::Call::new(self.client)
            }
            #[doc = "`chat/rooms/invitations/outbox`"]
            pub fn outbox(&self) -> outbox::Call<'a> {
                outbox::Call::new(self.client)
            }
        }
        #[doc = "`chat/rooms/invitations/create`"]
        pub mod create {
            #[doc = "`chat/rooms/invitations/create`"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                #[serde(rename = "roomId")]
                pub room_id: ::std::string::String,
                #[serde(rename = "userId")]
                pub user_id: ::std::string::String,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "chat/rooms/invitations/create";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response = crate::generated::v2025_10_0::types::ChatRoomInvitation;
            #[doc = "`chat/rooms/invitations/create` のビルダー。"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
            #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`、`user_id`。すべて設定すると、呼び出しを await できる。"]
            pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
                client: &'a crate::generated::v2025_10_0::Client,
                room_id: ::std::option::Option<::std::string::String>,
                user_id: ::std::option::Option<::std::string::String>,
                _state: ::std::marker::PhantomData<S>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                    Call {
                        client,
                        room_id: ::std::option::Option::None,
                        user_id: ::std::option::Option::None,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a, S0, S1> Call<'a, (S0, S1)> {
                pub fn room_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (crate::builder::Set, S1)> {
                    Call {
                        client: self.client,
                        room_id: ::std::option::Option::Some(value.into()),
                        user_id: self.user_id,
                        _state: ::std::marker::PhantomData,
                    }
                }
                pub fn user_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (S0, crate::builder::Set)> {
                    Call {
                        client: self.client,
                        room_id: self.room_id,
                        user_id: ::std::option::Option::Some(value.into()),
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        room_id: self
                            .room_id
                            .expect("required parameter is set by the type state"),
                        user_id: self
                            .user_id
                            .expect("required parameter is set by the type state"),
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
        #[doc = "`chat/rooms/invitations/ignore`"]
        pub mod ignore {
            #[doc = "`chat/rooms/invitations/ignore`"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                #[serde(rename = "roomId")]
                pub room_id: ::std::string::String,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "chat/rooms/invitations/ignore";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response = ();
            #[doc = "`chat/rooms/invitations/ignore` のビルダー。"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
            #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
            pub struct Call<'a, S = (crate::builder::Unset,)> {
                client: &'a crate::generated::v2025_10_0::Client,
                room_id: ::std::option::Option<::std::string::String>,
                _state: ::std::marker::PhantomData<S>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                    Call {
                        client,
                        room_id: ::std::option::Option::None,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a, S0> Call<'a, (S0,)> {
                pub fn room_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (crate::builder::Set,)> {
                    Call {
                        client: self.client,
                        room_id: ::std::option::Option::Some(value.into()),
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        room_id: self
                            .room_id
                            .expect("required parameter is set by the type state"),
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
        #[doc = "`chat/rooms/invitations/inbox`"]
        pub mod inbox {
            #[doc = "`chat/rooms/invitations/inbox`"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                #[doc = "1 以上 100 以下。省略時は `30`。"]
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
                const PATH: &'static str = "chat/rooms/invitations/inbox";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response =
                ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatRoomInvitation>;
            #[doc = "`chat/rooms/invitations/inbox` のビルダー。"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
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
                #[doc = "1 以上 100 以下。省略時は `30`。"]
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
                pub fn since_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
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
                pub fn until_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
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
        }
        #[doc = "`chat/rooms/invitations/outbox`"]
        pub mod outbox {
            #[doc = "`chat/rooms/invitations/outbox`"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                #[serde(rename = "roomId")]
                pub room_id: ::std::string::String,
                #[doc = "1 以上 100 以下。省略時は `30`。"]
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
                const PATH: &'static str = "chat/rooms/invitations/outbox";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            pub type Response =
                ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatRoomInvitation>;
            #[doc = "`chat/rooms/invitations/outbox` のビルダー。"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
            #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
            pub struct Call<'a, S = (crate::builder::Unset,)> {
                client: &'a crate::generated::v2025_10_0::Client,
                room_id: ::std::option::Option<::std::string::String>,
                limit: ::std::option::Option<i64>,
                since_id: ::std::option::Option<::std::string::String>,
                until_id: ::std::option::Option<::std::string::String>,
                since_date: ::std::option::Option<i64>,
                until_date: ::std::option::Option<i64>,
                _state: ::std::marker::PhantomData<S>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                    Call {
                        client,
                        room_id: ::std::option::Option::None,
                        limit: ::std::option::Option::None,
                        since_id: ::std::option::Option::None,
                        until_id: ::std::option::Option::None,
                        since_date: ::std::option::Option::None,
                        until_date: ::std::option::Option::None,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a, S0> Call<'a, (S0,)> {
                #[doc = "1 以上 100 以下。省略時は `30`。"]
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
                pub fn since_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
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
                pub fn until_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Self {
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
                pub fn room_id(
                    self,
                    value: impl ::std::convert::Into<::std::string::String>,
                ) -> Call<'a, (crate::builder::Set,)> {
                    Call {
                        client: self.client,
                        room_id: ::std::option::Option::Some(value.into()),
                        limit: self.limit,
                        since_id: self.since_id,
                        until_id: self.until_id,
                        since_date: self.since_date,
                        until_date: self.until_date,
                        _state: ::std::marker::PhantomData,
                    }
                }
            }
            impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        room_id: self
                            .room_id
                            .expect("required parameter is set by the type state"),
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
        }
    }
    #[doc = "`chat/rooms/join`"]
    pub mod join {
        #[doc = "`chat/rooms/join`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/rooms/join";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`chat/rooms/join` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            room_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    room_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    room_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/rooms/joining`"]
    pub mod joining {
        #[doc = "`chat/rooms/joining`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "1 以上 100 以下。省略時は `30`。"]
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
            const PATH: &'static str = "chat/rooms/joining";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response =
            ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatRoomMembership>;
        #[doc = "`chat/rooms/joining` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
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
            #[doc = "1 以上 100 以下。省略時は `30`。"]
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
    }
    #[doc = "`chat/rooms/leave`"]
    pub mod leave {
        #[doc = "`chat/rooms/leave`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/rooms/leave";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`chat/rooms/leave` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            room_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    room_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    room_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/rooms/members`"]
    pub mod members {
        #[doc = "`chat/rooms/members`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
            #[doc = "1 以上 100 以下。省略時は `30`。"]
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
            const PATH: &'static str = "chat/rooms/members";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response =
            ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatRoomMembership>;
        #[doc = "`chat/rooms/members` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            room_id: ::std::option::Option<::std::string::String>,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    room_id: ::std::option::Option::None,
                    limit: ::std::option::Option::None,
                    since_id: ::std::option::Option::None,
                    until_id: ::std::option::Option::None,
                    since_date: ::std::option::Option::None,
                    until_date: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "1 以上 100 以下。省略時は `30`。"]
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
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    room_id: ::std::option::Option::Some(value.into()),
                    limit: self.limit,
                    since_id: self.since_id,
                    until_id: self.until_id,
                    since_date: self.since_date,
                    until_date: self.until_date,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
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
    }
    #[doc = "`chat/rooms/mute`"]
    pub mod mute {
        #[doc = "`chat/rooms/mute`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
            pub mute: bool,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/rooms/mute";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`chat/rooms/mute` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`、`mute`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            room_id: ::std::option::Option<::std::string::String>,
            mute: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    room_id: ::std::option::Option::None,
                    mute: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    room_id: ::std::option::Option::Some(value.into()),
                    mute: self.mute,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn mute(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    room_id: self.room_id,
                    mute: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
                    mute: self
                        .mute
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/rooms/owned`"]
    pub mod owned {
        #[doc = "`chat/rooms/owned`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[doc = "1 以上 100 以下。省略時は `30`。"]
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
            const PATH: &'static str = "chat/rooms/owned";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::ChatRoom>;
        #[doc = "`chat/rooms/owned` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
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
            #[doc = "1 以上 100 以下。省略時は `30`。"]
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
    }
    #[doc = "`chat/rooms/show`"]
    pub mod show {
        #[doc = "`chat/rooms/show`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/rooms/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::ChatRoom;
        #[doc = "`chat/rooms/show` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            room_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    room_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    room_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`chat/rooms/update`"]
    pub mod update {
        #[doc = "`chat/rooms/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "roomId")]
            pub room_id: ::std::string::String,
            #[doc = "文字数は 256 以下。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[doc = "文字数は 1024 以下。"]
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub description: ::std::option::Option<::std::string::String>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "chat/rooms/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::ChatRoom;
        #[doc = "`chat/rooms/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:chat*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            room_id: ::std::option::Option<::std::string::String>,
            name: ::std::option::Option<::std::string::String>,
            description: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    room_id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    description: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            #[doc = "文字数は 256 以下。"]
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
            #[doc = "文字数は 1024 以下。"]
            pub fn description(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.maybe_description(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `description` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_description(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.description = value.map(|value| value.into());
                self
            }
            pub fn room_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    room_id: ::std::option::Option::Some(value.into()),
                    name: self.name,
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
                    room_id: self
                        .room_id
                        .expect("required parameter is set by the type state"),
                    name: self.name,
                    description: self.description,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
