use super::NestedJson;

use serde::{
    de::{DeserializeOwned, Visitor},
    Deserialize, Deserializer,
};
use std::{
    collections::VecDeque,
    fmt::{Formatter, Result as FmtResult},
    marker::PhantomData,
};

pub struct NestedJsonVisitor<T>(PhantomData<T>);

impl<T> NestedJsonVisitor<T> {
    fn new() -> Self {
        Self(PhantomData)
    }
}

pub fn unnest<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<T, D::Error> {
    d.deserialize_any(NestedJsonVisitor::<T>::new())
}

pub fn unnest_vec<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    Vec::<String>::deserialize(d)?
        .into_iter()
        .map(|s| serde_json::from_str(&s).map_err(serde::de::Error::custom))
        .collect::<Result<Vec<_>, _>>()
}

impl<'de, T> Visitor<'de> for NestedJsonVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("expected a string")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let reader = VecDeque::from(v.to_string().into_bytes());
        let inner = Self::Value::deserialize(&mut serde_json::Deserializer::from_reader(reader))
            .map_err(E::custom)?;

        Ok(inner)
    }
}

impl<'de, T> Deserialize<'de> for NestedJson<T>
where
    T: Deserialize<'de>,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let visitor = NestedJsonVisitor::<T>::new();
        let inner = deserializer.deserialize_any(visitor)?;
        let nested = Self(inner);
        Ok(nested)
    }
}
