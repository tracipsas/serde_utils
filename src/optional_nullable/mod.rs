use serde::{Deserializer, Deserialize, Serializer, Serialize};

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
    where
        D: Deserializer<'de>,
        T: Deserialize<'de>,
{
    Ok(Some(Option::deserialize(deserializer)?))
}
