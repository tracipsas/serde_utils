use serde::de::{
    self,
    IntoDeserializer,
};
use std::fmt;

pub fn deserialize<'de, D, T>(deserializer: D) -> std::result::Result<Vec<T>, D::Error>
where
    D: de::Deserializer<'de>,
    T: de::DeserializeOwned,
{
    struct StringVecVisitor<T>(std::marker::PhantomData<T>);

    impl<'de, T> de::Visitor<'de> for StringVecVisitor<T>
    where
        T: de::DeserializeOwned,
    {
        type Value = Vec<T>;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("a string containing a list")
        }

        fn visit_str<E>(self, v: &str) -> std::result::Result<Self::Value, E>
        where
            E: de::Error,
        {
            let mut ids = Vec::new();
            for id in v.split(",") {
                let id = T::deserialize(id.into_deserializer())?;
                ids.push(id);
            }
            Ok(ids)
        }
    }

    deserializer.deserialize_any(StringVecVisitor(std::marker::PhantomData::<T>))
}

pub fn serialize<T, S>(data: &Vec<T>, serializer: S) -> Result<S::Ok, S::Error>
where
    T: std::fmt::Display,
    S: serde::Serializer,
{
    serializer.collect_str(
        &data
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<_>>()
            .join(","),
    )
}
