#![doc = "`userList` チャンネル。"]
#![allow(clippy::type_complexity, clippy::large_enum_variant)]
#[doc = "`userList` につなぐときの params。"]
#[derive(Debug, Clone, PartialEq, Default, serde :: Serialize, serde :: Deserialize)]
pub struct Params {
    #[serde(rename = "listId")]
    pub list_id: ::std::string::String,
    #[serde(rename = "withFiles", default, skip_serializing_if = "Option::is_none")]
    pub with_files: ::std::option::Option<bool>,
    #[serde(
        rename = "withRenotes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub with_renotes: ::std::option::Option<bool>,
}
impl crate::stream::Channel for Params {
    const NAME: &'static str = "userList";
    type Event = Event;
    type Message = crate::stream::NoMessage;
}
#[doc = "`userList` から届くイベント。"]
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
#[doc = "`userList` への接続。"]
pub type Connection = crate::stream::Connection<Params>;
#[doc = "`userList` チャンネルにつなぐビルダー。await でつなぐ。"]
#[doc = "必須パラメータ（型状態のスロットの順）: `list_id`。すべて設定すると、呼び出しを await できる。"]
pub struct Connect<'a, S = (crate::builder::Unset,)> {
    stream: &'a crate::generated::v2026_5_1::streaming::Stream,
    list_id: ::std::option::Option<::std::string::String>,
    with_files: ::std::option::Option<bool>,
    with_renotes: ::std::option::Option<bool>,
    _state: ::std::marker::PhantomData<S>,
}
impl<'a> Connect<'a> {
    pub(crate) fn new(stream: &'a crate::generated::v2026_5_1::streaming::Stream) -> Self {
        Connect {
            stream,
            list_id: ::std::option::Option::None,
            with_files: ::std::option::Option::None,
            with_renotes: ::std::option::Option::None,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a, S0> Connect<'a, (S0,)> {
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
    pub fn list_id(
        self,
        value: impl ::std::convert::Into<::std::string::String>,
    ) -> Connect<'a, (crate::builder::Set,)> {
        Connect {
            stream: self.stream,
            list_id: ::std::option::Option::Some(value.into()),
            with_files: self.with_files,
            with_renotes: self.with_renotes,
            _state: ::std::marker::PhantomData,
        }
    }
}
impl<'a> ::std::future::IntoFuture for Connect<'a, (crate::builder::Set,)> {
    type Output = ::std::result::Result<Connection, crate::Error>;
    type IntoFuture = crate::builder::CallFuture<'a, Connection>;
    fn into_future(self) -> Self::IntoFuture {
        let request = Params {
            list_id: self
                .list_id
                .expect("required parameter is set by the type state"),
            with_files: self.with_files,
            with_renotes: self.with_renotes,
        };
        let stream = self.stream;
        ::std::boxed::Box::pin(async move { stream.connect(request).await })
    }
}
