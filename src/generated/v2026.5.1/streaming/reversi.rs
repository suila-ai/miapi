#![doc = "`reversi` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Matched {
    pub game: crate::generated::v2026_5_1::types::ReversiGameDetailed,
}
#[derive(Debug, Clone, PartialEq, serde :: Serialize, serde :: Deserialize)]
pub struct Invited {
    pub user: crate::generated::v2026_5_1::types::User,
}
#[doc = "`reversi` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "reversi";
    type Event = Event;
    type Message = crate::stream::NoMessage;
}
#[doc = "`reversi` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Matched(Matched),
    Invited(Invited),
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
            "matched" => Event::Matched(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "invited" => Event::Invited(
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
            Event::Matched(body) => (
                "matched",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::Invited(body) => (
                "invited",
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
#[doc = "`reversi` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`reversi` チャンネルにつなぐビルダー。await でつなぐ。"]
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
