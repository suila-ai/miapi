#![doc = "`reversiGame` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Started {
    pub game: crate::generated::v2026_5_1::types::ReversiGameDetailed,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Ended {
    #[serde(rename = "winnerId")]
    pub winner_id: ::std::option::Option<::std::string::String>,
    pub game: crate::generated::v2026_5_1::types::ReversiGameDetailed,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Canceled {
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ChangeReadyStates {
    pub user1: bool,
    pub user2: bool,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct UpdateSettings {
    #[serde(rename = "userId")]
    pub user_id: ::std::string::String,
    pub key: crate::generated::v2026_5_1::streaming::types::ReversiUpdateKey,
    pub value: ::serde_json::Value,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub enum LogOperation {
    #[serde(rename = "put")]
    Put,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Log {
    pub time: f64,
    pub player: bool,
    pub operation: LogOperation,
    pub pos: f64,
    pub id: ::std::option::Option<::std::string::String>,
}
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct PutStone {
    pub pos: f64,
    pub id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct ReversiUpdateSettings {
    pub key: crate::generated::v2026_5_1::streaming::types::ReversiUpdateKey,
    pub value: ::serde_json::Value,
}
#[doc = "`reversiGame` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {
    #[serde(rename = "gameId")]
    pub game_id: ::std::string::String,
}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "reversiGame";
    type Event = Event;
    type Message = Message;
}
#[doc = "`reversiGame` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Started(Started),
    Ended(Ended),
    Canceled(Canceled),
    ChangeReadyStates(ChangeReadyStates),
    UpdateSettings(UpdateSettings),
    Log(Log),
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
            "started" => Event::Started(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "ended" => Event::Ended(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "canceled" => Event::Canceled(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "changeReadyStates" => Event::ChangeReadyStates(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "updateSettings" => Event::UpdateSettings(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "log" => Event::Log(
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
            Event::Started(body) => (
                "started",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Ended(body) => (
                "ended",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Canceled(body) => (
                "canceled",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::ChangeReadyStates(body) => (
                "changeReadyStates",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::UpdateSettings(body) => (
                "updateSettings",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Log(body) => (
                "log",
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
#[doc = "`reversiGame` へ送るメッセージ。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    PutStone(PutStone),
    Ready(bool),
    Cancel,
    UpdateSettings(ReversiUpdateSettings),
    ClaimTimeIsUp,
}
impl<'de> ::serde::Deserialize<'de> for Message {
    fn deserialize<D: ::serde::Deserializer<'de>>(
        deserializer: D,
    ) -> ::std::result::Result<Self, D::Error> {
        let crate::stream::Envelope { r#type, body } =
            ::serde::Deserialize::deserialize(deserializer)?;
        ::std::result::Result::Ok(match r#type.as_str() {
            "putStone" => Message::PutStone(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "ready" => Message::Ready(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "cancel" => Message::Cancel,
            "updateSettings" => Message::UpdateSettings(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "claimTimeIsUp" => Message::ClaimTimeIsUp,
            _ => {
                return ::std::result::Result::Err(::serde::de::Error::unknown_variant(
                    &r#type,
                    &[
                        "putStone",
                        "ready",
                        "cancel",
                        "updateSettings",
                        "claimTimeIsUp",
                    ],
                ));
            }
        })
    }
}
impl ::serde::Serialize for Message {
    fn serialize<S: ::serde::Serializer>(
        &self,
        serializer: S,
    ) -> ::std::result::Result<S::Ok, S::Error> {
        let (r#type, body) = match self {
            Message::PutStone(body) => (
                "putStone",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Message::Ready(body) => (
                "ready",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Message::Cancel => ("cancel", ::serde_json::Value::Null),
            Message::UpdateSettings(body) => (
                "updateSettings",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Message::ClaimTimeIsUp => ("claimTimeIsUp", ::serde_json::Value::Null),
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
#[doc = "`reversiGame` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`reversiGame` チャンネルにつなぐビルダー。await でつなぐ。"]
#[doc = "必須パラメータ（型状態のスロットの順）: `game_id`。すべて設定すると、呼び出しを await できる。"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Connect<'a, S = (crate::builder::Unset,)> {
    stream: &'a crate::generated::v2026_5_1::streaming::Stream,
    game_id: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::v2026_5_1::streaming::Stream) -> Self {
        Connect {
            stream,
            game_id: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Connect<'a, (S0,)> {
    pub fn game_id(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Connect<'a, (crate::builder::Set,)> {
        Connect {
            stream: self.stream,
            game_id: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Connect<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Connection, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Connection>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Params {
            game_id: self
                .game_id
                .expect("required parameter is set by the type state"),
        };
        let stream = self.stream;
        ::std::boxed::Box::pin(async move { stream.connect(request).await })
    }
}
