use serde::{
    de::Error,
    Deserialize,
    Serialize,
};

fn naivedatetime_to_string(date: chrono::NaiveDateTime) -> String {
    date.format("%Y-%m-%d %H:%M:%S").to_string()
}

pub fn serialize<S: serde::Serializer>(
    date: &chrono::NaiveDateTime,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    naivedatetime_to_string(date.clone()).serialize(serializer)
}

pub fn deserialize<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<chrono::NaiveDateTime, D::Error> {
    let date: String = Deserialize::deserialize(deserializer)?;
    Ok(
        chrono::NaiveDateTime::parse_from_str(&date, "%Y-%m-%d %H:%M:%S")
            .map_err(D::Error::custom)?,
    )
}
