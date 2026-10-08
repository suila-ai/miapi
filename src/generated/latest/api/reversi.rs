#![doc = "`reversi/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`reversi/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::latest::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`reversi/cancel-match`"]
    pub fn cancel_match(&self) -> cancel_match::Call<'a> {
        cancel_match::Call::new(self.client)
    }
    #[doc = "`reversi/games`"]
    pub fn games(&self) -> games::Call<'a> {
        games::Call::new(self.client)
    }
    #[doc = "`reversi/invitations`"]
    pub fn invitations(&self) -> invitations::Call<'a> {
        invitations::Call::new(self.client)
    }
    #[doc = "`reversi/match`"]
    pub fn r#match(&self) -> r#match::Call<'a> {
        r#match::Call::new(self.client)
    }
    #[doc = "`reversi/show-game`"]
    pub fn show_game(&self) -> show_game::Call<'a> {
        show_game::Call::new(self.client)
    }
    #[doc = "`reversi/surrender`"]
    pub fn surrender(&self) -> surrender::Call<'a> {
        surrender::Call::new(self.client)
    }
    #[doc = "`reversi/verify`"]
    pub fn verify(&self) -> verify::Call<'a> {
        verify::Call::new(self.client)
    }
}
#[doc = "`reversi/cancel-match`"]
pub mod cancel_match {
    #[doc = "`reversi/cancel-match`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "reversi/cancel-match";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`reversi/cancel-match` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn user_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`userId` を `null` として送る。"]
        pub fn user_id_null(mut self) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self.user_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`reversi/games`"]
pub mod games {
    #[doc = "`reversi/games`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
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
        pub my: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "reversi/games";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::ReversiGameLite>;
    #[doc = "`reversi/games` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        my: ::std::option::Option<bool>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                my: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.since_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn since_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.since_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_date(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.until_date = ::std::option::Option::Some(value.into());
            self
        }
        pub fn my(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.my = ::std::option::Option::Some(value.into());
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
                my: self.my,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`reversi/invitations`"]
pub mod invitations {
    #[doc = "`reversi/invitations`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "reversi/invitations";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::latest::types::UserLite>;
    #[doc = "`reversi/invitations` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
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
#[doc = "`reversi/match`"]
pub mod r#match {
    #[doc = "`reversi/match`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "noIrregularRules",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_irregular_rules: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub multiple: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "reversi/match";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::option::Option<crate::generated::latest::types::ReversiGameDetailed>;
    #[doc = "`reversi/match` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::latest::Client,
        user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        no_irregular_rules: ::std::option::Option<bool>,
        multiple: ::std::option::Option<bool>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                no_irregular_rules: ::std::option::Option::None,
                multiple: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn user_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`userId` を `null` として送る。"]
        pub fn user_id_null(mut self) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn no_irregular_rules(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.no_irregular_rules = ::std::option::Option::Some(value.into());
            self
        }
        pub fn multiple(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.multiple = ::std::option::Option::Some(value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                user_id: self.user_id,
                no_irregular_rules: self.no_irregular_rules,
                multiple: self.multiple,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`reversi/show-game`"]
pub mod show_game {
    #[doc = "`reversi/show-game`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "gameId")]
        pub game_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "reversi/show-game";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::latest::types::ReversiGameDetailed;
    #[doc = "`reversi/show-game` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `game_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        game_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                game_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn game_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                game_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                game_id: self
                    .game_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`reversi/surrender`"]
pub mod surrender {
    #[doc = "`reversi/surrender`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "gameId")]
        pub game_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "reversi/surrender";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`reversi/surrender` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `game_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::latest::Client,
        game_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                game_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn game_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                game_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                game_id: self
                    .game_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`reversi/verify`"]
pub mod verify {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        pub desynced: bool,
        #[serde(default)]
        pub game: ::std::option::Option<crate::generated::latest::types::ReversiGameDetailed>,
    }
    #[doc = "`reversi/verify`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "gameId")]
        pub game_id: ::std::string::String,
        pub crc32: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "reversi/verify";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`reversi/verify` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `game_id`、`crc32`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::latest::Client,
        game_id: ::std::option::Option<::std::string::String>,
        crc32: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::latest::Client) -> Self {
            Call {
                client,
                game_id: ::std::option::Option::None,
                crc32: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn game_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                game_id: ::std::option::Option::Some(value.into()),
                crc32: self.crc32,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn crc32(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                game_id: self.game_id,
                crc32: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                game_id: self
                    .game_id
                    .expect("required parameter is set by the type state"),
                crc32: self
                    .crc32
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
