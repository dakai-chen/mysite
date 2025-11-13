pub mod iso8601 {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    use crate::util::time::UnixTimestampSecs;

    pub fn serialize<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        UnixTimestampSecs::new(*value)
            .into_str_iso8601()
            .map_err(serde::ser::Error::custom)?
            .serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<i64, D::Error>
    where
        D: Deserializer<'de>,
    {
        let time_str = String::deserialize(deserializer)?;
        UnixTimestampSecs::from_str_iso8601(&time_str)
            .map(|t| t.as_i64())
            .map_err(serde::de::Error::custom)
    }
}
