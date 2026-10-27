#![doc = "エンティティの型（`api.json` の `components.schemas`）。"]
#![allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserLiteAvatarDecorationsItem {
    pub id: ::std::string::String,
    #[serde(default)]
    pub angle: ::std::option::Option<f64>,
    #[serde(rename = "flipH", default)]
    pub flip_h: ::std::option::Option<bool>,
    pub url: ::std::string::String,
    #[serde(rename = "offsetX", default)]
    pub offset_x: ::std::option::Option<f64>,
    #[serde(rename = "offsetY", default)]
    pub offset_y: ::std::option::Option<f64>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserLiteInstance {
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "softwareName")]
    pub software_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "softwareVersion")]
    pub software_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "faviconUrl")]
    pub favicon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "themeColor")]
    pub theme_color: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum UserLiteOnlineStatus {
    #[serde(rename = "unknown")]
    Unknown,
    #[serde(rename = "online")]
    Online,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "offline")]
    Offline,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserLiteBadgeRolesItem {
    pub name: ::std::string::String,
    #[serde(rename = "iconUrl")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "displayOrder")]
    pub display_order: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserLite {
    pub id: ::std::string::String,
    pub name: ::std::option::Option<::std::string::String>,
    pub username: ::std::string::String,
    #[doc = "The local host is represented with `null`."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(rename = "avatarBlurhash")]
    pub avatar_blurhash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "avatarDecorations")]
    pub avatar_decorations:
        ::std::vec::Vec<crate::generated::latest::types::UserLiteAvatarDecorationsItem>,
    #[serde(rename = "isBot", default)]
    pub is_bot: ::std::option::Option<bool>,
    #[serde(rename = "isCat", default)]
    pub is_cat: ::std::option::Option<bool>,
    #[serde(rename = "requireSigninToViewContents", default)]
    pub require_signin_to_view_contents: ::std::option::Option<bool>,
    #[serde(rename = "makeNotesFollowersOnlyBefore", default)]
    pub make_notes_followers_only_before: ::std::option::Option<f64>,
    #[serde(rename = "makeNotesHiddenBefore", default)]
    pub make_notes_hidden_before: ::std::option::Option<f64>,
    #[serde(default)]
    pub instance: ::std::option::Option<crate::generated::latest::types::UserLiteInstance>,
    pub emojis: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(rename = "onlineStatus")]
    pub online_status: crate::generated::latest::types::UserLiteOnlineStatus,
    #[serde(rename = "badgeRoles", default)]
    pub badge_roles: ::std::option::Option<
        ::std::vec::Vec<crate::generated::latest::types::UserLiteBadgeRolesItem>,
    >,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserDetailedNotMeOnlyFieldsItem {
    pub name: ::std::string::String,
    pub value: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum UserDetailedNotMeOnlyFollowingVisibility {
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
pub enum UserDetailedNotMeOnlyFollowersVisibility {
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
pub enum UserDetailedNotMeOnlyChatScope {
    #[serde(rename = "everyone")]
    Everyone,
    #[serde(rename = "following")]
    Following,
    #[serde(rename = "followers")]
    Followers,
    #[serde(rename = "mutual")]
    Mutual,
    #[serde(rename = "none")]
    None,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum UserDetailedNotMeOnlyNotify {
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "none")]
    None,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserDetailedNotMeOnly {
    pub url: ::std::option::Option<::std::string::String>,
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "movedTo")]
    pub moved_to: ::std::option::Option<::std::string::String>,
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "lastFetchedAt")]
    pub last_fetched_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerBlurhash")]
    pub banner_blurhash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "isSilenced")]
    pub is_silenced: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    pub description: ::std::option::Option<::std::string::String>,
    pub location: ::std::option::Option<::std::string::String>,
    pub birthday: ::std::option::Option<::std::string::String>,
    pub lang: ::std::option::Option<::std::string::String>,
    pub fields: ::std::vec::Vec<crate::generated::latest::types::UserDetailedNotMeOnlyFieldsItem>,
    #[serde(rename = "verifiedLinks")]
    pub verified_links: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "followersCount")]
    pub followers_count: f64,
    #[serde(rename = "followingCount")]
    pub following_count: f64,
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "pinnedNoteIds")]
    pub pinned_note_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "pinnedNotes")]
    pub pinned_notes: ::std::vec::Vec<crate::generated::latest::types::Note>,
    #[serde(rename = "pinnedPageId")]
    pub pinned_page_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pinnedPage")]
    pub pinned_page: ::std::option::Option<crate::generated::latest::types::Page>,
    #[serde(rename = "publicReactions")]
    pub public_reactions: bool,
    #[serde(rename = "followingVisibility")]
    pub following_visibility:
        crate::generated::latest::types::UserDetailedNotMeOnlyFollowingVisibility,
    #[serde(rename = "followersVisibility")]
    pub followers_visibility:
        crate::generated::latest::types::UserDetailedNotMeOnlyFollowersVisibility,
    #[serde(rename = "chatScope")]
    pub chat_scope: crate::generated::latest::types::UserDetailedNotMeOnlyChatScope,
    #[serde(rename = "canChat")]
    pub can_chat: bool,
    pub roles: ::std::vec::Vec<crate::generated::latest::types::RoleLite>,
    #[serde(rename = "followedMessage", default)]
    pub followed_message: ::std::option::Option<::std::string::String>,
    pub memo: ::std::option::Option<::std::string::String>,
    #[serde(rename = "moderationNote", default)]
    pub moderation_note: ::std::option::Option<::std::string::String>,
    #[serde(rename = "twoFactorEnabled", default)]
    pub two_factor_enabled: ::std::option::Option<bool>,
    #[serde(rename = "usePasswordLessLogin", default)]
    pub use_password_less_login: ::std::option::Option<bool>,
    #[serde(rename = "securityKeys", default)]
    pub security_keys: ::std::option::Option<bool>,
    #[serde(rename = "isFollowing", default)]
    pub is_following: ::std::option::Option<bool>,
    #[serde(rename = "isFollowed", default)]
    pub is_followed: ::std::option::Option<bool>,
    #[serde(rename = "hasPendingFollowRequestFromYou", default)]
    pub has_pending_follow_request_from_you: ::std::option::Option<bool>,
    #[serde(rename = "hasPendingFollowRequestToYou", default)]
    pub has_pending_follow_request_to_you: ::std::option::Option<bool>,
    #[serde(rename = "isBlocking", default)]
    pub is_blocking: ::std::option::Option<bool>,
    #[serde(rename = "isBlocked", default)]
    pub is_blocked: ::std::option::Option<bool>,
    #[serde(rename = "isMuted", default)]
    pub is_muted: ::std::option::Option<bool>,
    #[serde(rename = "isRenoteMuted", default)]
    pub is_renote_muted: ::std::option::Option<bool>,
    #[serde(default)]
    pub notify: ::std::option::Option<crate::generated::latest::types::UserDetailedNotMeOnlyNotify>,
    #[serde(rename = "withReplies", default)]
    pub with_replies: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyTwoFactorBackupCodesStock {
    #[serde(rename = "full")]
    Full,
    #[serde(rename = "partial")]
    Partial,
    #[serde(rename = "none")]
    None,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigNoteListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigNoteList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigNoteListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigNoteObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigNoteObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigNoteObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigNote {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigNoteList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigNoteObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigFollowListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigFollowList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigFollowListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigFollowObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigFollowObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigFollowObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigFollow {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigFollowList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigFollowObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigMentionListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigMentionList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigMentionListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigMentionObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigMentionObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigMentionObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigMention {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigMentionList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigMentionObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigReplyListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigReplyList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReplyListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigReplyObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigReplyObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReplyObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigReply {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReplyList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReplyObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigRenoteListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigRenoteList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigRenoteListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigRenoteObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigRenoteObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigRenoteObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigRenote {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigRenoteList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigRenoteObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigQuoteListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigQuoteList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigQuoteListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigQuoteObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigQuoteObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigQuoteObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigQuote {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigQuoteList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigQuoteObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigReactionListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigReactionList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReactionListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigReactionObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigReactionObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReactionObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigReaction {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReactionList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigReactionObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigPollEndedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigPollEndedList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigPollEndedListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigPollEndedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigPollEndedObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigPollEndedObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigPollEnded {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigPollEndedList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigPollEndedObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigScheduledNotePosted {
    List (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedList) , Object (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostedObject) , # [doc = "このバージョンのバインディングが知らない形。"] Unrecognized (:: serde_json :: Value) , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailed {
    List (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedList) , Object (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailedObject) , # [doc = "このバージョンのバインディングが知らない形。"] Unrecognized (:: serde_json :: Value) , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequest {
    List (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestList) , Object (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequestObject) , # [doc = "このバージョンのバインディングが知らない形。"] Unrecognized (:: serde_json :: Value) , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigFollowRequestAccepted {
    List (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedList) , Object (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigFollowRequestAcceptedObject) , # [doc = "このバージョンのバインディングが知らない形。"] Unrecognized (:: serde_json :: Value) , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigRoleAssignedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigRoleAssignedList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigRoleAssignedListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigRoleAssignedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigRoleAssignedObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigRoleAssignedObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigRoleAssigned {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigRoleAssignedList),
    Object(
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigRoleAssignedObject,
    ),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceived {
    List (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedList) , Object (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceivedObject) , # [doc = "このバージョンのバインディングが知らない形。"] Unrecognized (:: serde_json :: Value) , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigAchievementEarnedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigAchievementEarnedList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigAchievementEarnedListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigAchievementEarnedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigAchievementEarnedObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigAchievementEarnedObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigAchievementEarned {
    List (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigAchievementEarnedList) , Object (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigAchievementEarnedObject) , # [doc = "このバージョンのバインディングが知らない形。"] Unrecognized (:: serde_json :: Value) , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigAppListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigAppList {
    pub r#type: crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigAppListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigAppObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigAppObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigAppObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigApp {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigAppList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigAppObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigTestListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigTestList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigTestListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigTestObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigTestObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigTestObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigTest {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigTestList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigTestObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigLoginListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigLoginList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigLoginListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigLoginObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigLoginObject {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigLoginObjectType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigLogin {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigLoginList),
    Object(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigLoginObject),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigCreateTokenListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigCreateTokenList {
    pub r#type:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigCreateTokenListType,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigCreateTokenObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigCreateTokenObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigCreateTokenObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigCreateToken {
    List(crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigCreateTokenList),
    Object(
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfigCreateTokenObject,
    ),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigExportCompletedListType {
    #[serde(rename = "list")]
    List,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfigExportCompletedList { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigExportCompletedListType , # [serde (rename = "userListId")] pub user_list_id : :: std :: string :: String , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MeDetailedOnlyNotificationRecieveConfigExportCompletedObjectType {
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
pub struct MeDetailedOnlyNotificationRecieveConfigExportCompletedObject { pub r#type : crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigExportCompletedObjectType , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum MeDetailedOnlyNotificationRecieveConfigExportCompleted {
    List (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigExportCompletedList) , Object (crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigExportCompletedObject) , # [doc = "このバージョンのバインディングが知らない形。"] Unrecognized (:: serde_json :: Value) , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlyNotificationRecieveConfig { # [serde (default)] pub note : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigNote > , # [serde (default)] pub follow : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigFollow > , # [serde (default)] pub mention : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigMention > , # [serde (default)] pub reply : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigReply > , # [serde (default)] pub renote : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigRenote > , # [serde (default)] pub quote : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigQuote > , # [serde (default)] pub reaction : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigReaction > , # [serde (rename = "pollEnded" , default)] pub poll_ended : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigPollEnded > , # [serde (rename = "scheduledNotePosted" , default)] pub scheduled_note_posted : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePosted > , # [serde (rename = "scheduledNotePostFailed" , default)] pub scheduled_note_post_failed : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigScheduledNotePostFailed > , # [serde (rename = "receiveFollowRequest" , default)] pub receive_follow_request : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigReceiveFollowRequest > , # [serde (rename = "followRequestAccepted" , default)] pub follow_request_accepted : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigFollowRequestAccepted > , # [serde (rename = "roleAssigned" , default)] pub role_assigned : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigRoleAssigned > , # [serde (rename = "chatRoomInvitationReceived" , default)] pub chat_room_invitation_received : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigChatRoomInvitationReceived > , # [serde (rename = "achievementEarned" , default)] pub achievement_earned : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigAchievementEarned > , # [serde (default)] pub app : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigApp > , # [serde (default)] pub test : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigTest > , # [serde (default)] pub login : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigLogin > , # [serde (rename = "createToken" , default)] pub create_token : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigCreateToken > , # [serde (rename = "exportCompleted" , default)] pub export_completed : :: std :: option :: Option < crate :: generated :: latest :: types :: MeDetailedOnlyNotificationRecieveConfigExportCompleted > , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnlySecurityKeysListItem {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "lastUsed")]
    pub last_used: ::chrono::DateTime<::chrono::Utc>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailedOnly {
    #[serde(rename = "avatarId")]
    pub avatar_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerId")]
    pub banner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "followedMessage")]
    pub followed_message: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isModerator")]
    pub is_moderator: bool,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "injectFeaturedNote")]
    pub inject_featured_note: bool,
    #[serde(rename = "receiveAnnouncementEmail")]
    pub receive_announcement_email: bool,
    #[serde(rename = "alwaysMarkNsfw")]
    pub always_mark_nsfw: bool,
    #[serde(rename = "autoSensitive")]
    pub auto_sensitive: bool,
    #[serde(rename = "carefulBot")]
    pub careful_bot: bool,
    #[serde(rename = "autoAcceptFollowed")]
    pub auto_accept_followed: bool,
    #[serde(rename = "followApprovalLocalSeconds")]
    pub follow_approval_local_seconds: ::std::option::Option<f64>,
    #[serde(rename = "followApprovalRemoteSeconds")]
    pub follow_approval_remote_seconds: ::std::option::Option<f64>,
    #[serde(rename = "noCrawle")]
    pub no_crawle: bool,
    #[serde(rename = "preventAiLearning")]
    pub prevent_ai_learning: bool,
    #[serde(rename = "isExplorable")]
    pub is_explorable: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "twoFactorBackupCodesStock")]
    pub two_factor_backup_codes_stock:
        crate::generated::latest::types::MeDetailedOnlyTwoFactorBackupCodesStock,
    #[serde(rename = "hideOnlineStatus")]
    pub hide_online_status: bool,
    #[serde(rename = "hasUnreadSpecifiedNotes")]
    pub has_unread_specified_notes: bool,
    #[serde(rename = "hasUnreadMentions")]
    pub has_unread_mentions: bool,
    #[serde(rename = "hasUnreadAnnouncement")]
    pub has_unread_announcement: bool,
    #[serde(rename = "unreadAnnouncements")]
    pub unread_announcements: ::std::vec::Vec<crate::generated::latest::types::Announcement>,
    #[serde(rename = "hasUnreadAntenna")]
    pub has_unread_antenna: bool,
    #[serde(rename = "hasUnreadChannel")]
    pub has_unread_channel: bool,
    #[serde(rename = "hasUnreadChatMessages")]
    pub has_unread_chat_messages: bool,
    #[serde(rename = "hasUnreadNotification")]
    pub has_unread_notification: bool,
    #[serde(rename = "hasPendingReceivedFollowRequest")]
    pub has_pending_received_follow_request: bool,
    #[serde(rename = "unreadNotificationsCount")]
    pub unread_notifications_count: f64,
    #[serde(rename = "mutedWords")]
    pub muted_words: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "hardMutedWords")]
    pub hard_muted_words: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mutedInstances")]
    pub muted_instances: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "notificationRecieveConfig")]
    pub notification_recieve_config:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfig,
    #[serde(rename = "emailNotificationTypes")]
    pub email_notification_types: ::std::vec::Vec<::std::string::String>,
    pub achievements: ::std::vec::Vec<crate::generated::latest::types::Achievement>,
    #[serde(rename = "loggedInDays")]
    pub logged_in_days: f64,
    pub policies: crate::generated::latest::types::RolePolicies,
    #[serde(rename = "twoFactorEnabled")]
    pub two_factor_enabled: bool,
    #[serde(rename = "usePasswordLessLogin")]
    pub use_password_less_login: bool,
    #[serde(rename = "securityKeys")]
    pub security_keys: bool,
    #[serde(default)]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "emailVerified", default)]
    pub email_verified: ::std::option::Option<bool>,
    #[serde(rename = "securityKeysList", default)]
    pub security_keys_list: ::std::option::Option<
        ::std::vec::Vec<crate::generated::latest::types::MeDetailedOnlySecurityKeysListItem>,
    >,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserDetailedNotMe {
    pub id: ::std::string::String,
    pub name: ::std::option::Option<::std::string::String>,
    pub username: ::std::string::String,
    #[doc = "The local host is represented with `null`."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(rename = "avatarBlurhash")]
    pub avatar_blurhash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "avatarDecorations")]
    pub avatar_decorations:
        ::std::vec::Vec<crate::generated::latest::types::UserLiteAvatarDecorationsItem>,
    #[serde(rename = "isBot", default)]
    pub is_bot: ::std::option::Option<bool>,
    #[serde(rename = "isCat", default)]
    pub is_cat: ::std::option::Option<bool>,
    #[serde(rename = "requireSigninToViewContents", default)]
    pub require_signin_to_view_contents: ::std::option::Option<bool>,
    #[serde(rename = "makeNotesFollowersOnlyBefore", default)]
    pub make_notes_followers_only_before: ::std::option::Option<f64>,
    #[serde(rename = "makeNotesHiddenBefore", default)]
    pub make_notes_hidden_before: ::std::option::Option<f64>,
    #[serde(default)]
    pub instance: ::std::option::Option<crate::generated::latest::types::UserLiteInstance>,
    pub emojis: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(rename = "onlineStatus")]
    pub online_status: crate::generated::latest::types::UserLiteOnlineStatus,
    #[serde(rename = "badgeRoles", default)]
    pub badge_roles: ::std::option::Option<
        ::std::vec::Vec<crate::generated::latest::types::UserLiteBadgeRolesItem>,
    >,
    pub url: ::std::option::Option<::std::string::String>,
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "movedTo")]
    pub moved_to: ::std::option::Option<::std::string::String>,
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "lastFetchedAt")]
    pub last_fetched_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerBlurhash")]
    pub banner_blurhash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "isSilenced")]
    pub is_silenced: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    pub description: ::std::option::Option<::std::string::String>,
    pub location: ::std::option::Option<::std::string::String>,
    pub birthday: ::std::option::Option<::std::string::String>,
    pub lang: ::std::option::Option<::std::string::String>,
    pub fields: ::std::vec::Vec<crate::generated::latest::types::UserDetailedNotMeOnlyFieldsItem>,
    #[serde(rename = "verifiedLinks")]
    pub verified_links: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "followersCount")]
    pub followers_count: f64,
    #[serde(rename = "followingCount")]
    pub following_count: f64,
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "pinnedNoteIds")]
    pub pinned_note_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "pinnedNotes")]
    pub pinned_notes: ::std::vec::Vec<crate::generated::latest::types::Note>,
    #[serde(rename = "pinnedPageId")]
    pub pinned_page_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pinnedPage")]
    pub pinned_page: ::std::option::Option<crate::generated::latest::types::Page>,
    #[serde(rename = "publicReactions")]
    pub public_reactions: bool,
    #[serde(rename = "followingVisibility")]
    pub following_visibility:
        crate::generated::latest::types::UserDetailedNotMeOnlyFollowingVisibility,
    #[serde(rename = "followersVisibility")]
    pub followers_visibility:
        crate::generated::latest::types::UserDetailedNotMeOnlyFollowersVisibility,
    #[serde(rename = "chatScope")]
    pub chat_scope: crate::generated::latest::types::UserDetailedNotMeOnlyChatScope,
    #[serde(rename = "canChat")]
    pub can_chat: bool,
    pub roles: ::std::vec::Vec<crate::generated::latest::types::RoleLite>,
    #[serde(rename = "followedMessage", default)]
    pub followed_message: ::std::option::Option<::std::string::String>,
    pub memo: ::std::option::Option<::std::string::String>,
    #[serde(rename = "moderationNote", default)]
    pub moderation_note: ::std::option::Option<::std::string::String>,
    #[serde(rename = "twoFactorEnabled", default)]
    pub two_factor_enabled: ::std::option::Option<bool>,
    #[serde(rename = "usePasswordLessLogin", default)]
    pub use_password_less_login: ::std::option::Option<bool>,
    #[serde(rename = "securityKeys", default)]
    pub security_keys: ::std::option::Option<bool>,
    #[serde(rename = "isFollowing", default)]
    pub is_following: ::std::option::Option<bool>,
    #[serde(rename = "isFollowed", default)]
    pub is_followed: ::std::option::Option<bool>,
    #[serde(rename = "hasPendingFollowRequestFromYou", default)]
    pub has_pending_follow_request_from_you: ::std::option::Option<bool>,
    #[serde(rename = "hasPendingFollowRequestToYou", default)]
    pub has_pending_follow_request_to_you: ::std::option::Option<bool>,
    #[serde(rename = "isBlocking", default)]
    pub is_blocking: ::std::option::Option<bool>,
    #[serde(rename = "isBlocked", default)]
    pub is_blocked: ::std::option::Option<bool>,
    #[serde(rename = "isMuted", default)]
    pub is_muted: ::std::option::Option<bool>,
    #[serde(rename = "isRenoteMuted", default)]
    pub is_renote_muted: ::std::option::Option<bool>,
    #[serde(default)]
    pub notify: ::std::option::Option<crate::generated::latest::types::UserDetailedNotMeOnlyNotify>,
    #[serde(rename = "withReplies", default)]
    pub with_replies: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MeDetailed {
    pub id: ::std::string::String,
    pub name: ::std::option::Option<::std::string::String>,
    pub username: ::std::string::String,
    #[doc = "The local host is represented with `null`."]
    pub host: ::std::option::Option<::std::string::String>,
    #[serde(rename = "avatarUrl")]
    pub avatar_url: ::std::string::String,
    #[serde(rename = "avatarBlurhash")]
    pub avatar_blurhash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "avatarDecorations")]
    pub avatar_decorations:
        ::std::vec::Vec<crate::generated::latest::types::UserLiteAvatarDecorationsItem>,
    #[serde(rename = "isBot", default)]
    pub is_bot: ::std::option::Option<bool>,
    #[serde(rename = "isCat", default)]
    pub is_cat: ::std::option::Option<bool>,
    #[serde(rename = "requireSigninToViewContents", default)]
    pub require_signin_to_view_contents: ::std::option::Option<bool>,
    #[serde(rename = "makeNotesFollowersOnlyBefore", default)]
    pub make_notes_followers_only_before: ::std::option::Option<f64>,
    #[serde(rename = "makeNotesHiddenBefore", default)]
    pub make_notes_hidden_before: ::std::option::Option<f64>,
    #[serde(default)]
    pub instance: ::std::option::Option<crate::generated::latest::types::UserLiteInstance>,
    pub emojis: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    #[serde(rename = "onlineStatus")]
    pub online_status: crate::generated::latest::types::UserLiteOnlineStatus,
    #[serde(rename = "badgeRoles", default)]
    pub badge_roles: ::std::option::Option<
        ::std::vec::Vec<crate::generated::latest::types::UserLiteBadgeRolesItem>,
    >,
    pub url: ::std::option::Option<::std::string::String>,
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(rename = "movedTo")]
    pub moved_to: ::std::option::Option<::std::string::String>,
    #[serde(rename = "alsoKnownAs")]
    pub also_known_as: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "lastFetchedAt")]
    pub last_fetched_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerBlurhash")]
    pub banner_blurhash: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isLocked")]
    pub is_locked: bool,
    #[serde(rename = "isSilenced")]
    pub is_silenced: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    pub description: ::std::option::Option<::std::string::String>,
    pub location: ::std::option::Option<::std::string::String>,
    pub birthday: ::std::option::Option<::std::string::String>,
    pub lang: ::std::option::Option<::std::string::String>,
    pub fields: ::std::vec::Vec<crate::generated::latest::types::UserDetailedNotMeOnlyFieldsItem>,
    #[serde(rename = "verifiedLinks")]
    pub verified_links: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "followersCount")]
    pub followers_count: f64,
    #[serde(rename = "followingCount")]
    pub following_count: f64,
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "pinnedNoteIds")]
    pub pinned_note_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "pinnedNotes")]
    pub pinned_notes: ::std::vec::Vec<crate::generated::latest::types::Note>,
    #[serde(rename = "pinnedPageId")]
    pub pinned_page_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pinnedPage")]
    pub pinned_page: ::std::option::Option<crate::generated::latest::types::Page>,
    #[serde(rename = "publicReactions")]
    pub public_reactions: bool,
    #[serde(rename = "followingVisibility")]
    pub following_visibility:
        crate::generated::latest::types::UserDetailedNotMeOnlyFollowingVisibility,
    #[serde(rename = "followersVisibility")]
    pub followers_visibility:
        crate::generated::latest::types::UserDetailedNotMeOnlyFollowersVisibility,
    #[serde(rename = "chatScope")]
    pub chat_scope: crate::generated::latest::types::UserDetailedNotMeOnlyChatScope,
    #[serde(rename = "canChat")]
    pub can_chat: bool,
    pub roles: ::std::vec::Vec<crate::generated::latest::types::RoleLite>,
    #[serde(rename = "followedMessage")]
    pub followed_message: ::std::option::Option<::std::string::String>,
    pub memo: ::std::option::Option<::std::string::String>,
    #[serde(rename = "moderationNote", default)]
    pub moderation_note: ::std::option::Option<::std::string::String>,
    #[serde(rename = "twoFactorEnabled")]
    pub two_factor_enabled: bool,
    #[serde(rename = "usePasswordLessLogin")]
    pub use_password_less_login: bool,
    #[serde(rename = "securityKeys")]
    pub security_keys: bool,
    #[serde(rename = "isFollowing", default)]
    pub is_following: ::std::option::Option<bool>,
    #[serde(rename = "isFollowed", default)]
    pub is_followed: ::std::option::Option<bool>,
    #[serde(rename = "hasPendingFollowRequestFromYou", default)]
    pub has_pending_follow_request_from_you: ::std::option::Option<bool>,
    #[serde(rename = "hasPendingFollowRequestToYou", default)]
    pub has_pending_follow_request_to_you: ::std::option::Option<bool>,
    #[serde(rename = "isBlocking", default)]
    pub is_blocking: ::std::option::Option<bool>,
    #[serde(rename = "isBlocked", default)]
    pub is_blocked: ::std::option::Option<bool>,
    #[serde(rename = "isMuted", default)]
    pub is_muted: ::std::option::Option<bool>,
    #[serde(rename = "isRenoteMuted", default)]
    pub is_renote_muted: ::std::option::Option<bool>,
    #[serde(default)]
    pub notify: ::std::option::Option<crate::generated::latest::types::UserDetailedNotMeOnlyNotify>,
    #[serde(rename = "withReplies", default)]
    pub with_replies: ::std::option::Option<bool>,
    #[serde(rename = "avatarId")]
    pub avatar_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerId")]
    pub banner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isModerator")]
    pub is_moderator: bool,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "injectFeaturedNote")]
    pub inject_featured_note: bool,
    #[serde(rename = "receiveAnnouncementEmail")]
    pub receive_announcement_email: bool,
    #[serde(rename = "alwaysMarkNsfw")]
    pub always_mark_nsfw: bool,
    #[serde(rename = "autoSensitive")]
    pub auto_sensitive: bool,
    #[serde(rename = "carefulBot")]
    pub careful_bot: bool,
    #[serde(rename = "autoAcceptFollowed")]
    pub auto_accept_followed: bool,
    #[serde(rename = "followApprovalLocalSeconds")]
    pub follow_approval_local_seconds: ::std::option::Option<f64>,
    #[serde(rename = "followApprovalRemoteSeconds")]
    pub follow_approval_remote_seconds: ::std::option::Option<f64>,
    #[serde(rename = "noCrawle")]
    pub no_crawle: bool,
    #[serde(rename = "preventAiLearning")]
    pub prevent_ai_learning: bool,
    #[serde(rename = "isExplorable")]
    pub is_explorable: bool,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    #[serde(rename = "twoFactorBackupCodesStock")]
    pub two_factor_backup_codes_stock:
        crate::generated::latest::types::MeDetailedOnlyTwoFactorBackupCodesStock,
    #[serde(rename = "hideOnlineStatus")]
    pub hide_online_status: bool,
    #[serde(rename = "hasUnreadSpecifiedNotes")]
    pub has_unread_specified_notes: bool,
    #[serde(rename = "hasUnreadMentions")]
    pub has_unread_mentions: bool,
    #[serde(rename = "hasUnreadAnnouncement")]
    pub has_unread_announcement: bool,
    #[serde(rename = "unreadAnnouncements")]
    pub unread_announcements: ::std::vec::Vec<crate::generated::latest::types::Announcement>,
    #[serde(rename = "hasUnreadAntenna")]
    pub has_unread_antenna: bool,
    #[serde(rename = "hasUnreadChannel")]
    pub has_unread_channel: bool,
    #[serde(rename = "hasUnreadChatMessages")]
    pub has_unread_chat_messages: bool,
    #[serde(rename = "hasUnreadNotification")]
    pub has_unread_notification: bool,
    #[serde(rename = "hasPendingReceivedFollowRequest")]
    pub has_pending_received_follow_request: bool,
    #[serde(rename = "unreadNotificationsCount")]
    pub unread_notifications_count: f64,
    #[serde(rename = "mutedWords")]
    pub muted_words: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "hardMutedWords")]
    pub hard_muted_words: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "mutedInstances")]
    pub muted_instances: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "notificationRecieveConfig")]
    pub notification_recieve_config:
        crate::generated::latest::types::MeDetailedOnlyNotificationRecieveConfig,
    #[serde(rename = "emailNotificationTypes")]
    pub email_notification_types: ::std::vec::Vec<::std::string::String>,
    pub achievements: ::std::vec::Vec<crate::generated::latest::types::Achievement>,
    #[serde(rename = "loggedInDays")]
    pub logged_in_days: f64,
    pub policies: crate::generated::latest::types::RolePolicies,
    #[serde(default)]
    pub email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "emailVerified", default)]
    pub email_verified: ::std::option::Option<bool>,
    #[serde(rename = "securityKeysList", default)]
    pub security_keys_list: ::std::option::Option<
        ::std::vec::Vec<crate::generated::latest::types::MeDetailedOnlySecurityKeysListItem>,
    >,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum UserDetailed {
    MeDetailed(crate::generated::latest::types::MeDetailed),
    UserDetailedNotMe(crate::generated::latest::types::UserDetailedNotMe),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum User {
    MeDetailed(crate::generated::latest::types::MeDetailed),
    UserDetailedNotMe(crate::generated::latest::types::UserDetailedNotMe),
    UserLite(crate::generated::latest::types::UserLite),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UserList {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub name: ::std::string::String,
    #[serde(rename = "userIds", default)]
    pub user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Achievement {
    pub name: crate::generated::latest::types::AchievementName,
    #[serde(rename = "unlockedAt")]
    pub unlocked_at: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AchievementName {
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
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Ad {
    pub id: ::std::string::String,
    #[serde(rename = "expiresAt")]
    pub expires_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "startsAt")]
    pub starts_at: ::chrono::DateTime<::chrono::Utc>,
    pub place: ::std::string::String,
    pub priority: ::std::string::String,
    pub ratio: f64,
    pub url: ::std::string::String,
    #[serde(rename = "imageUrl")]
    pub image_url: ::std::string::String,
    pub memo: ::std::string::String,
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: i64,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnnouncementIcon {
    #[serde(rename = "info")]
    Info,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
    #[serde(rename = "success")]
    Success,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AnnouncementDisplay {
    #[serde(rename = "dialog")]
    Dialog,
    #[serde(rename = "normal")]
    Normal,
    #[serde(rename = "banner")]
    Banner,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Announcement {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    pub text: ::std::string::String,
    pub title: ::std::string::String,
    #[serde(rename = "imageUrl")]
    pub image_url: ::std::option::Option<::std::string::String>,
    pub icon: crate::generated::latest::types::AnnouncementIcon,
    pub display: crate::generated::latest::types::AnnouncementDisplay,
    #[serde(rename = "needConfirmationToRead")]
    pub need_confirmation_to_read: bool,
    pub silence: bool,
    #[serde(rename = "forYou", default)]
    pub for_you: ::std::option::Option<bool>,
    #[serde(rename = "isRead", default)]
    pub is_read: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct App {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    #[serde(rename = "callbackUrl")]
    pub callback_url: ::std::option::Option<::std::string::String>,
    pub permission: ::std::vec::Vec<::std::string::String>,
    #[serde(default)]
    pub secret: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isAuthorized", default)]
    pub is_authorized: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NoteVisibility {
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
pub struct NotePollChoicesItem {
    #[serde(rename = "isVoted")]
    pub is_voted: bool,
    pub text: ::std::string::String,
    pub votes: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotePoll {
    #[serde(rename = "expiresAt", default)]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    pub multiple: bool,
    pub choices: ::std::vec::Vec<crate::generated::latest::types::NotePollChoicesItem>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteChannel {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub color: ::std::string::String,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    #[serde(rename = "allowRenoteToExternal")]
    pub allow_renote_to_external: bool,
    #[serde(rename = "userId")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NoteReactionAcceptance {
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
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Note {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "deletedAt", default)]
    pub deleted_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub cw: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "replyId", default)]
    pub reply_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "renoteId", default)]
    pub renote_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub reply: ::std::option::Option<::std::boxed::Box<crate::generated::latest::types::Note>>,
    #[serde(default)]
    pub renote: ::std::option::Option<::std::boxed::Box<crate::generated::latest::types::Note>>,
    #[serde(rename = "isHidden", default)]
    pub is_hidden: ::std::option::Option<bool>,
    pub visibility: crate::generated::latest::types::NoteVisibility,
    #[serde(default)]
    pub mentions: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "visibleUserIds", default)]
    pub visible_user_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "fileIds", default)]
    pub file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default)]
    pub files: ::std::option::Option<::std::vec::Vec<crate::generated::latest::types::DriveFile>>,
    #[serde(default)]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default)]
    pub poll: ::std::option::Option<crate::generated::latest::types::NotePoll>,
    #[serde(default)]
    pub emojis: ::std::option::Option<
        ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    >,
    #[serde(rename = "channelId", default)]
    pub channel_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub channel: ::std::option::Option<crate::generated::latest::types::NoteChannel>,
    #[serde(rename = "localOnly", default)]
    pub local_only: ::std::option::Option<bool>,
    #[serde(rename = "reactionAcceptance")]
    pub reaction_acceptance:
        ::std::option::Option<crate::generated::latest::types::NoteReactionAcceptance>,
    #[serde(rename = "reactionEmojis")]
    pub reaction_emojis: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub reactions: ::std::collections::HashMap<::std::string::String, f64>,
    #[serde(rename = "reactionCount")]
    pub reaction_count: f64,
    #[serde(rename = "renoteCount")]
    pub renote_count: f64,
    #[serde(rename = "repliesCount")]
    pub replies_count: f64,
    #[serde(default)]
    pub uri: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "reactionAndUserPairCache", default)]
    pub reaction_and_user_pair_cache: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "clippedCount", default)]
    pub clipped_count: ::std::option::Option<f64>,
    #[serde(rename = "hasPoll", default)]
    pub has_poll: ::std::option::Option<bool>,
    #[serde(rename = "myReaction", default)]
    pub my_reaction: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NoteDraftVisibility {
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
pub struct NoteDraftPoll {
    #[serde(rename = "expiresAt", default)]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "expiredAfter", default)]
    pub expired_after: ::std::option::Option<f64>,
    pub multiple: bool,
    pub choices: ::std::vec::Vec<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteDraftChannel {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub color: ::std::string::String,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    #[serde(rename = "allowRenoteToExternal")]
    pub allow_renote_to_external: bool,
    #[serde(rename = "userId")]
    pub user_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NoteDraftReactionAcceptance {
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
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteDraft {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub text: ::std::option::Option<::std::string::String>,
    pub cw: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "replyId")]
    pub reply_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "renoteId")]
    pub renote_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub reply: ::std::option::Option<crate::generated::latest::types::Note>,
    #[serde(default)]
    pub renote: ::std::option::Option<crate::generated::latest::types::Note>,
    pub visibility: crate::generated::latest::types::NoteDraftVisibility,
    #[serde(rename = "visibleUserIds")]
    pub visible_user_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "fileIds")]
    pub file_ids: ::std::vec::Vec<::std::string::String>,
    #[serde(default)]
    pub files: ::std::option::Option<::std::vec::Vec<crate::generated::latest::types::DriveFile>>,
    pub hashtag: ::std::option::Option<::std::string::String>,
    pub poll: ::std::option::Option<crate::generated::latest::types::NoteDraftPoll>,
    #[serde(rename = "channelId")]
    pub channel_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub channel: ::std::option::Option<crate::generated::latest::types::NoteDraftChannel>,
    #[serde(rename = "localOnly")]
    pub local_only: bool,
    #[serde(rename = "reactionAcceptance")]
    pub reaction_acceptance:
        ::std::option::Option<crate::generated::latest::types::NoteDraftReactionAcceptance>,
    #[serde(rename = "scheduledAt")]
    pub scheduled_at: ::std::option::Option<f64>,
    #[serde(rename = "isActuallyScheduled")]
    pub is_actually_scheduled: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteReaction {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub user: crate::generated::latest::types::UserLite,
    pub r#type: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteReactionWithNote {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub user: crate::generated::latest::types::UserLite,
    pub r#type: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteFavorite {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub note: crate::generated::latest::types::Note,
    #[serde(rename = "noteId")]
    pub note_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationReactionType {
    #[serde(rename = "reaction")]
    Reaction,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationReaction {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationReactionType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
    pub reaction: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationNoteType {
    #[serde(rename = "note")]
    Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationNote {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationNoteType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationMentionType {
    #[serde(rename = "mention")]
    Mention,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationMention {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationMentionType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationReplyType {
    #[serde(rename = "reply")]
    Reply,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationReply {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationReplyType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationRenoteType {
    #[serde(rename = "renote")]
    Renote,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationRenote {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationRenoteType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationQuoteType {
    #[serde(rename = "quote")]
    Quote,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationQuote {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationQuoteType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationPollEndedType {
    #[serde(rename = "pollEnded")]
    PollEnded,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationPollEnded {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationPollEndedType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationFollowRequestAcceptedType {
    #[serde(rename = "followRequestAccepted")]
    FollowRequestAccepted,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationFollowRequestAccepted {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationFollowRequestAcceptedType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub message: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationAppType {
    #[serde(rename = "app")]
    App,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationApp {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationAppType,
    pub body: ::std::string::String,
    pub header: ::std::option::Option<::std::string::String>,
    pub icon: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationFollowType {
    #[serde(rename = "follow")]
    Follow,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationFollow {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationFollowType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationReceiveFollowRequestType {
    #[serde(rename = "receiveFollowRequest")]
    ReceiveFollowRequest,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationReceiveFollowRequest {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationReceiveFollowRequestType,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationExportCompletedType {
    #[serde(rename = "exportCompleted")]
    ExportCompleted,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationExportCompletedExportedEntity {
    #[serde(rename = "antenna")]
    Antenna,
    #[serde(rename = "blocking")]
    Blocking,
    #[serde(rename = "clip")]
    Clip,
    #[serde(rename = "customEmoji")]
    CustomEmoji,
    #[serde(rename = "favorite")]
    Favorite,
    #[serde(rename = "following")]
    Following,
    #[serde(rename = "muting")]
    Muting,
    #[serde(rename = "note")]
    Note,
    #[serde(rename = "userList")]
    UserList,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationExportCompleted {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationExportCompletedType,
    #[serde(rename = "exportedEntity")]
    pub exported_entity: crate::generated::latest::types::NotificationExportCompletedExportedEntity,
    #[serde(rename = "fileId")]
    pub file_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationReactionGroupedType {
    #[serde(rename = "reaction:grouped")]
    ReactionGrouped,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationReactionGroupedReactionsItem {
    pub user: crate::generated::latest::types::UserLite,
    pub reaction: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationReactionGrouped {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationReactionGroupedType,
    pub note: crate::generated::latest::types::Note,
    pub reactions:
        ::std::vec::Vec<crate::generated::latest::types::NotificationReactionGroupedReactionsItem>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationRenoteGroupedType {
    #[serde(rename = "renote:grouped")]
    RenoteGrouped,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationRenoteGrouped {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationRenoteGroupedType,
    pub note: crate::generated::latest::types::Note,
    pub users: ::std::vec::Vec<crate::generated::latest::types::UserLite>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationScheduledNotePostedType {
    #[serde(rename = "scheduledNotePosted")]
    ScheduledNotePosted,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationScheduledNotePosted {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationScheduledNotePostedType,
    pub note: crate::generated::latest::types::Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationScheduledNotePostFailedType {
    #[serde(rename = "scheduledNotePostFailed")]
    ScheduledNotePostFailed,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationScheduledNotePostFailed {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationScheduledNotePostFailedType,
    #[serde(rename = "noteDraft")]
    pub note_draft: crate::generated::latest::types::NoteDraft,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationRoleAssignedType {
    #[serde(rename = "roleAssigned")]
    RoleAssigned,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationRoleAssigned {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationRoleAssignedType,
    pub role: crate::generated::latest::types::Role,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationChatRoomInvitationReceivedType {
    #[serde(rename = "chatRoomInvitationReceived")]
    ChatRoomInvitationReceived,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationChatRoomInvitationReceived {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationChatRoomInvitationReceivedType,
    pub invitation: crate::generated::latest::types::ChatRoomInvitation,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationAchievementEarnedType {
    #[serde(rename = "achievementEarned")]
    AchievementEarned,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationAchievementEarned {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationAchievementEarnedType,
    pub achievement: crate::generated::latest::types::AchievementName,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationLoginType {
    #[serde(rename = "login")]
    Login,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationLogin {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationLoginType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationCreateTokenType {
    #[serde(rename = "createToken")]
    CreateToken,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationCreateToken {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationCreateTokenType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum NotificationTestType {
    #[serde(rename = "test")]
    Test,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NotificationTest {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub r#type: crate::generated::latest::types::NotificationTestType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum Notification {
    Reaction(crate::generated::latest::types::NotificationReaction),
    Note(crate::generated::latest::types::NotificationNote),
    Mention(crate::generated::latest::types::NotificationMention),
    Reply(crate::generated::latest::types::NotificationReply),
    Renote(crate::generated::latest::types::NotificationRenote),
    Quote(crate::generated::latest::types::NotificationQuote),
    PollEnded(crate::generated::latest::types::NotificationPollEnded),
    FollowRequestAccepted(crate::generated::latest::types::NotificationFollowRequestAccepted),
    App(crate::generated::latest::types::NotificationApp),
    Follow(crate::generated::latest::types::NotificationFollow),
    ReceiveFollowRequest(crate::generated::latest::types::NotificationReceiveFollowRequest),
    ExportCompleted(crate::generated::latest::types::NotificationExportCompleted),
    ReactionGrouped(crate::generated::latest::types::NotificationReactionGrouped),
    RenoteGrouped(crate::generated::latest::types::NotificationRenoteGrouped),
    ScheduledNotePosted(crate::generated::latest::types::NotificationScheduledNotePosted),
    ScheduledNotePostFailed(crate::generated::latest::types::NotificationScheduledNotePostFailed),
    RoleAssigned(crate::generated::latest::types::NotificationRoleAssigned),
    ChatRoomInvitationReceived(
        crate::generated::latest::types::NotificationChatRoomInvitationReceived,
    ),
    AchievementEarned(crate::generated::latest::types::NotificationAchievementEarned),
    Login(crate::generated::latest::types::NotificationLogin),
    CreateToken(crate::generated::latest::types::NotificationCreateToken),
    Test(crate::generated::latest::types::NotificationTest),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct DriveFileProperties {
    #[serde(default)]
    pub width: ::std::option::Option<f64>,
    #[serde(default)]
    pub height: ::std::option::Option<f64>,
    #[serde(default)]
    pub orientation: ::std::option::Option<f64>,
    #[serde(rename = "avgColor", default)]
    pub avg_color: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct DriveFile {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub name: ::std::string::String,
    pub r#type: ::std::string::String,
    pub md5: ::std::string::String,
    pub size: f64,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    pub blurhash: ::std::option::Option<::std::string::String>,
    pub properties: crate::generated::latest::types::DriveFileProperties,
    pub url: ::std::string::String,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: ::std::option::Option<::std::string::String>,
    pub comment: ::std::option::Option<::std::string::String>,
    #[serde(rename = "folderId")]
    pub folder_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub folder: ::std::option::Option<crate::generated::latest::types::DriveFolder>,
    #[serde(rename = "userId")]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub user: ::std::option::Option<crate::generated::latest::types::UserLite>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct DriveFolder {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub name: ::std::string::String,
    #[serde(rename = "parentId")]
    pub parent_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "foldersCount", default)]
    pub folders_count: ::std::option::Option<f64>,
    #[serde(rename = "filesCount", default)]
    pub files_count: ::std::option::Option<f64>,
    #[serde(default)]
    pub parent:
        ::std::option::Option<::std::boxed::Box<crate::generated::latest::types::DriveFolder>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Following {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "followeeId")]
    pub followee_id: ::std::string::String,
    #[serde(rename = "followerId")]
    pub follower_id: ::std::string::String,
    #[serde(default)]
    pub followee: ::std::option::Option<crate::generated::latest::types::UserDetailedNotMe>,
    #[serde(default)]
    pub follower: ::std::option::Option<crate::generated::latest::types::UserDetailedNotMe>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Muting {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "expiresAt")]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "muteeId")]
    pub mutee_id: ::std::string::String,
    pub mutee: crate::generated::latest::types::UserDetailedNotMe,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RenoteMuting {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "muteeId")]
    pub mutee_id: ::std::string::String,
    pub mutee: crate::generated::latest::types::UserDetailedNotMe,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Blocking {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "blockeeId")]
    pub blockee_id: ::std::string::String,
    pub blockee: crate::generated::latest::types::UserDetailedNotMe,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Hashtag {
    pub tag: ::std::string::String,
    #[serde(rename = "mentionedUsersCount")]
    pub mentioned_users_count: f64,
    #[serde(rename = "mentionedLocalUsersCount")]
    pub mentioned_local_users_count: f64,
    #[serde(rename = "mentionedRemoteUsersCount")]
    pub mentioned_remote_users_count: f64,
    #[serde(rename = "attachedUsersCount")]
    pub attached_users_count: f64,
    #[serde(rename = "attachedLocalUsersCount")]
    pub attached_local_users_count: f64,
    #[serde(rename = "attachedRemoteUsersCount")]
    pub attached_remote_users_count: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct InviteCode {
    pub id: ::std::string::String,
    pub code: ::std::string::String,
    #[serde(rename = "expiresAt")]
    pub expires_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "createdBy")]
    pub created_by: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "usedBy")]
    pub used_by: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "usedAt")]
    pub used_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    pub used: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PageFont {
    #[serde(rename = "serif")]
    Serif,
    #[serde(rename = "sans-serif")]
    SansSerif,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Page {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
    pub content: ::std::vec::Vec<crate::generated::latest::types::PageBlock>,
    pub variables: ::std::vec::Vec<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    pub title: ::std::string::String,
    pub name: ::std::string::String,
    pub summary: ::std::option::Option<::std::string::String>,
    #[serde(rename = "hideTitleWhenPinned")]
    pub hide_title_when_pinned: bool,
    #[serde(rename = "alignCenter")]
    pub align_center: bool,
    pub font: crate::generated::latest::types::PageFont,
    pub script: ::std::string::String,
    #[serde(rename = "eyeCatchingImageId")]
    pub eye_catching_image_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "eyeCatchingImage")]
    pub eye_catching_image: ::std::option::Option<crate::generated::latest::types::DriveFile>,
    #[serde(rename = "attachedFiles")]
    pub attached_files: ::std::vec::Vec<crate::generated::latest::types::DriveFile>,
    #[serde(rename = "likedCount")]
    pub liked_count: f64,
    #[serde(rename = "isLiked", default)]
    pub is_liked: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PageBlockSectionType {
    #[serde(rename = "section")]
    Section,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PageBlockSection {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::PageBlockSectionType,
    pub title: ::std::string::String,
    pub children: ::std::vec::Vec<crate::generated::latest::types::PageBlock>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PageBlockNoteType {
    #[serde(rename = "note")]
    Note,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PageBlockNote {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::PageBlockNoteType,
    pub detailed: bool,
    pub note: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PageBlockTextType {
    #[serde(rename = "text")]
    Text,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PageBlockText {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::PageBlockTextType,
    pub text: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum PageBlockImageType {
    #[serde(rename = "image")]
    Image,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PageBlockImage {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::PageBlockImageType,
    #[serde(rename = "fileId")]
    pub file_id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum PageBlock {
    Section(crate::generated::latest::types::PageBlockSection),
    Note(crate::generated::latest::types::PageBlockNote),
    Text(crate::generated::latest::types::PageBlockText),
    Image(crate::generated::latest::types::PageBlockImage),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Channel {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "lastNotedAt")]
    pub last_noted_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    pub name: ::std::string::String,
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "userId")]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerUrl")]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "bannerId")]
    pub banner_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "pinnedNoteIds")]
    pub pinned_note_ids: ::std::vec::Vec<::std::string::String>,
    pub color: ::std::string::String,
    #[serde(rename = "isArchived")]
    pub is_archived: bool,
    #[serde(rename = "usersCount")]
    pub users_count: f64,
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    #[serde(rename = "allowRenoteToExternal")]
    pub allow_renote_to_external: bool,
    #[serde(rename = "isFollowing", default)]
    pub is_following: ::std::option::Option<bool>,
    #[serde(rename = "isFavorited", default)]
    pub is_favorited: ::std::option::Option<bool>,
    #[serde(rename = "isMuting", default)]
    pub is_muting: ::std::option::Option<bool>,
    #[serde(rename = "pinnedNotes", default)]
    pub pinned_notes: ::std::option::Option<::std::vec::Vec<crate::generated::latest::types::Note>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct QueueCount {
    pub waiting: f64,
    pub active: f64,
    pub completed: f64,
    pub failed: f64,
    pub delayed: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct QueueMetricsMeta {
    pub count: f64,
    #[serde(rename = "prevTS")]
    pub prev_ts: f64,
    #[serde(rename = "prevCount")]
    pub prev_count: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct QueueMetrics {
    pub meta: crate::generated::latest::types::QueueMetricsMeta,
    pub data: ::std::vec::Vec<f64>,
    pub count: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct QueueJob {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub data: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub opts: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub timestamp: f64,
    #[serde(rename = "processedOn", default)]
    pub processed_on: ::std::option::Option<f64>,
    #[serde(rename = "processedBy", default)]
    pub processed_by: ::std::option::Option<::std::string::String>,
    #[serde(rename = "finishedOn", default)]
    pub finished_on: ::std::option::Option<f64>,
    pub progress: ::serde_json::Value,
    pub attempts: f64,
    pub delay: f64,
    #[serde(rename = "failedReason", default)]
    pub failed_reason: ::std::option::Option<::std::string::String>,
    pub stacktrace: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "returnValue")]
    pub return_value: ::serde_json::Value,
    #[serde(rename = "isFailed")]
    pub is_failed: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AntennaSrc {
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
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Antenna {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub name: ::std::string::String,
    pub keywords: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "excludeKeywords")]
    pub exclude_keywords: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
    pub src: crate::generated::latest::types::AntennaSrc,
    #[serde(rename = "userListId")]
    pub user_list_id: ::std::option::Option<::std::string::String>,
    pub users: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "caseSensitive")]
    pub case_sensitive: bool,
    #[serde(rename = "localOnly")]
    pub local_only: bool,
    #[serde(rename = "excludeBots")]
    pub exclude_bots: bool,
    #[serde(rename = "withReplies")]
    pub with_replies: bool,
    #[serde(rename = "withFile")]
    pub with_file: bool,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "hasUnreadNote")]
    pub has_unread_note: bool,
    pub notify: bool,
    #[serde(rename = "excludeNotesInSensitiveChannel")]
    pub exclude_notes_in_sensitive_channel: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Clip {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "lastClippedAt")]
    pub last_clipped_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
    pub name: ::std::string::String,
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "favoritedCount")]
    pub favorited_count: f64,
    #[serde(rename = "isFavorited", default)]
    pub is_favorited: ::std::option::Option<bool>,
    #[serde(rename = "notesCount", default)]
    pub notes_count: ::std::option::Option<i64>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum FederationInstanceSuspensionState {
    #[serde(rename = "none")]
    None,
    #[serde(rename = "manuallySuspended")]
    ManuallySuspended,
    #[serde(rename = "goneSuspended")]
    GoneSuspended,
    #[serde(rename = "autoSuspendedForNotResponding")]
    AutoSuspendedForNotResponding,
    #[serde(rename = "softwareSuspended")]
    SoftwareSuspended,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct FederationInstance {
    pub id: ::std::string::String,
    #[serde(rename = "firstRetrievedAt")]
    pub first_retrieved_at: ::chrono::DateTime<::chrono::Utc>,
    pub host: ::std::string::String,
    #[serde(rename = "usersCount")]
    pub users_count: f64,
    #[serde(rename = "notesCount")]
    pub notes_count: f64,
    #[serde(rename = "followingCount")]
    pub following_count: f64,
    #[serde(rename = "followersCount")]
    pub followers_count: f64,
    #[serde(rename = "isNotResponding")]
    pub is_not_responding: bool,
    #[serde(rename = "isSuspended")]
    pub is_suspended: bool,
    #[serde(rename = "suspensionState")]
    pub suspension_state: crate::generated::latest::types::FederationInstanceSuspensionState,
    #[serde(rename = "isBlocked")]
    pub is_blocked: bool,
    #[serde(rename = "softwareName")]
    pub software_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "softwareVersion")]
    pub software_version: ::std::option::Option<::std::string::String>,
    #[serde(rename = "openRegistrations")]
    pub open_registrations: ::std::option::Option<bool>,
    pub name: ::std::option::Option<::std::string::String>,
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maintainerName")]
    pub maintainer_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maintainerEmail")]
    pub maintainer_email: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isSilenced")]
    pub is_silenced: bool,
    #[serde(rename = "isMediaSilenced")]
    pub is_media_silenced: bool,
    #[serde(rename = "iconUrl")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "faviconUrl")]
    pub favicon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "themeColor")]
    pub theme_color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "infoUpdatedAt")]
    pub info_updated_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "latestRequestReceivedAt")]
    pub latest_request_received_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "moderationNote", default)]
    pub moderation_note: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct GalleryPost {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
    pub title: ::std::string::String,
    pub description: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileIds", default)]
    pub file_ids: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(default)]
    pub files: ::std::option::Option<::std::vec::Vec<crate::generated::latest::types::DriveFile>>,
    #[serde(default)]
    pub tags: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    #[serde(rename = "likedCount")]
    pub liked_count: f64,
    #[serde(rename = "isLiked", default)]
    pub is_liked: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct EmojiSimple {
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub category: ::std::option::Option<::std::string::String>,
    pub url: ::std::string::String,
    #[serde(rename = "localOnly", default)]
    pub local_only: ::std::option::Option<bool>,
    #[serde(rename = "isSensitive", default)]
    pub is_sensitive: ::std::option::Option<bool>,
    #[serde(rename = "roleIdsThatCanBeUsedThisEmojiAsReaction", default)]
    pub role_ids_that_can_be_used_this_emoji_as_reaction:
        ::std::option::Option<::std::vec::Vec<::std::string::String>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct EmojiDetailed {
    pub id: ::std::string::String,
    pub aliases: ::std::vec::Vec<::std::string::String>,
    pub name: ::std::string::String,
    pub category: ::std::option::Option<::std::string::String>,
    #[doc = "The local host is represented with `null`."]
    pub host: ::std::option::Option<::std::string::String>,
    pub url: ::std::string::String,
    pub license: ::std::option::Option<::std::string::String>,
    #[serde(rename = "isSensitive")]
    pub is_sensitive: bool,
    #[serde(rename = "localOnly")]
    pub local_only: bool,
    #[serde(rename = "roleIdsThatCanBeUsedThisEmojiAsReaction")]
    pub role_ids_that_can_be_used_this_emoji_as_reaction: ::std::vec::Vec<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct EmojiDetailedAdminRoleIdsThatCanBeUsedThisEmojiAsReactionItem {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct EmojiDetailedAdmin { pub id : :: std :: string :: String , # [serde (rename = "updatedAt")] pub updated_at : :: std :: option :: Option < :: chrono :: DateTime < :: chrono :: Utc > > , pub name : :: std :: string :: String , # [doc = "The local host is represented with `null`."] pub host : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "publicUrl")] pub public_url : :: std :: string :: String , # [serde (rename = "originalUrl")] pub original_url : :: std :: string :: String , pub uri : :: std :: option :: Option < :: std :: string :: String > , pub r#type : :: std :: option :: Option < :: std :: string :: String > , pub aliases : :: std :: vec :: Vec < :: std :: string :: String > , pub category : :: std :: option :: Option < :: std :: string :: String > , pub license : :: std :: option :: Option < :: std :: string :: String > , # [serde (rename = "localOnly")] pub local_only : bool , # [serde (rename = "isSensitive")] pub is_sensitive : bool , # [serde (rename = "roleIdsThatCanBeUsedThisEmojiAsReaction")] pub role_ids_that_can_be_used_this_emoji_as_reaction : :: std :: vec :: Vec < crate :: generated :: latest :: types :: EmojiDetailedAdminRoleIdsThatCanBeUsedThisEmojiAsReactionItem > , }
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum FlashVisibility {
    #[serde(rename = "private")]
    Private,
    #[serde(rename = "public")]
    Public,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Flash {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
    pub title: ::std::string::String,
    pub summary: ::std::string::String,
    pub script: ::std::string::String,
    pub visibility: crate::generated::latest::types::FlashVisibility,
    #[serde(rename = "likedCount")]
    pub liked_count: f64,
    #[serde(rename = "isLiked", default)]
    pub is_liked: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Signin {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    pub ip: ::std::string::String,
    pub headers: ::serde_json::Map<::std::string::String, ::serde_json::Value>,
    pub success: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleCondFormulaLogicsType {
    #[serde(rename = "and")]
    And,
    #[serde(rename = "or")]
    Or,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleCondFormulaLogics {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::RoleCondFormulaLogicsType,
    pub values: ::std::vec::Vec<crate::generated::latest::types::RoleCondFormulaValue>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleCondFormulaValueNotType {
    #[serde(rename = "not")]
    Not,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleCondFormulaValueNot {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::RoleCondFormulaValueNotType,
    pub value: ::std::boxed::Box<crate::generated::latest::types::RoleCondFormulaValue>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleCondFormulaValueIsLocalOrRemoteType {
    #[serde(rename = "isLocal")]
    IsLocal,
    #[serde(rename = "isRemote")]
    IsRemote,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleCondFormulaValueIsLocalOrRemote {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::RoleCondFormulaValueIsLocalOrRemoteType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleCondFormulaValueUserSettingBooleanSchemaType {
    #[serde(rename = "isSuspended")]
    IsSuspended,
    #[serde(rename = "isLocked")]
    IsLocked,
    #[serde(rename = "isBot")]
    IsBot,
    #[serde(rename = "isCat")]
    IsCat,
    #[serde(rename = "isExplorable")]
    IsExplorable,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleCondFormulaValueUserSettingBooleanSchema {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::RoleCondFormulaValueUserSettingBooleanSchemaType,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleCondFormulaValueAssignedRoleType {
    #[serde(rename = "roleAssignedTo")]
    RoleAssignedTo,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleCondFormulaValueAssignedRole {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::RoleCondFormulaValueAssignedRoleType,
    #[serde(rename = "roleId")]
    pub role_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleCondFormulaValueCreatedType {
    #[serde(rename = "createdLessThan")]
    CreatedLessThan,
    #[serde(rename = "createdMoreThan")]
    CreatedMoreThan,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleCondFormulaValueCreated {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::RoleCondFormulaValueCreatedType,
    pub sec: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleCondFormulaFollowersOrFollowingOrNotesType {
    #[serde(rename = "followersLessThanOrEq")]
    FollowersLessThanOrEq,
    #[serde(rename = "followersMoreThanOrEq")]
    FollowersMoreThanOrEq,
    #[serde(rename = "followingLessThanOrEq")]
    FollowingLessThanOrEq,
    #[serde(rename = "followingMoreThanOrEq")]
    FollowingMoreThanOrEq,
    #[serde(rename = "notesLessThanOrEq")]
    NotesLessThanOrEq,
    #[serde(rename = "notesMoreThanOrEq")]
    NotesMoreThanOrEq,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleCondFormulaFollowersOrFollowingOrNotes {
    pub id: ::std::string::String,
    pub r#type: crate::generated::latest::types::RoleCondFormulaFollowersOrFollowingOrNotesType,
    pub value: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum RoleCondFormulaValue {
    RoleCondFormulaLogics(crate::generated::latest::types::RoleCondFormulaLogics),
    RoleCondFormulaValueNot(crate::generated::latest::types::RoleCondFormulaValueNot),
    RoleCondFormulaValueAssignedRole(
        crate::generated::latest::types::RoleCondFormulaValueAssignedRole,
    ),
    RoleCondFormulaValueCreated(crate::generated::latest::types::RoleCondFormulaValueCreated),
    RoleCondFormulaFollowersOrFollowingOrNotes(
        crate::generated::latest::types::RoleCondFormulaFollowersOrFollowingOrNotes,
    ),
    RoleCondFormulaValueIsLocalOrRemote(
        crate::generated::latest::types::RoleCondFormulaValueIsLocalOrRemote,
    ),
    RoleCondFormulaValueUserSettingBooleanSchema(
        crate::generated::latest::types::RoleCondFormulaValueUserSettingBooleanSchema,
    ),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RoleLite {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    pub description: ::std::string::String,
    #[serde(rename = "isModerator")]
    pub is_moderator: bool,
    #[serde(rename = "isAdministrator")]
    pub is_administrator: bool,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RoleTarget {
    #[serde(rename = "manual")]
    Manual,
    #[serde(rename = "conditional")]
    Conditional,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RolePoliciesValue {
    #[serde(default)]
    pub value: ::std::option::Option<::serde_json::Value>,
    #[serde(default)]
    pub priority: ::std::option::Option<i64>,
    #[serde(rename = "useDefault", default)]
    pub use_default: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Role {
    pub id: ::std::string::String,
    pub name: ::std::string::String,
    pub color: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    pub description: ::std::string::String,
    #[serde(rename = "isModerator")]
    pub is_moderator: bool,
    #[serde(rename = "isAdministrator")]
    pub is_administrator: bool,
    #[serde(rename = "displayOrder")]
    pub display_order: i64,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    pub target: crate::generated::latest::types::RoleTarget,
    #[serde(rename = "condFormula")]
    pub cond_formula: crate::generated::latest::types::RoleCondFormulaValue,
    #[serde(rename = "isPublic")]
    pub is_public: bool,
    #[serde(rename = "isExplorable")]
    pub is_explorable: bool,
    #[serde(rename = "asBadge")]
    pub as_badge: bool,
    #[serde(rename = "preserveAssignmentOnMoveAccount")]
    pub preserve_assignment_on_move_account: bool,
    #[serde(rename = "canEditMembersByModerator")]
    pub can_edit_members_by_moderator: bool,
    pub policies: ::std::collections::HashMap<
        ::std::string::String,
        crate::generated::latest::types::RolePoliciesValue,
    >,
    #[serde(rename = "usersCount")]
    pub users_count: i64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum RolePoliciesChatAvailability {
    #[serde(rename = "available")]
    Available,
    #[serde(rename = "readonly")]
    Readonly,
    #[serde(rename = "unavailable")]
    Unavailable,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RolePolicies {
    #[serde(rename = "gtlAvailable")]
    pub gtl_available: bool,
    #[serde(rename = "ltlAvailable")]
    pub ltl_available: bool,
    #[serde(rename = "canPublicNote")]
    pub can_public_note: bool,
    #[serde(rename = "mentionLimit")]
    pub mention_limit: i64,
    #[serde(rename = "canInvite")]
    pub can_invite: bool,
    #[serde(rename = "inviteLimit")]
    pub invite_limit: i64,
    #[serde(rename = "inviteLimitCycle")]
    pub invite_limit_cycle: i64,
    #[serde(rename = "inviteExpirationTime")]
    pub invite_expiration_time: i64,
    #[serde(rename = "canManageCustomEmojis")]
    pub can_manage_custom_emojis: bool,
    #[serde(rename = "canManageAvatarDecorations")]
    pub can_manage_avatar_decorations: bool,
    #[serde(rename = "canSearchNotes")]
    pub can_search_notes: bool,
    #[serde(rename = "canSearchUsers")]
    pub can_search_users: bool,
    #[serde(rename = "canUseTranslator")]
    pub can_use_translator: bool,
    #[serde(rename = "canHideAds")]
    pub can_hide_ads: bool,
    #[serde(rename = "canCreateChannel")]
    pub can_create_channel: bool,
    #[serde(rename = "driveCapacityMb")]
    pub drive_capacity_mb: i64,
    #[serde(rename = "maxFileSizeMb")]
    pub max_file_size_mb: i64,
    #[serde(rename = "uploadableFileTypes")]
    pub uploadable_file_types: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "alwaysMarkNsfw")]
    pub always_mark_nsfw: bool,
    #[serde(rename = "canUpdateBioMedia")]
    pub can_update_bio_media: bool,
    #[serde(rename = "pinLimit")]
    pub pin_limit: i64,
    #[serde(rename = "antennaLimit")]
    pub antenna_limit: i64,
    #[serde(rename = "wordMuteLimit")]
    pub word_mute_limit: i64,
    #[serde(rename = "webhookLimit")]
    pub webhook_limit: i64,
    #[serde(rename = "clipLimit")]
    pub clip_limit: i64,
    #[serde(rename = "noteEachClipsLimit")]
    pub note_each_clips_limit: i64,
    #[serde(rename = "userListLimit")]
    pub user_list_limit: i64,
    #[serde(rename = "userEachUserListsLimit")]
    pub user_each_user_lists_limit: i64,
    #[serde(rename = "rateLimitFactor")]
    pub rate_limit_factor: f64,
    #[serde(rename = "avatarDecorationLimit")]
    pub avatar_decoration_limit: i64,
    #[serde(rename = "canImportAntennas")]
    pub can_import_antennas: bool,
    #[serde(rename = "canImportBlocking")]
    pub can_import_blocking: bool,
    #[serde(rename = "canImportFollowing")]
    pub can_import_following: bool,
    #[serde(rename = "canImportMuting")]
    pub can_import_muting: bool,
    #[serde(rename = "canImportUserLists")]
    pub can_import_user_lists: bool,
    #[serde(rename = "chatAvailability")]
    pub chat_availability: crate::generated::latest::types::RolePoliciesChatAvailability,
    #[serde(rename = "noteDraftLimit")]
    pub note_draft_limit: i64,
    #[serde(rename = "scheduledNoteLimit")]
    pub scheduled_note_limit: i64,
    #[serde(rename = "watermarkAvailable")]
    pub watermark_available: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum ReversiGameLiteBw {
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ReversiGameLite {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "startedAt")]
    pub started_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "endedAt")]
    pub ended_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "isStarted")]
    pub is_started: bool,
    #[serde(rename = "isEnded")]
    pub is_ended: bool,
    #[serde(rename = "user1Id")]
    pub user1_id: ::std::string::String,
    #[serde(rename = "user2Id")]
    pub user2_id: ::std::string::String,
    pub user1: crate::generated::latest::types::UserLite,
    pub user2: crate::generated::latest::types::UserLite,
    #[serde(rename = "winnerId")]
    pub winner_id: ::std::option::Option<::std::string::String>,
    pub winner: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "surrenderedUserId")]
    pub surrendered_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeoutUserId")]
    pub timeout_user_id: ::std::option::Option<::std::string::String>,
    pub black: ::std::option::Option<f64>,
    pub bw: crate::generated::latest::types::ReversiGameLiteBw,
    #[serde(rename = "noIrregularRules")]
    pub no_irregular_rules: bool,
    #[serde(rename = "isLlotheo")]
    pub is_llotheo: bool,
    #[serde(rename = "canPutEverywhere")]
    pub can_put_everywhere: bool,
    #[serde(rename = "loopedBoard")]
    pub looped_board: bool,
    #[serde(rename = "timeLimitForEachTurn")]
    pub time_limit_for_each_turn: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum ReversiGameDetailedBw {
    #[serde(rename = "random")]
    Random,
    #[serde(rename = "1")]
    _1,
    #[serde(rename = "2")]
    _2,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ReversiGameDetailed {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "startedAt")]
    pub started_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "endedAt")]
    pub ended_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "isStarted")]
    pub is_started: bool,
    #[serde(rename = "isEnded")]
    pub is_ended: bool,
    pub form1: ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    pub form2: ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(rename = "user1Ready")]
    pub user1_ready: bool,
    #[serde(rename = "user2Ready")]
    pub user2_ready: bool,
    #[serde(rename = "user1Id")]
    pub user1_id: ::std::string::String,
    #[serde(rename = "user2Id")]
    pub user2_id: ::std::string::String,
    pub user1: crate::generated::latest::types::UserLite,
    pub user2: crate::generated::latest::types::UserLite,
    #[serde(rename = "winnerId")]
    pub winner_id: ::std::option::Option<::std::string::String>,
    pub winner: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "surrenderedUserId")]
    pub surrendered_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "timeoutUserId")]
    pub timeout_user_id: ::std::option::Option<::std::string::String>,
    pub black: ::std::option::Option<f64>,
    pub bw: crate::generated::latest::types::ReversiGameDetailedBw,
    #[serde(rename = "noIrregularRules")]
    pub no_irregular_rules: bool,
    #[serde(rename = "isLlotheo")]
    pub is_llotheo: bool,
    #[serde(rename = "canPutEverywhere")]
    pub can_put_everywhere: bool,
    #[serde(rename = "loopedBoard")]
    pub looped_board: bool,
    #[serde(rename = "timeLimitForEachTurn")]
    pub time_limit_for_each_turn: f64,
    pub logs: ::std::vec::Vec<::std::vec::Vec<f64>>,
    pub map: ::std::vec::Vec<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaLiteAdsItem {
    pub id: ::std::string::String,
    pub url: ::std::string::String,
    pub place: ::std::string::String,
    pub ratio: f64,
    #[serde(rename = "imageUrl")]
    pub image_url: ::std::string::String,
    #[serde(rename = "dayOfWeek")]
    pub day_of_week: i64,
    #[serde(rename = "isSensitive", default)]
    pub is_sensitive: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaLiteSentryForFrontendOptions {
    pub dsn: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaLiteSentryForFrontend {
    pub options: crate::generated::latest::types::MetaLiteSentryForFrontendOptions,
    #[serde(rename = "vueIntegration", default)]
    pub vue_integration:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(rename = "browserTracingIntegration", default)]
    pub browser_tracing_integration:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
    #[serde(rename = "replayIntegration", default)]
    pub replay_integration:
        ::std::option::Option<::serde_json::Map<::std::string::String, ::serde_json::Value>>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MetaLiteNoteSearchableScope {
    #[serde(rename = "local")]
    Local,
    #[serde(rename = "global")]
    Global,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MetaLiteFederation {
    #[serde(rename = "all")]
    All,
    #[serde(rename = "specified")]
    Specified,
    #[serde(rename = "none")]
    None,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaLite {
    #[serde(rename = "maintainerName")]
    pub maintainer_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maintainerEmail")]
    pub maintainer_email: ::std::option::Option<::std::string::String>,
    pub version: ::std::string::String,
    #[serde(rename = "providesTarball")]
    pub provides_tarball: bool,
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shortName")]
    pub short_name: ::std::option::Option<::std::string::String>,
    pub uri: ::std::string::String,
    pub description: ::std::option::Option<::std::string::String>,
    pub langs: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "tosUrl")]
    pub tos_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repositoryUrl")]
    pub repository_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "feedbackUrl")]
    pub feedback_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultDarkTheme")]
    pub default_dark_theme: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultLightTheme")]
    pub default_light_theme: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientOptions")]
    pub client_options: crate::generated::latest::types::MetaClientOptions,
    #[serde(rename = "disableRegistration")]
    pub disable_registration: bool,
    #[serde(rename = "emailRequiredForSignup")]
    pub email_required_for_signup: bool,
    #[serde(rename = "enableHcaptcha")]
    pub enable_hcaptcha: bool,
    #[serde(rename = "hcaptchaSiteKey")]
    pub hcaptcha_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableMcaptcha")]
    pub enable_mcaptcha: bool,
    #[serde(rename = "mcaptchaSiteKey")]
    pub mcaptcha_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mcaptchaInstanceUrl")]
    pub mcaptcha_instance_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableRecaptcha")]
    pub enable_recaptcha: bool,
    #[serde(rename = "recaptchaSiteKey")]
    pub recaptcha_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableTurnstile")]
    pub enable_turnstile: bool,
    #[serde(rename = "turnstileSiteKey")]
    pub turnstile_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableTestcaptcha")]
    pub enable_testcaptcha: bool,
    #[serde(rename = "googleAnalyticsMeasurementId")]
    pub google_analytics_measurement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "swPublickey")]
    pub sw_publickey: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mascotImageUrl")]
    pub mascot_image_url: ::std::string::String,
    #[serde(rename = "bannerUrl")]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serverErrorImageUrl")]
    pub server_error_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "infoImageUrl")]
    pub info_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notFoundImageUrl")]
    pub not_found_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxNoteTextLength")]
    pub max_note_text_length: f64,
    pub ads: ::std::vec::Vec<crate::generated::latest::types::MetaLiteAdsItem>,
    #[serde(rename = "notesPerOneAd")]
    pub notes_per_one_ad: f64,
    #[serde(rename = "enableEmail")]
    pub enable_email: bool,
    #[serde(rename = "enableServiceWorker")]
    pub enable_service_worker: bool,
    #[serde(rename = "translatorAvailable")]
    pub translator_available: bool,
    #[serde(rename = "sentryForFrontend")]
    pub sentry_for_frontend:
        ::std::option::Option<crate::generated::latest::types::MetaLiteSentryForFrontend>,
    #[serde(rename = "mediaProxy")]
    pub media_proxy: ::std::string::String,
    #[serde(rename = "enableUrlPreview")]
    pub enable_url_preview: bool,
    #[serde(rename = "backgroundImageUrl")]
    pub background_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "impressumUrl")]
    pub impressum_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logoImageUrl")]
    pub logo_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inquiryUrl")]
    pub inquiry_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serverRules")]
    pub server_rules: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "themeColor")]
    pub theme_color: ::std::option::Option<::std::string::String>,
    pub policies: crate::generated::latest::types::RolePolicies,
    #[serde(rename = "noteSearchableScope")]
    pub note_searchable_scope: crate::generated::latest::types::MetaLiteNoteSearchableScope,
    #[serde(rename = "maxFileSize")]
    pub max_file_size: f64,
    pub federation: crate::generated::latest::types::MetaLiteFederation,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaDetailedOnlyFeatures {
    pub registration: bool,
    #[serde(rename = "emailRequiredForSignup")]
    pub email_required_for_signup: bool,
    #[serde(rename = "localTimeline", default)]
    pub local_timeline: ::std::option::Option<bool>,
    #[serde(rename = "globalTimeline", default)]
    pub global_timeline: ::std::option::Option<bool>,
    pub hcaptcha: bool,
    pub turnstile: bool,
    pub recaptcha: bool,
    #[serde(rename = "objectStorage")]
    pub object_storage: bool,
    #[serde(rename = "serviceWorker")]
    pub service_worker: bool,
    #[serde(default)]
    pub miauth: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaDetailedOnly {
    #[serde(default)]
    pub features: ::std::option::Option<crate::generated::latest::types::MetaDetailedOnlyFeatures>,
    #[serde(rename = "proxyAccountName")]
    pub proxy_account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requireSetup")]
    pub require_setup: bool,
    #[serde(rename = "cacheRemoteFiles")]
    pub cache_remote_files: bool,
    #[serde(rename = "cacheRemoteSensitiveFiles")]
    pub cache_remote_sensitive_files: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaDetailed {
    #[serde(rename = "maintainerName")]
    pub maintainer_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maintainerEmail")]
    pub maintainer_email: ::std::option::Option<::std::string::String>,
    pub version: ::std::string::String,
    #[serde(rename = "providesTarball")]
    pub provides_tarball: bool,
    pub name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "shortName")]
    pub short_name: ::std::option::Option<::std::string::String>,
    pub uri: ::std::string::String,
    pub description: ::std::option::Option<::std::string::String>,
    pub langs: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "tosUrl")]
    pub tos_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "repositoryUrl")]
    pub repository_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "feedbackUrl")]
    pub feedback_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultDarkTheme")]
    pub default_dark_theme: ::std::option::Option<::std::string::String>,
    #[serde(rename = "defaultLightTheme")]
    pub default_light_theme: ::std::option::Option<::std::string::String>,
    #[serde(rename = "clientOptions")]
    pub client_options: crate::generated::latest::types::MetaClientOptions,
    #[serde(rename = "disableRegistration")]
    pub disable_registration: bool,
    #[serde(rename = "emailRequiredForSignup")]
    pub email_required_for_signup: bool,
    #[serde(rename = "enableHcaptcha")]
    pub enable_hcaptcha: bool,
    #[serde(rename = "hcaptchaSiteKey")]
    pub hcaptcha_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableMcaptcha")]
    pub enable_mcaptcha: bool,
    #[serde(rename = "mcaptchaSiteKey")]
    pub mcaptcha_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mcaptchaInstanceUrl")]
    pub mcaptcha_instance_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableRecaptcha")]
    pub enable_recaptcha: bool,
    #[serde(rename = "recaptchaSiteKey")]
    pub recaptcha_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableTurnstile")]
    pub enable_turnstile: bool,
    #[serde(rename = "turnstileSiteKey")]
    pub turnstile_site_key: ::std::option::Option<::std::string::String>,
    #[serde(rename = "enableTestcaptcha")]
    pub enable_testcaptcha: bool,
    #[serde(rename = "googleAnalyticsMeasurementId")]
    pub google_analytics_measurement_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "swPublickey")]
    pub sw_publickey: ::std::option::Option<::std::string::String>,
    #[serde(rename = "mascotImageUrl")]
    pub mascot_image_url: ::std::string::String,
    #[serde(rename = "bannerUrl")]
    pub banner_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serverErrorImageUrl")]
    pub server_error_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "infoImageUrl")]
    pub info_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "notFoundImageUrl")]
    pub not_found_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "maxNoteTextLength")]
    pub max_note_text_length: f64,
    pub ads: ::std::vec::Vec<crate::generated::latest::types::MetaLiteAdsItem>,
    #[serde(rename = "notesPerOneAd")]
    pub notes_per_one_ad: f64,
    #[serde(rename = "enableEmail")]
    pub enable_email: bool,
    #[serde(rename = "enableServiceWorker")]
    pub enable_service_worker: bool,
    #[serde(rename = "translatorAvailable")]
    pub translator_available: bool,
    #[serde(rename = "sentryForFrontend")]
    pub sentry_for_frontend:
        ::std::option::Option<crate::generated::latest::types::MetaLiteSentryForFrontend>,
    #[serde(rename = "mediaProxy")]
    pub media_proxy: ::std::string::String,
    #[serde(rename = "enableUrlPreview")]
    pub enable_url_preview: bool,
    #[serde(rename = "backgroundImageUrl")]
    pub background_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "impressumUrl")]
    pub impressum_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "logoImageUrl")]
    pub logo_image_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "privacyPolicyUrl")]
    pub privacy_policy_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "inquiryUrl")]
    pub inquiry_url: ::std::option::Option<::std::string::String>,
    #[serde(rename = "serverRules")]
    pub server_rules: ::std::vec::Vec<::std::string::String>,
    #[serde(rename = "themeColor")]
    pub theme_color: ::std::option::Option<::std::string::String>,
    pub policies: crate::generated::latest::types::RolePolicies,
    #[serde(rename = "noteSearchableScope")]
    pub note_searchable_scope: crate::generated::latest::types::MetaLiteNoteSearchableScope,
    #[serde(rename = "maxFileSize")]
    pub max_file_size: f64,
    pub federation: crate::generated::latest::types::MetaLiteFederation,
    #[serde(default)]
    pub features: ::std::option::Option<crate::generated::latest::types::MetaDetailedOnlyFeatures>,
    #[serde(rename = "proxyAccountName")]
    pub proxy_account_name: ::std::option::Option<::std::string::String>,
    #[serde(rename = "requireSetup")]
    pub require_setup: bool,
    #[serde(rename = "cacheRemoteFiles")]
    pub cache_remote_files: bool,
    #[serde(rename = "cacheRemoteSensitiveFiles")]
    pub cache_remote_sensitive_files: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum MetaClientOptionsEntrancePageStyle {
    #[serde(rename = "classic")]
    Classic,
    #[serde(rename = "simple")]
    Simple,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct MetaClientOptions {
    #[serde(rename = "entrancePageStyle", default)]
    pub entrance_page_style:
        ::std::option::Option<crate::generated::latest::types::MetaClientOptionsEntrancePageStyle>,
    #[serde(rename = "showTimelineForVisitor", default)]
    pub show_timeline_for_visitor: ::std::option::Option<bool>,
    #[serde(rename = "showActivitiesForVisitor", default)]
    pub show_activities_for_visitor: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum UserWebhookOnItem {
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
pub struct UserWebhook {
    pub id: ::std::string::String,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub name: ::std::string::String,
    pub on: ::std::vec::Vec<crate::generated::latest::types::UserWebhookOnItem>,
    pub url: ::std::string::String,
    pub secret: ::std::string::String,
    pub active: bool,
    #[serde(rename = "latestSentAt")]
    pub latest_sent_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "latestStatus")]
    pub latest_status: ::std::option::Option<i64>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum SystemWebhookOnItem {
    #[serde(rename = "abuseReport")]
    AbuseReport,
    #[serde(rename = "abuseReportResolved")]
    AbuseReportResolved,
    #[serde(rename = "userCreated")]
    UserCreated,
    #[serde(rename = "inactiveModeratorsWarning")]
    InactiveModeratorsWarning,
    #[serde(rename = "inactiveModeratorsInvitationOnlyChanged")]
    InactiveModeratorsInvitationOnlyChanged,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct SystemWebhook {
    pub id: ::std::string::String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "latestSentAt")]
    pub latest_sent_at: ::std::option::Option<::chrono::DateTime<::chrono::Utc>>,
    #[serde(rename = "latestStatus")]
    pub latest_status: ::std::option::Option<f64>,
    pub name: ::std::string::String,
    pub on: ::std::vec::Vec<crate::generated::latest::types::SystemWebhookOnItem>,
    pub url: ::std::string::String,
    pub secret: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum AbuseReportNotificationRecipientMethod {
    #[serde(rename = "email")]
    Email,
    #[serde(rename = "webhook")]
    Webhook,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct AbuseReportNotificationRecipient {
    pub id: ::std::string::String,
    #[serde(rename = "isActive")]
    pub is_active: bool,
    #[serde(rename = "updatedAt")]
    pub updated_at: ::chrono::DateTime<::chrono::Utc>,
    pub name: ::std::string::String,
    pub method: crate::generated::latest::types::AbuseReportNotificationRecipientMethod,
    #[serde(rename = "userId", default)]
    pub user_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub user: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "systemWebhookId", default)]
    pub system_webhook_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "systemWebhook", default)]
    pub system_webhook: ::std::option::Option<crate::generated::latest::types::SystemWebhook>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessageReactionsItem {
    pub reaction: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessage {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "fromUserId")]
    pub from_user_id: ::std::string::String,
    #[serde(rename = "fromUser")]
    pub from_user: crate::generated::latest::types::UserLite,
    #[serde(rename = "toUserId", default)]
    pub to_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "toUser", default)]
    pub to_user: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "toRoomId", default)]
    pub to_room_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "toRoom", default)]
    pub to_room: ::std::option::Option<crate::generated::latest::types::ChatRoom>,
    #[serde(default)]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileId", default)]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub file: ::std::option::Option<crate::generated::latest::types::DriveFile>,
    #[serde(rename = "isRead", default)]
    pub is_read: ::std::option::Option<bool>,
    pub reactions: ::std::vec::Vec<crate::generated::latest::types::ChatMessageReactionsItem>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessageLiteReactionsItem {
    pub reaction: ::std::string::String,
    #[serde(default)]
    pub user: ::std::option::Option<crate::generated::latest::types::UserLite>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessageLite {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "fromUserId")]
    pub from_user_id: ::std::string::String,
    #[serde(rename = "fromUser", default)]
    pub from_user: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "toUserId", default)]
    pub to_user_id: ::std::option::Option<::std::string::String>,
    #[serde(rename = "toRoomId", default)]
    pub to_room_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileId", default)]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub file: ::std::option::Option<crate::generated::latest::types::DriveFile>,
    pub reactions: ::std::vec::Vec<crate::generated::latest::types::ChatMessageLiteReactionsItem>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessageLiteFor1on1ReactionsItem {
    pub reaction: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessageLiteFor1on1 {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "fromUserId")]
    pub from_user_id: ::std::string::String,
    #[serde(rename = "toUserId")]
    pub to_user_id: ::std::string::String,
    #[serde(default)]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileId", default)]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub file: ::std::option::Option<crate::generated::latest::types::DriveFile>,
    pub reactions:
        ::std::vec::Vec<crate::generated::latest::types::ChatMessageLiteFor1on1ReactionsItem>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessageLiteForRoomReactionsItem {
    pub reaction: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatMessageLiteForRoom {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "fromUserId")]
    pub from_user_id: ::std::string::String,
    #[serde(rename = "fromUser")]
    pub from_user: crate::generated::latest::types::UserLite,
    #[serde(rename = "toRoomId")]
    pub to_room_id: ::std::string::String,
    #[serde(default)]
    pub text: ::std::option::Option<::std::string::String>,
    #[serde(rename = "fileId", default)]
    pub file_id: ::std::option::Option<::std::string::String>,
    #[serde(default)]
    pub file: ::std::option::Option<crate::generated::latest::types::DriveFile>,
    pub reactions:
        ::std::vec::Vec<crate::generated::latest::types::ChatMessageLiteForRoomReactionsItem>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatRoom {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "ownerId")]
    pub owner_id: ::std::string::String,
    pub owner: crate::generated::latest::types::UserLite,
    pub name: ::std::string::String,
    pub description: ::std::string::String,
    #[serde(rename = "isMuted", default)]
    pub is_muted: ::std::option::Option<bool>,
    #[serde(rename = "invitationExists", default)]
    pub invitation_exists: ::std::option::Option<bool>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatRoomInvitation {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::latest::types::UserLite,
    #[serde(rename = "roomId")]
    pub room_id: ::std::string::String,
    pub room: crate::generated::latest::types::ChatRoom,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChatRoomMembership {
    pub id: ::std::string::String,
    #[serde(rename = "createdAt")]
    pub created_at: ::chrono::DateTime<::chrono::Utc>,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    #[serde(default)]
    pub user: ::std::option::Option<crate::generated::latest::types::UserLite>,
    #[serde(rename = "roomId")]
    pub room_id: ::std::string::String,
    #[serde(default)]
    pub room: ::std::option::Option<crate::generated::latest::types::ChatRoom>,
}
