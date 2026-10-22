#![doc = "`main` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum Followed {
    MeDetailed(crate::generated::v2026_5_1::types::MeDetailed),
    UserDetailedNotMe(crate::generated::v2026_5_1::types::UserDetailedNotMe),
    UserLite(crate::generated::v2026_5_1::types::UserLite),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UrlUploadFinished {
    pub marker: ::std::option::Option<::std::string::String>,
    pub file: crate::generated::v2026_5_1::types::DriveFile,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RegistryUpdated {
    #[serde(default)]
    pub scope: ::std::option::Option<::std::vec::Vec<::std::string::String>>,
    pub key: ::std::string::String,
    pub value: ::std::option::Option<::serde_json::Value>,
}
#[doc = "`main` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "main";
    type Event = Event;
    type Message = crate::stream::NoMessage;
}
#[doc = "`main` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Notification(crate::generated::v2026_5_1::types::Notification),
    Mention(crate::generated::v2026_5_1::types::Note),
    Reply(crate::generated::v2026_5_1::types::Note),
    Renote(crate::generated::v2026_5_1::types::Note),
    #[doc = "自分が他人をフォローしたとき"]
    Follow(crate::generated::v2026_5_1::types::UserDetailedNotMe),
    #[doc = "他人が自分をフォローしたとき"]
    Followed(Followed),
    #[doc = "自分が他人をフォロー解除したとき"]
    Unfollow(crate::generated::v2026_5_1::types::UserDetailed),
    MeUpdated(crate::generated::v2026_5_1::types::UserDetailed),
    PageEvent(crate::generated::v2026_5_1::streaming::types::PageEvent),
    UrlUploadFinished(UrlUploadFinished),
    ReadAllNotifications,
    UnreadNotification(crate::generated::v2026_5_1::types::Notification),
    NotificationFlushed,
    UnreadAntenna(crate::generated::v2026_5_1::types::Antenna),
    NewChatMessage(crate::generated::v2026_5_1::types::ChatMessage),
    ReadAllAnnouncements,
    MyTokenRegenerated,
    Signin(crate::generated::v2026_5_1::types::Signin),
    RegistryUpdated(RegistryUpdated),
    DriveFileCreated(crate::generated::v2026_5_1::types::DriveFile),
    ReadAntenna(crate::generated::v2026_5_1::types::Antenna),
    ReceiveFollowRequest(crate::generated::v2026_5_1::types::User),
    AnnouncementCreated(crate::generated::v2026_5_1::streaming::types::AnnouncementCreated),
    #[doc = "このバージョンのバインディングが知らないイベント。"]
    Unrecognized {
        r#type: ::std::string::String,
        body: ::serde_json::Value,
    },
}
impl<'de> ::serde::Deserialize<'de> for Event {
    fn deserialize<D: ::serde::Deserializer<'de>>(
        deserializer: D,
    ) -> ::std::result::Result<Self, D::Error> {
        let crate::stream::Envelope { r#type, body } =
            ::serde::Deserialize::deserialize(deserializer)?;
        ::std::result::Result::Ok(match r#type.as_str() {
            "notification" => Event::Notification(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "mention" => Event::Mention(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "reply" => Event::Reply(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "renote" => Event::Renote(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "follow" => Event::Follow(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "followed" => Event::Followed(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "unfollow" => Event::Unfollow(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "meUpdated" => Event::MeUpdated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "pageEvent" => Event::PageEvent(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "urlUploadFinished" => Event::UrlUploadFinished(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "readAllNotifications" => Event::ReadAllNotifications,
            "unreadNotification" => Event::UnreadNotification(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "notificationFlushed" => Event::NotificationFlushed,
            "unreadAntenna" => Event::UnreadAntenna(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "newChatMessage" => Event::NewChatMessage(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "readAllAnnouncements" => Event::ReadAllAnnouncements,
            "myTokenRegenerated" => Event::MyTokenRegenerated,
            "signin" => Event::Signin(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "registryUpdated" => Event::RegistryUpdated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "driveFileCreated" => Event::DriveFileCreated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "readAntenna" => Event::ReadAntenna(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "receiveFollowRequest" => Event::ReceiveFollowRequest(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "announcementCreated" => Event::AnnouncementCreated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            _ => Event::Unrecognized { r#type, body },
        })
    }
}
impl ::serde::Serialize for Event {
    fn serialize<S: ::serde::Serializer>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error> {
        let (r#type, body) = match self {
            Event::Notification(body) => (
                "notification",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Mention(body) => (
                "mention",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Reply(body) => (
                "reply",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Renote(body) => (
                "renote",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Follow(body) => (
                "follow",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Followed(body) => (
                "followed",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Unfollow(body) => (
                "unfollow",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::MeUpdated(body) => (
                "meUpdated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::PageEvent(body) => (
                "pageEvent",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::UrlUploadFinished(body) => (
                "urlUploadFinished",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::ReadAllNotifications => ("readAllNotifications", ::serde_json::Value::Null),
            Event::UnreadNotification(body) => (
                "unreadNotification",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::NotificationFlushed => ("notificationFlushed", ::serde_json::Value::Null),
            Event::UnreadAntenna(body) => (
                "unreadAntenna",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::NewChatMessage(body) => (
                "newChatMessage",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::ReadAllAnnouncements => ("readAllAnnouncements", ::serde_json::Value::Null),
            Event::MyTokenRegenerated => ("myTokenRegenerated", ::serde_json::Value::Null),
            Event::Signin(body) => (
                "signin",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::RegistryUpdated(body) => (
                "registryUpdated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::DriveFileCreated(body) => (
                "driveFileCreated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::ReadAntenna(body) => (
                "readAntenna",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::ReceiveFollowRequest(body) => (
                "receiveFollowRequest",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::AnnouncementCreated(body) => (
                "announcementCreated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Unrecognized { r#type, body } => (r#type.as_str(), body.clone()),
        };
        ::serde::Serialize::serialize(
            &crate::stream::Envelope {
                r#type: r#type.to_owned(),
                body,
            },
            serializer,
        )
    }
}
#[doc = "`main` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`main` チャンネルにつなぐビルダー。await でつなぐ。"]
pub struct Connect<'a> {
    stream: &'a crate::generated::v2026_5_1::streaming::Stream,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::v2026_5_1::streaming::Stream) -> Self {
        Connect { stream }
    }
}
impl<'a> Connect<'a> {}
impl<'a> ::std::future::IntoFuture for Connect<'a> {
    type Output = ::std::result::Result<Connection, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Connection>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Params {};
        let stream = self.stream;
        ::std::boxed::Box::pin(async move { stream.connect(request).await })
    }
}
