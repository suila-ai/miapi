#![doc = "`antenna` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`antenna` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {
    #[serde(rename = "antennaId")]
    pub antenna_id: ::std::string::String,
}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "antenna";
    type Event = Event;
    type Message = crate::stream::NoMessage;
}
#[doc = "`antenna` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Note(crate::generated::v2025_10_0::types::Note),
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
#[doc = "`antenna` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`antenna` チャンネルにつなぐビルダー。await でつなぐ。"]
#[doc = "必須パラメータ（型状態のスロットの順）: `antenna_id`。すべて設定すると、呼び出しを await できる。"]
#[must_use = "builders do nothing unless you `.await` them"]
pub struct Connect<'a, S = (crate::builder::Unset,)> {
    stream: &'a crate::generated::v2025_10_0::streaming::Stream,
    antenna_id: ::std::option::Option<::std::string::String>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::v2025_10_0::streaming::Stream) -> Self {
        Connect {
            stream,
            antenna_id: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Connect<'a, (S0,)> {
    pub fn antenna_id(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Connect<'a, (crate::builder::Set,)> {
        Connect {
            stream: self.stream,
            antenna_id: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Connect<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Connection, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Connection>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Params {
            antenna_id: self
                .antenna_id
                .expect("required parameter is set by the type state"),
        };
        let stream = self.stream;
        ::std::boxed::Box::pin(async move { stream.connect(request).await })
    }
}
