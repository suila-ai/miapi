#![doc = "`reversi/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`reversi/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
        client: &'a crate::generated::v2025_10_0::Client,
        user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
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
        #[doc = "省略時は `false`。"]
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
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::ReversiGameLite>;
    #[doc = "`reversi/games` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        my: ::std::option::Option<bool>,
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
                my: ::std::option::Option::None,
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
        #[doc = "省略時は `false`。"]
        pub fn my(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_my(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `my` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_my(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.my = value.map(|value| value.into());
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
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::UserLite>;
    #[doc = "`reversi/invitations` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
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
        #[doc = "省略時は `false`。"]
        #[serde(
            rename = "noIrregularRules",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_irregular_rules: ::std::option::Option<bool>,
        #[doc = "省略時は `false`。"]
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
    pub type Response =
        ::std::option::Option<crate::generated::v2025_10_0::types::ReversiGameDetailed>;
    #[doc = "`reversi/match` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        no_irregular_rules: ::std::option::Option<bool>,
        multiple: ::std::option::Option<bool>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                user_id: ::std::option::Option::None,
                no_irregular_rules: ::std::option::Option::None,
                multiple: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
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
        #[doc = "省略時は `false`。"]
        pub fn no_irregular_rules(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_no_irregular_rules(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `noIrregularRules` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_no_irregular_rules(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.no_irregular_rules = value.map(|value| value.into());
            self
        }
        #[doc = "省略時は `false`。"]
        pub fn multiple(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_multiple(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `multiple` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_multiple(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.multiple = value.map(|value| value.into());
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
    pub type Response = crate::generated::v2025_10_0::types::ReversiGameDetailed;
    #[doc = "`reversi/show-game` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `game_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        game_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
        client: &'a crate::generated::v2025_10_0::Client,
        game_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
        pub game: ::std::option::Option<crate::generated::v2025_10_0::types::ReversiGameDetailed>,
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
        client: &'a crate::generated::v2025_10_0::Client,
        game_id: ::std::option::Option<::std::string::String>,
        crc32: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
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
