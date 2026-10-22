#![doc = "`drive` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`drive` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "drive";
    type Event = Event;
    type Message = crate::stream::NoMessage;
}
#[doc = "`drive` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    FileCreated(crate::generated::v2025_10_0::types::DriveFile),
    FileDeleted(::std::string::String),
    FileUpdated(crate::generated::v2025_10_0::types::DriveFile),
    FolderCreated(crate::generated::v2025_10_0::types::DriveFolder),
    FolderDeleted(::std::string::String),
    FolderUpdated(crate::generated::v2025_10_0::types::DriveFolder),
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
            "fileCreated" => Event::FileCreated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "fileDeleted" => Event::FileDeleted(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "fileUpdated" => Event::FileUpdated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "folderCreated" => Event::FolderCreated(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "folderDeleted" => Event::FolderDeleted(
                ::serde::Deserialize::deserialize(body).map_err(::serde::de::Error::custom)?,
            ),
            "folderUpdated" => Event::FolderUpdated(
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
            Event::FileCreated(body) => (
                "fileCreated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::FileDeleted(body) => (
                "fileDeleted",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::FileUpdated(body) => (
                "fileUpdated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::FolderCreated(body) => (
                "folderCreated",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::FolderDeleted(body) => (
                "folderDeleted",
                ::serde_json::to_value(body).map_err(::serde::ser::Error::custom)?,
            ),
            Event::FolderUpdated(body) => (
                "folderUpdated",
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
#[doc = "`drive` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`drive` チャンネルにつなぐビルダー。await でつなぐ。"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Connect<'a> {
    stream: &'a crate::generated::v2025_10_0::streaming::Stream,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::v2025_10_0::streaming::Stream) -> Self {
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
