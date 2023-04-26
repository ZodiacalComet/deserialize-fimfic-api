use std::fmt;

use serde::de::{self, Unexpected, Visitor};
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The different ratings a [`Story`](crate::Story) can have.
///
/// Implements [`Display`](fmt::Display) for `String` representations of each variant:
/// ```
/// # use fimfiction_api::StoryRating;
/// assert_eq!(StoryRating::Everyone.to_string(), "Everyone");
/// assert_eq!(StoryRating::Teen.to_string(), "Teen");
/// assert_eq!(StoryRating::Mature.to_string(), "Mature");
/// ```
#[derive(Debug, Clone, Copy)]
pub enum StoryRating {
    /// A story rated as for `Everyone`.
    Everyone,
    /// A story rated as `Teen`.
    Teen,
    /// A story rated as `Mature`.
    Mature,
}

impl PartialEq for StoryRating {
    fn eq(&self, other: &Self) -> bool {
        (*self as u8) == (*other as u8)
    }
}

impl fmt::Display for StoryRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            StoryRating::Everyone => write!(f, "Everyone"),
            StoryRating::Teen => write!(f, "Teen"),
            StoryRating::Mature => write!(f, "Mature"),
        }
    }
}

impl Serialize for StoryRating {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u8(*self as u8)
    }
}

struct RatingVisitor;

impl<'de> Visitor<'de> for RatingVisitor {
    type Value = StoryRating;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("an integer between 0 and 3")
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match value {
            0 => Ok(StoryRating::Everyone),
            1 => Ok(StoryRating::Teen),
            2 => Ok(StoryRating::Mature),
            _ => Err(E::invalid_value(Unexpected::Unsigned(value), &self)),
        }
    }
}

impl<'de> Deserialize<'de> for StoryRating {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RatingVisitor)
    }
}

pub(crate) mod serde_text {
    use super::*;

    struct RatingTextVisitor;

    impl<'de> Visitor<'de> for RatingTextVisitor {
        type Value = StoryRating;

        fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
            formatter.write_str("one \"Everyone\", \"Teen\" or \"Mature\"")
        }

        fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where
            E: de::Error,
        {
            match value {
                "Everyone" => Ok(StoryRating::Everyone),
                "Teen" => Ok(StoryRating::Teen),
                "Mature" => Ok(StoryRating::Mature),
                _ => Err(E::invalid_value(Unexpected::Str(value), &self)),
            }
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<StoryRating, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(RatingTextVisitor)
    }

    pub fn serialize<S>(rating: &StoryRating, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&rating.to_string())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::json;

    #[derive(Deserialize, Serialize, Debug)]
    struct Test {
        content_rating: StoryRating,
    }

    macro_rules! assert_deserialize {
        ($value:expr => $variant:ident) => {
            let json = json!({ "content_rating": $value });
            let test: Test =
                serde_json::from_value(json).expect("StoryRating should be deserializable");
            assert_eq!(test.content_rating, StoryRating::$variant);
        };
    }

    macro_rules! assert_serialize {
        ($variant:ident => $value:expr) => {
            let test = Test {
                content_rating: StoryRating::$variant,
            };
            let json = serde_json::to_string(&test).expect("StoryRating should be serializable");
            let expect = json!({ "content_rating": $value }).to_string();
            assert_eq!(json, expect);
        };
    }

    #[test]
    fn deserialize() {
        assert_deserialize!(0 => Everyone);
        assert_deserialize!(1 => Teen);
        assert_deserialize!(2 => Mature);
    }

    #[test]
    fn serialize() {
        assert_serialize!(Everyone => 0);
        assert_serialize!(Teen => 1);
        assert_serialize!(Mature => 2);
    }
}
