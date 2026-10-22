#![doc = "misskey-js の型定義（`entities.ts`・`consts.ts`・`streaming.types.ts`）にある、ストリーミングだけが使う型。"]
#![allow(clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct PageEvent {
    #[serde(rename = "pageId")]
    pub page_id: ::std::string::String,
    pub event: ::std::string::String,
    pub var: ::serde_json::Value,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub user: crate::generated::v2025_10_0::types::User,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct AnnouncementCreated {
    pub announcement: crate::generated::v2025_10_0::types::Announcement,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ServerStatsMem {
    pub used: f64,
    pub active: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ServerStatsNet {
    pub rx: f64,
    pub tx: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ServerStatsFs {
    pub r: f64,
    pub w: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ServerStats {
    pub cpu: f64,
    pub mem: crate::generated::v2025_10_0::streaming::types::ServerStatsMem,
    pub net: crate::generated::v2025_10_0::streaming::types::ServerStatsNet,
    pub fs: crate::generated::v2025_10_0::streaming::types::ServerStatsFs,
}
pub type ServerStatsLog =
    ::std::vec::Vec<crate::generated::v2025_10_0::streaming::types::ServerStats>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct QueueStatsDeliver {
    #[serde(rename = "activeSincePrevTick")]
    pub active_since_prev_tick: f64,
    pub active: f64,
    pub waiting: f64,
    pub delayed: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct QueueStatsInbox {
    #[serde(rename = "activeSincePrevTick")]
    pub active_since_prev_tick: f64,
    pub active: f64,
    pub waiting: f64,
    pub delayed: f64,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct QueueStats {
    pub deliver: crate::generated::v2025_10_0::streaming::types::QueueStatsDeliver,
    pub inbox: crate::generated::v2025_10_0::streaming::types::QueueStatsInbox,
}
pub type QueueStatsLog =
    ::std::vec::Vec<crate::generated::v2025_10_0::streaming::types::QueueStats>;
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum ReversiUpdateKey {
    #[serde(rename = "map")]
    Map,
    #[serde(rename = "bw")]
    Bw,
    #[serde(rename = "isLlotheo")]
    IsLlotheo,
    #[serde(rename = "canPutEverywhere")]
    CanPutEverywhere,
    #[serde(rename = "loopedBoard")]
    LoopedBoard,
    #[serde(rename = "timeLimitForEachTurn")]
    TimeLimitForEachTurn,
    #[doc = "このバージョンのバインディングが知らない値。"]
    #[serde(untagged)]
    Unrecognized(::std::string::String),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventReactedBodyEmoji {
    pub name: ::std::string::String,
    pub url: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventReactedBody {
    pub reaction: ::std::string::String,
    pub emoji: ::std::option::Option<
        crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventReactedBodyEmoji,
    >,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventReacted {
    pub id: ::std::string::String,
    pub body: crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventReactedBody,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventUnreactedBody {
    pub reaction: ::std::string::String,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventUnreacted {
    pub id: ::std::string::String,
    pub body: crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventUnreactedBody,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventDeletedBody {
    #[serde(rename = "deletedAt")]
    pub deleted_at: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventDeleted {
    pub id: ::std::string::String,
    pub body: crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventDeletedBody,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventPollVotedBody {
    pub choice: f64,
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct NoteUpdatedEventPollVoted {
    pub id: ::std::string::String,
    pub body: crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventPollVotedBody,
}
#[derive(Debug, Clone, PartialEq)]
pub enum NoteUpdatedEvent {
    Reacted(crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventReacted),
    Unreacted(crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventUnreacted),
    Deleted(crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventDeleted),
    PollVoted(crate::generated::v2025_10_0::streaming::types::NoteUpdatedEventPollVoted),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
impl<'de> ::serde::Deserialize<'de> for NoteUpdatedEvent {
    fn deserialize<D: ::serde::Deserializer<'de>>(
        deserializer: D,
    ) -> ::std::result::Result<Self, D::Error> {
        let value = <::serde_json::Value as ::serde::Deserialize>::deserialize(deserializer)?;
        let tag = value
            .get("type")
            .and_then(::serde_json::Value::as_str)
            .map(str::to_owned);
        ::std::result::Result::Ok(match tag.as_deref() {
            ::std::option::Option::Some("reacted") => NoteUpdatedEvent::Reacted(
                ::serde::Deserialize::deserialize(value).map_err(::serde::de::Error::custom)?,
            ),
            ::std::option::Option::Some("unreacted") => NoteUpdatedEvent::Unreacted(
                ::serde::Deserialize::deserialize(value).map_err(::serde::de::Error::custom)?,
            ),
            ::std::option::Option::Some("deleted") => NoteUpdatedEvent::Deleted(
                ::serde::Deserialize::deserialize(value).map_err(::serde::de::Error::custom)?,
            ),
            ::std::option::Option::Some("pollVoted") => NoteUpdatedEvent::PollVoted(
                ::serde::Deserialize::deserialize(value).map_err(::serde::de::Error::custom)?,
            ),
            _ => NoteUpdatedEvent::Unrecognized(value),
        })
    }
}
impl ::serde::Serialize for NoteUpdatedEvent {
    fn serialize<S: ::serde::Serializer>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error> {
        match self {
            NoteUpdatedEvent::Reacted(inner) => {
                let mut value =
                    ::serde_json::to_value(inner).map_err(::serde::ser::Error::custom)?;
                if let ::serde_json::Value::Object(object) = &mut value {
                    object.insert(
                        "type".to_owned(),
                        ::serde_json::Value::String("reacted".to_owned()),
                    );
                }
                ::serde::Serialize::serialize(&value, serializer)
            }
            NoteUpdatedEvent::Unreacted(inner) => {
                let mut value =
                    ::serde_json::to_value(inner).map_err(::serde::ser::Error::custom)?;
                if let ::serde_json::Value::Object(object) = &mut value {
                    object.insert(
                        "type".to_owned(),
                        ::serde_json::Value::String("unreacted".to_owned()),
                    );
                }
                ::serde::Serialize::serialize(&value, serializer)
            }
            NoteUpdatedEvent::Deleted(inner) => {
                let mut value =
                    ::serde_json::to_value(inner).map_err(::serde::ser::Error::custom)?;
                if let ::serde_json::Value::Object(object) = &mut value {
                    object.insert(
                        "type".to_owned(),
                        ::serde_json::Value::String("deleted".to_owned()),
                    );
                }
                ::serde::Serialize::serialize(&value, serializer)
            }
            NoteUpdatedEvent::PollVoted(inner) => {
                let mut value =
                    ::serde_json::to_value(inner).map_err(::serde::ser::Error::custom)?;
                if let ::serde_json::Value::Object(object) = &mut value {
                    object.insert(
                        "type".to_owned(),
                        ::serde_json::Value::String("pollVoted".to_owned()),
                    );
                }
                ::serde::Serialize::serialize(&value, serializer)
            }
            NoteUpdatedEvent::Unrecognized(value) => {
                ::serde::Serialize::serialize(value, serializer)
            }
        }
    }
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct EmojiAdded {
    pub emoji: crate::generated::v2025_10_0::types::EmojiDetailed,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct EmojiUpdated {
    pub emojis: ::std::vec::Vec<crate::generated::v2025_10_0::types::EmojiDetailed>,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct EmojiDeleted {
    pub emojis: ::std::vec::Vec<crate::generated::v2025_10_0::types::EmojiDetailed>,
}
