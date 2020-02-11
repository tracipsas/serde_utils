use serde::Deserialize;

pub fn serialize<T, S>(data: &T, serializer: S) -> Result<S::Ok, S::Error>
    where
        T: std::fmt::Display,
        S: serde::Serializer,
{
    serializer.collect_str(data)
}

pub fn deserialize<'de, T, D>(deserializer: D) -> Result<T, D::Error>
    where
        T: std::str::FromStr,
        T::Err: std::fmt::Display,
        D: serde::Deserializer<'de>,
{
    String::deserialize(deserializer)?
        .parse()
        .map_err(serde::de::Error::custom)
}

pub mod vec {
    use serde::Deserialize;
    use std::str::FromStr;

    pub fn serialize<T, S>(data: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
        where
            T: std::fmt::Display,
            S: serde::Serializer,
    {
        serializer.collect_seq(data.iter().map(|elt| elt.to_string()))
    }

    pub fn deserialize<'de, T, D>(deserializer: D) -> Result<Vec<T>, D::Error>
        where
            T: FromStr,
            T::Err: std::fmt::Display,
            D: serde::Deserializer<'de>,
    {
        let string_seq = Vec::<String>::deserialize(deserializer)?;
        string_seq
            .into_iter()
            .map(|string| string.parse::<T>().map_err(serde::de::Error::custom))
            .collect()
    }
}
