use serde::{
    Deserialize,
    Deserializer,
    Serialize,
    Serializer,
};

// https://stackoverflow.com/questions/44331037/how-can-i-distinguish-between-a-deserialized-field-that-is-missing-and-one-that

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}
