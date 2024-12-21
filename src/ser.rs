use super::NestedJson;
use serde::{Serialize, Serializer};

pub fn nest<S: Serializer, T: Serialize>(field: &T, s: S) -> Result<S::Ok, S::Error> {
    NestedJson(field).serialize(s)
}

pub fn nest_iter<'a, S: Serializer, T: Serialize>(field: &'a T, s: S) -> Result<S::Ok, S::Error>
where
    &'a T: IntoIterator,
    <&'a T as IntoIterator>::Item: Serialize,
{
    field
        .into_iter()
        .map(|i| serde_json::to_string(&i))
        .collect::<Result<Vec<_>, _>>()
        .map_err(serde::ser::Error::custom)?
        .serialize(s)
}

impl<T: Serialize> Serialize for NestedJson<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer
            .serialize_str(&serde_json::to_string(&self.0).map_err(serde::ser::Error::custom)?)
    }
}
