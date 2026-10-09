#![doc = "`notes` と、`notes/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`notes`"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Request {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub local: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reply: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub renote: ::std::option::Option<bool>,
    #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
    pub with_files: ::std::option::Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub poll: ::std::option::Option<bool>,
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
    const PATH: &'static str = "notes";
    type Response = Response;
    fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
        crate::endpoint::json_body(&self)
    }
}
pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
#[doc = "`notes` のビルダー。"]
#[doc = "No description provided.\n\n**Credential required**: *No*"]
pub struct Call<'a> {
    client: &'a crate::generated::v2025_10_0::Client,
    local: ::std::option::Option<bool>,
    reply: ::std::option::Option<bool>,
    renote: ::std::option::Option<bool>,
    with_files: ::std::option::Option<bool>,
    poll: ::std::option::Option<bool>,
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
            local: ::std::option::Option::None,
            reply: ::std::option::Option::None,
            renote: ::std::option::Option::None,
            with_files: ::std::option::Option::None,
            poll: ::std::option::Option::None,
            limit: ::std::option::Option::None,
            since_id: ::std::option::Option::None,
            until_id: ::std::option::Option::None,
            since_date: ::std::option::Option::None,
            until_date: ::std::option::Option::None,
        }
    }
}
impl<'a> Call<'a> {
    #[doc = "`notes/children`"]
    pub fn children(&self) -> children::Call<'a> {
        children::Call::new(self.client)
    }
    #[doc = "`notes/clips`"]
    pub fn clips(&self) -> clips::Call<'a> {
        clips::Call::new(self.client)
    }
    #[doc = "`notes/conversation`"]
    pub fn conversation(&self) -> conversation::Call<'a> {
        conversation::Call::new(self.client)
    }
    #[doc = "`notes/create`"]
    pub fn create(&self) -> create::Call<'a> {
        create::Call::new(self.client)
    }
    #[doc = "`notes/delete`"]
    pub fn delete(&self) -> delete::Call<'a> {
        delete::Call::new(self.client)
    }
    #[doc = "`notes/drafts/` 以下のエンドポイント"]
    pub fn drafts(&self) -> drafts::Call<'a> {
        drafts::Call::new(self.client)
    }
    #[doc = "`notes/favorites/` 以下のエンドポイント"]
    pub fn favorites(&self) -> favorites::Call<'a> {
        favorites::Call::new(self.client)
    }
    #[doc = "`notes/featured`"]
    pub fn featured(&self) -> featured::Call<'a> {
        featured::Call::new(self.client)
    }
    #[doc = "`notes/global-timeline`"]
    pub fn global_timeline(&self) -> global_timeline::Call<'a> {
        global_timeline::Call::new(self.client)
    }
    #[doc = "`notes/hybrid-timeline`"]
    pub fn hybrid_timeline(&self) -> hybrid_timeline::Call<'a> {
        hybrid_timeline::Call::new(self.client)
    }
    #[doc = "`notes/local-timeline`"]
    pub fn local_timeline(&self) -> local_timeline::Call<'a> {
        local_timeline::Call::new(self.client)
    }
    #[doc = "`notes/mentions`"]
    pub fn mentions(&self) -> mentions::Call<'a> {
        mentions::Call::new(self.client)
    }
    #[doc = "`notes/polls/` 以下のエンドポイント"]
    pub fn polls(&self) -> polls::Call<'a> {
        polls::Call::new(self.client)
    }
    #[doc = "`notes/reactions` と、`notes/reactions/` 以下のエンドポイント"]
    pub fn reactions(&self) -> reactions::Call<'a> {
        reactions::Call::new(self.client)
    }
    #[doc = "`notes/renotes`"]
    pub fn renotes(&self) -> renotes::Call<'a> {
        renotes::Call::new(self.client)
    }
    #[doc = "`notes/replies`"]
    pub fn replies(&self) -> replies::Call<'a> {
        replies::Call::new(self.client)
    }
    #[doc = "`notes/search`"]
    pub fn search(&self) -> search::Call<'a> {
        search::Call::new(self.client)
    }
    #[doc = "`notes/search-by-tag`"]
    pub fn search_by_tag(&self) -> search_by_tag::Call<'a> {
        search_by_tag::Call::new(self.client)
    }
    #[doc = "`notes/show`"]
    pub fn show(&self) -> show::Call<'a> {
        show::Call::new(self.client)
    }
    #[doc = "`notes/show-partial-bulk`"]
    pub fn show_partial_bulk(&self) -> show_partial_bulk::Call<'a> {
        show_partial_bulk::Call::new(self.client)
    }
    #[doc = "`notes/state`"]
    pub fn state(&self) -> state::Call<'a> {
        state::Call::new(self.client)
    }
    #[doc = "`notes/thread-muting/` 以下のエンドポイント"]
    pub fn thread_muting(&self) -> thread_muting::Call<'a> {
        thread_muting::Call::new(self.client)
    }
    #[doc = "`notes/timeline`"]
    pub fn timeline(&self) -> timeline::Call<'a> {
        timeline::Call::new(self.client)
    }
    #[doc = "`notes/translate`"]
    pub fn translate(&self) -> translate::Call<'a> {
        translate::Call::new(self.client)
    }
    #[doc = "`notes/unrenote`"]
    pub fn unrenote(&self) -> unrenote::Call<'a> {
        unrenote::Call::new(self.client)
    }
    #[doc = "`notes/user-list-timeline`"]
    pub fn user_list_timeline(&self) -> user_list_timeline::Call<'a> {
        user_list_timeline::Call::new(self.client)
    }
    pub fn local(mut self, value: impl ::std::convert::Into<bool>) -> Self {
        self.local = ::std::option::Option::Some(value.into());
        self
    }
    pub fn reply(mut self, value: impl ::std::convert::Into<bool>) -> Self {
        self.reply = ::std::option::Option::Some(value.into());
        self
    }
    pub fn renote(mut self, value: impl ::std::convert::Into<bool>) -> Self {
        self.renote = ::std::option::Option::Some(value.into());
        self
    }
    pub fn with_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
        self.with_files = ::std::option::Option::Some(value.into());
        self
    }
    pub fn poll(mut self, value: impl ::std::convert::Into<bool>) -> Self {
        self.poll = ::std::option::Option::Some(value.into());
        self
    }
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
}
impl<'a> ::std::future::IntoFuture for Call<'a> {
    type Output = ::std::result::Result<Response, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Response>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Request {
            local: self.local,
            reply: self.reply,
            renote: self.renote,
            with_files: self.with_files,
            poll: self.poll,
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
#[doc = "`notes/children`"]
pub mod children {
    #[doc = "`notes/children`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
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
        const PATH: &'static str = "notes/children";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/children` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
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
                note_id: ::std::option::Option::None,
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
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
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
                note_id: self
                    .note_id
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
#[doc = "`notes/clips`"]
pub mod clips {
    #[doc = "`notes/clips`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/clips";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Clip>;
    #[doc = "`notes/clips` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
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
#[doc = "`notes/conversation`"]
pub mod conversation {
    #[doc = "`notes/conversation`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/conversation";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/conversation` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                note_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn offset(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.offset = ::std::option::Option::Some(value.into());
            self
        }
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                offset: self.offset,
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
                limit: self.limit,
                offset: self.offset,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/create`"]
pub mod create {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestVisibility {
        #[serde(rename = "public")]
        Public,
        #[serde(rename = "home")]
        Home,
        #[serde(rename = "followers")]
        Followers,
        #[serde(rename = "specified")]
        Specified,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub enum RequestReactionAcceptance {
        #[serde(rename = "likeOnly")]
        LikeOnly,
        #[serde(rename = "likeOnlyForRemote")]
        LikeOnlyForRemote,
        #[serde(rename = "nonSensitiveOnly")]
        NonSensitiveOnly,
        #[serde(rename = "nonSensitiveOnlyForLocalLikeOnlyForRemote")]
        NonSensitiveOnlyForLocalLikeOnlyForRemote,
        #[doc = "このバージョンのバインディングが知らない値。"]
        #[serde(untagged)]
        Unrecognized(::std::string::String),
    }
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct RequestPoll {
        pub choices: ::std::vec::Vec<::std::string::String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub multiple: ::std::option::Option<bool>,
        #[serde(
            rename = "expiresAt",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub expires_at: ::std::option::Option<::std::option::Option<i64>>,
        #[serde(
            rename = "expiredAfter",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub expired_after: ::std::option::Option<::std::option::Option<i64>>,
    }
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "createdNote")]
        pub created_note: crate::generated::v2025_10_0::types::Note,
    }
    #[doc = "`notes/create`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub visibility: ::std::option::Option<RequestVisibility>,
        #[serde(
            rename = "visibleUserIds",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub visible_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub cw: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
        pub local_only: ::std::option::Option<bool>,
        #[serde(
            rename = "reactionAcceptance",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub reaction_acceptance:
            ::std::option::Option<::std::option::Option<RequestReactionAcceptance>>,
        #[serde(
            rename = "noExtractMentions",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_extract_mentions: ::std::option::Option<bool>,
        #[serde(
            rename = "noExtractHashtags",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_extract_hashtags: ::std::option::Option<bool>,
        #[serde(
            rename = "noExtractEmojis",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub no_extract_emojis: ::std::option::Option<bool>,
        #[serde(
            rename = "replyId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub reply_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "renoteId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub renote_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "channelId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub text: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(rename = "fileIds", default, skip_serializing_if = "Option::is_none")]
        pub file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(rename = "mediaIds", default, skip_serializing_if = "Option::is_none")]
        pub media_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub poll: ::std::option::Option<::std::option::Option<RequestPoll>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/create";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`notes/create` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notes*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        visibility: ::std::option::Option<RequestVisibility>,
        visible_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        cw: ::std::option::Option<::std::option::Option<::std::string::String>>,
        local_only: ::std::option::Option<bool>,
        reaction_acceptance:
            ::std::option::Option<::std::option::Option<RequestReactionAcceptance>>,
        no_extract_mentions: ::std::option::Option<bool>,
        no_extract_hashtags: ::std::option::Option<bool>,
        no_extract_emojis: ::std::option::Option<bool>,
        reply_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        renote_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        text: ::std::option::Option<::std::option::Option<::std::string::String>>,
        file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        media_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        poll: ::std::option::Option<::std::option::Option<RequestPoll>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                visibility: ::std::option::Option::None,
                visible_user_ids: ::std::option::Option::None,
                cw: ::std::option::Option::None,
                local_only: ::std::option::Option::None,
                reaction_acceptance: ::std::option::Option::None,
                no_extract_mentions: ::std::option::Option::None,
                no_extract_hashtags: ::std::option::Option::None,
                no_extract_emojis: ::std::option::Option::None,
                reply_id: ::std::option::Option::None,
                renote_id: ::std::option::Option::None,
                channel_id: ::std::option::Option::None,
                text: ::std::option::Option::None,
                file_ids: ::std::option::Option::None,
                media_ids: ::std::option::Option::None,
                poll: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn visibility(mut self, value: impl ::std::convert::Into<RequestVisibility>) -> Self {
            self.visibility = ::std::option::Option::Some(value.into());
            self
        }
        pub fn visible_user_ids(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.visible_user_ids = ::std::option::Option::Some(value.into());
            self
        }
        pub fn cw(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.cw = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`cw` を `null` として送る。"]
        pub fn cw_null(mut self) -> Self {
            self.cw = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn local_only(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.local_only = ::std::option::Option::Some(value.into());
            self
        }
        pub fn reaction_acceptance(
            mut self,
            value: impl ::std::convert::Into<RequestReactionAcceptance>,
        ) -> Self {
            self.reaction_acceptance =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`reactionAcceptance` を `null` として送る。"]
        pub fn reaction_acceptance_null(mut self) -> Self {
            self.reaction_acceptance = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn no_extract_mentions(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.no_extract_mentions = ::std::option::Option::Some(value.into());
            self
        }
        pub fn no_extract_hashtags(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.no_extract_hashtags = ::std::option::Option::Some(value.into());
            self
        }
        pub fn no_extract_emojis(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.no_extract_emojis = ::std::option::Option::Some(value.into());
            self
        }
        pub fn reply_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.reply_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`replyId` を `null` として送る。"]
        pub fn reply_id_null(mut self) -> Self {
            self.reply_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn renote_id(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.renote_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`renoteId` を `null` として送る。"]
        pub fn renote_id_null(mut self) -> Self {
            self.renote_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn channel_id(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.channel_id =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`channelId` を `null` として送る。"]
        pub fn channel_id_null(mut self) -> Self {
            self.channel_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn text(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.text = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`text` を `null` として送る。"]
        pub fn text_null(mut self) -> Self {
            self.text = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn file_ids(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.file_ids = ::std::option::Option::Some(value.into());
            self
        }
        pub fn media_ids(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Self {
            self.media_ids = ::std::option::Option::Some(value.into());
            self
        }
        pub fn poll(mut self, value: impl ::std::convert::Into<RequestPoll>) -> Self {
            self.poll = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`poll` を `null` として送る。"]
        pub fn poll_null(mut self) -> Self {
            self.poll = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                visibility: self.visibility,
                visible_user_ids: self.visible_user_ids,
                cw: self.cw,
                local_only: self.local_only,
                reaction_acceptance: self.reaction_acceptance,
                no_extract_mentions: self.no_extract_mentions,
                no_extract_hashtags: self.no_extract_hashtags,
                no_extract_emojis: self.no_extract_emojis,
                reply_id: self.reply_id,
                renote_id: self.renote_id,
                channel_id: self.channel_id,
                text: self.text,
                file_ids: self.file_ids,
                media_ids: self.media_ids,
                poll: self.poll,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/delete`"]
pub mod delete {
    #[doc = "`notes/delete`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/delete";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`notes/delete` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notes*"]
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
#[doc = "`notes/drafts/` 以下のエンドポイント"]
pub mod drafts {
    #[doc = "`notes/drafts/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`notes/drafts/count`"]
        pub fn count(&self) -> count::Call<'a> {
            count::Call::new(self.client)
        }
        #[doc = "`notes/drafts/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`notes/drafts/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        #[doc = "`notes/drafts/list`"]
        pub fn list(&self) -> list::Call<'a> {
            list::Call::new(self.client)
        }
        #[doc = "`notes/drafts/update`"]
        pub fn update(&self) -> update::Call<'a> {
            update::Call::new(self.client)
        }
    }
    #[doc = "`notes/drafts/count`"]
    pub mod count {
        #[doc = "`notes/drafts/count`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {}
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/drafts/count";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = f64;
        #[doc = "`notes/drafts/count` のビルダー。"]
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
    #[doc = "`notes/drafts/create`"]
    pub mod create {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestVisibility {
            #[serde(rename = "public")]
            Public,
            #[serde(rename = "home")]
            Home,
            #[serde(rename = "followers")]
            Followers,
            #[serde(rename = "specified")]
            Specified,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestReactionAcceptance {
            #[serde(rename = "likeOnly")]
            LikeOnly,
            #[serde(rename = "likeOnlyForRemote")]
            LikeOnlyForRemote,
            #[serde(rename = "nonSensitiveOnly")]
            NonSensitiveOnly,
            #[serde(rename = "nonSensitiveOnlyForLocalLikeOnlyForRemote")]
            NonSensitiveOnlyForLocalLikeOnlyForRemote,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct RequestPoll {
            pub choices: ::std::vec::Vec<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub multiple: ::std::option::Option<bool>,
            #[serde(
                rename = "expiresAt",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub expires_at: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(
                rename = "expiredAfter",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub expired_after: ::std::option::Option<::std::option::Option<i64>>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            #[serde(rename = "createdDraft")]
            pub created_draft: crate::generated::v2025_10_0::types::NoteDraft,
        }
        #[doc = "`notes/drafts/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub visibility: ::std::option::Option<RequestVisibility>,
            #[serde(
                rename = "visibleUserIds",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub visible_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub cw: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub hashtag: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
            pub local_only: ::std::option::Option<bool>,
            #[serde(
                rename = "reactionAcceptance",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub reaction_acceptance:
                ::std::option::Option<::std::option::Option<RequestReactionAcceptance>>,
            #[serde(
                rename = "replyId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub reply_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "renoteId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub renote_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "channelId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(rename = "fileIds", default, skip_serializing_if = "Option::is_none")]
            pub file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub poll: ::std::option::Option<::std::option::Option<RequestPoll>>,
            #[serde(
                rename = "scheduledAt",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub scheduled_at: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(
                rename = "isActuallyScheduled",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_actually_scheduled: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/drafts/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`notes/drafts/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
            visibility: ::std::option::Option<RequestVisibility>,
            visible_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            cw: ::std::option::Option<::std::option::Option<::std::string::String>>,
            hashtag: ::std::option::Option<::std::option::Option<::std::string::String>>,
            local_only: ::std::option::Option<bool>,
            reaction_acceptance:
                ::std::option::Option<::std::option::Option<RequestReactionAcceptance>>,
            reply_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            renote_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            poll: ::std::option::Option<::std::option::Option<RequestPoll>>,
            scheduled_at: ::std::option::Option<::std::option::Option<i64>>,
            is_actually_scheduled: ::std::option::Option<bool>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    visibility: ::std::option::Option::None,
                    visible_user_ids: ::std::option::Option::None,
                    cw: ::std::option::Option::None,
                    hashtag: ::std::option::Option::None,
                    local_only: ::std::option::Option::None,
                    reaction_acceptance: ::std::option::Option::None,
                    reply_id: ::std::option::Option::None,
                    renote_id: ::std::option::Option::None,
                    channel_id: ::std::option::Option::None,
                    text: ::std::option::Option::None,
                    file_ids: ::std::option::Option::None,
                    poll: ::std::option::Option::None,
                    scheduled_at: ::std::option::Option::None,
                    is_actually_scheduled: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn visibility(
                mut self,
                value: impl ::std::convert::Into<RequestVisibility>,
            ) -> Self {
                self.visibility = ::std::option::Option::Some(value.into());
                self
            }
            pub fn visible_user_ids(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.visible_user_ids = ::std::option::Option::Some(value.into());
                self
            }
            pub fn cw(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.cw = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`cw` を `null` として送る。"]
            pub fn cw_null(mut self) -> Self {
                self.cw = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn hashtag(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.hashtag =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`hashtag` を `null` として送る。"]
            pub fn hashtag_null(mut self) -> Self {
                self.hashtag = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn local_only(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.local_only = ::std::option::Option::Some(value.into());
                self
            }
            pub fn reaction_acceptance(
                mut self,
                value: impl ::std::convert::Into<RequestReactionAcceptance>,
            ) -> Self {
                self.reaction_acceptance =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`reactionAcceptance` を `null` として送る。"]
            pub fn reaction_acceptance_null(mut self) -> Self {
                self.reaction_acceptance = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn reply_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.reply_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`replyId` を `null` として送る。"]
            pub fn reply_id_null(mut self) -> Self {
                self.reply_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn renote_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.renote_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`renoteId` を `null` として送る。"]
            pub fn renote_id_null(mut self) -> Self {
                self.renote_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn channel_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.channel_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`channelId` を `null` として送る。"]
            pub fn channel_id_null(mut self) -> Self {
                self.channel_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn text(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.text = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`text` を `null` として送る。"]
            pub fn text_null(mut self) -> Self {
                self.text = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn file_ids(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.file_ids = ::std::option::Option::Some(value.into());
                self
            }
            pub fn poll(mut self, value: impl ::std::convert::Into<RequestPoll>) -> Self {
                self.poll = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`poll` を `null` として送る。"]
            pub fn poll_null(mut self) -> Self {
                self.poll = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn scheduled_at(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.scheduled_at =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`scheduledAt` を `null` として送る。"]
            pub fn scheduled_at_null(mut self) -> Self {
                self.scheduled_at = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn is_actually_scheduled(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_actually_scheduled = ::std::option::Option::Some(value.into());
                self
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    visibility: self.visibility,
                    visible_user_ids: self.visible_user_ids,
                    cw: self.cw,
                    hashtag: self.hashtag,
                    local_only: self.local_only,
                    reaction_acceptance: self.reaction_acceptance,
                    reply_id: self.reply_id,
                    renote_id: self.renote_id,
                    channel_id: self.channel_id,
                    text: self.text,
                    file_ids: self.file_ids,
                    poll: self.poll,
                    scheduled_at: self.scheduled_at,
                    is_actually_scheduled: self.is_actually_scheduled,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`notes/drafts/delete`"]
    pub mod delete {
        #[doc = "`notes/drafts/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "draftId")]
            pub draft_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/drafts/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/drafts/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `draft_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            draft_id: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    draft_id: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn draft_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    draft_id: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    draft_id: self
                        .draft_id
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`notes/drafts/list`"]
    pub mod list {
        #[doc = "`notes/drafts/list`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
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
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub scheduled: ::std::option::Option<::std::option::Option<bool>>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/drafts/list";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::NoteDraft>;
        #[doc = "`notes/drafts/list` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
            limit: ::std::option::Option<i64>,
            since_id: ::std::option::Option<::std::string::String>,
            until_id: ::std::option::Option<::std::string::String>,
            since_date: ::std::option::Option<i64>,
            until_date: ::std::option::Option<i64>,
            scheduled: ::std::option::Option<::std::option::Option<bool>>,
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
                    scheduled: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn since_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.since_id = ::std::option::Option::Some(value.into());
                self
            }
            pub fn until_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
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
            pub fn scheduled(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.scheduled =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`scheduled` を `null` として送る。"]
            pub fn scheduled_null(mut self) -> Self {
                self.scheduled = ::std::option::Option::Some(::std::option::Option::None);
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
                    scheduled: self.scheduled,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`notes/drafts/update`"]
    pub mod update {
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestVisibility {
            #[serde(rename = "public")]
            Public,
            #[serde(rename = "home")]
            Home,
            #[serde(rename = "followers")]
            Followers,
            #[serde(rename = "specified")]
            Specified,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub enum RequestReactionAcceptance {
            #[serde(rename = "likeOnly")]
            LikeOnly,
            #[serde(rename = "likeOnlyForRemote")]
            LikeOnlyForRemote,
            #[serde(rename = "nonSensitiveOnly")]
            NonSensitiveOnly,
            #[serde(rename = "nonSensitiveOnlyForLocalLikeOnlyForRemote")]
            NonSensitiveOnlyForLocalLikeOnlyForRemote,
            #[doc = "このバージョンのバインディングが知らない値。"]
            #[serde(untagged)]
            Unrecognized(::std::string::String),
        }
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct RequestPoll {
            pub choices: ::std::vec::Vec<::std::string::String>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub multiple: ::std::option::Option<bool>,
            #[serde(
                rename = "expiresAt",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub expires_at: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(
                rename = "expiredAfter",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub expired_after: ::std::option::Option<::std::option::Option<i64>>,
        }
        #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
        pub struct Response {
            #[serde(rename = "updatedDraft")]
            pub updated_draft: crate::generated::v2025_10_0::types::NoteDraft,
        }
        #[doc = "`notes/drafts/update`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "draftId")]
            pub draft_id: ::std::string::String,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub visibility: ::std::option::Option<RequestVisibility>,
            #[serde(
                rename = "visibleUserIds",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub visible_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub cw: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub hashtag: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
            pub local_only: ::std::option::Option<bool>,
            #[serde(
                rename = "reactionAcceptance",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub reaction_acceptance:
                ::std::option::Option<::std::option::Option<RequestReactionAcceptance>>,
            #[serde(
                rename = "replyId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub reply_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "renoteId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub renote_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                rename = "channelId",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            #[serde(rename = "fileIds", default, skip_serializing_if = "Option::is_none")]
            pub file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            #[serde(
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub poll: ::std::option::Option<::std::option::Option<RequestPoll>>,
            #[serde(
                rename = "scheduledAt",
                default,
                skip_serializing_if = "Option::is_none",
                with = "crate::serde_helpers::double_option"
            )]
            pub scheduled_at: ::std::option::Option<::std::option::Option<i64>>,
            #[serde(
                rename = "isActuallyScheduled",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub is_actually_scheduled: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/drafts/update";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        #[doc = "`notes/drafts/update` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `draft_id`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset,)> {
            client: &'a crate::generated::v2025_10_0::Client,
            draft_id: ::std::option::Option<::std::string::String>,
            visibility: ::std::option::Option<RequestVisibility>,
            visible_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            cw: ::std::option::Option<::std::option::Option<::std::string::String>>,
            hashtag: ::std::option::Option<::std::option::Option<::std::string::String>>,
            local_only: ::std::option::Option<bool>,
            reaction_acceptance:
                ::std::option::Option<::std::option::Option<RequestReactionAcceptance>>,
            reply_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            renote_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
            text: ::std::option::Option<::std::option::Option<::std::string::String>>,
            file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            poll: ::std::option::Option<::std::option::Option<RequestPoll>>,
            scheduled_at: ::std::option::Option<::std::option::Option<i64>>,
            is_actually_scheduled: ::std::option::Option<bool>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    draft_id: ::std::option::Option::None,
                    visibility: ::std::option::Option::None,
                    visible_user_ids: ::std::option::Option::None,
                    cw: ::std::option::Option::None,
                    hashtag: ::std::option::Option::None,
                    local_only: ::std::option::Option::None,
                    reaction_acceptance: ::std::option::Option::None,
                    reply_id: ::std::option::Option::None,
                    renote_id: ::std::option::Option::None,
                    channel_id: ::std::option::Option::None,
                    text: ::std::option::Option::None,
                    file_ids: ::std::option::Option::None,
                    poll: ::std::option::Option::None,
                    scheduled_at: ::std::option::Option::None,
                    is_actually_scheduled: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0> Call<'a, (S0,)> {
            pub fn visibility(
                mut self,
                value: impl ::std::convert::Into<RequestVisibility>,
            ) -> Self {
                self.visibility = ::std::option::Option::Some(value.into());
                self
            }
            pub fn visible_user_ids(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.visible_user_ids = ::std::option::Option::Some(value.into());
                self
            }
            pub fn cw(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.cw = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`cw` を `null` として送る。"]
            pub fn cw_null(mut self) -> Self {
                self.cw = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn hashtag(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.hashtag =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`hashtag` を `null` として送る。"]
            pub fn hashtag_null(mut self) -> Self {
                self.hashtag = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn local_only(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.local_only = ::std::option::Option::Some(value.into());
                self
            }
            pub fn reaction_acceptance(
                mut self,
                value: impl ::std::convert::Into<RequestReactionAcceptance>,
            ) -> Self {
                self.reaction_acceptance =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`reactionAcceptance` を `null` として送る。"]
            pub fn reaction_acceptance_null(mut self) -> Self {
                self.reaction_acceptance = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn reply_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.reply_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`replyId` を `null` として送る。"]
            pub fn reply_id_null(mut self) -> Self {
                self.reply_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn renote_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.renote_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`renoteId` を `null` として送る。"]
            pub fn renote_id_null(mut self) -> Self {
                self.renote_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn channel_id(
                mut self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Self {
                self.channel_id =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`channelId` を `null` として送る。"]
            pub fn channel_id_null(mut self) -> Self {
                self.channel_id = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn text(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
                self.text = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`text` を `null` として送る。"]
            pub fn text_null(mut self) -> Self {
                self.text = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn file_ids(
                mut self,
                value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
            ) -> Self {
                self.file_ids = ::std::option::Option::Some(value.into());
                self
            }
            pub fn poll(mut self, value: impl ::std::convert::Into<RequestPoll>) -> Self {
                self.poll = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`poll` を `null` として送る。"]
            pub fn poll_null(mut self) -> Self {
                self.poll = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn scheduled_at(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.scheduled_at =
                    ::std::option::Option::Some(::std::option::Option::Some(value.into()));
                self
            }
            #[doc = "`scheduledAt` を `null` として送る。"]
            pub fn scheduled_at_null(mut self) -> Self {
                self.scheduled_at = ::std::option::Option::Some(::std::option::Option::None);
                self
            }
            pub fn is_actually_scheduled(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.is_actually_scheduled = ::std::option::Option::Some(value.into());
                self
            }
            pub fn draft_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set,)> {
                Call {
                    client: self.client,
                    draft_id: ::std::option::Option::Some(value.into()),
                    visibility: self.visibility,
                    visible_user_ids: self.visible_user_ids,
                    cw: self.cw,
                    hashtag: self.hashtag,
                    local_only: self.local_only,
                    reaction_acceptance: self.reaction_acceptance,
                    reply_id: self.reply_id,
                    renote_id: self.renote_id,
                    channel_id: self.channel_id,
                    text: self.text,
                    file_ids: self.file_ids,
                    poll: self.poll,
                    scheduled_at: self.scheduled_at,
                    is_actually_scheduled: self.is_actually_scheduled,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    draft_id: self
                        .draft_id
                        .expect("required parameter is set by the type state"),
                    visibility: self.visibility,
                    visible_user_ids: self.visible_user_ids,
                    cw: self.cw,
                    hashtag: self.hashtag,
                    local_only: self.local_only,
                    reaction_acceptance: self.reaction_acceptance,
                    reply_id: self.reply_id,
                    renote_id: self.renote_id,
                    channel_id: self.channel_id,
                    text: self.text,
                    file_ids: self.file_ids,
                    poll: self.poll,
                    scheduled_at: self.scheduled_at,
                    is_actually_scheduled: self.is_actually_scheduled,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`notes/favorites/` 以下のエンドポイント"]
pub mod favorites {
    #[doc = "`notes/favorites/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`notes/favorites/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`notes/favorites/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
    }
    #[doc = "`notes/favorites/create`"]
    pub mod create {
        #[doc = "`notes/favorites/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:favorites*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/favorites/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/favorites/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:favorites*"]
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
    #[doc = "`notes/favorites/delete`"]
    pub mod delete {
        #[doc = "`notes/favorites/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:favorites*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/favorites/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/favorites/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:favorites*"]
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
}
#[doc = "`notes/featured`"]
pub mod featured {
    #[doc = "`notes/featured`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "channelId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/featured";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/featured` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        until_id: ::std::option::Option<::std::string::String>,
        channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                limit: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                channel_id: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn until_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.until_id = ::std::option::Option::Some(value.into());
            self
        }
        pub fn channel_id(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.channel_id =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`channelId` を `null` として送る。"]
        pub fn channel_id_null(mut self) -> Self {
            self.channel_id = ::std::option::Option::Some(::std::option::Option::None);
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
                channel_id: self.channel_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/global-timeline`"]
pub mod global_timeline {
    #[doc = "`notes/global-timeline`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
        pub with_files: ::std::option::Option<bool>,
        #[serde(
            rename = "withRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_renotes: ::std::option::Option<bool>,
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
        const PATH: &'static str = "notes/global-timeline";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/global-timeline` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        with_files: ::std::option::Option<bool>,
        with_renotes: ::std::option::Option<bool>,
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
                with_files: ::std::option::Option::None,
                with_renotes: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn with_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_renotes = ::std::option::Option::Some(value.into());
            self
        }
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
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                with_files: self.with_files,
                with_renotes: self.with_renotes,
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
#[doc = "`notes/hybrid-timeline`"]
pub mod hybrid_timeline {
    #[doc = "`notes/hybrid-timeline`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
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
            rename = "allowPartial",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_partial: ::std::option::Option<bool>,
        #[serde(
            rename = "includeMyRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_my_renotes: ::std::option::Option<bool>,
        #[serde(
            rename = "includeRenotedMyNotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_renoted_my_notes: ::std::option::Option<bool>,
        #[serde(
            rename = "includeLocalRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_local_renotes: ::std::option::Option<bool>,
        #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
        pub with_files: ::std::option::Option<bool>,
        #[serde(
            rename = "withRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_renotes: ::std::option::Option<bool>,
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/hybrid-timeline";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/hybrid-timeline` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        allow_partial: ::std::option::Option<bool>,
        include_my_renotes: ::std::option::Option<bool>,
        include_renoted_my_notes: ::std::option::Option<bool>,
        include_local_renotes: ::std::option::Option<bool>,
        with_files: ::std::option::Option<bool>,
        with_renotes: ::std::option::Option<bool>,
        with_replies: ::std::option::Option<bool>,
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
                allow_partial: ::std::option::Option::None,
                include_my_renotes: ::std::option::Option::None,
                include_renoted_my_notes: ::std::option::Option::None,
                include_local_renotes: ::std::option::Option::None,
                with_files: ::std::option::Option::None,
                with_renotes: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
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
        pub fn allow_partial(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.allow_partial = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_my_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_my_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_renoted_my_notes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_renoted_my_notes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_local_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_local_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_replies(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_replies = ::std::option::Option::Some(value.into());
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
                allow_partial: self.allow_partial,
                include_my_renotes: self.include_my_renotes,
                include_renoted_my_notes: self.include_renoted_my_notes,
                include_local_renotes: self.include_local_renotes,
                with_files: self.with_files,
                with_renotes: self.with_renotes,
                with_replies: self.with_replies,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/local-timeline`"]
pub mod local_timeline {
    #[doc = "`notes/local-timeline`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
        pub with_files: ::std::option::Option<bool>,
        #[serde(
            rename = "withRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_renotes: ::std::option::Option<bool>,
        #[serde(
            rename = "withReplies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_replies: ::std::option::Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub limit: ::std::option::Option<i64>,
        #[serde(rename = "sinceId", default, skip_serializing_if = "Option::is_none")]
        pub since_id: ::std::option::Option<::std::string::String>,
        #[serde(rename = "untilId", default, skip_serializing_if = "Option::is_none")]
        pub until_id: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "allowPartial",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_partial: ::std::option::Option<bool>,
        #[serde(rename = "sinceDate", default, skip_serializing_if = "Option::is_none")]
        pub since_date: ::std::option::Option<i64>,
        #[serde(rename = "untilDate", default, skip_serializing_if = "Option::is_none")]
        pub until_date: ::std::option::Option<i64>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/local-timeline";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/local-timeline` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        with_files: ::std::option::Option<bool>,
        with_renotes: ::std::option::Option<bool>,
        with_replies: ::std::option::Option<bool>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        allow_partial: ::std::option::Option<bool>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                with_files: ::std::option::Option::None,
                with_renotes: ::std::option::Option::None,
                with_replies: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                allow_partial: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn with_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_replies(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_replies = ::std::option::Option::Some(value.into());
            self
        }
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
        pub fn allow_partial(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.allow_partial = ::std::option::Option::Some(value.into());
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
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                with_files: self.with_files,
                with_renotes: self.with_renotes,
                with_replies: self.with_replies,
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                allow_partial: self.allow_partial,
                since_date: self.since_date,
                until_date: self.until_date,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/mentions`"]
pub mod mentions {
    #[doc = "`notes/mentions`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub following: ::std::option::Option<bool>,
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
        pub visibility: ::std::option::Option<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/mentions";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/mentions` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        following: ::std::option::Option<bool>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        visibility: ::std::option::Option<::std::string::String>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                following: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                visibility: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn following(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.following = ::std::option::Option::Some(value.into());
            self
        }
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
        pub fn visibility(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.visibility = ::std::option::Option::Some(value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                following: self.following,
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                visibility: self.visibility,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/polls/` 以下のエンドポイント"]
pub mod polls {
    #[doc = "`notes/polls/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`notes/polls/recommendation`"]
        pub fn recommendation(&self) -> recommendation::Call<'a> {
            recommendation::Call::new(self.client)
        }
        #[doc = "`notes/polls/vote`"]
        pub fn vote(&self) -> vote::Call<'a> {
            vote::Call::new(self.client)
        }
    }
    #[doc = "`notes/polls/recommendation`"]
    pub mod recommendation {
        #[doc = "`notes/polls/recommendation`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub limit: ::std::option::Option<i64>,
            #[serde(default, skip_serializing_if = "Option::is_none")]
            pub offset: ::std::option::Option<i64>,
            #[serde(
                rename = "excludeChannels",
                default,
                skip_serializing_if = "Option::is_none"
            )]
            pub exclude_channels: ::std::option::Option<bool>,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/polls/recommendation";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
        #[doc = "`notes/polls/recommendation` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2025_10_0::Client,
            limit: ::std::option::Option<i64>,
            offset: ::std::option::Option<i64>,
            exclude_channels: ::std::option::Option<bool>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    limit: ::std::option::Option::None,
                    offset: ::std::option::Option::None,
                    exclude_channels: ::std::option::Option::None,
                }
            }
        }
        impl<'a> Call<'a> {
            pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.limit = ::std::option::Option::Some(value.into());
                self
            }
            pub fn offset(mut self, value: impl ::std::convert::Into<i64>) -> Self {
                self.offset = ::std::option::Option::Some(value.into());
                self
            }
            pub fn exclude_channels(mut self, value: impl ::std::convert::Into<bool>) -> Self {
                self.exclude_channels = ::std::option::Option::Some(value.into());
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
                    exclude_channels: self.exclude_channels,
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
    #[doc = "`notes/polls/vote`"]
    pub mod vote {
        #[doc = "`notes/polls/vote`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:votes*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
            pub choice: i64,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/polls/vote";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/polls/vote` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:votes*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`、`choice`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            note_id: ::std::option::Option<::std::string::String>,
            choice: ::std::option::Option<i64>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    note_id: ::std::option::Option::None,
                    choice: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn note_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    note_id: ::std::option::Option::Some(value.into()),
                    choice: self.choice,
                    _state: ::std::marker::PhantomData,
                }
            }
            pub fn choice(
                self,
                value: impl ::std::convert::Into<i64>,
            ) -> Call<'a, (S0, crate::builder::Set)> {
                Call {
                    client: self.client,
                    note_id: self.note_id,
                    choice: ::std::option::Option::Some(value.into()),
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
            type Output = ::std::result::Result<Response, crate::Error>;
            type IntoFuture = crate::builder::CallFuture<'a, Response>;
            fn into_future(self) -> Self::IntoFuture {
                let request = Request {
                    note_id: self
                        .note_id
                        .expect("required parameter is set by the type state"),
                    choice: self
                        .choice
                        .expect("required parameter is set by the type state"),
                };
                let client = self.client;
                ::std::boxed::Box::pin(async move { client.call(request).await })
            }
        }
    }
}
#[doc = "`notes/reactions` と、`notes/reactions/` 以下のエンドポイント"]
pub mod reactions {
    #[doc = "`notes/reactions`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
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
        const PATH: &'static str = "notes/reactions";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::NoteReaction>;
    #[doc = "`notes/reactions` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
        r#type: ::std::option::Option<::std::option::Option<::std::string::String>>,
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
                note_id: ::std::option::Option::None,
                r#type: ::std::option::Option::None,
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
        #[doc = "`notes/reactions/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`notes/reactions/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
        pub fn r#type(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.r#type = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`type` を `null` として送る。"]
        pub fn type_null(mut self) -> Self {
            self.r#type = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
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
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
                r#type: self.r#type,
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
                note_id: self
                    .note_id
                    .expect("required parameter is set by the type state"),
                r#type: self.r#type,
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
    #[doc = "`notes/reactions/create`"]
    pub mod create {
        #[doc = "`notes/reactions/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:reactions*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
            pub reaction: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/reactions/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/reactions/create` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:reactions*"]
        #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`、`reaction`。すべて設定すると、呼び出しを await できる。"]
        pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
            client: &'a crate::generated::v2025_10_0::Client,
            note_id: ::std::option::Option<::std::string::String>,
            reaction: ::std::option::Option<::std::string::String>,
            _state: ::std::marker::PhantomData<S>,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
                Call {
                    client,
                    note_id: ::std::option::Option::None,
                    reaction: ::std::option::Option::None,
                    _state: ::std::marker::PhantomData,
                }
            }
        }
        impl<'a, S0, S1> Call<'a, (S0, S1)> {
            pub fn note_id(
                self,
                value: impl ::std::convert::Into<::std::string::String>,
            ) -> Call<'a, (crate::builder::Set, S1)> {
                Call {
                    client: self.client,
                    note_id: ::std::option::Option::Some(value.into()),
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
                    note_id: self.note_id,
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
                    note_id: self
                        .note_id
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
    #[doc = "`notes/reactions/delete`"]
    pub mod delete {
        #[doc = "`notes/reactions/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:reactions*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/reactions/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/reactions/delete` のビルダー。"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:reactions*"]
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
}
#[doc = "`notes/renotes`"]
pub mod renotes {
    #[doc = "`notes/renotes`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
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
        const PATH: &'static str = "notes/renotes";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/renotes` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
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
                note_id: ::std::option::Option::None,
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
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
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
                note_id: self
                    .note_id
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
#[doc = "`notes/replies`"]
pub mod replies {
    #[doc = "`notes/replies`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
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
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/replies";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/replies` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                note_id: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
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
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/search`"]
pub mod search {
    #[doc = "`notes/search`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        pub query: ::std::string::String,
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
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub offset: ::std::option::Option<i64>,
        #[doc = "The local host is represented with `.`."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub host: ::std::option::Option<::std::string::String>,
        #[serde(
            rename = "userId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        #[serde(
            rename = "channelId",
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/search";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/search` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `query`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        query: ::std::option::Option<::std::string::String>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
        offset: ::std::option::Option<i64>,
        host: ::std::option::Option<::std::string::String>,
        user_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        channel_id: ::std::option::Option<::std::option::Option<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                query: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                offset: ::std::option::Option::None,
                host: ::std::option::Option::None,
                user_id: ::std::option::Option::None,
                channel_id: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
        pub fn offset(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.offset = ::std::option::Option::Some(value.into());
            self
        }
        #[doc = "The local host is represented with `.`."]
        pub fn host(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.host = ::std::option::Option::Some(value.into());
            self
        }
        pub fn user_id(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`userId` を `null` として送る。"]
        pub fn user_id_null(mut self) -> Self {
            self.user_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn channel_id(
            mut self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Self {
            self.channel_id =
                ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`channelId` を `null` として送る。"]
        pub fn channel_id_null(mut self) -> Self {
            self.channel_id = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn query(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                query: ::std::option::Option::Some(value.into()),
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
                offset: self.offset,
                host: self.host,
                user_id: self.user_id,
                channel_id: self.channel_id,
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
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
                offset: self.offset,
                host: self.host,
                user_id: self.user_id,
                channel_id: self.channel_id,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/search-by-tag`"]
pub mod search_by_tag {
    #[doc = "`notes/search-by-tag`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須: `tag` / `query` のいずれか。"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub tag: ::std::option::Option<::std::string::String>,
        #[doc = "The outer arrays are chained with OR, the inner arrays are chained with AND."]
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub query: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub reply: ::std::option::Option<::std::option::Option<bool>>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub renote: ::std::option::Option<::std::option::Option<bool>>,
        #[doc = "Only show notes that have attached files."]
        #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
        pub with_files: ::std::option::Option<bool>,
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            with = "crate::serde_helpers::double_option"
        )]
        pub poll: ::std::option::Option<::std::option::Option<bool>>,
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
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/search-by-tag";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/search-by-tag` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        tag: ::std::option::Option<::std::string::String>,
        query: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        reply: ::std::option::Option<::std::option::Option<bool>>,
        renote: ::std::option::Option<::std::option::Option<bool>>,
        with_files: ::std::option::Option<bool>,
        poll: ::std::option::Option<::std::option::Option<bool>>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        limit: ::std::option::Option<i64>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                tag: ::std::option::Option::None,
                query: ::std::option::Option::None,
                reply: ::std::option::Option::None,
                renote: ::std::option::Option::None,
                with_files: ::std::option::Option::None,
                poll: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                limit: ::std::option::Option::None,
            }
        }
    }
    impl<'a> Call<'a> {
        pub fn tag(mut self, value: impl ::std::convert::Into<::std::string::String>) -> Self {
            self.tag = ::std::option::Option::Some(value.into());
            self
        }
        #[doc = "The outer arrays are chained with OR, the inner arrays are chained with AND."]
        pub fn query(
            mut self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
        ) -> Self {
            self.query = ::std::option::Option::Some(value.into());
            self
        }
        pub fn reply(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.reply = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`reply` を `null` として送る。"]
        pub fn reply_null(mut self) -> Self {
            self.reply = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        pub fn renote(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.renote = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`renote` を `null` として送る。"]
        pub fn renote_null(mut self) -> Self {
            self.renote = ::std::option::Option::Some(::std::option::Option::None);
            self
        }
        #[doc = "Only show notes that have attached files."]
        pub fn with_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn poll(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.poll = ::std::option::Option::Some(::std::option::Option::Some(value.into()));
            self
        }
        #[doc = "`poll` を `null` として送る。"]
        pub fn poll_null(mut self) -> Self {
            self.poll = ::std::option::Option::Some(::std::option::Option::None);
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
        pub fn limit(mut self, value: impl ::std::convert::Into<i64>) -> Self {
            self.limit = ::std::option::Option::Some(value.into());
            self
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                tag: self.tag,
                query: self.query,
                reply: self.reply,
                renote: self.renote,
                with_files: self.with_files,
                poll: self.poll,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                limit: self.limit,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/show`"]
pub mod show {
    #[doc = "`notes/show`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/show";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = crate::generated::v2025_10_0::types::Note;
    #[doc = "`notes/show` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
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
#[doc = "`notes/show-partial-bulk`"]
pub mod show_partial_bulk {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseItem {
        pub id: ::std::string::String,
        pub reactions: ::std::collections::HashMap<::std::string::String, f64>,
        #[serde(rename = "reactionEmojis")]
        pub reaction_emojis:
            ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    }
    #[doc = "`notes/show-partial-bulk`"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteIds")]
        pub note_ids: ::std::vec::Vec<::std::string::String>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/show-partial-bulk";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<ResponseItem>;
    #[doc = "`notes/show-partial-bulk` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *No*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_ids`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                note_ids: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
        pub fn note_ids(
            self,
            value: impl ::std::convert::Into<::std::vec::Vec<::std::string::String>>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                note_ids: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                note_ids: self
                    .note_ids
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/state`"]
pub mod state {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct Response {
        #[serde(rename = "isFavorited")]
        pub is_favorited: bool,
        #[serde(rename = "isMutedThread")]
        pub is_muted_thread: bool,
    }
    #[doc = "`notes/state`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/state";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    #[doc = "`notes/state` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
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
#[doc = "`notes/thread-muting/` 以下のエンドポイント"]
pub mod thread_muting {
    #[doc = "`notes/thread-muting/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`notes/thread-muting/create`"]
        pub fn create(&self) -> create::Call<'a> {
            create::Call::new(self.client)
        }
        #[doc = "`notes/thread-muting/delete`"]
        pub fn delete(&self) -> delete::Call<'a> {
            delete::Call::new(self.client)
        }
    }
    #[doc = "`notes/thread-muting/create`"]
    pub mod create {
        #[doc = "`notes/thread-muting/create`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/thread-muting/create";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/thread-muting/create` のビルダー。"]
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
    #[doc = "`notes/thread-muting/delete`"]
    pub mod delete {
        #[doc = "`notes/thread-muting/delete`"]
        #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:account*"]
        #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
        pub struct Request {
            #[serde(rename = "noteId")]
            pub note_id: ::std::string::String,
        }
        impl crate::Endpoint for Request {
            const PATH: &'static str = "notes/thread-muting/delete";
            type Response = Response;
            fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                crate::endpoint::json_body(&self)
            }
        }
        pub type Response = ();
        #[doc = "`notes/thread-muting/delete` のビルダー。"]
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
}
#[doc = "`notes/timeline`"]
pub mod timeline {
    #[doc = "`notes/timeline`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
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
            rename = "allowPartial",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_partial: ::std::option::Option<bool>,
        #[serde(
            rename = "includeMyRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_my_renotes: ::std::option::Option<bool>,
        #[serde(
            rename = "includeRenotedMyNotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_renoted_my_notes: ::std::option::Option<bool>,
        #[serde(
            rename = "includeLocalRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_local_renotes: ::std::option::Option<bool>,
        #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
        pub with_files: ::std::option::Option<bool>,
        #[serde(
            rename = "withRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_renotes: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/timeline";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/timeline` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2025_10_0::Client,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        allow_partial: ::std::option::Option<bool>,
        include_my_renotes: ::std::option::Option<bool>,
        include_renoted_my_notes: ::std::option::Option<bool>,
        include_local_renotes: ::std::option::Option<bool>,
        with_files: ::std::option::Option<bool>,
        with_renotes: ::std::option::Option<bool>,
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
                allow_partial: ::std::option::Option::None,
                include_my_renotes: ::std::option::Option::None,
                include_renoted_my_notes: ::std::option::Option::None,
                include_local_renotes: ::std::option::Option::None,
                with_files: ::std::option::Option::None,
                with_renotes: ::std::option::Option::None,
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
        pub fn allow_partial(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.allow_partial = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_my_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_my_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_renoted_my_notes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_renoted_my_notes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_local_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_local_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_renotes = ::std::option::Option::Some(value.into());
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
                allow_partial: self.allow_partial,
                include_my_renotes: self.include_my_renotes,
                include_renoted_my_notes: self.include_renoted_my_notes,
                include_local_renotes: self.include_local_renotes,
                with_files: self.with_files,
                with_renotes: self.with_renotes,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/translate`"]
pub mod translate {
    #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
    pub struct ResponseBody {
        #[serde(rename = "sourceLang")]
        pub source_lang: ::std::string::String,
        pub text: ::std::string::String,
    }
    #[doc = "`notes/translate`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
        #[serde(rename = "targetLang")]
        pub target_lang: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/translate";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::option::Option<ResponseBody>;
    #[doc = "`notes/translate` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `note_id`、`target_lang`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset, crate::builder::Unset)> {
        client: &'a crate::generated::v2025_10_0::Client,
        note_id: ::std::option::Option<::std::string::String>,
        target_lang: ::std::option::Option<::std::string::String>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                note_id: ::std::option::Option::None,
                target_lang: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0, S1> Call<'a, (S0, S1)> {
        pub fn note_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set, S1)> {
            Call {
                client: self.client,
                note_id: ::std::option::Option::Some(value.into()),
                target_lang: self.target_lang,
                _state: ::std::marker::PhantomData,
            }
        }
        pub fn target_lang(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (S0, crate::builder::Set)> {
            Call {
                client: self.client,
                note_id: self.note_id,
                target_lang: ::std::option::Option::Some(value.into()),
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set, crate::builder::Set)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                note_id: self
                    .note_id
                    .expect("required parameter is set by the type state"),
                target_lang: self
                    .target_lang
                    .expect("required parameter is set by the type state"),
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
#[doc = "`notes/unrenote`"]
pub mod unrenote {
    #[doc = "`notes/unrenote`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notes*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "noteId")]
        pub note_id: ::std::string::String,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/unrenote";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ();
    #[doc = "`notes/unrenote` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *write:notes*"]
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
#[doc = "`notes/user-list-timeline`"]
pub mod user_list_timeline {
    #[doc = "`notes/user-list-timeline`"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
    pub struct Request {
        #[serde(rename = "listId")]
        pub list_id: ::std::string::String,
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
            rename = "allowPartial",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_partial: ::std::option::Option<bool>,
        #[serde(
            rename = "includeMyRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_my_renotes: ::std::option::Option<bool>,
        #[serde(
            rename = "includeRenotedMyNotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_renoted_my_notes: ::std::option::Option<bool>,
        #[serde(
            rename = "includeLocalRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub include_local_renotes: ::std::option::Option<bool>,
        #[serde(
            rename = "withRenotes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub with_renotes: ::std::option::Option<bool>,
        #[doc = "Only show notes that have attached files."]
        #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
        pub with_files: ::std::option::Option<bool>,
    }
    impl crate::Endpoint for Request {
        const PATH: &'static str = "notes/user-list-timeline";
        type Response = Response;
        fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
            crate::endpoint::json_body(&self)
        }
    }
    pub type Response = ::std::vec::Vec<crate::generated::v2025_10_0::types::Note>;
    #[doc = "`notes/user-list-timeline` のビルダー。"]
    #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:account*"]
    #[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
    pub struct Call<'a, S = (crate::builder::Unset,)> {
        client: &'a crate::generated::v2025_10_0::Client,
        list_id: ::std::option::Option<::std::string::String>,
        limit: ::std::option::Option<i64>,
        since_id: ::std::option::Option<::std::string::String>,
        until_id: ::std::option::Option<::std::string::String>,
        since_date: ::std::option::Option<i64>,
        until_date: ::std::option::Option<i64>,
        allow_partial: ::std::option::Option<bool>,
        include_my_renotes: ::std::option::Option<bool>,
        include_renoted_my_notes: ::std::option::Option<bool>,
        include_local_renotes: ::std::option::Option<bool>,
        with_renotes: ::std::option::Option<bool>,
        with_files: ::std::option::Option<bool>,
        _state: ::std::marker::PhantomData<S>,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2025_10_0::Client) -> Self {
            Call {
                client,
                list_id: ::std::option::Option::None,
                limit: ::std::option::Option::None,
                since_id: ::std::option::Option::None,
                until_id: ::std::option::Option::None,
                since_date: ::std::option::Option::None,
                until_date: ::std::option::Option::None,
                allow_partial: ::std::option::Option::None,
                include_my_renotes: ::std::option::Option::None,
                include_renoted_my_notes: ::std::option::Option::None,
                include_local_renotes: ::std::option::Option::None,
                with_renotes: ::std::option::Option::None,
                with_files: ::std::option::Option::None,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a, S0> Call<'a, (S0,)> {
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
        pub fn allow_partial(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.allow_partial = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_my_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_my_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_renoted_my_notes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_renoted_my_notes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn include_local_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.include_local_renotes = ::std::option::Option::Some(value.into());
            self
        }
        pub fn with_renotes(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_renotes = ::std::option::Option::Some(value.into());
            self
        }
        #[doc = "Only show notes that have attached files."]
        pub fn with_files(mut self, value: impl ::std::convert::Into<bool>) -> Self {
            self.with_files = ::std::option::Option::Some(value.into());
            self
        }
        pub fn list_id(
            self,
            value: impl ::std::convert::Into<::std::string::String>,
        ) -> Call<'a, (crate::builder::Set,)> {
            Call {
                client: self.client,
                list_id: ::std::option::Option::Some(value.into()),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                allow_partial: self.allow_partial,
                include_my_renotes: self.include_my_renotes,
                include_renoted_my_notes: self.include_renoted_my_notes,
                include_local_renotes: self.include_local_renotes,
                with_renotes: self.with_renotes,
                with_files: self.with_files,
                _state: ::std::marker::PhantomData,
            }
        }
    }
    impl<'a> ::std::future::IntoFuture for Call<'a, (crate::builder::Set,)> {
        type Output = ::std::result::Result<Response, crate::Error>;
        type IntoFuture = crate::builder::CallFuture<'a, Response>;
        fn into_future(self) -> Self::IntoFuture {
            let request = Request {
                list_id: self
                    .list_id
                    .expect("required parameter is set by the type state"),
                limit: self.limit,
                since_id: self.since_id,
                until_id: self.until_id,
                since_date: self.since_date,
                until_date: self.until_date,
                allow_partial: self.allow_partial,
                include_my_renotes: self.include_my_renotes,
                include_renoted_my_notes: self.include_renoted_my_notes,
                include_local_renotes: self.include_local_renotes,
                with_renotes: self.with_renotes,
                with_files: self.with_files,
            };
            let client = self.client;
            ::std::boxed::Box::pin(async move { client.call(request).await })
        }
    }
}
