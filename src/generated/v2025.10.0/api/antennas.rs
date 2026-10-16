#![doc = "`antennas/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`antennas/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`antennas/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`antennas/delete`"]
    pub fn delete(&self) -> delete::Call<'a> {
        delete::Call::new(self.client)
    }
    #[doc = "`antennas/list`"]
    pub fn list(&self) -> list::Call<'a> {
        list::Call::new(self.client)
    }
    #[doc = "`antennas/notes`"]
    pub fn notes(&self) -> notes::Call<'a> {
        notes::Call::new(self.client)
    }
    #[doc = "`antennas/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`antennas/update`"]
    pub fn update(&self) -> update::Call<'a> {
        update::Call::new(self.client)
    }
}
#[doc = "`antennas/create`"]
pub mod create {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSrc {
        #[serde(rename = "home")]
        Home,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "users")]
        Users,
        #[serde(rename = "list")]
        List,
        #[serde(rename = "users_blacklist")]
        UsersBlacklist,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`antennas/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[doc = "文字数は 1 以上 100 以下。"]
        pub name: ::std::string::String,
        pub src: RequestSrc,
        #[serde(
            rename = "userListId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub user_list_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        pub keywords: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        #[serde(rename = "excludeKeywords")]
        pub exclude_keywords: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
        pub users: ::std::vec::Vec<::std::string::String>,
        #[serde(rename = "caseSensitive")]
        pub case_sensitive: bool,
        #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
        pub local_only: ::std::option::Option<bool>,
        #[serde(
            rename = "excludeBots",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_bots: ::std::option::Option<bool>,
        #[serde(rename = "withReplies")]
        pub with_replies: bool,
        #[serde(rename = "withFile")]
        pub with_file: bool,
        #[serde(
            rename = "excludeNotesInSensitiveChannel",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_notes_in_sensitive_channel: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "antennas/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::Antenna;
    #[doc = "`antennas/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `name`、`src`、`keywords`、`exclude_keywords`、`users`、`case_sensitive`、`with_replies`、`with_file`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<
        'a,
        S = (
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
            crate::builder::Unset,
        ),
    > {
        client: &'a crate::generated::v2025_10_0::Client,
        name: ::std::option::Option<::std::string::String>,
        src: ::std::option::Option<RequestSrc>,
        user_list_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        keywords: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        exclude_keywords:
            ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        case_sensitive: ::std::option::Option<bool>,
        local_only: ::std::option::Option<bool>,
        exclude_bots: ::std::option::Option<bool>,
        with_replies: ::std::option::Option<bool>,
        with_file: ::std::option::Option<bool>,
        exclude_notes_in_sensitive_channel: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                name: ::std::option::Option::None,
                src: ::std::option::Option::None,
                user_list_id: ::std::option::Option::None,
                keywords: ::std::option::Option::None,
                exclude_keywords: ::std::option::Option::None,
                users: ::std::option::Option::None,
                case_sensitive: ::std::option::Option::None,
                local_only: ::std::option::Option::None,
                exclude_bots: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
                with_file: ::std::option::Option::None,
                exclude_notes_in_sensitive_channel: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1, S2, S3, S4, S5, S6, S7> Call<'a, (S0, S1, S2, S3, S4, S5, S6, S7)> {
        pub fn user_list_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_user_list_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `userListId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`user_list_id_null`](Self::user_list_id_null) を使う。"]
        pub fn maybe_user_list_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.user_list_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`userListId` を `null` として送る。"]
        pub fn user_list_id_null(mut self) -> Self {
            self.user_list_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn local_only(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_local_only(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `localOnly` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_local_only(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.local_only = value.map(|value| value.into());
            self
        }
        pub fn exclude_bots(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_exclude_bots(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeBots` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_bots(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.exclude_bots = value.map(|value| value.into());
            self
        }
        pub fn exclude_notes_in_sensitive_channel(
            self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.maybe_exclude_notes_in_sensitive_channel(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeNotesInSensitiveChannel` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_notes_in_sensitive_channel(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.exclude_notes_in_sensitive_channel = value.map(|value| value.into());
            self
        }
        #[doc = "文字数は 1 以上 100 以下。"]
        pub fn name(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1, S2, S3, S4, S5, S6, S7)> {
            Call {
                client: self.client,
                name: ::std::option::Option::Some(value.into()),
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn src(
            self,
            value: impl ::std::convert::Into<RequestSrc>,
        ) -> Call<'a, (S0, crate::builder::Set, S2, S3, S4, S5, S6, S7)> {
            Call {
                client: self.client,
                name: self.name,
                src: ::std::option::Option::Some(value.into()),
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn keywords(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        ) -> Call<'a, (S0, S1, crate::builder::Set, S3, S4, S5, S6, S7)> {
            Call {
                client: self.client,
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: ::std::option::Option::Some(value.into()),
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn exclude_keywords(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        ) -> Call<'a, (S0, S1, S2, crate::builder::Set, S4, S5, S6, S7)> {
            Call {
                client: self.client,
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: ::std::option::Option::Some(value.into()),
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn users(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Call<'a, (S0, S1, S2, S3, crate::builder::Set, S5, S6, S7)> {
            Call {
                client: self.client,
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: ::std::option::Option::Some(value.into()),
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn case_sensitive(
            self,
            value: impl ::std::convert::Into<bool>,
        ) -> Call<'a, (S0, S1, S2, S3, S4, crate::builder::Set, S6, S7)> {
            Call {
                client: self.client,
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: ::std::option::Option::Some(value.into()),
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn with_replies(
            self,
            value: impl ::std::convert::Into<bool>,
        ) -> Call<'a, (S0, S1, S2, S3, S4, S5, crate::builder::Set, S7)> {
            Call {
                client: self.client,
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: ::std::option::Option::Some(value.into()),
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn with_file(
            self,
            value: impl ::std::convert::Into<bool>,
        ) -> Call<'a, (S0, S1, S2, S3, S4, S5, S6, crate::builder::Set)> {
            Call {
                client: self.client,
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: ::std::option::Option::Some(value.into()),
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
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
                src: self
                    .src
                    .expect("required parameter is set by the type state"),
                user_list_id: self.user_list_id,
                keywords: self
                    .keywords
                    .expect("required parameter is set by the type state"),
                exclude_keywords: self
                    .exclude_keywords
                    .expect("required parameter is set by the type state"),
                users: self
                    .users
                    .expect("required parameter is set by the type state"),
                case_sensitive: self
                    .case_sensitive
                    .expect("required parameter is set by the type state"),
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self
                    .with_replies
                    .expect("required parameter is set by the type state"),
                with_file: self
                    .with_file
                    .expect("required parameter is set by the type state"),
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`antennas/delete`"]
pub mod delete {
    #[doc = "`antennas/delete`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "antennaId")]
        pub antenna_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "antennas/delete";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`antennas/delete` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `antenna_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        antenna_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                antenna_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn antenna_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                antenna_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                antenna_id: self
                    .antenna_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`antennas/list`"]
pub mod list {
    #[doc = "`antennas/list`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {}
    impl crate::Endpoint for Request {
        const PATH: &'static str = "antennas/list";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Antenna>;
    #[doc = "`antennas/list` のビルダー。"]
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
#[doc = "`antennas/notes`"]
pub mod notes {
    #[doc = "`antennas/notes`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "antennaId")]
        pub antenna_id: ::std::string::String,
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
        const PATH: &'static str = "antennas/notes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`antennas/notes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `antenna_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        antenna_id: ::std::option::Option<::std::string::String>,
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
                antenna_id: ::std::option::Option::None,
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
        pub fn antenna_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                antenna_id: ::std::option::Option::Some(value.into()),
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
                antenna_id: self
                    .antenna_id
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
#[doc = "`antennas/show`"]
pub mod show {
    #[doc = "`antennas/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "antennaId")]
        pub antenna_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "antennas/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::Antenna;
    #[doc = "`antennas/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `antenna_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        antenna_id: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                antenna_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn antenna_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                antenna_id: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                antenna_id: self
                    .antenna_id
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`antennas/update`"]
pub mod update {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestSrc {
        #[serde(rename = "home")]
        Home,
        #[serde(rename = "all")]
        All,
        #[serde(rename = "users")]
        Users,
        #[serde(rename = "list")]
        List,
        #[serde(rename = "users_blacklist")]
        UsersBlacklist,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[doc = "`antennas/update`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "antennaId")]
        pub antenna_id: ::std::string::String,
        #[doc = "文字数は 1 以上 100 以下。"]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: ::std::option::Option<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub src: ::std::option::Option<RequestSrc>,
        #[serde(
            rename = "userListId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub user_list_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub keywords:
            ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            rename = "excludeKeywords",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_keywords:
            ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            rename = "caseSensitive",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub case_sensitive: ::std::option::Option<bool>,
        #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
        pub local_only: ::std::option::Option<bool>,
        #[serde(
            rename = "excludeBots",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_bots: ::std::option::Option<bool>,
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
        #[serde(rename = "withFile", default, skip_serializing_if = "Option::is_none")]
        pub with_file: ::std::option::Option<bool>,
        #[serde(
            rename = "excludeNotesInSensitiveChannel",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub exclude_notes_in_sensitive_channel: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "antennas/update";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::Antenna;
    #[doc = "`antennas/update` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `antenna_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        antenna_id: ::std::option::Option<::std::string::String>,
        name: ::std::option::Option<::std::string::String>,
        src: ::std::option::Option<RequestSrc>,
        user_list_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        keywords: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        exclude_keywords:
            ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        users: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        case_sensitive: ::std::option::Option<bool>,
        local_only: ::std::option::Option<bool>,
        exclude_bots: ::std::option::Option<bool>,
        with_replies: ::std::option::Option<bool>,
        with_file: ::std::option::Option<bool>,
        exclude_notes_in_sensitive_channel: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                antenna_id: ::std::option::Option::None,
                name: ::std::option::Option::None,
                src: ::std::option::Option::None,
                user_list_id: ::std::option::Option::None,
                keywords: ::std::option::Option::None,
                exclude_keywords: ::std::option::Option::None,
                users: ::std::option::Option::None,
                case_sensitive: ::std::option::Option::None,
                local_only: ::std::option::Option::None,
                exclude_bots: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
                with_file: ::std::option::Option::None,
                exclude_notes_in_sensitive_channel: ::std::option::Option::None,
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
        pub fn src(self, value: impl ::std::convert::Into<RequestSrc>) -> Self {
            self.maybe_src(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `src` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_src(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<RequestSrc>>,
        ) -> Self {
            self.src = value.map(|value| value.into());
            self
        }
        pub fn user_list_id(self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.maybe_user_list_id(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `userListId` を設定し、`None` なら設定を消して送らない。`null` を送るには [`user_list_id_null`](Self::user_list_id_null) を使う。"]
        pub fn maybe_user_list_id(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<::std::string::String>>,
        ) -> Self {
            self.user_list_id = value.map(|value| ::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`userListId` を `null` として送る。"]
        pub fn user_list_id_null(mut self) -> Self {
            self.user_list_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn keywords(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        ) -> Self {
            self.maybe_keywords(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `keywords` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_keywords(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
            >,
        ) -> Self {
            self.keywords = value.map(|value| value.into());
            self
        }
        pub fn exclude_keywords(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        ) -> Self {
            self.maybe_exclude_keywords(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeKeywords` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_keywords(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
            >,
        ) -> Self {
            self.exclude_keywords = value.map(|value| value.into());
            self
        }
        pub fn users(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.maybe_users(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `users` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_users(
            mut self,
            value: ::std::option::Option<
                impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            >,
        ) -> Self {
            self.users = value.map(|value| value.into());
            self
        }
        pub fn case_sensitive(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_case_sensitive(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `caseSensitive` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_case_sensitive(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.case_sensitive = value.map(|value| value.into());
            self
        }
        pub fn local_only(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_local_only(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `localOnly` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_local_only(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.local_only = value.map(|value| value.into());
            self
        }
        pub fn exclude_bots(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_exclude_bots(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeBots` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_bots(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.exclude_bots = value.map(|value| value.into());
            self
        }
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
        pub fn with_file(self, value: impl ::std::convert::Into<bool>) -> Self {
            self.maybe_with_file(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `withFile` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_with_file(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.with_file = value.map(|value| value.into());
            self
        }
        pub fn exclude_notes_in_sensitive_channel(
            self,
            value: impl ::std::convert::Into<bool>,
        ) -> Self {
            self.maybe_exclude_notes_in_sensitive_channel(::std::option::Option::Some(value))
        }
        #[doc = "`Some` なら `excludeNotesInSensitiveChannel` を設定し、`None` なら設定を消して送らない。"]
        pub fn maybe_exclude_notes_in_sensitive_channel(
            mut self,
            value: ::std::option::Option<impl ::std::convert::Into<bool>>,
        ) -> Self {
            self.exclude_notes_in_sensitive_channel = value.map(|value| value.into());
            self
        }
        pub fn antenna_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                antenna_id: ::std::option::Option::Some(value.into()),
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                antenna_id: self
                    .antenna_id
                    .expect("required parameter is set by the type state"),
                name: self.name,
                src: self.src,
                user_list_id: self.user_list_id,
                keywords: self.keywords,
                exclude_keywords: self.exclude_keywords,
                users: self.users,
                case_sensitive: self.case_sensitive,
                local_only: self.local_only,
                exclude_bots: self.exclude_bots,
                with_replies: self.with_replies,
                with_file: self.with_file,
                exclude_notes_in_sensitive_channel: self.exclude_notes_in_sensitive_channel,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
