#![doc = "ストリーミング API のチャンネル。misskey-js の `streaming.types.ts` から生成した。"]
#![allow(clippy::large_enum_variant)]
pub mod admin;
pub mod antenna;
pub mod channel;
pub mod chat_room;
pub mod chat_user;
pub mod drive;
pub mod global_timeline;
pub mod hashtag;
pub mod home_timeline;
pub mod hybrid_timeline;
pub mod local_timeline;
pub mod main;
pub mod queue_stats;
pub mod reversi;
pub mod reversi_game;
pub mod role_timeline;
pub mod server_stats;
pub mod types;
pub mod user_list;
#[doc = "このバージョンセットでストリーミング API を使うストリーム。チャンネルのビルダーは、ここからメソッドで作る。"]
pub type Stream = crate::stream::Stream<super::Version>;
impl Stream {
    #[doc = "`main` チャンネルにつなぐビルダー。"]
    pub fn main(&self) -> main::Connect<'_> {
        main::Connect::new(self)
    }
    #[doc = "`homeTimeline` チャンネルにつなぐビルダー。"]
    pub fn home_timeline(&self) -> home_timeline::Connect<'_> {
        home_timeline::Connect::new(self)
    }
    #[doc = "`localTimeline` チャンネルにつなぐビルダー。"]
    pub fn local_timeline(&self) -> local_timeline::Connect<'_> {
        local_timeline::Connect::new(self)
    }
    #[doc = "`hybridTimeline` チャンネルにつなぐビルダー。"]
    pub fn hybrid_timeline(&self) -> hybrid_timeline::Connect<'_> {
        hybrid_timeline::Connect::new(self)
    }
    #[doc = "`globalTimeline` チャンネルにつなぐビルダー。"]
    pub fn global_timeline(&self) -> global_timeline::Connect<'_> {
        global_timeline::Connect::new(self)
    }
    #[doc = "`userList` チャンネルにつなぐビルダー。"]
    pub fn user_list(&self) -> user_list::Connect<'_> {
        user_list::Connect::new(self)
    }
    #[doc = "`hashtag` チャンネルにつなぐビルダー。"]
    pub fn hashtag(&self) -> hashtag::Connect<'_> {
        hashtag::Connect::new(self)
    }
    #[doc = "`roleTimeline` チャンネルにつなぐビルダー。"]
    pub fn role_timeline(&self) -> role_timeline::Connect<'_> {
        role_timeline::Connect::new(self)
    }
    #[doc = "`antenna` チャンネルにつなぐビルダー。"]
    pub fn antenna(&self) -> antenna::Connect<'_> {
        antenna::Connect::new(self)
    }
    #[doc = "`channel` チャンネルにつなぐビルダー。"]
    pub fn channel(&self) -> channel::Connect<'_> {
        channel::Connect::new(self)
    }
    #[doc = "`drive` チャンネルにつなぐビルダー。"]
    pub fn drive(&self) -> drive::Connect<'_> {
        drive::Connect::new(self)
    }
    #[doc = "`serverStats` チャンネルにつなぐビルダー。"]
    pub fn server_stats(&self) -> server_stats::Connect<'_> {
        server_stats::Connect::new(self)
    }
    #[doc = "`queueStats` チャンネルにつなぐビルダー。"]
    pub fn queue_stats(&self) -> queue_stats::Connect<'_> {
        queue_stats::Connect::new(self)
    }
    #[doc = "`admin` チャンネルにつなぐビルダー。"]
    pub fn admin(&self) -> admin::Connect<'_> {
        admin::Connect::new(self)
    }
    #[doc = "`reversi` チャンネルにつなぐビルダー。"]
    pub fn reversi(&self) -> reversi::Connect<'_> {
        reversi::Connect::new(self)
    }
    #[doc = "`reversiGame` チャンネルにつなぐビルダー。"]
    pub fn reversi_game(&self) -> reversi_game::Connect<'_> {
        reversi_game::Connect::new(self)
    }
    #[doc = "`chatUser` チャンネルにつなぐビルダー。"]
    pub fn chat_user(&self) -> chat_user::Connect<'_> {
        chat_user::Connect::new(self)
    }
    #[doc = "`chatRoom` チャンネルにつなぐビルダー。"]
    pub fn chat_room(&self) -> chat_room::Connect<'_> {
        chat_room::Connect::new(self)
    }
    #[doc = "ストリーム全体のイベント（`noteUpdated`・`emojiAdded` など）の受け口。"]
    pub fn broadcast(&self) -> crate::stream::Receiver<BroadcastEvent> {
        self.broadcast_receiver()
    }
}
#[doc = "ストリーム全体のイベント（misskey-js の `BroadcastEvents`）。"]
#[doc = "`noteUpdated` は、`Stream::subscribe_note` で購読したノートの分だけ届く。"]
#[derive(Debug, Clone, PartialEq)]
pub enum BroadcastEvent {
    NoteUpdated(crate::generated::v2025_10_0::streaming::types::NoteUpdatedEvent),
    EmojiAdded(crate::generated::v2025_10_0::streaming::types::EmojiAdded),
    EmojiUpdated(crate::generated::v2025_10_0::streaming::types::EmojiUpdated),
    EmojiDeleted(crate::generated::v2025_10_0::streaming::types::EmojiDeleted),
    AnnouncementCreated(crate::generated::v2025_10_0::streaming::types::AnnouncementCreated),
    #[doc = "このバージョンのバインディングが知らないイベント。"]
    Unrecognized {
        r#type: ::std::string::String,
        body: ::serde_json::Value,
    },
}
impl<'de> ::serde::Deserialize<'de> for BroadcastEvent {
    fn deserialize<D: ::serde::Deserializer<'de>>(
        deserializer: D,
    ) -> ::std::result::Result<Self, D::Error> {
        let crate::stream::Envelope { r#type, body } =
            ::serde::Deserialize::deserialize(deserializer)?;
        ::std::result::Result::Ok(match r#type.as_str() {
            "noteUpdated" => BroadcastEvent::NoteUpdated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "emojiAdded" => BroadcastEvent::EmojiAdded(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "emojiUpdated" => BroadcastEvent::EmojiUpdated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "emojiDeleted" => BroadcastEvent::EmojiDeleted(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "announcementCreated" => BroadcastEvent::AnnouncementCreated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            _ => BroadcastEvent::Unrecognized { r#type, body },
        })
    }
}
impl ::serde::Serialize for BroadcastEvent {
    fn serialize<S: ::serde::Serializer>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error> {
        let (r#type, body) = match self {
            BroadcastEvent::NoteUpdated(body) => (
                "noteUpdated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            BroadcastEvent::EmojiAdded(body) => (
                "emojiAdded",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            BroadcastEvent::EmojiUpdated(body) => (
                "emojiUpdated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            BroadcastEvent::EmojiDeleted(body) => (
                "emojiDeleted",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            BroadcastEvent::AnnouncementCreated(body) => (
                "announcementCreated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            BroadcastEvent::Unrecognized { r#type, body } => (r#type.as_str(), body.clone()),
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
