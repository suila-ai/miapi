#![doc = "`queueStats` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
#[serde(untagged)]
pub enum RequestLogId {
    String(::std::string::String),
    Number(f64),
    #[doc = "このバージョンのバインディングが知らない形。"]
    Unrecognized(::serde_json::Value),
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct RequestLog {
    pub id: RequestLogId,
    pub length: f64,
}
#[doc = "`queueStats` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "queueStats";
    type Event = Event;
    type Message = Message;
}
#[doc = "`queueStats` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Stats(crate::generated::latest::streaming::types::QueueStats),
    StatsLog(crate::generated::latest::streaming::types::QueueStatsLog),
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
            "stats" => Event::Stats(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "statsLog" => Event::StatsLog(
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
            Event::Stats(body) => (
                "stats",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::StatsLog(body) => (
                "statsLog",
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
#[doc = "`queueStats` へ送るメッセージ。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    RequestLog(RequestLog),
}
impl<'de> ::serde::Deserialize<'de> for Message {
    fn deserialize<D: ::serde::Deserializer<'de>>(
        deserializer: D,
    ) -> ::std::result::Result<Self, D::Error> {
        let crate::stream::Envelope { r#type, body } =
            ::serde::Deserialize::deserialize(deserializer)?;
        ::std::result::Result::Ok(match r#type.as_str() {
            "requestLog" => Message::RequestLog(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            _ => {
                return ::std::result::Result::Err(::serde::de::Error::unknown_variant(
                    &r#type,
                    &["requestLog"],
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
            Message::RequestLog(body) => (
                "requestLog",
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
#[doc = "`queueStats` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`queueStats` チャンネルにつなぐビルダー。await でつなぐ。"]
pub struct Connect<'a> {
    stream: &'a crate::generated::latest::streaming::Stream,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::latest::streaming::Stream) -> Self {
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
