#![doc = "`homeTimeline` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`homeTimeline` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {
    #[serde(
        rename = "withRenotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub with_renotes: ::std::option::Option<bool>,
    #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
    pub with_files: ::std::option::Option<bool>,
}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "homeTimeline";
    type Event = Event;
    type Message = crate::stream::NoMessage;
}
#[doc = "`homeTimeline` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Note(crate::generated::v2026_5_1::types::Note),
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
            "note" => Event::Note(
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
            Event::Note(body) => (
                "note",
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
#[doc = "`homeTimeline` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`homeTimeline` チャンネルにつなぐビルダー。await でつなぐ。"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Connect<'a> {
    stream: &'a crate::generated::v2026_5_1::streaming::Stream,
    with_renotes: ::std::option::Option<bool>,
    with_files: ::std::option::Option<bool>,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::v2026_5_1::streaming::Stream) -> Self {
        Connect {
            stream,
            with_renotes: ::std::option::Option::None,
            with_files: ::std::option::Option::None,
        }
    }
}
impl<'a> Connect<'a> {
    pub fn with_renotes(self, value: impl ::std::convert::Into<bool>) -> Self {
        self.maybe_with_renotes(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `withRenotes` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_with_renotes(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<bool>>,
    ) -> Self {
        self.with_renotes = value.map(|value| value.into());
        self
    }
    pub fn with_files(self, value: impl ::std::convert::Into<bool>) -> Self {
        self.maybe_with_files(::std::option::Option::Some(value))
    }
    #[doc = "`Some` なら `withFiles` を設定し、`None` なら設定を消して送らない。"]
    pub fn maybe_with_files(
        mut self,
        value: ::std::option::Option<impl ::std::convert::Into<bool>>,
    ) -> Self {
        self.with_files = value.map(|value| value.into());
        self
    }
}
impl<'a> ::std::future::IntoFuture for Connect<'a> {
    type Output = ::std::result::Result<Connection, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Connection>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Params {
            with_renotes: self.with_renotes,
            with_files: self.with_files,
        };
        let stream = self.stream;
        ::std::boxed::Box::pin(async move { stream.connect(request).await })
    }
}
