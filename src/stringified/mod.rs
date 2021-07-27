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

pub mod hashmap_key {
    use std::{
        collections::HashMap,
        hash::Hash,
        str::FromStr,
    };

    use serde::{
        Deserialize,
        Serialize,
    };

    pub fn serialize<K, V, S>(data: &HashMap<K, V>, serializer: S) -> Result<S::Ok, S::Error>
    where
        K: std::fmt::Display,
        V: Serialize,
        S: serde::Serializer,
    {
        serializer.collect_map(data.iter().map(|(k, v)| (k.to_string(), v)))
    }

    pub fn deserialize<'de, K, V, D>(deserializer: D) -> Result<HashMap<K, V>, D::Error>
    where
        K: FromStr + Eq + Hash,
        V: Deserialize<'de>,
        K::Err: std::fmt::Display,
        D: serde::Deserializer<'de>,
    {
        let string_map = HashMap::<String, V>::deserialize(deserializer)?;
        string_map
            .into_iter()
            .map(|(k_string, v)| {
                k_string
                    .parse::<K>()
                    .map_err(serde::de::Error::custom)
                    .map(move |k| (k, v))
            })
            .collect()
    }

    #[cfg(test)]
    mod tests {
        use std::collections::HashMap;

        use serde::{
            Deserialize,
            Serialize,
        };
        use serde_json;

        #[derive(Serialize, Deserialize, Debug, Eq, PartialEq)]
        struct TestStruct {
            #[serde(with = "super")]
            field: HashMap<i32, Vec<String>>,
        }

        fn serialized_value() -> String {
            r#"{
                "field": {
                    "1": ["a", "b"],
                    "2": ["c", "d"]
                }
            }"#
            .to_owned()
        }

        fn deserialized_value() -> TestStruct {
            let mut value = TestStruct {
                field: HashMap::new(),
            };
            value.field.insert(1, vec!["a".to_owned(), "b".to_owned()]);
            value.field.insert(2, vec!["c".to_owned(), "d".to_owned()]);
            value
        }

        fn without_blank_characters(input: &str) -> String {
            let mut output = input.replace(" ", "");
            output = output.replace("\t", "");
            output = output.replace("\n", "");
            output
        }

        #[test]
        fn deserialize() {
            let actual_value: TestStruct = serde_json::from_str(&serialized_value()).unwrap();
            assert_eq!(actual_value, deserialized_value());
        }

        #[test]
        fn serialize() {
            let actual_value = serde_json::to_string(&deserialized_value()).unwrap();
            assert_eq!(
                without_blank_characters(&actual_value),
                without_blank_characters(&serialized_value())
            );
        }
    }
}
