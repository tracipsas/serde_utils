use serde::{
    de::Error,
    Serialize,
};

pub fn serialize<S: serde::Serializer>(data: &Vec<u8>, serializer: S) -> Result<S::Ok, S::Error> {
    hex::encode(data).serialize(serializer)
}

pub fn deserialize<'de, D: serde::Deserializer<'de>>(deserializer: D) -> Result<Vec<u8>, D::Error> {
    let hex_data: String = serde::Deserialize::deserialize(deserializer)?;
    hex::decode(&hex_data).map_err(D::Error::custom)
}

pub mod option {
    use serde::{
        de::Error,
        Serialize,
    };

    pub fn serialize<S: serde::Serializer>(
        opt_data: &Option<Vec<u8>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        opt_data
            .as_ref()
            .map(|data| hex::encode(data))
            .serialize(serializer)
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Option<Vec<u8>>, D::Error> {
        let hex_data_opt: Option<String> = serde::Deserialize::deserialize(deserializer)?;
        hex_data_opt
            .map(|hex_data| hex::decode(&hex_data).map_err(D::Error::custom))
            .transpose()
    }
}

pub mod vec {
    use serde::Deserialize;

    pub fn serialize<S: serde::Serializer>(
        data: &Vec<Vec<u8>>,
        serializer: S,
    ) -> Result<S::Ok, S::Error> {
        serializer.collect_seq(data.iter().map(|elt| hex::encode(elt)))
    }

    pub fn deserialize<'de, D: serde::Deserializer<'de>>(
        deserializer: D,
    ) -> Result<Vec<Vec<u8>>, D::Error> {
        let string_seq = Vec::<String>::deserialize(deserializer)?;
        string_seq
            .into_iter()
            .map(|string| hex::decode(string).map_err(serde::de::Error::custom))
            .collect()
    }
}
