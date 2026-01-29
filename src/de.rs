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

pub fn unnest<'de, D: Deserializer<'de>, T: Deserialize<'de>>(d: D) -> Result<T, D::Error> {
    d.deserialize_any(NestedJsonVisitor(PhantomData))
}

pub fn unnest_vec<'de, D: Deserializer<'de>, T: DeserializeOwned>(
    d: D,
) -> Result<Vec<T>, D::Error> {
    Vec::<String>::deserialize(d)?
        .into_iter()
        .enumerate()
        .map(|(idx, s)| {
            serde_json::from_str(&s).map_err(|e| {
                serde::de::Error::custom(format!(
                    "error parsing nested JSON string at index {}: {} (note: line/column are relative to the nested string, not the outer document)",
                    idx, e
                ))
            })
        })
        .collect::<Result<Vec<_>, _>>()
}

impl<'de, T> Visitor<'de> for NestedJsonVisitor<T>
where
    T: Deserialize<'de>,
{
    type Value = T;

    fn expecting(&self, formatter: &mut Formatter) -> FmtResult {
        formatter.write_str("a string or null")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        let reader = VecDeque::from(v.to_string().into_bytes());
        let inner = Self::Value::deserialize(&mut serde_json::Deserializer::from_reader(reader))
            .map_err(|e| {
                E::custom(format!(
                    "error parsing nested JSON string: {} (note: line/column are relative to the nested string, not the outer document)",
                    e
                ))
            })?;

        Ok(inner)
    }

    fn visit_unit<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // `null` maps naturally to T::deserialize(()).
        // This works when T is `Option<_>` (becomes None).
        T::deserialize(serde::de::value::UnitDeserializer::new())
    }

    fn visit_none<E>(self) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        // same as visit unit
        T::deserialize(serde::de::value::UnitDeserializer::new())
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
        let visitor = NestedJsonVisitor(PhantomData);
        let inner = deserializer.deserialize_any(visitor)?;
        let nested = Self(inner);
        Ok(nested)
    }
}
