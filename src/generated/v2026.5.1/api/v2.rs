#![doc = "`v2/` 以下のエンドポイント"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`v2/` 以下のエンドポイント"]
pub struct Call<'a> {
    client: &'a crate::generated::v2026_5_1::Client,
}
impl<'a> Call<'a> {
    pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
        Call { client }
    }
}
impl<'a> Call<'a> {
    #[doc = "`v2/admin/` 以下のエンドポイント"]
    pub fn admin(&self) -> admin::Call<'a> {
        admin::Call::new(self.client)
    }
}
#[doc = "`v2/admin/` 以下のエンドポイント"]
pub mod admin {
    #[doc = "`v2/admin/` 以下のエンドポイント"]
    pub struct Call<'a> {
        client: &'a crate::generated::v2026_5_1::Client,
    }
    impl<'a> Call<'a> {
        pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
            Call { client }
        }
    }
    impl<'a> Call<'a> {
        #[doc = "`v2/admin/emoji/` 以下のエンドポイント"]
        pub fn emoji(&self) -> emoji::Call<'a> {
            emoji::Call::new(self.client)
        }
    }
    #[doc = "`v2/admin/emoji/` 以下のエンドポイント"]
    pub mod emoji {
        #[doc = "`v2/admin/emoji/` 以下のエンドポイント"]
        pub struct Call<'a> {
            client: &'a crate::generated::v2026_5_1::Client,
        }
        impl<'a> Call<'a> {
            pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
                Call { client }
            }
        }
        impl<'a> Call<'a> {
            #[doc = "`v2/admin/emoji/list`"]
            pub fn list(&self) -> list::Call<'a> {
                list::Call::new(self.client)
            }
        }
        #[doc = "`v2/admin/emoji/list`"]
        pub mod list {
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub enum RequestQueryHostType {
                #[serde(rename = "local")]
                Local,
                #[serde(rename = "remote")]
                Remote,
                #[serde(rename = "all")]
                All,
                #[doc = "このバージョンのバインディングが知らない値。"]
                #[serde(untagged)]
                Unrecognized(::std::string::String),
            }
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct RequestQuery {
                #[serde(
                    rename = "updatedAtFrom",
                    default,
                    skip_serializing_if = "Option::is_none"
                )]
                pub updated_at_from: ::std::option::Option<::std::string::String>,
                #[serde(
                    rename = "updatedAtTo",
                    default,
                    skip_serializing_if = "Option::is_none"
                )]
                pub updated_at_to: ::std::option::Option<::std::string::String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub name: ::std::option::Option<::std::string::String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub host: ::std::option::Option<::std::string::String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub uri: ::std::option::Option<::std::string::String>,
                #[serde(rename = "publicUrl", default, skip_serializing_if = "Option::is_none")]
                pub public_url: ::std::option::Option<::std::string::String>,
                #[serde(
                    rename = "originalUrl",
                    default,
                    skip_serializing_if = "Option::is_none"
                )]
                pub original_url: ::std::option::Option<::std::string::String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub r#type: ::std::option::Option<::std::string::String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub aliases: ::std::option::Option<::std::string::String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub category: ::std::option::Option<::std::string::String>,
                #[serde(default, skip_serializing_if = "Option::is_none")]
                pub license: ::std::option::Option<::std::string::String>,
                #[serde(
                    rename = "isSensitive",
                    default,
                    skip_serializing_if = "Option::is_none"
                )]
                pub is_sensitive: ::std::option::Option<bool>,
                #[serde(rename = "localOnly", default, skip_serializing_if = "Option::is_none")]
                pub local_only: ::std::option::Option<bool>,
                #[serde(rename = "hostType", default, skip_serializing_if = "Option::is_none")]
                pub host_type: ::std::option::Option<RequestQueryHostType>,
                #[serde(rename = "roleIds", default, skip_serializing_if = "Option::is_none")]
                pub role_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
            }
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub enum RequestSortKeysItem {
                #[serde(rename = "+id")]
                PlusId,
                #[serde(rename = "-id")]
                MinusId,
                #[serde(rename = "+updatedAt")]
                PlusUpdatedAt,
                #[serde(rename = "-updatedAt")]
                MinusUpdatedAt,
                #[serde(rename = "+name")]
                PlusName,
                #[serde(rename = "-name")]
                MinusName,
                #[serde(rename = "+host")]
                PlusHost,
                #[serde(rename = "-host")]
                MinusHost,
                #[serde(rename = "+uri")]
                PlusUri,
                #[serde(rename = "-uri")]
                MinusUri,
                #[serde(rename = "+publicUrl")]
                PlusPublicUrl,
                #[serde(rename = "-publicUrl")]
                MinusPublicUrl,
                #[serde(rename = "+type")]
                PlusType,
                #[serde(rename = "-type")]
                MinusType,
                #[serde(rename = "+aliases")]
                PlusAliases,
                #[serde(rename = "-aliases")]
                MinusAliases,
                #[serde(rename = "+category")]
                PlusCategory,
                #[serde(rename = "-category")]
                MinusCategory,
                #[serde(rename = "+license")]
                PlusLicense,
                #[serde(rename = "-license")]
                MinusLicense,
                #[serde(rename = "+isSensitive")]
                PlusIsSensitive,
                #[serde(rename = "-isSensitive")]
                MinusIsSensitive,
                #[serde(rename = "+localOnly")]
                PlusLocalOnly,
                #[serde(rename = "-localOnly")]
                MinusLocalOnly,
                #[serde(rename = "+roleIdsThatCanBeUsedThisEmojiAsReaction")]
                PlusRoleIdsThatCanBeUsedThisEmojiAsReaction,
                #[serde(rename = "-roleIdsThatCanBeUsedThisEmojiAsReaction")]
                MinusRoleIdsThatCanBeUsedThisEmojiAsReaction,
                #[doc = "このバージョンのバインディングが知らない値。"]
                #[serde(untagged)]
                Unrecognized(::std::string::String),
            }
            #[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
            pub struct Response {
                pub emojis: ::std::vec::Vec<crate::generated::v2026_5_1::types::EmojiDetailedAdmin>,
                pub count: i64,
                #[serde(rename = "allCount")]
                pub all_count: i64,
                #[serde(rename = "allPages")]
                pub all_pages: i64,
            }
            #[doc = "`v2/admin/emoji/list`"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
            #[derive(
                Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize,
            )]
            pub struct Request {
                #[serde(
                    default,
                    skip_serializing_if = "Option::is_none",
                    with = "crate::serde_helpers::double_option"
                )]
                pub query: ::std::option::Option<::std::option::Option<RequestQuery>>,
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
                pub page: ::std::option::Option<i64>,
                #[serde(rename = "sortKeys", default, skip_serializing_if = "Option::is_none")]
                pub sort_keys: ::std::option::Option<::std::vec::Vec<RequestSortKeysItem>>,
            }
            impl crate::Endpoint for Request {
                const PATH: &'static str = "v2/admin/emoji/list";
                type Response = Response;
                fn into_body(self) -> ::std::result::Result<crate::Body, crate::Error> {
                    crate::endpoint::json_body(&self)
                }
            }
            #[doc = "`v2/admin/emoji/list` のビルダー。"]
            #[doc = "No description provided.\n\n**Credential required**: *Yes* / **Permission**: *read:admin:emoji*"]
            pub struct Call<'a> {
                client: &'a crate::generated::v2026_5_1::Client,
                query: ::std::option::Option<::std::option::Option<RequestQuery>>,
                since_id: ::std::option::Option<::std::string::String>,
                until_id: ::std::option::Option<::std::string::String>,
                since_date: ::std::option::Option<i64>,
                until_date: ::std::option::Option<i64>,
                limit: ::std::option::Option<i64>,
                page: ::std::option::Option<i64>,
                sort_keys: ::std::option::Option<::std::vec::Vec<RequestSortKeysItem>>,
            }
            impl<'a> Call<'a> {
                pub(crate) fn new(client: &'a crate::generated::v2026_5_1::Client) -> Self {
                    Call {
                        client,
                        query: ::std::option::Option::None,
                        since_id: ::std::option::Option::None,
                        until_id: ::std::option::Option::None,
                        since_date: ::std::option::Option::None,
                        until_date: ::std::option::Option::None,
                        limit: ::std::option::Option::None,
                        page: ::std::option::Option::None,
                        sort_keys: ::std::option::Option::None,
                    }
                }
            }
            impl<'a> Call<'a> {
                pub fn query(self, value: impl ::std::convert::Into<RequestQuery>) -> Self {
                    self.maybe_query(::std::option::Option::Some(value))
                }
                #[doc = "`Some` なら `query` を設定し、`None` なら設定を消して送らない。`null` を送るには [`query_null`](Self::query_null) を使う。"]
                pub fn maybe_query(
                    mut self,
                    value: ::std::option::Option<impl ::std::convert::Into<RequestQuery>>,
                ) -> Self {
                    self.query = value.map(|value| ::std::option::Option::Some(value.into()));
                    self
                }
                #[doc = "`query` を `null` として送る。"]
                pub fn query_null(mut self) -> Self {
                    self.query = ::std::option::Option::Some(::std::option::Option::None);
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
                pub fn page(self, value: impl ::std::convert::Into<i64>) -> Self {
                    self.maybe_page(::std::option::Option::Some(value))
                }
                #[doc = "`Some` なら `page` を設定し、`None` なら設定を消して送らない。"]
                pub fn maybe_page(
                    mut self,
                    value: ::std::option::Option<impl ::std::convert::Into<i64>>,
                ) -> Self {
                    self.page = value.map(|value| value.into());
                    self
                }
                pub fn sort_keys(
                    self,
                    value: impl ::std::convert::Into<::std::vec::Vec<RequestSortKeysItem>>,
                ) -> Self {
                    self.maybe_sort_keys(::std::option::Option::Some(value))
                }
                #[doc = "`Some` なら `sortKeys` を設定し、`None` なら設定を消して送らない。"]
                pub fn maybe_sort_keys(
                    mut self,
                    value: ::std::option::Option<
                        impl ::std::convert::Into<::std::vec::Vec<RequestSortKeysItem>>,
                    >,
                ) -> Self {
                    self.sort_keys = value.map(|value| value.into());
                    self
                }
            }
            impl<'a> ::std::future::IntoFuture for Call<'a> {
                type Output = ::std::result::Result<Response, crate::Error>;
                type IntoFuture = crate::builder::CallFuture<'a, Response>;
                fn into_future(self) -> Self::IntoFuture {
                    let request = Request {
                        query: self.query,
                        since_id: self.since_id,
                        until_id: self.until_id,
                        since_date: self.since_date,
                        until_date: self.until_date,
                        limit: self.limit,
                        page: self.page,
                        sort_keys: self.sort_keys,
                    };
                    let client = self.client;
                    ::std::boxed::Box::pin(async move { client.call(request).await })
                }
            }
        }
    }
}
