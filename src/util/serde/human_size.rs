use bytesize::ByteSize;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub fn serialize<S, T>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    for<'a> Serde<&'a T>: Serialize,
    S: Serializer,
{
    Serde(value).serialize(serializer)
}

pub fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    Serde<T>: Deserialize<'de>,
    D: Deserializer<'de>,
{
    Serde::deserialize(deserializer).map(|v| v.0)
}

pub struct Serde<T>(T);

impl Serialize for Serde<&'_ Option<u64>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self.0).map(ByteSize::b).serialize(serializer)
    }
}

impl Serialize for Serde<&'_ u64> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        ByteSize::b(*self.0).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Serde<Option<u64>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<ByteSize>::deserialize(deserializer)?;
        Ok(Serde(value.map(|v| v.as_u64())))
    }
}

impl<'de> Deserialize<'de> for Serde<u64> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = ByteSize::deserialize(deserializer)?;
        Ok(Serde(value.as_u64()))
    }
}

impl Serialize for Serde<&'_ Option<usize>> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (*self.0)
            .map(u64::try_from)
            .transpose()
            .map_err(serde::ser::Error::custom)?
            .map(ByteSize::b)
            .serialize(serializer)
    }
}

impl Serialize for Serde<&'_ usize> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = u64::try_from(*self.0).map_err(serde::ser::Error::custom)?;
        ByteSize::b(value).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for Serde<Option<usize>> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = Option::<ByteSize>::deserialize(deserializer)?;
        let value = value
            .map(|v| v.as_u64())
            .map(usize::try_from)
            .transpose()
            .map_err(serde::de::Error::custom)?;
        Ok(Serde(value))
    }
}

impl<'de> Deserialize<'de> for Serde<usize> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = ByteSize::deserialize(deserializer)?;
        let value = usize::try_from(value.as_u64()).map_err(serde::de::Error::custom)?;
        Ok(Serde(value))
    }
}
