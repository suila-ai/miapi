#![doc = "`bubble-game/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`bubble-game/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`bubble-game/ranking`"]
    pub fn ranking(&self) -> ranking::Call<'a> {
        ranking::Call::new(self.client)
    }
    #[doc = "`bubble-game/register`"]
    pub fn register(&self) -> register::Call<'a> {
        register::Call::new(self.client)
    }
}
#[doc = "`bubble-game/ranking`"]
pub mod ranking {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        pub score: i64,
        #[serde(default)]
        pub user: ::std::option::Option<crate::generated::v2025_10_0::types::UserLite>,
    }
    #[doc = "`bubble-game/ranking`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "gameMode")]
        pub game_mode: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "bubble-game/ranking";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`bubble-game/ranking` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `game_mode`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        game_mode: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                game_mode: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn game_mode(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                game_mode: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                game_mode: self
                    .game_mode
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`bubble-game/register`"]
pub mod register {
    #[doc = "`bubble-game/register`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "0 以上。"]
        pub score: i64,
        #[doc = "文字数は 1 以上 1024 以下。"]
        pub seed: ::std::string::String,
        pub logs: ::std::vec::Vec<::std::vec::Vec<f64>>,
        #[serde(rename = "gameMode")]
        pub game_mode: ::std::string::String,
        #[serde(rename = "gameVersion")]
        pub game_version: i64,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "bubble-game/register";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`bubble-game/register` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `score`、`seed`、`logs`、`game_mode`、`game_version`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<
        'a,
        S = (
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
        ),
    > {
        client: &'a crate::generated::v2025_10_0::Client,
        score: ::std::option::Option<i64>,
        seed: ::std::option::Option<::std::string::String>,
        logs: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<f64>>>,
        game_mode: ::std::option::Option<::std::string::String>,
        game_version: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                score: ::std::option::Option::None,
                seed: ::std::option::Option::None,
                logs: ::std::option::Option::None,
                game_mode: ::std::option::Option::None,
                game_version: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1, S2, S3, S4> Call<'a, (S0, S1, S2, S3, S4)> {
        #[doc = "0 以上。"]
        pub fn score(
            self,
            value: impl ::std::convert::Into<i64>,
        ) -> Call<'a, (crate::builder::Set, S1, S2, S3, S4)> {
            Call {
                client: self.client,
                score: ::std::option::Option::Some(value.into()),
                seed: self.seed,
                logs: self.logs,
                game_mode: self.game_mode,
                game_version: self.game_version,
                _state: ::std::marker::PhantomData,
            }
        }
        #[doc = "文字数は 1 以上 1024 以下。"]
        pub fn seed(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set, S2, S3, S4)> {
            Call {
                client: self.client,
                score: self.score,
                seed: ::std::option::Option::Some(value.into()),
                logs: self.logs,
                game_mode: self.game_mode,
                game_version: self.game_version,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn logs(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<f64>>>,
        ) -> Call<'a, (S0, S1, crate::builder::Set, S3, S4)> {
            Call {
                client: self.client,
                score: self.score,
                seed: self.seed,
                logs: ::std::option::Option::Some(value.into()),
                game_mode: self.game_mode,
                game_version: self.game_version,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn game_mode(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, S1, S2, crate::builder::Set, S4)> {
            Call {
                client: self.client,
                score: self.score,
                seed: self.seed,
                logs: self.logs,
                game_mode: ::std::option::Option::Some(value.into()),
                game_version: self.game_version,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn game_version(
            self,
            value: impl ::std::convert::Into<i64>,
        ) -> Call<'a, (S0, S1, S2, S3, crate::builder::Set)> {
            Call {
                client: self.client,
                score: self.score,
                seed: self.seed,
                logs: self.logs,
                game_mode: self.game_mode,
                game_version: ::std::option::Option::Some(value.into()),
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
                crate::builder::Set,
            ),
        >
    {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                score: self
                    .score
                    .expect("required parameter is set by the type state"),
                seed: self
                    .seed
                    .expect("required parameter is set by the type state"),
                logs: self
                    .logs
                    .expect("required parameter is set by the type state"),
                game_mode: self
                    .game_mode
                    .expect("required parameter is set by the type state"),
                game_version: self
                    .game_version
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
