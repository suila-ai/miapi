#![doc = "`hashtag` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`hashtag` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {
    pub q: ::std::vec::Vec<::std::vec::Vec<::std::string::String>>,
}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "hashtag";
    type Event = Event;
    type Message = crate::stream::NoMessage;
}
#[doc = "`hashtag` から届くイベント。"]
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Note(crate::generated::latest::types::Note),
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
#[doc = "`hashtag` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`hashtag` チャンネルにつなぐビルダー。await でつなぐ。"]
#[doc = "必須パラメータ（型状態のスロットの順）: `q`。すべて設定すると、呼び出しを await できる。"]
pub struct Connect<'a, S = (crate::builder::Unset,)> {
    stream: &'a crate::generated::latest::streaming::Stream,
    q: ::std::option::Option<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::latest::streaming::Stream) -> Self {
        Connect {
            stream,
            q: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Connect<'a, (S0,)> {
    pub fn q(
        self,
        value: impl ::std::convert::Into<::std::vec::Vec<::std::vec::Vec<::std::string::String>>>,
    ) -> Connect<'a, (crate::builder::Set,)> {
        Connect {
            stream: self.stream,
            q: ::std::option::Option::Some(value.into()),
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Connect<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Connection, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Connection>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Params {
            q: self.q.expect("required parameter is set by the type state"),
        };
        let stream = self.stream;
        ::std::boxed::Box::pin(async move { stream.connect(request).await })
    }
}
