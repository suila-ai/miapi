#![doc = "`i` と、`i/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`i`"]
#[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {}
impl crate::Endpoint for Request {
    const PATH: &'static str = "i";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = crate::generated::v2025_10_0::types::MeDetailed;
#[doc = "`i` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`i/2fa/` 以下のエンドポイント"]
    pub fn _2fa(&self) -> _2fa::Call<'a> {
        _2fa::Call::new(self.client)
    }
    #[doc = "`i/apps`"]
    pub fn apps(&self) -> apps::Call<'a> {
        apps::Call::new(self.client)
    }
    #[doc = "`i/authorized-apps`"]
    pub fn authorized_apps(&self) -> authorized_apps::Call<'a> {
        authorized_apps::Call::new(self.client)
    }
    #[doc = "`i/change-password`"]
    pub fn change_password(&self) -> change_password::Call<'a> {
        change_password::Call::new(self.client)
    }
    #[doc = "`i/claim-achievement`"]
    pub fn claim_achievement(&self) -> claim_achievement::Call<'a> {
        claim_achievement::Call::new(self.client)
    }
    #[doc = "`i/delete-account`"]
    pub fn delete_account(&self) -> delete_account::Call<'a> {
        delete_account::Call::new(self.client)
    }
    #[doc = "`i/export-antennas`"]
    pub fn export_antennas(&self) -> export_antennas::Call<'a> {
        export_antennas::Call::new(self.client)
    }
    #[doc = "`i/export-blocking`"]
    pub fn export_blocking(&self) -> export_blocking::Call<'a> {
        export_blocking::Call::new(self.client)
    }
    #[doc = "`i/export-clips`"]
    pub fn export_clips(&self) -> export_clips::Call<'a> {
        export_clips::Call::new(self.client)
    }
    #[doc = "`i/export-favorites`"]
    pub fn export_favorites(&self) -> export_favorites::Call<'a> {
        export_favorites::Call::new(self.client)
    }
    #[doc = "`i/export-following`"]
    pub fn export_following(&self) -> export_following::Call<'a> {
        export_following::Call::new(self.client)
    }
    #[doc = "`i/export-mute`"]
    pub fn export_mute(&self) -> export_mute::Call<'a> {
        export_mute::Call::new(self.client)
    }
    #[doc = "`i/export-notes`"]
    pub fn export_notes(&self) -> export_notes::Call<'a> {
        export_notes::Call::new(self.client)
    }
    #[doc = "`i/export-user-lists`"]
    pub fn export_user_lists(&self) -> export_user_lists::Call<'a> {
        export_user_lists::Call::new(self.client)
    }
    #[doc = "`i/favorites`"]
    pub fn favorites(&self) -> favorites::Call<'a> {
        favorites::Call::new(self.client)
    }
    #[doc = "`i/gallery/` 以下のエンドポイント"]
    pub fn gallery(&self) -> gallery::Call<'a> {
        gallery::Call::new(self.client)
    }
    #[doc = "`i/import-antennas`"]
    pub fn import_antennas(&self) -> import_antennas::Call<'a> {
        import_antennas::Call::new(self.client)
    }
    #[doc = "`i/import-blocking`"]
    pub fn import_blocking(&self) -> import_blocking::Call<'a> {
        import_blocking::Call::new(self.client)
    }
    #[doc = "`i/import-following`"]
    pub fn import_following(&self) -> import_following::Call<'a> {
        import_following::Call::new(self.client)
    }
    #[doc = "`i/import-muting`"]
    pub fn import_muting(&self) -> import_muting::Call<'a> {
        import_muting::Call::new(self.client)
    }
    #[doc = "`i/import-user-lists`"]
    pub fn import_user_lists(&self) -> import_user_lists::Call<'a> {
        import_user_lists::Call::new(self.client)
    }
    #[doc = "`i/move`"]
    pub fn r#move(&self) -> r#move::Call<'a> {
        r#move::Call::new(self.client)
    }
    #[doc = "`i/notifications`"]
    pub fn notifications(&self) -> notifications::Call<'a> {
        notifications::Call::new(self.client)
    }
    #[doc = "`i/notifications-grouped`"]
    pub fn notifications_grouped(&self) -> notifications_grouped::Call<'a> {
        notifications_grouped::Call::new(self.client)
    }
    #[doc = "`i/page-likes`"]
    pub fn page_likes(&self) -> page_likes::Call<'a> {
        page_likes::Call::new(self.client)
    }
    #[doc = "`i/pages`"]
    pub fn pages(&self) -> pages::Call<'a> {
        pages::Call::new(self.client)
    }
    #[doc = "`i/pin`"]
    pub fn pin(&self) -> pin::Call<'a> {
        pin::Call::new(self.client)
    }
    #[doc = "`i/read-announcement`"]
    pub fn read_announcement(&self) -> read_announcement::Call<'a> {
        read_announcement::Call::new(self.client)
    }
    #[doc = "`i/regenerate-token`"]
    pub fn regenerate_token(&self) -> regenerate_token::Call<'a> {
        regenerate_token::Call::new(self.client)
    }
    #[doc = "`i/registry/` 以下のエンドポイント"]
    pub fn registry(&self) -> registry::Call<'a> {
        registry::Call::new(self.client)
    }
    #[doc = "`i/revoke-token`"]
    pub fn revoke_token(&self) -> revoke_token::Call<'a> {
        revoke_token::Call::new(self.client)
    }
    #[doc = "`i/signin-history`"]
    pub fn signin_history(&self) -> signin_history::Call<'a> {
        signin_history::Call::new(self.client)
    }
    #[doc = "`i/unpin`"]
    pub fn unpin(&self) -> unpin::Call<'a> {
        unpin::Call::new(self.client)
    }
    #[doc = "`i/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
    }
    #[doc = "`i/update-email`"]
    pub fn update_email(&self) -> update_email::Call<'a> {
        update_email::Call::new(self.client)
    }
    #[doc = "`i/webhooks/` 以下のエンドポイント"]
    pub fn webhooks(&self) -> webhooks::Call<'a> {
        webhooks::Call::new(self.client)
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
#[doc = "`i/2fa/` 以下のエンドポイント"]
pub mod _2fa {
    #[doc = "`i/2fa/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`i/2fa/done`"]
        pub fn done(&self) -> done::Call<'a> {
            done::Call::new(self.client)
        }
        #[doc = "`i/2fa/key-done`"]
        pub fn key_done(&self) -> key_done::Call<'a> {
            key_done::Call::new(self.client)
        }
        #[doc = "`i/2fa/password-less`"]
        pub fn password_less(&self) -> password_less::Call<'a> {
            password_less::Call::new(self.client)
        }
        #[doc = "`i/2fa/register`"]
        pub fn register(&self) -> register::Call<'a> {
            register::Call::new(self.client)
        }
        #[doc = "`i/2fa/register-key`"]
        pub fn register_key(&self) -> register_key::Call<'a> {
            register_key::Call::new(self.client)
        }
        #[doc = "`i/2fa/remove-key`"]
        pub fn remove_key(&self) -> remove_key::Call<'a> {
            remove_key::Call::new(self.client)
        }
        #[doc = "`i/2fa/unregister`"]
        pub fn unregister(&self) -> unregister::Call<'a> {
            unregister::Call::new(self.client)
        }
        #[doc = "`i/2fa/update-key`"]
        pub fn update_key(&self) -> update_key::Call<'a> {
            update_key::Call::new(self.client)
        }
    }
    #[doc = "`i/2fa/done`"]
    pub mod done {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            #[serde(rename = "backupCodes")]
            pub backup_codes: ::std::vec::Vec<::std::string::String>,
        }
        #[doc = "`i/2fa/done`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub token: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/done";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`i/2fa/done` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `token`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            token: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    token: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn token(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    token: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    token: self
                        .token
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/2fa/key-done`"]
    pub mod key_done {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
            pub name: ::std::string::String,
        }
        #[doc = "`i/2fa/key-done`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub password: ::std::string::String,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
            pub name: ::std::string::String,
            pub credential: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/key-done";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`i/2fa/key-done` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `password`、`name`、`credential`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2025_10_0::Client,
            password: ::std::option::Option<::std::string::String>,
            token: ::std::option::Option<::std::option::Option<::std::string::String>>,
            name: ::std::option::Option<::std::string::String>,
            credential: ::std::option::Option<
                ::serde_json::Map<::std::string::String, ::serde_json::Value>,
            >,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    password: ::std::option::Option::None,
                    token: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    credential: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
            pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_token(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
            pub fn maybe_token(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.token = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`token` を `null` として送る。"]
            pub fn token_null(mut self) -> Self {
                self.token = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn password(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1, S2)> {
                Call {
                    client: self.client,
                    password: ::std::option::Option::Some(value.into()),
                    token: self.token,
                    name: self.name,
                    credential: self.credential,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set, S2)> {
                Call {
                    client: self.client,
                    password: self.password,
                    token: self.token,
                    name: ::std::option::Option::Some(value.into()),
                    credential: self.credential,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn credential(
                self,
                value: impl ::std::convert::Into<
                    ::serde_json::Map<::std::string::String, ::serde_json::Value>,
                >,
            ) -> Call<'a, (S0, S1, crate::builder::Set)> {
                Call {
                    client: self.client,
                    password: self.password,
                    token: self.token,
                    name: self.name,
                    credential: ::std::option::Option::Some(value.into()),
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
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    password: self
                        .password
                        .expect("required parameter is set by the type state"),
                    token: self.token,
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    credential: self
                        .credential
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/2fa/password-less`"]
    pub mod password_less {
        #[doc = "`i/2fa/password-less`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub value: bool,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/password-less";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/2fa/password-less` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `value`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            value: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    value: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn value(
                self,
                value: impl ::std::convert::Into<bool>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    value: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    value: self
                        .value
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/2fa/register`"]
    pub mod register {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub qr: ::std::string::String,
            pub url: ::std::string::String,
            pub secret: ::std::string::String,
            pub label: ::std::string::String,
            pub issuer: ::std::string::String,
        }
        #[doc = "`i/2fa/register`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub password: ::std::string::String,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/register";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`i/2fa/register` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `password`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            password: ::std::option::Option<::std::string::String>,
            token: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    password: ::std::option::Option::None,
                    token: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_token(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
            pub fn maybe_token(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.token = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`token` を `null` として送る。"]
            pub fn token_null(mut self) -> Self {
                self.token = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn password(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    password: ::std::option::Option::Some(value.into()),
                    token: self.token,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    password: self
                        .password
                        .expect("required parameter is set by the type state"),
                    token: self.token,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/2fa/register-key`"]
    pub mod register_key {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseRp {
            #[serde(default)]
            pub id: ::std::option::Option<::std::string::String>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseUser {
            pub id: ::std::string::String,
            pub name: ::std::string::String,
            #[serde(rename = "displayName")]
            pub display_name: ::std::string::String,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponsePubKeyCredParamsItem {
            pub r#type: ::std::string::String,
            pub alg: f64,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum ResponseExcludeCredentialsItemTransportsItem {
            #[serde(rename = "ble")]
            Ble,
            #[serde(rename = "cable")]
            Cable,
            #[serde(rename = "hybrid")]
            Hybrid,
            #[serde(rename = "internal")]
            Internal,
            #[serde(rename = "nfc")]
            Nfc,
            #[serde(rename = "smart-card")]
            SmartCard,
            #[serde(rename = "usb")]
            Usb,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseExcludeCredentialsItem {
            pub id: ::std::string::String,
            pub r#type: ::std::string::String,
            pub transports: ::std::vec::Vec<ResponseExcludeCredentialsItemTransportsItem>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum ResponseAuthenticatorSelectionAuthenticatorAttachment {
            #[serde(rename = "cross-platform")]
            CrossPlatform,
            #[serde(rename = "platform")]
            Platform,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum ResponseAuthenticatorSelectionUserVerification {
            #[serde(rename = "discouraged")]
            Discouraged,
            #[serde(rename = "preferred")]
            Preferred,
            #[serde(rename = "required")]
            Required,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseAuthenticatorSelection {
            #[serde(rename = "authenticatorAttachment")]
            pub authenticator_attachment: ResponseAuthenticatorSelectionAuthenticatorAttachment,
            #[serde(rename = "requireResidentKey")]
            pub require_resident_key: bool,
            #[serde(rename = "userVerification")]
            pub user_verification: ResponseAuthenticatorSelectionUserVerification,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum ResponseAttestation {
            #[serde(rename = "direct")]
            Direct,
            #[serde(rename = "enterprise")]
            Enterprise,
            #[serde(rename = "indirect")]
            Indirect,
            #[serde(rename = "none")]
            None,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseExtensions {
            pub appid: ::std::option::Option<::std::string::String>,
            #[serde(rename = "credProps")]
            pub cred_props: ::std::option::Option<bool>,
            #[serde(rename = "hmacCreateSecret")]
            pub hmac_create_secret: ::std::option::Option<bool>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub rp: ResponseRp,
            pub user: ResponseUser,
            pub challenge: ::std::string::String,
            #[serde(rename = "pubKeyCredParams")]
            pub pub_key_cred_params: ::std::vec::Vec<ResponsePubKeyCredParamsItem>,
            pub timeout: ::std::option::Option<f64>,
            #[serde(rename = "excludeCredentials")]
            pub exclude_credentials:
                ::std::option::Option<::std::vec::Vec<ResponseExcludeCredentialsItem>>,
            #[serde(rename = "authenticatorSelection")]
            pub authenticator_selection: ::std::option::Option<ResponseAuthenticatorSelection>,
            pub attestation: ::std::option::Option<ResponseAttestation>,
            pub extensions: ::std::option::Option<ResponseExtensions>,
        }
        #[doc = "`i/2fa/register-key`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub password: ::std::string::String,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/register-key";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`i/2fa/register-key` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `password`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            password: ::std::option::Option<::std::string::String>,
            token: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    password: ::std::option::Option::None,
                    token: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_token(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
            pub fn maybe_token(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.token = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`token` を `null` として送る。"]
            pub fn token_null(mut self) -> Self {
                self.token = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn password(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    password: ::std::option::Option::Some(value.into()),
                    token: self.token,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    password: self
                        .password
                        .expect("required parameter is set by the type state"),
                    token: self.token,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/2fa/remove-key`"]
    pub mod remove_key {
        #[doc = "`i/2fa/remove-key`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub password: ::std::string::String,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(rename = "credentialId")]
            pub credential_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/remove-key";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/2fa/remove-key` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `password`、`credential_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            password: ::std::option::Option<::std::string::String>,
            token: ::std::option::Option<::std::option::Option<::std::string::String>>,
            credential_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    password: ::std::option::Option::None,
                    token: ::std::option::Option::None,
                    credential_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_token(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
            pub fn maybe_token(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.token = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`token` を `null` として送る。"]
            pub fn token_null(mut self) -> Self {
                self.token = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn password(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    password: ::std::option::Option::Some(value.into()),
                    token: self.token,
                    credential_id: self.credential_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn credential_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    password: self.password,
                    token: self.token,
                    credential_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    password: self
                        .password
                        .expect("required parameter is set by the type state"),
                    token: self.token,
                    credential_id: self
                        .credential_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/2fa/unregister`"]
    pub mod unregister {
        #[doc = "`i/2fa/unregister`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub password: ::std::string::String,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/unregister";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/2fa/unregister` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `password`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            password: ::std::option::Option<::std::string::String>,
            token: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    password: ::std::option::Option::None,
                    token: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_token(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
            pub fn maybe_token(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.token = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`token` を `null` として送る。"]
            pub fn token_null(mut self) -> Self {
                self.token = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn password(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    password: ::std::option::Option::Some(value.into()),
                    token: self.token,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    password: self
                        .password
                        .expect("required parameter is set by the type state"),
                    token: self.token,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/2fa/update-key`"]
    pub mod update_key {
        #[doc = "`i/2fa/update-key`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub name: ::std::string::String,
            #[serde(rename = "credentialId")]
            pub credential_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/2fa/update-key";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/2fa/update-key` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`credential_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            name: ::std::option::Option<::std::string::String>,
            credential_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    credential_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    credential_id: self.credential_id,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn credential_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    name: self.name,
                    credential_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    credential_id: self
                        .credential_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`i/apps`"]
pub mod apps {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSort {
        #[serde(rename = "+createdAt")]
        PlusCreatedAt,
        #[serde(rename = "-createdAt")]
        MinusCreatedAt,
        #[serde(rename = "+lastUsedAt")]
        PlusLastUsedAt,
        #[serde(rename = "-lastUsedAt")]
        MinusLastUsedAt,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        #[serde(default)]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(rename = "createdAt")]
        pub created_at: ::chrono::DateTime<::chrono::Utc>,
        #[serde(rename = "lastUsedAt", default)]
        pub last_used_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
        pub permission: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "iconUrl", default)]
        pub icon_url: ::std::option::Option<::std::string::String>,
        #[serde(default)]
        pub description: ::std::option::Option<::std::string::String>,
    }
    #[doc = "`i/apps`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sort: ::std::option::Option<RequestSort>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/apps";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`i/apps` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        sort: ::std::option::Option<RequestSort>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                sort: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn sort(self, value: impl ::std::convert::Into<RequestSort>) -> Self {
            self.maybe_sort(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sort` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_sort(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestSort>>,
        ) -> Self {
            self.sort = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request { sort: self.sort };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/authorized-apps`"]
pub mod authorized_apps {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSort {
        #[serde(rename = "desc")]
        Desc,
        #[serde(rename = "asc")]
        Asc,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        pub name: ::std::string::String,
        #[serde(rename = "callbackUrl")]
        pub callback_url: ::std::option::Option<::std::string::String>,
        pub permission: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "isAuthorized", default)]
        pub is_authorized: ::std::option::Option<bool>,
    }
    #[doc = "`i/authorized-apps`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sort: ::std::option::Option<RequestSort>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/authorized-apps";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`i/authorized-apps` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
        sort: ::std::option::Option<RequestSort>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                sort: ::std::option::Option::None,
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
        pub fn sort(self, value: impl ::std::convert::Into<RequestSort>) -> Self {
            self.maybe_sort(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `sort` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_sort(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestSort>>,
        ) -> Self {
            self.sort = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                limit: self.limit,
                offset: self.offset,
                sort: self.sort,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/change-password`"]
pub mod change_password {
    #[doc = "`i/change-password`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "currentPassword")]
        pub current_password: ::std::string::String,
        #[serde(rename = "newPassword")]
        pub new_password: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/change-password";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/change-password` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `current_password`、`new_password`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::v2025_10_0::Client,
        current_password: ::std::option::Option<::std::string::String>,
        new_password: ::std::option::Option<::std::string::String>,
        token: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                current_password: ::std::option::Option::None,
                new_password: ::std::option::Option::None,
                token: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_token(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
        pub fn maybe_token(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.token = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`token` を `null` として送る。"]
        pub fn token_null(mut self) -> Self {
            self.token = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn current_password(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                current_password: ::std::option::Option::Some(value.into()),
                new_password: self.new_password,
                token: self.token,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn new_password(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                current_password: self.current_password,
                new_password: ::std::option::Option::Some(value.into()),
                token: self.token,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                current_password: self
                    .current_password
                    .expect("required parameter is set by the type state"),
                new_password: self
                    .new_password
                    .expect("required parameter is set by the type state"),
                token: self.token,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/claim-achievement`"]
pub mod claim_achievement {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestName {
        #[serde(rename = "notes1")]
        Notes1,
        #[serde(rename = "notes10")]
        Notes10,
        #[serde(rename = "notes100")]
        Notes100,
        #[serde(rename = "notes500")]
        Notes500,
        #[serde(rename = "notes1000")]
        Notes1000,
        #[serde(rename = "notes5000")]
        Notes5000,
        #[serde(rename = "notes10000")]
        Notes10000,
        #[serde(rename = "notes20000")]
        Notes20000,
        #[serde(rename = "notes30000")]
        Notes30000,
        #[serde(rename = "notes40000")]
        Notes40000,
        #[serde(rename = "notes50000")]
        Notes50000,
        #[serde(rename = "notes60000")]
        Notes60000,
        #[serde(rename = "notes70000")]
        Notes70000,
        #[serde(rename = "notes80000")]
        Notes80000,
        #[serde(rename = "notes90000")]
        Notes90000,
        #[serde(rename = "notes100000")]
        Notes100000,
        #[serde(rename = "login3")]
        Login3,
        #[serde(rename = "login7")]
        Login7,
        #[serde(rename = "login15")]
        Login15,
        #[serde(rename = "login30")]
        Login30,
        #[serde(rename = "login60")]
        Login60,
        #[serde(rename = "login100")]
        Login100,
        #[serde(rename = "login200")]
        Login200,
        #[serde(rename = "login300")]
        Login300,
        #[serde(rename = "login400")]
        Login400,
        #[serde(rename = "login500")]
        Login500,
        #[serde(rename = "login600")]
        Login600,
        #[serde(rename = "login700")]
        Login700,
        #[serde(rename = "login800")]
        Login800,
        #[serde(rename = "login900")]
        Login900,
        #[serde(rename = "login1000")]
        Login1000,
        #[serde(rename = "passedSinceAccountCreated1")]
        PassedSinceAccountCreated1,
        #[serde(rename = "passedSinceAccountCreated2")]
        PassedSinceAccountCreated2,
        #[serde(rename = "passedSinceAccountCreated3")]
        PassedSinceAccountCreated3,
        #[serde(rename = "loggedInOnBirthday")]
        LoggedInOnBirthday,
        #[serde(rename = "loggedInOnNewYearsDay")]
        LoggedInOnNewYearsDay,
        #[serde(rename = "noteClipped1")]
        NoteClipped1,
        #[serde(rename = "noteFavorited1")]
        NoteFavorited1,
        #[serde(rename = "myNoteFavorited1")]
        MyNoteFavorited1,
        #[serde(rename = "profileFilled")]
        ProfileFilled,
        #[serde(rename = "markedAsCat")]
        MarkedAsCat,
        #[serde(rename = "following1")]
        Following1,
        #[serde(rename = "following10")]
        Following10,
        #[serde(rename = "following50")]
        Following50,
        #[serde(rename = "following100")]
        Following100,
        #[serde(rename = "following300")]
        Following300,
        #[serde(rename = "followers1")]
        Followers1,
        #[serde(rename = "followers10")]
        Followers10,
        #[serde(rename = "followers50")]
        Followers50,
        #[serde(rename = "followers100")]
        Followers100,
        #[serde(rename = "followers300")]
        Followers300,
        #[serde(rename = "followers500")]
        Followers500,
        #[serde(rename = "followers1000")]
        Followers1000,
        #[serde(rename = "collectAchievements30")]
        CollectAchievements30,
        #[serde(rename = "viewAchievements3min")]
        ViewAchievements3min,
        #[serde(rename = "iLoveMisskey")]
        ILoveMisskey,
        #[serde(rename = "foundTreasure")]
        FoundTreasure,
        #[serde(rename = "client30min")]
        Client30min,
        #[serde(rename = "client60min")]
        Client60min,
        #[serde(rename = "noteDeletedWithin1min")]
        NoteDeletedWithin1min,
        #[serde(rename = "postedAtLateNight")]
        PostedAtLateNight,
        #[serde(rename = "postedAt0min0sec")]
        PostedAt0min0sec,
        #[serde(rename = "selfQuote")]
        SelfQuote,
        #[serde(rename = "htl20npm")]
        Htl20npm,
        #[serde(rename = "viewInstanceChart")]
        ViewInstanceChart,
        #[serde(rename = "outputHelloWorldOnScratchpad")]
        OutputHelloWorldOnScratchpad,
        #[serde(rename = "open3windows")]
        Open3windows,
        #[serde(rename = "driveFolderCircularReference")]
        DriveFolderCircularReference,
        #[serde(rename = "reactWithoutRead")]
        ReactWithoutRead,
        #[serde(rename = "clickedClickHere")]
        ClickedClickHere,
        #[serde(rename = "justPlainLucky")]
        JustPlainLucky,
        #[serde(rename = "setNameToSyuilo")]
        SetNameToSyuilo,
        #[serde(rename = "cookieClicked")]
        CookieClicked,
        #[serde(rename = "brainDiver")]
        BrainDiver,
        #[serde(rename = "smashTestNotificationButton")]
        SmashTestNotificationButton,
        #[serde(rename = "tutorialCompleted")]
        TutorialCompleted,
        #[serde(rename = "bubbleGameExplodingHead")]
        BubbleGameExplodingHead,
        #[serde(rename = "bubbleGameDoubleExplodingHead")]
        BubbleGameDoubleExplodingHead,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`i/claim-achievement`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub name: RequestName,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/claim-achievement";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/claim-achievement` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `name`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        name: ::std::option::Option<RequestName>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                name: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn name(
            self,
            value: impl ::std::convert::Into<RequestName>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                name: ::std::option::Option::Some(value.into()),
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
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/delete-account`"]
pub mod delete_account {
    #[doc = "`i/delete-account`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub password: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/delete-account";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/delete-account` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `password`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        password: ::std::option::Option<::std::string::String>,
        token: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                password: ::std::option::Option::None,
                token: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_token(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
        pub fn maybe_token(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.token = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`token` を `null` として送る。"]
        pub fn token_null(mut self) -> Self {
            self.token = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn password(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                password: ::std::option::Option::Some(value.into()),
                token: self.token,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                password: self
                    .password
                    .expect("required parameter is set by the type state"),
                token: self.token,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/export-antennas`"]
pub mod export_antennas {
    #[doc = "`i/export-antennas`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-antennas";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-antennas` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/export-blocking`"]
pub mod export_blocking {
    #[doc = "`i/export-blocking`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-blocking";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-blocking` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/export-clips`"]
pub mod export_clips {
    #[doc = "`i/export-clips`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-clips";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-clips` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/export-favorites`"]
pub mod export_favorites {
    #[doc = "`i/export-favorites`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-favorites";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-favorites` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/export-following`"]
pub mod export_following {
    #[doc = "`i/export-following`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(
            rename = "excludeMuting",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_muting: ::std::option::Option<bool>,
        #[serde(
            rename = "excludeInactive",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_inactive: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-following";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-following` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        exclude_muting: ::std::option::Option<bool>,
        exclude_inactive: ::std::option::Option<bool>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                exclude_muting: ::std::option::Option::None,
                exclude_inactive: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn exclude_muting(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_exclude_muting(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeMuting` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_muting(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.exclude_muting = value.map(|value| value.into());
            self
        }
        pub fn exclude_inactive(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_exclude_inactive(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeInactive` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_inactive(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.exclude_inactive = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                exclude_muting: self.exclude_muting,
                exclude_inactive: self.exclude_inactive,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/export-mute`"]
pub mod export_mute {
    #[doc = "`i/export-mute`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-mute";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-mute` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/export-notes`"]
pub mod export_notes {
    #[doc = "`i/export-notes`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-notes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-notes` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/export-user-lists`"]
pub mod export_user_lists {
    #[doc = "`i/export-user-lists`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/export-user-lists";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/export-user-lists` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/favorites`"]
pub mod favorites {
    #[doc = "`i/favorites`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:favorites*"]
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
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/favorites";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::NoteFavorite>;
    #[doc = "`i/favorites` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:favorites*"]
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
#[doc = "`i/gallery/` 以下のエンドポイント"]
pub mod gallery {
    #[doc = "`i/gallery/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`i/gallery/likes`"]
        pub fn likes(&self) -> likes::Call<'a> {
            likes::Call::new(self.client)
        }
        #[doc = "`i/gallery/posts`"]
        pub fn posts(&self) -> posts::Call<'a> {
            posts::Call::new(self.client)
        }
    }
    #[doc = "`i/gallery/likes`"]
    pub mod likes {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub id: ::std::string::String,
            pub post: crate::generated::v2025_10_0::types::GalleryPost,
        }
        #[doc = "`i/gallery/likes`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:gallery-likes*"]
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
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/gallery/likes";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`i/gallery/likes` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:gallery-likes*"]
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
    #[doc = "`i/gallery/posts`"]
    pub mod posts {
        #[doc = "`i/gallery/posts`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:gallery*"]
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
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/gallery/posts";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::GalleryPost>;
        #[doc = "`i/gallery/posts` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:gallery*"]
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
}
#[doc = "`i/import-antennas`"]
pub mod import_antennas {
    #[doc = "`i/import-antennas`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "fileId")]
        pub file_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/import-antennas";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/import-antennas` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/import-blocking`"]
pub mod import_blocking {
    #[doc = "`i/import-blocking`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "fileId")]
        pub file_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/import-blocking";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/import-blocking` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/import-following`"]
pub mod import_following {
    #[doc = "`i/import-following`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "fileId")]
        pub file_id: ::std::string::String,
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/import-following";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/import-following` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `file_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        file_id: ::std::option::Option<::std::string::String>,
        with_replies: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                file_id: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn with_replies(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_replies(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withReplies` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_replies(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_replies = value.map(|value| value.into());
            self
        }
        pub fn file_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                file_id: ::std::option::Option::Some(value.into()),
                with_replies: self.with_replies,
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
                with_replies: self.with_replies,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/import-muting`"]
pub mod import_muting {
    #[doc = "`i/import-muting`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "fileId")]
        pub file_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/import-muting";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/import-muting` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/import-user-lists`"]
pub mod import_user_lists {
    #[doc = "`i/import-user-lists`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "fileId")]
        pub file_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/import-user-lists";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/import-user-lists` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/move`"]
pub mod r#move {
    #[doc = "`i/move`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "moveToAccount")]
        pub move_to_account: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/move";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
    #[doc = "`i/move` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `move_to_account`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        move_to_account: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                move_to_account: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn move_to_account(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                move_to_account: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                move_to_account: self
                    .move_to_account
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/notifications`"]
pub mod notifications {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestIncludeTypesItem {
        #[serde(rename = "note")]
        Note,
        #[serde(rename = "follow")]
        Follow,
        #[serde(rename = "mention")]
        Mention,
        #[serde(rename = "reply")]
        Reply,
        #[serde(rename = "renote")]
        Renote,
        #[serde(rename = "quote")]
        Quote,
        #[serde(rename = "reaction")]
        Reaction,
        #[serde(rename = "pollEnded")]
        PollEnded,
        #[serde(rename = "scheduledNotePosted")]
        ScheduledNotePosted,
        #[serde(rename = "scheduledNotePostFailed")]
        ScheduledNotePostFailed,
        #[serde(rename = "receiveFollowRequest")]
        ReceiveFollowRequest,
        #[serde(rename = "followRequestAccepted")]
        FollowRequestAccepted,
        #[serde(rename = "roleAssigned")]
        RoleAssigned,
        #[serde(rename = "chatRoomInvitationReceived")]
        ChatRoomInvitationReceived,
        #[serde(rename = "achievementEarned")]
        AchievementEarned,
        #[serde(rename = "exportCompleted")]
        ExportCompleted,
        #[serde(rename = "login")]
        Login,
        #[serde(rename = "createToken")]
        CreateToken,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "test")]
        Test,
        #[serde(rename = "pollVote")]
        PollVote,
        #[serde(rename = "groupInvited")]
        GroupInvited,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestExcludeTypesItem {
        #[serde(rename = "note")]
        Note,
        #[serde(rename = "follow")]
        Follow,
        #[serde(rename = "mention")]
        Mention,
        #[serde(rename = "reply")]
        Reply,
        #[serde(rename = "renote")]
        Renote,
        #[serde(rename = "quote")]
        Quote,
        #[serde(rename = "reaction")]
        Reaction,
        #[serde(rename = "pollEnded")]
        PollEnded,
        #[serde(rename = "scheduledNotePosted")]
        ScheduledNotePosted,
        #[serde(rename = "scheduledNotePostFailed")]
        ScheduledNotePostFailed,
        #[serde(rename = "receiveFollowRequest")]
        ReceiveFollowRequest,
        #[serde(rename = "followRequestAccepted")]
        FollowRequestAccepted,
        #[serde(rename = "roleAssigned")]
        RoleAssigned,
        #[serde(rename = "chatRoomInvitationReceived")]
        ChatRoomInvitationReceived,
        #[serde(rename = "achievementEarned")]
        AchievementEarned,
        #[serde(rename = "exportCompleted")]
        ExportCompleted,
        #[serde(rename = "login")]
        Login,
        #[serde(rename = "createToken")]
        CreateToken,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "test")]
        Test,
        #[serde(rename = "pollVote")]
        PollVote,
        #[serde(rename = "groupInvited")]
        GroupInvited,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`i/notifications`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:notifications*"]
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
            rename = "markAsRead",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mark_as_read: ::std::option::Option<bool>,
        #[serde(
            rename = "includeTypes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_types: ::std::option::Option<::std::vec::Vec<RequestIncludeTypesItem>>,
        #[serde(
            rename = "excludeTypes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_types: ::std::option::Option<::std::vec::Vec<RequestExcludeTypesItem>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/notifications";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Notification>;
    #[doc = "`i/notifications` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:notifications*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        mark_as_read: ::std::option::Option<bool>,
        include_types: ::std::option::Option<::std::vec::Vec<RequestIncludeTypesItem>>,
        exclude_types: ::std::option::Option<::std::vec::Vec<RequestExcludeTypesItem>>,
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
                mark_as_read: ::std::option::Option::None,
                include_types: ::std::option::Option::None,
                exclude_types: ::std::option::Option::None,
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
        pub fn mark_as_read(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_mark_as_read(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `markAsRead` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_mark_as_read(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.mark_as_read = value.map(|value| value.into());
            self
        }
        pub fn include_types(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestIncludeTypesItem>>,
        ) -> Self {
            self.maybe_include_types(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `includeTypes` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_include_types(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestIncludeTypesItem>>,
            >,
        ) -> Self {
            self.include_types = value.map(|value| value.into());
            self
        }
        pub fn exclude_types(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestExcludeTypesItem>>,
        ) -> Self {
            self.maybe_exclude_types(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeTypes` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_types(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestExcludeTypesItem>>,
            >,
        ) -> Self {
            self.exclude_types = value.map(|value| value.into());
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
                mark_as_read: self.mark_as_read,
                include_types: self.include_types,
                exclude_types: self.exclude_types,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/notifications-grouped`"]
pub mod notifications_grouped {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestIncludeTypesItem {
        #[serde(rename = "note")]
        Note,
        #[serde(rename = "follow")]
        Follow,
        #[serde(rename = "mention")]
        Mention,
        #[serde(rename = "reply")]
        Reply,
        #[serde(rename = "renote")]
        Renote,
        #[serde(rename = "quote")]
        Quote,
        #[serde(rename = "reaction")]
        Reaction,
        #[serde(rename = "pollEnded")]
        PollEnded,
        #[serde(rename = "scheduledNotePosted")]
        ScheduledNotePosted,
        #[serde(rename = "scheduledNotePostFailed")]
        ScheduledNotePostFailed,
        #[serde(rename = "receiveFollowRequest")]
        ReceiveFollowRequest,
        #[serde(rename = "followRequestAccepted")]
        FollowRequestAccepted,
        #[serde(rename = "roleAssigned")]
        RoleAssigned,
        #[serde(rename = "chatRoomInvitationReceived")]
        ChatRoomInvitationReceived,
        #[serde(rename = "achievementEarned")]
        AchievementEarned,
        #[serde(rename = "exportCompleted")]
        ExportCompleted,
        #[serde(rename = "login")]
        Login,
        #[serde(rename = "createToken")]
        CreateToken,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "test")]
        Test,
        #[serde(rename = "pollVote")]
        PollVote,
        #[serde(rename = "groupInvited")]
        GroupInvited,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestExcludeTypesItem {
        #[serde(rename = "note")]
        Note,
        #[serde(rename = "follow")]
        Follow,
        #[serde(rename = "mention")]
        Mention,
        #[serde(rename = "reply")]
        Reply,
        #[serde(rename = "renote")]
        Renote,
        #[serde(rename = "quote")]
        Quote,
        #[serde(rename = "reaction")]
        Reaction,
        #[serde(rename = "pollEnded")]
        PollEnded,
        #[serde(rename = "scheduledNotePosted")]
        ScheduledNotePosted,
        #[serde(rename = "scheduledNotePostFailed")]
        ScheduledNotePostFailed,
        #[serde(rename = "receiveFollowRequest")]
        ReceiveFollowRequest,
        #[serde(rename = "followRequestAccepted")]
        FollowRequestAccepted,
        #[serde(rename = "roleAssigned")]
        RoleAssigned,
        #[serde(rename = "chatRoomInvitationReceived")]
        ChatRoomInvitationReceived,
        #[serde(rename = "achievementEarned")]
        AchievementEarned,
        #[serde(rename = "exportCompleted")]
        ExportCompleted,
        #[serde(rename = "login")]
        Login,
        #[serde(rename = "createToken")]
        CreateToken,
        #[serde(rename = "app")]
        App,
        #[serde(rename = "test")]
        Test,
        #[serde(rename = "pollVote")]
        PollVote,
        #[serde(rename = "groupInvited")]
        GroupInvited,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`i/notifications-grouped`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:notifications*"]
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
            rename = "markAsRead",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mark_as_read: ::std::option::Option<bool>,
        #[serde(
            rename = "includeTypes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_types: ::std::option::Option<::std::vec::Vec<RequestIncludeTypesItem>>,
        #[serde(
            rename = "excludeTypes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_types: ::std::option::Option<::std::vec::Vec<RequestExcludeTypesItem>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/notifications-grouped";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Notification>;
    #[doc = "`i/notifications-grouped` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:notifications*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        mark_as_read: ::std::option::Option<bool>,
        include_types: ::std::option::Option<::std::vec::Vec<RequestIncludeTypesItem>>,
        exclude_types: ::std::option::Option<::std::vec::Vec<RequestExcludeTypesItem>>,
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
                mark_as_read: ::std::option::Option::None,
                include_types: ::std::option::Option::None,
                exclude_types: ::std::option::Option::None,
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
        pub fn mark_as_read(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_mark_as_read(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `markAsRead` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_mark_as_read(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.mark_as_read = value.map(|value| value.into());
            self
        }
        pub fn include_types(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestIncludeTypesItem>>,
        ) -> Self {
            self.maybe_include_types(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `includeTypes` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_include_types(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestIncludeTypesItem>>,
            >,
        ) -> Self {
            self.include_types = value.map(|value| value.into());
            self
        }
        pub fn exclude_types(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestExcludeTypesItem>>,
        ) -> Self {
            self.maybe_exclude_types(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeTypes` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_types(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestExcludeTypesItem>>,
            >,
        ) -> Self {
            self.exclude_types = value.map(|value| value.into());
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
                mark_as_read: self.mark_as_read,
                include_types: self.include_types,
                exclude_types: self.exclude_types,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/page-likes`"]
pub mod page_likes {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        pub page: crate::generated::v2025_10_0::types::Page,
    }
    #[doc = "`i/page-likes`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:page-likes*"]
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
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/page-likes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`i/page-likes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:page-likes*"]
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
#[doc = "`i/pages`"]
pub mod pages {
    #[doc = "`i/pages`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:pages*"]
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
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/pages";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Page>;
    #[doc = "`i/pages` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:pages*"]
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
#[doc = "`i/pin`"]
pub mod pin {
    #[doc = "`i/pin`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/pin";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::MeDetailed;
    #[doc = "`i/pin` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                note_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                note_id: self
                    .note_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/read-announcement`"]
pub mod read_announcement {
    #[doc = "`i/read-announcement`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "announcementId")]
        pub announcement_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/read-announcement";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/read-announcement` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `announcement_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        announcement_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                announcement_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn announcement_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                announcement_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                announcement_id: self
                    .announcement_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/regenerate-token`"]
pub mod regenerate_token {
    #[doc = "`i/regenerate-token`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub password: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/regenerate-token";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/regenerate-token` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `password`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        password: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                password: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn password(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                password: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                password: self
                    .password
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/registry/` 以下のエンドポイント"]
pub mod registry {
    #[doc = "`i/registry/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`i/registry/get`"]
        pub fn get(&self) -> get::Call<'a> {
            get::Call::new(self.client)
        }
        #[doc = "`i/registry/get-all`"]
        pub fn get_all(&self) -> get_all::Call<'a> {
            get_all::Call::new(self.client)
        }
        #[doc = "`i/registry/get-detail`"]
        pub fn get_detail(&self) -> get_detail::Call<'a> {
            get_detail::Call::new(self.client)
        }
        #[doc = "`i/registry/keys`"]
        pub fn keys(&self) -> keys::Call<'a> {
            keys::Call::new(self.client)
        }
        #[doc = "`i/registry/keys-with-type`"]
        pub fn keys_with_type(&self) -> keys_with_type::Call<'a> {
            keys_with_type::Call::new(self.client)
        }
        #[doc = "`i/registry/remove`"]
        pub fn remove(&self) -> remove::Call<'a> {
            remove::Call::new(self.client)
        }
        #[doc = "`i/registry/scopes-with-domain`"]
        pub fn scopes_with_domain(&self) -> scopes_with_domain::Call<'a> {
            scopes_with_domain::Call::new(self.client)
        }
        #[doc = "`i/registry/set`"]
        pub fn set(&self) -> set::Call<'a> {
            set::Call::new(self.client)
        }
    }
    #[doc = "`i/registry/get`"]
    pub mod get {
        #[doc = "`i/registry/get`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub key: ::std::string::String,
            pub scope: ::std::vec::Vec<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/get";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        #[doc = "`i/registry/get` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `key`、`scope`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            key: ::std::option::Option<::std::string::String>,
            scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    key: ::std::option::Option::None,
                    scope: ::std::option::Option::None,
                    domain: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn domain(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_domain(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `domain` を設定し、`None` なら設定を消して送らない。`null` を送るには [`domain_null`](Self::domain_null) を使う。"]
            pub fn maybe_domain(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.domain = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`domain` を `null` として送る。"]
            pub fn domain_null(mut self) -> Self {
                self.domain = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn key(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    key: ::std::option::Option::Some(value.into()),
                    scope: self.scope,
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn scope(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    key: self.key,
                    scope: ::std::option::Option::Some(value.into()),
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    key: self
                        .key
                        .expect("required parameter is set by the type state"),
                    scope: self
                        .scope
                        .expect("required parameter is set by the type state"),
                    domain: self.domain,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/registry/get-all`"]
    pub mod get_all {
        #[doc = "`i/registry/get-all`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub scope: ::std::vec::Vec<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/get-all";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::serde_json::Map<::std::string::String, ::serde_json::Value>;
        #[doc = "`i/registry/get-all` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `scope`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    scope: ::std::option::Option::None,
                    domain: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn domain(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_domain(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `domain` を設定し、`None` なら設定を消して送らない。`null` を送るには [`domain_null`](Self::domain_null) を使う。"]
            pub fn maybe_domain(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.domain = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`domain` を `null` として送る。"]
            pub fn domain_null(mut self) -> Self {
                self.domain = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn scope(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    scope: ::std::option::Option::Some(value.into()),
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    scope: self
                        .scope
                        .expect("required parameter is set by the type state"),
                    domain: self.domain,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/registry/get-detail`"]
    pub mod get_detail {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            #[serde(rename = "updatedAt")]
            pub updated_at: ::std::string::String,
            pub value: ::serde_json::Value,
        }
        #[doc = "`i/registry/get-detail`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub key: ::std::string::String,
            pub scope: ::std::vec::Vec<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/get-detail";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`i/registry/get-detail` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `key`、`scope`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            key: ::std::option::Option<::std::string::String>,
            scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    key: ::std::option::Option::None,
                    scope: ::std::option::Option::None,
                    domain: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn domain(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_domain(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `domain` を設定し、`None` なら設定を消して送らない。`null` を送るには [`domain_null`](Self::domain_null) を使う。"]
            pub fn maybe_domain(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.domain = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`domain` を `null` として送る。"]
            pub fn domain_null(mut self) -> Self {
                self.domain = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn key(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    key: ::std::option::Option::Some(value.into()),
                    scope: self.scope,
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn scope(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    key: self.key,
                    scope: ::std::option::Option::Some(value.into()),
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    key: self
                        .key
                        .expect("required parameter is set by the type state"),
                    scope: self
                        .scope
                        .expect("required parameter is set by the type state"),
                    domain: self.domain,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/registry/keys`"]
    pub mod keys {
        #[doc = "`i/registry/keys`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub scope: ::std::vec::Vec<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/keys";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<::std::string::String>;
        #[doc = "`i/registry/keys` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `scope`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    scope: ::std::option::Option::None,
                    domain: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn domain(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_domain(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `domain` を設定し、`None` なら設定を消して送らない。`null` を送るには [`domain_null`](Self::domain_null) を使う。"]
            pub fn maybe_domain(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.domain = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`domain` を `null` として送る。"]
            pub fn domain_null(mut self) -> Self {
                self.domain = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn scope(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    scope: ::std::option::Option::Some(value.into()),
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    scope: self
                        .scope
                        .expect("required parameter is set by the type state"),
                    domain: self.domain,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/registry/keys-with-type`"]
    pub mod keys_with_type {
        #[doc = "`i/registry/keys-with-type`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub scope: ::std::vec::Vec<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/keys-with-type";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response =
            ::std::collections::HashMap<::std::string::String, ::std::string::String>;
        #[doc = "`i/registry/keys-with-type` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `scope`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    scope: ::std::option::Option::None,
                    domain: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn domain(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_domain(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `domain` を設定し、`None` なら設定を消して送らない。`null` を送るには [`domain_null`](Self::domain_null) を使う。"]
            pub fn maybe_domain(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.domain = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`domain` を `null` として送る。"]
            pub fn domain_null(mut self) -> Self {
                self.domain = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn scope(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    scope: ::std::option::Option::Some(value.into()),
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    scope: self
                        .scope
                        .expect("required parameter is set by the type state"),
                    domain: self.domain,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/registry/remove`"]
    pub mod remove {
        #[doc = "`i/registry/remove`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub key: ::std::string::String,
            pub scope: ::std::vec::Vec<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/remove";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/registry/remove` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `key`、`scope`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            key: ::std::option::Option<::std::string::String>,
            scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    key: ::std::option::Option::None,
                    scope: ::std::option::Option::None,
                    domain: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn domain(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_domain(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `domain` を設定し、`None` なら設定を消して送らない。`null` を送るには [`domain_null`](Self::domain_null) を使う。"]
            pub fn maybe_domain(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.domain = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`domain` を `null` として送る。"]
            pub fn domain_null(mut self) -> Self {
                self.domain = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn key(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    key: ::std::option::Option::Some(value.into()),
                    scope: self.scope,
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn scope(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    key: self.key,
                    scope: ::std::option::Option::Some(value.into()),
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    key: self
                        .key
                        .expect("required parameter is set by the type state"),
                    scope: self
                        .scope
                        .expect("required parameter is set by the type state"),
                    domain: self.domain,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/registry/scopes-with-domain`"]
    pub mod scopes_with_domain {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct ResponseItem {
            pub scopes: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
            pub domain: ::std::option::Option<::std::string::String>,
        }
        #[doc = "`i/registry/scopes-with-domain`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/scopes-with-domain";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<ResponseItem>;
        #[doc = "`i/registry/scopes-with-domain` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
    #[doc = "`i/registry/set`"]
    pub mod set {
        #[doc = "`i/registry/set`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub key: ::std::string::String,
            pub value: ::serde_json::Value,
            pub scope: ::std::vec::Vec<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/registry/set";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/registry/set` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `key`、`value`、`scope`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2025_10_0::Client,
            key: ::std::option::Option<::std::string::String>,
            value: ::std::option::Option<::serde_json::Value>,
            scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            domain: ::std::option::Option<::std::option::Option<::std::string::String>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    key: ::std::option::Option::None,
                    value: ::std::option::Option::None,
                    scope: ::std::option::Option::None,
                    domain: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
            pub fn domain(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_domain(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `domain` を設定し、`None` なら設定を消して送らない。`null` を送るには [`domain_null`](Self::domain_null) を使う。"]
            pub fn maybe_domain(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.domain = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`domain` を `null` として送る。"]
            pub fn domain_null(mut self) -> Self {
                self.domain = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn key(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1, S2)> {
                Call {
                    client: self.client,
                    key: ::std::option::Option::Some(value.into()),
                    value: self.value,
                    scope: self.scope,
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn value(
                self,
                value: impl ::std::convert::Into<::serde_json::Value>,
            ) -> Call<'a, (S0, crate::builder::Set, S2)> {
                Call {
                    client: self.client,
                    key: self.key,
                    value: ::std::option::Option::Some(value.into()),
                    scope: self.scope,
                    domain: self.domain,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn scope(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Call<'a, (S0, S1, crate::builder::Set)> {
                Call {
                    client: self.client,
                    key: self.key,
                    value: self.value,
                    scope: ::std::option::Option::Some(value.into()),
                    domain: self.domain,
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
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    key: self
                        .key
                        .expect("required parameter is set by the type state"),
                    value: self
                        .value
                        .expect("required parameter is set by the type state"),
                    scope: self
                        .scope
                        .expect("required parameter is set by the type state"),
                    domain: self.domain,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`i/revoke-token`"]
pub mod revoke_token {
    #[doc = "`i/revoke-token`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須: `tokenId` / `token` のいずれか。"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "tokenId", default, skip_serializing_if = "Option::is_none")]
        pub token_id: ::std::option::Option<::std::string::String>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/revoke-token";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`i/revoke-token` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        token_id: ::std::option::Option<::std::string::String>,
        token: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                token_id: ::std::option::Option::None,
                token: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn token_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_token_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `tokenId` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_token_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.token_id = value.map(|value| value.into());
            self
        }
        pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_token(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
        pub fn maybe_token(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.token = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`token` を `null` として送る。"]
        pub fn token_null(mut self) -> Self {
            self.token = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                token_id: self.token_id,
                token: self.token,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/signin-history`"]
pub mod signin_history {
    #[doc = "`i/signin-history`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/signin-history";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Signin>;
    #[doc = "`i/signin-history` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
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
#[doc = "`i/unpin`"]
pub mod unpin {
    #[doc = "`i/unpin`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/unpin";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::MeDetailed;
    #[doc = "`i/unpin` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                note_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                note_id: self
                    .note_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/update`"]
pub mod update {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestLang {
        #[serde(rename = "ach")]
        Ach,
        #[serde(rename = "ady")]
        Ady,
        #[serde(rename = "af")]
        Af,
        #[serde(rename = "af-NA")]
        AfNa,
        #[serde(rename = "af-ZA")]
        AfZa,
        #[serde(rename = "ak")]
        Ak,
        #[serde(rename = "ar")]
        Ar,
        #[serde(rename = "ar-AR")]
        ArAr,
        #[serde(rename = "ar-MA")]
        ArMa,
        #[serde(rename = "ar-SA")]
        ArSa,
        #[serde(rename = "ay-BO")]
        AyBo,
        #[serde(rename = "az")]
        Az,
        #[serde(rename = "az-AZ")]
        AzAz,
        #[serde(rename = "be-BY")]
        BeBy,
        #[serde(rename = "bg")]
        Bg,
        #[serde(rename = "bg-BG")]
        BgBg,
        #[serde(rename = "bn")]
        Bn,
        #[serde(rename = "bn-IN")]
        BnIn,
        #[serde(rename = "bn-BD")]
        BnBd,
        #[serde(rename = "br")]
        Br,
        #[serde(rename = "bs-BA")]
        BsBa,
        #[serde(rename = "ca")]
        Ca,
        #[serde(rename = "ca-ES")]
        CaEs,
        #[serde(rename = "cak")]
        Cak,
        #[serde(rename = "ck-US")]
        CkUs,
        #[serde(rename = "cs")]
        Cs,
        #[serde(rename = "cs-CZ")]
        CsCz,
        #[serde(rename = "cy")]
        Cy,
        #[serde(rename = "cy-GB")]
        CyGb,
        #[serde(rename = "da")]
        Da,
        #[serde(rename = "da-DK")]
        DaDk,
        #[serde(rename = "de")]
        De,
        #[serde(rename = "de-AT")]
        DeAt,
        #[serde(rename = "de-DE")]
        DeDe,
        #[serde(rename = "de-CH")]
        DeCh,
        #[serde(rename = "dsb")]
        Dsb,
        #[serde(rename = "el")]
        El,
        #[serde(rename = "el-GR")]
        ElGr,
        #[serde(rename = "en")]
        En,
        #[serde(rename = "en-GB")]
        EnGb,
        #[serde(rename = "en-AU")]
        EnAu,
        #[serde(rename = "en-CA")]
        EnCa,
        #[serde(rename = "en-IE")]
        EnIe,
        #[serde(rename = "en-IN")]
        EnIn,
        #[serde(rename = "en-PI")]
        EnPi,
        #[serde(rename = "en-SG")]
        EnSg,
        #[serde(rename = "en-UD")]
        EnUd,
        #[serde(rename = "en-US")]
        EnUs,
        #[serde(rename = "en-ZA")]
        EnZa,
        #[serde(rename = "en@pirate")]
        EnPirate,
        #[serde(rename = "eo")]
        Eo,
        #[serde(rename = "eo-EO")]
        EoEo,
        #[serde(rename = "es")]
        Es,
        #[serde(rename = "es-AR")]
        EsAr,
        #[serde(rename = "es-419")]
        Es419,
        #[serde(rename = "es-CL")]
        EsCl,
        #[serde(rename = "es-CO")]
        EsCo,
        #[serde(rename = "es-EC")]
        EsEc,
        #[serde(rename = "es-ES")]
        EsEs,
        #[serde(rename = "es-LA")]
        EsLa,
        #[serde(rename = "es-NI")]
        EsNi,
        #[serde(rename = "es-MX")]
        EsMx,
        #[serde(rename = "es-US")]
        EsUs,
        #[serde(rename = "es-VE")]
        EsVe,
        #[serde(rename = "et")]
        Et,
        #[serde(rename = "et-EE")]
        EtEe,
        #[serde(rename = "eu")]
        Eu,
        #[serde(rename = "eu-ES")]
        EuEs,
        #[serde(rename = "fa")]
        Fa,
        #[serde(rename = "fa-IR")]
        FaIr,
        #[serde(rename = "fb-LT")]
        FbLt,
        #[serde(rename = "ff")]
        Ff,
        #[serde(rename = "fi")]
        Fi,
        #[serde(rename = "fi-FI")]
        FiFi,
        #[serde(rename = "fo")]
        Fo,
        #[serde(rename = "fo-FO")]
        FoFo,
        #[serde(rename = "fr")]
        Fr,
        #[serde(rename = "fr-CA")]
        FrCa,
        #[serde(rename = "fr-FR")]
        FrFr,
        #[serde(rename = "fr-BE")]
        FrBe,
        #[serde(rename = "fr-CH")]
        FrCh,
        #[serde(rename = "fy-NL")]
        FyNl,
        #[serde(rename = "ga")]
        Ga,
        #[serde(rename = "ga-IE")]
        GaIe,
        #[serde(rename = "gd")]
        Gd,
        #[serde(rename = "gl")]
        Gl,
        #[serde(rename = "gl-ES")]
        GlEs,
        #[serde(rename = "gn-PY")]
        GnPy,
        #[serde(rename = "gu-IN")]
        GuIn,
        #[serde(rename = "gv")]
        Gv,
        #[serde(rename = "gx-GR")]
        GxGr,
        #[serde(rename = "he")]
        He,
        #[serde(rename = "he-IL")]
        HeIl,
        #[serde(rename = "hi")]
        Hi,
        #[serde(rename = "hi-IN")]
        HiIn,
        #[serde(rename = "hr")]
        Hr,
        #[serde(rename = "hr-HR")]
        HrHr,
        #[serde(rename = "hsb")]
        Hsb,
        #[serde(rename = "ht")]
        Ht,
        #[serde(rename = "hu")]
        Hu,
        #[serde(rename = "hu-HU")]
        HuHu,
        #[serde(rename = "hy")]
        Hy,
        #[serde(rename = "hy-AM")]
        HyAm,
        #[serde(rename = "id")]
        Id,
        #[serde(rename = "id-ID")]
        IdId,
        #[serde(rename = "is")]
        Is,
        #[serde(rename = "is-IS")]
        IsIs,
        #[serde(rename = "it")]
        It,
        #[serde(rename = "it-IT")]
        ItIt,
        #[serde(rename = "ja")]
        Ja,
        #[serde(rename = "ja-JP")]
        JaJp,
        #[serde(rename = "jv-ID")]
        JvId,
        #[serde(rename = "ka-GE")]
        KaGe,
        #[serde(rename = "kk-KZ")]
        KkKz,
        #[serde(rename = "km")]
        Km,
        #[serde(rename = "kl")]
        Kl,
        #[serde(rename = "km-KH")]
        KmKh,
        #[serde(rename = "kab")]
        Kab,
        #[serde(rename = "kn")]
        Kn,
        #[serde(rename = "kn-IN")]
        KnIn,
        #[serde(rename = "ko")]
        Ko,
        #[serde(rename = "ko-KR")]
        KoKr,
        #[serde(rename = "ku-TR")]
        KuTr,
        #[serde(rename = "kw")]
        Kw,
        #[serde(rename = "la")]
        La,
        #[serde(rename = "la-VA")]
        LaVa,
        #[serde(rename = "lb")]
        Lb,
        #[serde(rename = "li-NL")]
        LiNl,
        #[serde(rename = "lt")]
        Lt,
        #[serde(rename = "lt-LT")]
        LtLt,
        #[serde(rename = "lv")]
        Lv,
        #[serde(rename = "lv-LV")]
        LvLv,
        #[serde(rename = "mai")]
        Mai,
        #[serde(rename = "mg-MG")]
        MgMg,
        #[serde(rename = "mk")]
        Mk,
        #[serde(rename = "mk-MK")]
        MkMk,
        #[serde(rename = "ml")]
        Ml,
        #[serde(rename = "ml-IN")]
        MlIn,
        #[serde(rename = "mn-MN")]
        MnMn,
        #[serde(rename = "mr")]
        Mr,
        #[serde(rename = "mr-IN")]
        MrIn,
        #[serde(rename = "ms")]
        Ms,
        #[serde(rename = "ms-MY")]
        MsMy,
        #[serde(rename = "mt")]
        Mt,
        #[serde(rename = "mt-MT")]
        MtMt,
        #[serde(rename = "my")]
        My,
        #[serde(rename = "no")]
        No,
        #[serde(rename = "nb")]
        Nb,
        #[serde(rename = "nb-NO")]
        NbNo,
        #[serde(rename = "ne")]
        Ne,
        #[serde(rename = "ne-NP")]
        NeNp,
        #[serde(rename = "nl")]
        Nl,
        #[serde(rename = "nl-BE")]
        NlBe,
        #[serde(rename = "nl-NL")]
        NlNl,
        #[serde(rename = "nn-NO")]
        NnNo,
        #[serde(rename = "oc")]
        Oc,
        #[serde(rename = "or-IN")]
        OrIn,
        #[serde(rename = "pa")]
        Pa,
        #[serde(rename = "pa-IN")]
        PaIn,
        #[serde(rename = "pl")]
        Pl,
        #[serde(rename = "pl-PL")]
        PlPl,
        #[serde(rename = "ps-AF")]
        PsAf,
        #[serde(rename = "pt")]
        Pt,
        #[serde(rename = "pt-BR")]
        PtBr,
        #[serde(rename = "pt-PT")]
        PtPt,
        #[serde(rename = "qu-PE")]
        QuPe,
        #[serde(rename = "rm-CH")]
        RmCh,
        #[serde(rename = "ro")]
        Ro,
        #[serde(rename = "ro-RO")]
        RoRo,
        #[serde(rename = "ru")]
        Ru,
        #[serde(rename = "ru-RU")]
        RuRu,
        #[serde(rename = "sa-IN")]
        SaIn,
        #[serde(rename = "se-NO")]
        SeNo,
        #[serde(rename = "sh")]
        Sh,
        #[serde(rename = "si-LK")]
        SiLk,
        #[serde(rename = "sk")]
        Sk,
        #[serde(rename = "sk-SK")]
        SkSk,
        #[serde(rename = "sl")]
        Sl,
        #[serde(rename = "sl-SI")]
        SlSi,
        #[serde(rename = "so-SO")]
        SoSo,
        #[serde(rename = "sq")]
        Sq,
        #[serde(rename = "sq-AL")]
        SqAl,
        #[serde(rename = "sr")]
        Sr,
        #[serde(rename = "sr-RS")]
        SrRs,
        #[serde(rename = "su")]
        Su,
        #[serde(rename = "sv")]
        Sv,
        #[serde(rename = "sv-SE")]
        SvSe,
        #[serde(rename = "sw")]
        Sw,
        #[serde(rename = "sw-KE")]
        SwKe,
        #[serde(rename = "ta")]
        Ta,
        #[serde(rename = "ta-IN")]
        TaIn,
        #[serde(rename = "te")]
        Te,
        #[serde(rename = "te-IN")]
        TeIn,
        #[serde(rename = "tg")]
        Tg,
        #[serde(rename = "tg-TJ")]
        TgTj,
        #[serde(rename = "th")]
        Th,
        #[serde(rename = "th-TH")]
        ThTh,
        #[serde(rename = "fil")]
        Fil,
        #[serde(rename = "tlh")]
        Tlh,
        #[serde(rename = "tr")]
        Tr,
        #[serde(rename = "tr-TR")]
        TrTr,
        #[serde(rename = "tt-RU")]
        TtRu,
        #[serde(rename = "uk")]
        Uk,
        #[serde(rename = "uk-UA")]
        UkUa,
        #[serde(rename = "ur")]
        Ur,
        #[serde(rename = "ur-PK")]
        UrPk,
        #[serde(rename = "uz")]
        Uz,
        #[serde(rename = "uz-UZ")]
        UzUz,
        #[serde(rename = "vi")]
        Vi,
        #[serde(rename = "vi-VN")]
        ViVn,
        #[serde(rename = "xh-ZA")]
        XhZa,
        #[serde(rename = "yi")]
        Yi,
        #[serde(rename = "yi-DE")]
        YiDe,
        #[serde(rename = "zh")]
        Zh,
        #[serde(rename = "zh-Hans")]
        ZhHans,
        #[serde(rename = "zh-Hant")]
        ZhHant,
        #[serde(rename = "zh-CN")]
        ZhCn,
        #[serde(rename = "zh-HK")]
        ZhHk,
        #[serde(rename = "zh-SG")]
        ZhSg,
        #[serde(rename = "zh-TW")]
        ZhTw,
        #[serde(rename = "zu-ZA")]
        ZuZa,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestAvatarDecorationsItem {
        pub id: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub angle: ::std::option::Option<::std::option::Option<f64>>,
        #[serde(
            rename = "flipH",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub flip_h: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            rename = "offsetX",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset_x: ::std::option::Option<::std::option::Option<f64>>,
        #[serde(
            rename = "offsetY",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub offset_y: ::std::option::Option<::std::option::Option<f64>>,
    }
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestFieldsItem {
        pub name: ::std::string::String,
        pub value: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestFollowingVisibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "followers")]
        Followers,
        #[serde(rename = "private")]
        Private,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestFollowersVisibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "followers")]
        Followers,
        #[serde(rename = "private")]
        Private,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestChatScope {
        #[serde(rename = "everyone")]
        Everyone,
        #[serde(rename = "followers")]
        Followers,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "mutual")]
        Mutual,
        #[serde(rename = "none")]
        None,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestMutedWordsItem {
        Array(::std::vec::Vec<::std::string::String>),
        String(::std::string::String),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestHardMutedWordsItem {
        Array(::std::vec::Vec<::std::string::String>),
        String(::std::string::String),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigNoteListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigNoteList {
        pub r#type: RequestNotificationRecieveConfigNoteListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigNoteObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigNoteObject {
        pub r#type: RequestNotificationRecieveConfigNoteObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigNote {
        List(RequestNotificationRecieveConfigNoteList),
        Object(RequestNotificationRecieveConfigNoteObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigFollowListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigFollowList {
        pub r#type: RequestNotificationRecieveConfigFollowListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigFollowObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigFollowObject {
        pub r#type: RequestNotificationRecieveConfigFollowObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigFollow {
        List(RequestNotificationRecieveConfigFollowList),
        Object(RequestNotificationRecieveConfigFollowObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigMentionListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigMentionList {
        pub r#type: RequestNotificationRecieveConfigMentionListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigMentionObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigMentionObject {
        pub r#type: RequestNotificationRecieveConfigMentionObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigMention {
        List(RequestNotificationRecieveConfigMentionList),
        Object(RequestNotificationRecieveConfigMentionObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigReplyListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigReplyList {
        pub r#type: RequestNotificationRecieveConfigReplyListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigReplyObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigReplyObject {
        pub r#type: RequestNotificationRecieveConfigReplyObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigReply {
        List(RequestNotificationRecieveConfigReplyList),
        Object(RequestNotificationRecieveConfigReplyObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigRenoteListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigRenoteList {
        pub r#type: RequestNotificationRecieveConfigRenoteListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigRenoteObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigRenoteObject {
        pub r#type: RequestNotificationRecieveConfigRenoteObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigRenote {
        List(RequestNotificationRecieveConfigRenoteList),
        Object(RequestNotificationRecieveConfigRenoteObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigQuoteListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigQuoteList {
        pub r#type: RequestNotificationRecieveConfigQuoteListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigQuoteObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigQuoteObject {
        pub r#type: RequestNotificationRecieveConfigQuoteObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigQuote {
        List(RequestNotificationRecieveConfigQuoteList),
        Object(RequestNotificationRecieveConfigQuoteObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigReactionListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigReactionList {
        pub r#type: RequestNotificationRecieveConfigReactionListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigReactionObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigReactionObject {
        pub r#type: RequestNotificationRecieveConfigReactionObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigReaction {
        List(RequestNotificationRecieveConfigReactionList),
        Object(RequestNotificationRecieveConfigReactionObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigPollEndedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigPollEndedList {
        pub r#type: RequestNotificationRecieveConfigPollEndedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigPollEndedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigPollEndedObject {
        pub r#type: RequestNotificationRecieveConfigPollEndedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigPollEnded {
        List(RequestNotificationRecieveConfigPollEndedList),
        Object(RequestNotificationRecieveConfigPollEndedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigScheduledNotePostedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigScheduledNotePostedList {
        pub r#type: RequestNotificationRecieveConfigScheduledNotePostedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigScheduledNotePostedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigScheduledNotePostedObject {
        pub r#type: RequestNotificationRecieveConfigScheduledNotePostedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigScheduledNotePosted {
        List(RequestNotificationRecieveConfigScheduledNotePostedList),
        Object(RequestNotificationRecieveConfigScheduledNotePostedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigScheduledNotePostFailedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigScheduledNotePostFailedList {
        pub r#type: RequestNotificationRecieveConfigScheduledNotePostFailedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigScheduledNotePostFailedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigScheduledNotePostFailedObject {
        pub r#type: RequestNotificationRecieveConfigScheduledNotePostFailedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigScheduledNotePostFailed {
        List(RequestNotificationRecieveConfigScheduledNotePostFailedList),
        Object(RequestNotificationRecieveConfigScheduledNotePostFailedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigReceiveFollowRequestListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigReceiveFollowRequestList {
        pub r#type: RequestNotificationRecieveConfigReceiveFollowRequestListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigReceiveFollowRequestObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigReceiveFollowRequestObject {
        pub r#type: RequestNotificationRecieveConfigReceiveFollowRequestObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigReceiveFollowRequest {
        List(RequestNotificationRecieveConfigReceiveFollowRequestList),
        Object(RequestNotificationRecieveConfigReceiveFollowRequestObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigFollowRequestAcceptedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigFollowRequestAcceptedList {
        pub r#type: RequestNotificationRecieveConfigFollowRequestAcceptedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigFollowRequestAcceptedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigFollowRequestAcceptedObject {
        pub r#type: RequestNotificationRecieveConfigFollowRequestAcceptedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigFollowRequestAccepted {
        List(RequestNotificationRecieveConfigFollowRequestAcceptedList),
        Object(RequestNotificationRecieveConfigFollowRequestAcceptedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigRoleAssignedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigRoleAssignedList {
        pub r#type: RequestNotificationRecieveConfigRoleAssignedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigRoleAssignedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigRoleAssignedObject {
        pub r#type: RequestNotificationRecieveConfigRoleAssignedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigRoleAssigned {
        List(RequestNotificationRecieveConfigRoleAssignedList),
        Object(RequestNotificationRecieveConfigRoleAssignedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigChatRoomInvitationReceivedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigChatRoomInvitationReceivedList {
        pub r#type: RequestNotificationRecieveConfigChatRoomInvitationReceivedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigChatRoomInvitationReceivedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigChatRoomInvitationReceivedObject {
        pub r#type: RequestNotificationRecieveConfigChatRoomInvitationReceivedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigChatRoomInvitationReceived {
        List(RequestNotificationRecieveConfigChatRoomInvitationReceivedList),
        Object(RequestNotificationRecieveConfigChatRoomInvitationReceivedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigAchievementEarnedListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigAchievementEarnedList {
        pub r#type: RequestNotificationRecieveConfigAchievementEarnedListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigAchievementEarnedObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigAchievementEarnedObject {
        pub r#type: RequestNotificationRecieveConfigAchievementEarnedObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigAchievementEarned {
        List(RequestNotificationRecieveConfigAchievementEarnedList),
        Object(RequestNotificationRecieveConfigAchievementEarnedObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigAppListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigAppList {
        pub r#type: RequestNotificationRecieveConfigAppListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigAppObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigAppObject {
        pub r#type: RequestNotificationRecieveConfigAppObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigApp {
        List(RequestNotificationRecieveConfigAppList),
        Object(RequestNotificationRecieveConfigAppObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigTestListType {
        #[serde(rename = "list")]
        List,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigTestList {
        pub r#type: RequestNotificationRecieveConfigTestListType,
        #[serde(rename = "userListId")]
        pub user_list_id: ::std::string::String,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestNotificationRecieveConfigTestObjectType {
        #[serde(rename = "all")]
        All,
        #[serde(rename = "following")]
        Following,
        #[serde(rename = "follower")]
        Follower,
        #[serde(rename = "mutualFollow")]
        MutualFollow,
        #[serde(rename = "followingOrFollower")]
        FollowingOrFollower,
        #[serde(rename = "never")]
        Never,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfigTestObject {
        pub r#type: RequestNotificationRecieveConfigTestObjectType,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    #[serde(untagged)]
    pub enum RequestNotificationRecieveConfigTest {
        List(RequestNotificationRecieveConfigTestList),
        Object(RequestNotificationRecieveConfigTestObject),
        #[doc = "このバージョンのバインディングが知らない形。"]
        Unrecognized(::serde_json::Value),
    }
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestNotificationRecieveConfig {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub note: ::std::option::Option<RequestNotificationRecieveConfigNote>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub follow: ::std::option::Option<RequestNotificationRecieveConfigFollow>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub mention: ::std::option::Option<RequestNotificationRecieveConfigMention>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reply: ::std::option::Option<RequestNotificationRecieveConfigReply>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub renote: ::std::option::Option<RequestNotificationRecieveConfigRenote>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub quote: ::std::option::Option<RequestNotificationRecieveConfigQuote>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub reaction: ::std::option::Option<RequestNotificationRecieveConfigReaction>,
        #[serde(rename = "pollEnded", default, skip_serializing_if = "Option::is_none")]
        pub poll_ended: ::std::option::Option<RequestNotificationRecieveConfigPollEnded>,
        #[serde(
            rename = "scheduledNotePosted",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scheduled_note_posted:
            ::std::option::Option<RequestNotificationRecieveConfigScheduledNotePosted>,
        #[serde(
            rename = "scheduledNotePostFailed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scheduled_note_post_failed:
            ::std::option::Option<RequestNotificationRecieveConfigScheduledNotePostFailed>,
        #[serde(
            rename = "receiveFollowRequest",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub receive_follow_request:
            ::std::option::Option<RequestNotificationRecieveConfigReceiveFollowRequest>,
        #[serde(
            rename = "followRequestAccepted",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub follow_request_accepted:
            ::std::option::Option<RequestNotificationRecieveConfigFollowRequestAccepted>,
        #[serde(
            rename = "roleAssigned",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub role_assigned: ::std::option::Option<RequestNotificationRecieveConfigRoleAssigned>,
        #[serde(
            rename = "chatRoomInvitationReceived",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub chat_room_invitation_received:
            ::std::option::Option<RequestNotificationRecieveConfigChatRoomInvitationReceived>,
        #[serde(
            rename = "achievementEarned",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub achievement_earned:
            ::std::option::Option<RequestNotificationRecieveConfigAchievementEarned>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub app: ::std::option::Option<RequestNotificationRecieveConfigApp>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub test: ::std::option::Option<RequestNotificationRecieveConfigTest>,
    }
    #[doc = "`i/update`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
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
        pub description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "followedMessage",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub followed_message: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub location: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub birthday: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub lang: ::std::option::Option<::std::option::Option<RequestLang>>,
        #[serde(
            rename = "avatarId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub avatar_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "avatarDecorations",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub avatar_decorations:
            ::std::option::Option<::std::vec::Vec<RequestAvatarDecorationsItem>>,
        #[serde(
            rename = "bannerId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub banner_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub fields: ::std::option::Option<::std::vec::Vec<RequestFieldsItem>>,
        #[serde(rename = "isLocked", default, skip_serializing_if = "Option::is_none")]
        pub is_locked: ::std::option::Option<bool>,
        #[serde(
            rename = "isExplorable",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub is_explorable: ::std::option::Option<bool>,
        #[serde(
            rename = "hideOnlineStatus",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub hide_online_status: ::std::option::Option<bool>,
        #[serde(
            rename = "publicReactions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub public_reactions: ::std::option::Option<bool>,
        #[serde(
            rename = "carefulBot",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub careful_bot: ::std::option::Option<bool>,
        #[serde(
            rename = "autoAcceptFollowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auto_accept_followed: ::std::option::Option<bool>,
        #[serde(rename = "noCrawle", default, skip_serializing_if = "Option::is_none")]
        pub no_crawle: ::std::option::Option<bool>,
        #[serde(
            rename = "preventAiLearning",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub prevent_ai_learning: ::std::option::Option<bool>,
        #[serde(
            rename = "requireSigninToViewContents",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub require_signin_to_view_contents: ::std::option::Option<bool>,
        #[serde(
            rename = "makeNotesFollowersOnlyBefore",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub make_notes_followers_only_before: ::std::option::Option<::std::option::Option<i64>>,
        #[serde(
            rename = "makeNotesHiddenBefore",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub make_notes_hidden_before: ::std::option::Option<::std::option::Option<i64>>,
        #[serde(rename = "isBot", default, skip_serializing_if = "Option::is_none")]
        pub is_bot: ::std::option::Option<bool>,
        #[serde(rename = "isCat", default, skip_serializing_if = "Option::is_none")]
        pub is_cat: ::std::option::Option<bool>,
        #[serde(
            rename = "injectFeaturedNote",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub inject_featured_note: ::std::option::Option<bool>,
        #[serde(
            rename = "receiveAnnouncementEmail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub receive_announcement_email: ::std::option::Option<bool>,
        #[serde(
            rename = "alwaysMarkNsfw",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub always_mark_nsfw: ::std::option::Option<bool>,
        #[serde(
            rename = "autoSensitive",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auto_sensitive: ::std::option::Option<bool>,
        #[serde(
            rename = "followingVisibility",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub following_visibility: ::std::option::Option<RequestFollowingVisibility>,
        #[serde(
            rename = "followersVisibility",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub followers_visibility: ::std::option::Option<RequestFollowersVisibility>,
        #[serde(rename = "chatScope", default, skip_serializing_if = "Option::is_none")]
        pub chat_scope: ::std::option::Option<RequestChatScope>,
        #[serde(
            rename = "pinnedPageId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub pinned_page_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "mutedWords",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub muted_words: ::std::option::Option<::std::vec::Vec<RequestMutedWordsItem>>,
        #[serde(
            rename = "hardMutedWords",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub hard_muted_words: ::std::option::Option<::std::vec::Vec<RequestHardMutedWordsItem>>,
        #[serde(
            rename = "mutedInstances",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub muted_instances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "notificationRecieveConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub notification_recieve_config: ::std::option::Option<RequestNotificationRecieveConfig>,
        #[serde(
            rename = "emailNotificationTypes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub email_notification_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "alsoKnownAs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/update";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::MeDetailed;
    #[doc = "`i/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        name: ::std::option::Option<::std::option::Option<::std::string::String>>,
        description: ::std::option::Option<::std::option::Option<::std::string::String>>,
        followed_message: ::std::option::Option<::std::option::Option<::std::string::String>>,
        location: ::std::option::Option<::std::option::Option<::std::string::String>>,
        birthday: ::std::option::Option<::std::option::Option<::std::string::String>>,
        lang: ::std::option::Option<::std::option::Option<RequestLang>>,
        avatar_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        avatar_decorations: ::std::option::Option<::std::vec::Vec<RequestAvatarDecorationsItem>>,
        banner_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        fields: ::std::option::Option<::std::vec::Vec<RequestFieldsItem>>,
        is_locked: ::std::option::Option<bool>,
        is_explorable: ::std::option::Option<bool>,
        hide_online_status: ::std::option::Option<bool>,
        public_reactions: ::std::option::Option<bool>,
        careful_bot: ::std::option::Option<bool>,
        auto_accept_followed: ::std::option::Option<bool>,
        no_crawle: ::std::option::Option<bool>,
        prevent_ai_learning: ::std::option::Option<bool>,
        require_signin_to_view_contents: ::std::option::Option<bool>,
        make_notes_followers_only_before: ::std::option::Option<::std::option::Option<i64>>,
        make_notes_hidden_before: ::std::option::Option<::std::option::Option<i64>>,
        is_bot: ::std::option::Option<bool>,
        is_cat: ::std::option::Option<bool>,
        inject_featured_note: ::std::option::Option<bool>,
        receive_announcement_email: ::std::option::Option<bool>,
        always_mark_nsfw: ::std::option::Option<bool>,
        auto_sensitive: ::std::option::Option<bool>,
        following_visibility: ::std::option::Option<RequestFollowingVisibility>,
        followers_visibility: ::std::option::Option<RequestFollowersVisibility>,
        chat_scope: ::std::option::Option<RequestChatScope>,
        pinned_page_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        muted_words: ::std::option::Option<::std::vec::Vec<RequestMutedWordsItem>>,
        hard_muted_words: ::std::option::Option<::std::vec::Vec<RequestHardMutedWordsItem>>,
        muted_instances: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        notification_recieve_config: ::std::option::Option<RequestNotificationRecieveConfig>,
        email_notification_types: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                name: ::std::option::Option::None,
                description: ::std::option::Option::None,
                followed_message: ::std::option::Option::None,
                location: ::std::option::Option::None,
                birthday: ::std::option::Option::None,
                lang: ::std::option::Option::None,
                avatar_id: ::std::option::Option::None,
                avatar_decorations: ::std::option::Option::None,
                banner_id: ::std::option::Option::None,
                fields: ::std::option::Option::None,
                is_locked: ::std::option::Option::None,
                is_explorable: ::std::option::Option::None,
                hide_online_status: ::std::option::Option::None,
                public_reactions: ::std::option::Option::None,
                careful_bot: ::std::option::Option::None,
                auto_accept_followed: ::std::option::Option::None,
                no_crawle: ::std::option::Option::None,
                prevent_ai_learning: ::std::option::Option::None,
                require_signin_to_view_contents: ::std::option::Option::None,
                make_notes_followers_only_before: ::std::option::Option::None,
                make_notes_hidden_before: ::std::option::Option::None,
                is_bot: ::std::option::Option::None,
                is_cat: ::std::option::Option::None,
                inject_featured_note: ::std::option::Option::None,
                receive_announcement_email: ::std::option::Option::None,
                always_mark_nsfw: ::std::option::Option::None,
                auto_sensitive: ::std::option::Option::None,
                following_visibility: ::std::option::Option::None,
                followers_visibility: ::std::option::Option::None,
                chat_scope: ::std::option::Option::None,
                pinned_page_id: ::std::option::Option::None,
                muted_words: ::std::option::Option::None,
                hard_muted_words: ::std::option::Option::None,
                muted_instances: ::std::option::Option::None,
                notification_recieve_config: ::std::option::Option::None,
                email_notification_types: ::std::option::Option::None,
                also_known_as: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
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
        pub fn followed_message(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.maybe_followed_message(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `followedMessage` を設定し、`None` なら設定を消して送らない。`null` を送るには [`followed_message_null`](Self::followed_message_null) を使う。"]
        pub fn maybe_followed_message(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.followed_message = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`followedMessage` を `null` として送る。"]
        pub fn followed_message_null(mut self) -> Self {
            self.followed_message = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn location(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_location(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `location` を設定し、`None` なら設定を消して送らない。`null` を送るには [`location_null`](Self::location_null) を使う。"]
        pub fn maybe_location(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.location = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`location` を `null` として送る。"]
        pub fn location_null(mut self) -> Self {
            self.location = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn birthday(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_birthday(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `birthday` を設定し、`None` なら設定を消して送らない。`null` を送るには [`birthday_null`](Self::birthday_null) を使う。"]
        pub fn maybe_birthday(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.birthday = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`birthday` を `null` として送る。"]
        pub fn birthday_null(mut self) -> Self {
            self.birthday = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn lang(self, value: impl ::std::convert::Into<RequestLang>) -> Self {
            self.maybe_lang(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `lang` を設定し、`None` なら設定を消して送らない。`null` を送るには [`lang_null`](Self::lang_null) を使う。"]
        pub fn maybe_lang(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestLang>>,
        ) -> Self {
            self.lang = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`lang` を `null` として送る。"]
        pub fn lang_null(mut self) -> Self {
            self.lang = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn avatar_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_avatar_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `avatarId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`avatar_id_null`](Self::avatar_id_null) を使う。"]
        pub fn maybe_avatar_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.avatar_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`avatarId` を `null` として送る。"]
        pub fn avatar_id_null(mut self) -> Self {
            self.avatar_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn avatar_decorations(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestAvatarDecorationsItem>>,
        ) -> Self {
            self.maybe_avatar_decorations(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `avatarDecorations` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_avatar_decorations(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestAvatarDecorationsItem>>,
            >,
        ) -> Self {
            self.avatar_decorations = value.map(|value| value.into());
            self
        }
        pub fn banner_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_banner_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `bannerId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`banner_id_null`](Self::banner_id_null) を使う。"]
        pub fn maybe_banner_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.banner_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`bannerId` を `null` として送る。"]
        pub fn banner_id_null(mut self) -> Self {
            self.banner_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn fields(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestFieldsItem>>,
        ) -> Self {
            self.maybe_fields(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `fields` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_fields(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestFieldsItem>>,
            >,
        ) -> Self {
            self.fields = value.map(|value| value.into());
            self
        }
        pub fn is_locked(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_locked(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isLocked` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_is_locked(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_locked = value.map(|value| value.into());
            self
        }
        pub fn is_explorable(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_explorable(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isExplorable` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_is_explorable(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_explorable = value.map(|value| value.into());
            self
        }
        pub fn hide_online_status(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_hide_online_status(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `hideOnlineStatus` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_hide_online_status(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.hide_online_status = value.map(|value| value.into());
            self
        }
        pub fn public_reactions(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_public_reactions(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `publicReactions` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_public_reactions(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.public_reactions = value.map(|value| value.into());
            self
        }
        pub fn careful_bot(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_careful_bot(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `carefulBot` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_careful_bot(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.careful_bot = value.map(|value| value.into());
            self
        }
        pub fn auto_accept_followed(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_auto_accept_followed(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `autoAcceptFollowed` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_auto_accept_followed(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.auto_accept_followed = value.map(|value| value.into());
            self
        }
        pub fn no_crawle(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_no_crawle(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `noCrawle` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_no_crawle(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.no_crawle = value.map(|value| value.into());
            self
        }
        pub fn prevent_ai_learning(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_prevent_ai_learning(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `preventAiLearning` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_prevent_ai_learning(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.prevent_ai_learning = value.map(|value| value.into());
            self
        }
        pub fn require_signin_to_view_contents(
            self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.maybe_require_signin_to_view_contents(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `requireSigninToViewContents` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_require_signin_to_view_contents(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.require_signin_to_view_contents = value.map(|value| value.into());
            self
        }
        pub fn make_notes_followers_only_before(
            self,
            value: impl ::std::convert::Into<i64>,
        ) -> Self {
            self.maybe_make_notes_followers_only_before(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `makeNotesFollowersOnlyBefore` を設定し、`None` なら設定を消して送らない。`null` を送るには [`make_notes_followers_only_before_null`](Self::make_notes_followers_only_before_null) を使う。"]
        pub fn maybe_make_notes_followers_only_before(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.make_notes_followers_only_before =
                value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`makeNotesFollowersOnlyBefore` を `null` として送る。"]
        pub fn make_notes_followers_only_before_null(mut self) -> Self {
            self.make_notes_followers_only_before =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn make_notes_hidden_before(self, value: impl ::std::convert::Into<i64>) -> Self {
            self.maybe_make_notes_hidden_before(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `makeNotesHiddenBefore` を設定し、`None` なら設定を消して送らない。`null` を送るには [`make_notes_hidden_before_null`](Self::make_notes_hidden_before_null) を使う。"]
        pub fn maybe_make_notes_hidden_before(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<i64>>,
        ) -> Self {
            self.make_notes_hidden_before =
                value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`makeNotesHiddenBefore` を `null` として送る。"]
        pub fn make_notes_hidden_before_null(mut self) -> Self {
            self.make_notes_hidden_before =
                ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn is_bot(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_bot(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isBot` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_is_bot(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_bot = value.map(|value| value.into());
            self
        }
        pub fn is_cat(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_is_cat(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `isCat` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_is_cat(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.is_cat = value.map(|value| value.into());
            self
        }
        pub fn inject_featured_note(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_inject_featured_note(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `injectFeaturedNote` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_inject_featured_note(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.inject_featured_note = value.map(|value| value.into());
            self
        }
        pub fn receive_announcement_email(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_receive_announcement_email(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `receiveAnnouncementEmail` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_receive_announcement_email(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.receive_announcement_email = value.map(|value| value.into());
            self
        }
        pub fn always_mark_nsfw(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_always_mark_nsfw(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `alwaysMarkNsfw` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_always_mark_nsfw(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.always_mark_nsfw = value.map(|value| value.into());
            self
        }
        pub fn auto_sensitive(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_auto_sensitive(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `autoSensitive` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_auto_sensitive(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.auto_sensitive = value.map(|value| value.into());
            self
        }
        pub fn following_visibility(
            self,
            value: impl ::std::convert::Into<RequestFollowingVisibility>,
        ) -> Self {
            self.maybe_following_visibility(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `followingVisibility` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_following_visibility(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestFollowingVisibility>>,
        ) -> Self {
            self.following_visibility = value.map(|value| value.into());
            self
        }
        pub fn followers_visibility(
            self,
            value: impl ::std::convert::Into<RequestFollowersVisibility>,
        ) -> Self {
            self.maybe_followers_visibility(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `followersVisibility` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_followers_visibility(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestFollowersVisibility>>,
        ) -> Self {
            self.followers_visibility = value.map(|value| value.into());
            self
        }
        pub fn chat_scope(self, value: impl ::std::convert::Into<RequestChatScope>) -> Self {
            self.maybe_chat_scope(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `chatScope` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_chat_scope(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestChatScope>>,
        ) -> Self {
            self.chat_scope = value.map(|value| value.into());
            self
        }
        pub fn pinned_page_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.maybe_pinned_page_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `pinnedPageId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`pinned_page_id_null`](Self::pinned_page_id_null) を使う。"]
        pub fn maybe_pinned_page_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.pinned_page_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`pinnedPageId` を `null` として送る。"]
        pub fn pinned_page_id_null(mut self) -> Self {
            self.pinned_page_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn muted_words(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestMutedWordsItem>>,
        ) -> Self {
            self.maybe_muted_words(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `mutedWords` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_muted_words(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestMutedWordsItem>>,
            >,
        ) -> Self {
            self.muted_words = value.map(|value| value.into());
            self
        }
        pub fn hard_muted_words(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<RequestHardMutedWordsItem>>,
        ) -> Self {
            self.maybe_hard_muted_words(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `hardMutedWords` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_hard_muted_words(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<RequestHardMutedWordsItem>>,
            >,
        ) -> Self {
            self.hard_muted_words = value.map(|value| value.into());
            self
        }
        pub fn muted_instances(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.maybe_muted_instances(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `mutedInstances` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_muted_instances(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.muted_instances = value.map(|value| value.into());
            self
        }
        pub fn notification_recieve_config(
            self,
            value: impl ::std::convert::Into<RequestNotificationRecieveConfig>,
        ) -> Self {
            self.maybe_notification_recieve_config(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `notificationRecieveConfig` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_notification_recieve_config(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<RequestNotificationRecieveConfig>,
            >,
        ) -> Self {
            self.notification_recieve_config = value.map(|value| value.into());
            self
        }
        pub fn email_notification_types(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.maybe_email_notification_types(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `emailNotificationTypes` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_email_notification_types(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.email_notification_types = value.map(|value| value.into());
            self
        }
        pub fn also_known_as(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.maybe_also_known_as(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `alsoKnownAs` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_also_known_as(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.also_known_as = value.map(|value| value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                name: self.name,
                description: self.description,
                followed_message: self.followed_message,
                location: self.location,
                birthday: self.birthday,
                lang: self.lang,
                avatar_id: self.avatar_id,
                avatar_decorations: self.avatar_decorations,
                banner_id: self.banner_id,
                fields: self.fields,
                is_locked: self.is_locked,
                is_explorable: self.is_explorable,
                hide_online_status: self.hide_online_status,
                public_reactions: self.public_reactions,
                careful_bot: self.careful_bot,
                auto_accept_followed: self.auto_accept_followed,
                no_crawle: self.no_crawle,
                prevent_ai_learning: self.prevent_ai_learning,
                require_signin_to_view_contents: self.require_signin_to_view_contents,
                make_notes_followers_only_before: self.make_notes_followers_only_before,
                make_notes_hidden_before: self.make_notes_hidden_before,
                is_bot: self.is_bot,
                is_cat: self.is_cat,
                inject_featured_note: self.inject_featured_note,
                receive_announcement_email: self.receive_announcement_email,
                always_mark_nsfw: self.always_mark_nsfw,
                auto_sensitive: self.auto_sensitive,
                following_visibility: self.following_visibility,
                followers_visibility: self.followers_visibility,
                chat_scope: self.chat_scope,
                pinned_page_id: self.pinned_page_id,
                muted_words: self.muted_words,
                hard_muted_words: self.hard_muted_words,
                muted_instances: self.muted_instances,
                notification_recieve_config: self.notification_recieve_config,
                email_notification_types: self.email_notification_types,
                also_known_as: self.also_known_as,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/update-email`"]
pub mod update_email {
    #[doc = "`i/update-email`"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub password: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub email: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub token: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "i/update-email";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::MeDetailed;
    #[doc = "`i/update-email` のビルダー。"]
    #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `password`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        password: ::std::option::Option<::std::string::String>,
        email: ::std::option::Option<::std::option::Option<::std::string::String>>,
        token: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                password: ::std::option::Option::None,
                email: ::std::option::Option::None,
                token: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn email(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_email(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `email` を設定し、`None` なら設定を消して送らない。`null` を送るには [`email_null`](Self::email_null) を使う。"]
        pub fn maybe_email(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.email = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`email` を `null` として送る。"]
        pub fn email_null(mut self) -> Self {
            self.email = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn token(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_token(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `token` を設定し、`None` なら設定を消して送らない。`null` を送るには [`token_null`](Self::token_null) を使う。"]
        pub fn maybe_token(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.token = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`token` を `null` として送る。"]
        pub fn token_null(mut self) -> Self {
            self.token = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn password(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                password: ::std::option::Option::Some(value.into()),
                email: self.email,
                token: self.token,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                password: self
                    .password
                    .expect("required parameter is set by the type state"),
                email: self.email,
                token: self.token,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`i/webhooks/` 以下のエンドポイント"]
pub mod webhooks {
    #[doc = "`i/webhooks/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`i/webhooks/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`i/webhooks/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`i/webhooks/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`i/webhooks/show`"]
        pub fn show(&self) -> show::Call<'a> {
            show::Call::new(self.client)
        }
        #[doc = "`i/webhooks/test`"]
        pub fn test(&self) -> test::Call<'a> {
            test::Call::new(self.client)
        }
        #[doc = "`i/webhooks/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
    }
    #[doc = "`i/webhooks/create`"]
    pub mod create {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestOnItem {
            #[serde(rename = "mention")]
            Mention,
            #[serde(rename = "unfollow")]
            Unfollow,
            #[serde(rename = "follow")]
            Follow,
            #[serde(rename = "followed")]
            Followed,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "reply")]
            Reply,
            #[serde(rename = "renote")]
            Renote,
            #[serde(rename = "reaction")]
            Reaction,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum ResponseOnItem {
            #[serde(rename = "mention")]
            Mention,
            #[serde(rename = "unfollow")]
            Unfollow,
            #[serde(rename = "follow")]
            Follow,
            #[serde(rename = "followed")]
            Followed,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "reply")]
            Reply,
            #[serde(rename = "renote")]
            Renote,
            #[serde(rename = "reaction")]
            Reaction,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            pub id: ::std::string::String,
            #[serde(rename = "userId")]
            pub user_id: ::std::string::String,
            pub name: ::std::string::String,
            pub on: ::std::vec::Vec<ResponseOnItem>,
            pub url: ::std::string::String,
            pub secret: ::std::string::String,
            pub active: bool,
            #[serde(rename = "latestSentAt")]
            pub latest_sent_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
            #[serde(rename = "latestStatus")]
            pub latest_status: ::std::option::Option<i64>,
        }
        #[doc = "`i/webhooks/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            pub name: ::std::string::String,
            pub url: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub secret: ::std::option::Option<::std::string::String>,
            pub on: ::std::vec::Vec<RequestOnItem>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/webhooks/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`i/webhooks/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`url`、`on`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<
            'a,
            S = (
                crate::builder::Unset,
                crate::builder::Unset,
                crate::builder::Unset,
            ),
        > {
            client: &'a crate::generated::v2025_10_0::Client,
            name: ::std::option::Option<::std::string::String>,
            url: ::std::option::Option<::std::string::String>,
            secret: ::std::option::Option<::std::string::String>,
            on: ::std::option::Option<::std::vec::Vec<RequestOnItem>>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    name: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                    secret: ::std::option::Option::None,
                    on: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1, S2> Call<'a, (S0, S1, S2)> {
            pub fn secret(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_secret(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `secret` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_secret(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.secret = value.map(|value| value.into());
                self
            }
            pub fn name(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1, S2)> {
                Call {
                    client: self.client,
                    name: ::std::option::Option::Some(value.into()),
                    url: self.url,
                    secret: self.secret,
                    on: self.on,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn url(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (S0, crate::builder::Set, S2)> {
                Call {
                    client: self.client,
                    name: self.name,
                    url: ::std::option::Option::Some(value.into()),
                    secret: self.secret,
                    on: self.on,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn on(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<RequestOnItem>>,
            ) -> Call<'a, (S0, S1, crate::builder::Set)> {
                Call {
                    client: self.client,
                    name: self.name,
                    url: self.url,
                    secret: self.secret,
                    on: ::std::option::Option::Some(value.into()),
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
                ),
            >
        {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    name: self
                        .name
                        .expect("required parameter is set by the type state"),
                    url: self
                        .url
                        .expect("required parameter is set by the type state"),
                    secret: self.secret,
                    on: self
                        .on
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/webhooks/delete`"]
    pub mod delete {
        #[doc = "`i/webhooks/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "webhookId")]
            pub webhook_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/webhooks/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/webhooks/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `webhook_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            webhook_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    webhook_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn webhook_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    webhook_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    webhook_id: self
                        .webhook_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/webhooks/list`"]
    pub mod list {
        #[doc = "`i/webhooks/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/webhooks/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::UserWebhook>;
        #[doc = "`i/webhooks/list` のビルダー。"]
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
    #[doc = "`i/webhooks/show`"]
    pub mod show {
        #[doc = "`i/webhooks/show`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "webhookId")]
            pub webhook_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/webhooks/show";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = crate::generated::v2025_10_0::types::UserWebhook;
        #[doc = "`i/webhooks/show` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `webhook_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            webhook_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    webhook_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn webhook_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    webhook_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    webhook_id: self
                        .webhook_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/webhooks/test`"]
    pub mod test {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestType {
            #[serde(rename = "mention")]
            Mention,
            #[serde(rename = "unfollow")]
            Unfollow,
            #[serde(rename = "follow")]
            Follow,
            #[serde(rename = "followed")]
            Followed,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "reply")]
            Reply,
            #[serde(rename = "renote")]
            Renote,
            #[serde(rename = "reaction")]
            Reaction,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct RequestOverride {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub url: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub secret: ::std::option::Option<::std::string::String>,
        }
        #[doc = "`i/webhooks/test`"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "webhookId")]
            pub webhook_id: ::std::string::String,
            pub r#type: RequestType,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub r#override: ::std::option::Option<RequestOverride>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/webhooks/test";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/webhooks/test` のビルダー。"]
        #[doc = "No description provided.\n\n**Internal Endpoint**: This endpoint is an API for the misskey mainframe and is not intended for use by third parties.\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `webhook_id`、`type`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            webhook_id: ::std::option::Option<::std::string::String>,
            r#type: ::std::option::Option<RequestType>,
            r#override: ::std::option::Option<RequestOverride>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    webhook_id: ::std::option::Option::None,
                    r#type: ::std::option::Option::None,
                    r#override: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn r#override(self, value: impl ::std::convert::Into<RequestOverride>) -> Self {
                self.maybe_override(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `override` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_override(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<RequestOverride>>,
            ) -> Self {
                self.r#override = value.map(|value| value.into());
                self
            }
            pub fn webhook_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    webhook_id: ::std::option::Option::Some(value.into()),
                    r#type: self.r#type,
                    r#override: self.r#override,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn r#type(
                self,
                value: impl ::std::convert::Into<RequestType>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    webhook_id: self.webhook_id,
                    r#type: ::std::option::Option::Some(value.into()),
                    r#override: self.r#override,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    webhook_id: self
                        .webhook_id
                        .expect("required parameter is set by the type state"),
                    r#type: self
                        .r#type
                        .expect("required parameter is set by the type state"),
                    r#override: self.r#override,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`i/webhooks/update`"]
    pub mod update {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestOnItem {
            #[serde(rename = "mention")]
            Mention,
            #[serde(rename = "unfollow")]
            Unfollow,
            #[serde(rename = "follow")]
            Follow,
            #[serde(rename = "followed")]
            Followed,
            #[serde(rename = "note")]
            Note,
            #[serde(rename = "reply")]
            Reply,
            #[serde(rename = "renote")]
            Renote,
            #[serde(rename = "reaction")]
            Reaction,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[doc = "`i/webhooks/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "webhookId")]
            pub webhook_id: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub name: ::std::option::Option<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub url: ::std::option::Option<::std::string::String>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub secret: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub on: ::std::option::Option<::std::vec::Vec<RequestOnItem>>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub active: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "i/webhooks/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`i/webhooks/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `webhook_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            webhook_id: ::std::option::Option<::std::string::String>,
            name: ::std::option::Option<::std::string::String>,
            url: ::std::option::Option<::std::string::String>,
            secret: ::std::option::Option<::std::option::Option<::std::string::String>>,
            on: ::std::option::Option<::std::vec::Vec<RequestOnItem>>,
            active: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    webhook_id: ::std::option::Option::None,
                    name: ::std::option::Option::None,
                    url: ::std::option::Option::None,
                    secret: ::std::option::Option::None,
                    on: ::std::option::Option::None,
                    active: ::std::option::Option::None,
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
            pub fn secret(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.maybe_secret(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `secret` を設定し、`None` なら設定を消して送らない。`null` を送るには [`secret_null`](Self::secret_null) を使う。"]
            pub fn maybe_secret(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
            ) -> Self {
                self.secret = value.map(|value| ::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`secret` を `null` として送る。"]
            pub fn secret_null(mut self) -> Self {
                self.secret = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn on(
                self,
                value: impl ::std::convert::Into<::std::vec::Vec<RequestOnItem>>,
            ) -> Self {
                self.maybe_on(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `on` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_on(
                mut self,
                value: ::std::option::Option<
                    impl ::std::convert::Into<::std::vec::Vec<RequestOnItem>>,
                >,
            ) -> Self {
                self.on = value.map(|value| value.into());
                self
            }
            pub fn active(self, value: impl ::std::convert::Into<bool>) -> Self {
                self.maybe_active(::std::option::Option::Some(value))
            }
            #[doc = "`Some` なら `active` を設定し、`None` なら設定を消して送らない。"]
            pub fn maybe_active(
                mut self,
                value: ::std::option::Option<impl ::std::convert::Into<bool>>,
            ) -> Self {
                self.active = value.map(|value| value.into());
                self
            }
            pub fn webhook_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    webhook_id: ::std::option::Option::Some(value.into()),
                    name: self.name,
                    url: self.url,
                    secret: self.secret,
                    on: self.on,
                    active: self.active,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    webhook_id: self
                        .webhook_id
                        .expect("required parameter is set by the type state"),
                    name: self.name,
                    url: self.url,
                    secret: self.secret,
                    on: self.on,
                    active: self.active,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
