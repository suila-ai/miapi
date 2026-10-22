#![doc = "`chatRoom` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct React {
    pub reaction: ::std::string::String,
    #[serde(default)]
    pub user: ::std::option::Option<crate::generated::v2026_5_1::types::UserLite>,
    #[serde(rename = "messageId")]
    pub message_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Unreact {
    pub reaction: ::std::string::String,
    #[serde(default)]
    pub user: ::std::option::Option<crate::generated::v2026_5_1::types::UserLite>,
    #[serde(rename = "messageId")]
    pub message_id: ::std::string::String,
}
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Read {
    pub id: ::std::string::String,
}
#[doc = "`chatRoom` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {
    #[serde(rename = "roomId")]
    pub room_id: ::std::string::String,
}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "chatRoom";
    type Event = Event;
    type Message = Message;
}
#[doc = "`chatRoom` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Message(crate::generated::v2026_5_1::types::ChatMessageLite),
    Deleted(::std::string::String),
    React(React),
    Unreact(Unreact),
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
            "message" => Event::Message(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "deleted" => Event::Deleted(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "react" => Event::React(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "unreact" => Event::Unreact(
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
            Event::Message(body) => (
                "message",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Deleted(body) => (
                "deleted",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::React(body) => (
                "react",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Unreact(body) => (
                "unreact",
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
#[doc = "`chatRoom` へ送るメッセージ。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Read(Read),
}
impl<'de> ::serde::Deserialize<'de> for Message {
    fn deserialize<D: ::serde::Deserializer<'de>>(
        deserializer: D,
    ) -> ::std::result::Result<Self, D::Error> {
        let crate::stream::Envelope { r#type, body } =
            ::serde::Deserialize::deserialize(deserializer)?;
        ::std::result::Result::Ok(match r#type.as_str() {
            "read" => Message::Read(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            _ => {
                return ::std::result::Result::Err(::serde::de::Error::unknown_variant(
                    &r#type,
                    &["read"],
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
            Message::Read(body) => (
                "read",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
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
#[doc = "`chatRoom` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`chatRoom` チャンネルにつなぐビルダー。await でつなぐ。"]
#[doc = "必須パラメータ（型状態のスロットの順）: `room_id`。すべて設定すると、呼び出しを await できる。"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Connect<'a, S = (crate::builder::Unset,)> {
    stream: &'a crate::generated::v2026_5_1::streaming::Stream,
    room_id: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::v2026_5_1::streaming::Stream) -> Self {
        Connect {
            stream,
            room_id: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Connect<'a, (S0,)> {
    pub fn room_id(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Connect<'a, (crate::builder::Set,)> {
        Connect {
            stream: self.stream,
            room_id: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Connect<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Connection, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Connection>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Params {
            room_id: self
                .room_id
                .expect("required parameter is set by the type state"),
        };
        let stream = self.stream;
        ::std::boxed::Box::pin(async move { stream.connect(request).await })
    }
}
