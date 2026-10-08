//! 生成されるリクエスト型が使う serde のアダプタ。

/// `Option<Option<T>>` のフィールド用。
/// `None` は省略し（`skip_serializing_if = "Option::is_none"` も指定する必要がある）、
/// `Some(None)` は `null` として、`Some(Some(v))` はその値として送る。
pub mod double_option {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<T, S>(value: &Option<Option<T>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: Serialize,
        S: Serializer,
    {
        match value {
            Some(inner) => inner.serialize(serializer),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
    where
        T: Deserialize<'de>,
        D: Deserializer<'de>,
    {
        Option::<T>::deserialize(deserializer).map(Some)
    }
}
