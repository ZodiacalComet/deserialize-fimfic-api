use std::{convert::TryInto, fmt};

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserializer, Serializer};

struct VoteVisitor;

impl<'de> Visitor<'de> for VoteVisitor {
    type Value = Option<u32>;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_fmt(format_args!(
            "a negative integer or an integer between 0 and {}",
            u32::MAX
        ))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        value
            .try_into()
            .map(Some)
            .map_err(|_| E::invalid_value(Unexpected::Unsigned(value), &self))
    }

    fn visit_i64<E>(self, _value: i64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(None)
    }
}

pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<u32>, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(VoteVisitor)
}

pub fn serialize<S>(vote: &Option<u32>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match vote {
        Some(vote) => serializer.serialize_u32(*vote),
        None => serializer.serialize_i32(-1),
    }
}
