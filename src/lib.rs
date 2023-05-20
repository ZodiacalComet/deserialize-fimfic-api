//! Deserialization for the JSON responses of the [Fimfiction][fimfiction] story API
//! (`https://www.fimfiction.net/api/story.php?story={ID}`).
//!
//! ```no_run
//! # use fimfiction_api::{Story, StoryError};
//! # let response = String::new();
//! let story: Story = fimfiction_api::from_str(&response)?;
//! # Ok::<(), StoryError>(())
//! ```
//!
//! # Disclaimer
//!
//! This small crate is not affiliated not endorsed in any way by [Fimfiction][fimfiction] or its
//! team. Do not bother them when it doesn't work as intended.
//!
//! # The `chrono` feature
//!
//! Changes all date fields to use `DateTime<Utc>` instead of an `i64`.
//!
//! [fimfiction]: https://www.fimfiction.net/
#![deny(missing_docs, missing_debug_implementations, dead_code)]

#[cfg(feature = "chrono")]
use chrono::{offset::Utc, DateTime};
use serde::{Deserialize, Serialize};
use thiserror::Error;

mod rating;
mod status;
mod vote;

pub use rating::StoryRating;
pub use status::StoryStatus;

/// A Fimfiction ID.
pub type Id = u32;

/// Container struct of the author response given by the Fimfiction story API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Author {
    /// Author's ID.
    pub id: Id,
    /// Username of the author.
    pub name: String,
}

/// Container struct for all chapter response data given by the Fimfiction story API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Chapter {
    /// Chapter's ID.
    pub id: Id,
    /// Title of the chapter.
    pub title: String,
    /// The amount of words the chapter has.
    pub words: u64,
    /// The amount of views the chapter has.
    pub views: u32,
    /// Fimfiction URL to the story's chapter.
    pub link: String,

    #[cfg(not(feature = "chrono"))]
    /// Last chapter update timestamp.
    pub date_modified: i64,
    #[cfg(feature = "chrono")]
    #[serde(with = "chrono::serde::ts_seconds")]
    /// Last chapter update datetime.
    pub date_modified: DateTime<Utc>,
}

/// Container struct for all relevant story response data given by the Fimfiction story API.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Story {
    /// Unique story ID.
    pub id: Id,
    /// Title of the story.
    pub title: String,
    /// Fimfiction URL to the story.
    pub url: String,
    /// Summary of the story. Showed on story cards present in the main page, groups and sidebars
    /// story listing.
    pub short_description: String,
    /// Complete story description, showed on the main story page.
    pub description: String,

    #[cfg(not(feature = "chrono"))]
    /// Last story update timestamp.
    pub date_modified: i64,
    #[cfg(feature = "chrono")]
    #[serde(with = "chrono::serde::ts_seconds")]
    /// Last story update datetime.
    pub date_modified: DateTime<Utc>,

    /// Story cover image in thumbnail size if any.
    pub image: Option<String>,
    /// Story cover image in full size if any.
    pub full_image: Option<String>,
    /// The views the story has.
    pub views: u32,
    /// The total views the story has.
    pub total_views: u32,
    /// The amount of words the story has.
    pub words: u64,
    /// The amount of chapters the story has.
    pub chapter_count: u64,
    /// The amount of comments the story has.
    pub comments: u32,
    /// Author of the story.
    pub author: Author,
    /// Story completion status.
    pub status: StoryStatus,
    /// Rating of the story as a String.
    ///
    /// This is needed to get back the complete original response content from serializing this
    /// struct.
    #[serde(with = "rating::serde_text")]
    content_rating_text: StoryRating,
    /// Rating given to the story.
    pub content_rating: StoryRating,
    /// The amount of likes the story has, if not disabled.
    #[serde(with = "vote")]
    pub likes: Option<u32>,
    /// The amount of dislikes the story has, if not disabled.
    #[serde(with = "vote")]
    pub dislikes: Option<u32>,
    /// Chapters of the story.
    #[serde(default)]
    pub chapters: Vec<Chapter>,
}

/// Represents errors that can occur while deserializing a [`Story`].
#[derive(Debug, Error)]
pub enum StoryError {
    /// A deserialization error.
    #[error("json deserialization error: {0}")]
    Json(#[from] serde_json::Error),

    /// Alias for `"Invalid story id"` API error message.
    #[error("API error: Invalid story ID")]
    InvalidId,

    /// An API error message which doesn't have its own variant.
    #[error("API error: {0}")]
    Api(String),
}

/// Represents the different responses that the Fimfiction story API can return.
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Response {
    /// The API returned a [`Story`].
    Story(Story),
    /// The API returned an error.
    Error(String),
}

/// Deserialize an instance of [`Story`] from an API response String.
///
/// # Errors
/// * On a deserialization error (see [`serde_json::from_str()`]).
/// * The resulting [`Response`] is of the [`Error`](Response::Error) variant.
pub fn from_str(input: &str) -> Result<Story, StoryError> {
    let res = serde_json::from_str::<Response>(input)?;

    match res {
        Response::Story(story) => Ok(story),
        Response::Error(err) => Err(match err.as_str() {
            "Invalid story id" => StoryError::InvalidId,
            _ => StoryError::Api(err),
        }),
    }
}

/// Serialize a [`Story`] as a Fimfiction story response String.
///
/// A convenience function for wrapping `story` into a [`Response`] and getting the string from
/// [`serde_json::to_string()`].
pub fn to_string(story: Story) -> Result<String, serde_json::Error> {
    serde_json::to_string(&Response::Story(story))
}

#[cfg(test)]
mod test {
    use super::*;

    use serde_json::Value;

    static RESPONSE_SAMPLE: &str = r#"{
  "story": {
    "id": 428991,
    "title": "How the Tantabus Parses Sleep",
    "url": "https://www.fimfiction.net/story/428991/how-the-tantabus-parses-sleep",
    "short_description": "The second Tantabus continues to grow, learn, and flourish. And maybe screw with certain ponies on the side.",
    "description": "What started with laziness and a tiny slip-up in arcane intelligence creation has blossomed into its own being. With Luna's blessing, the second Tantabus is free to flit around the dreamscape as it pleases, unburdened by oversight. And what pleases it is making good dreams. Who couldn't be happy by making others happy?\r\n\r\nAs the Tantabus settles into its life, its horizons keep expanding, forever revealing more of the world and of ponies. There's always new things to learn. New dreams to spice up. New nightmares to beat down. New ponies to help. New ponies to annoy.\r\n\r\nDon't tell Luna about that last part.",
    "date_modified": 1671122514,
    "image": "https://cdn-img.fimfiction.net/story/iwqb-1673322192-428991-medium",
    "full_image": "https://cdn-img.fimfiction.net/story/iwqb-1673322192-428991-full",
    "views": 10712,
    "total_views": 178800,
    "words": 275949,
    "chapter_count": 40,
    "comments": 1929,
    "author": {
      "id": 253168,
      "name": "Rambling Writer"
    },
    "status": "Incomplete",
    "content_rating_text": "Everyone",
    "content_rating": 0,
    "likes": 1020,
    "dislikes": 8,
    "chapters": [
      {
        "id": 1273271,
        "title": "Nightmares and the Deletion Thereof",
        "words": 5215,
        "views": 10712,
        "link": "https://www.fimfiction.net/story/428991/1/how-the-tantabus-parses-sleep/nightmares-and-the-deletion-thereof",
        "date_modified": 1631067292
      },
      {
        "id": 1291292,
        "title": "Robbery, He Dreamed",
        "words": 4094,
        "views": 7059,
        "link": "https://www.fimfiction.net/story/428991/2/how-the-tantabus-parses-sleep/robbery-he-dreamed",
        "date_modified": 1577915443
      },
      {
        "id": 1306118,
        "title": "Parallel Friendshipping",
        "words": 6734,
        "views": 6990,
        "link": "https://www.fimfiction.net/story/428991/3/how-the-tantabus-parses-sleep/parallel-friendshipping",
        "date_modified": 1557744585
      },
      {
        "id": 1315597,
        "title": "Tulpa ex Somnium",
        "words": 5918,
        "views": 6305,
        "link": "https://www.fimfiction.net/story/428991/4/how-the-tantabus-parses-sleep/tulpa-ex-somnium",
        "date_modified": 1560006968
      },
      {
        "id": 1327818,
        "title": "Halt and Catch Fire",
        "words": 8041,
        "views": 6796,
        "link": "https://www.fimfiction.net/story/428991/5/how-the-tantabus-parses-sleep/halt-and-catch-fire",
        "date_modified": 1563895815
      },
      {
        "id": 1341288,
        "title": "Corner Cases",
        "words": 8133,
        "views": 6279,
        "link": "https://www.fimfiction.net/story/428991/6/how-the-tantabus-parses-sleep/corner-cases",
        "date_modified": 1617627814
      },
      {
        "id": 1349152,
        "title": "Class Conversion",
        "words": 5143,
        "views": 6222,
        "link": "https://www.fimfiction.net/story/428991/7/how-the-tantabus-parses-sleep/class-conversion",
        "date_modified": 1597239916
      },
      {
        "id": 1356526,
        "title": "Frightening Foals for Fun and Finances",
        "words": 6626,
        "views": 5577,
        "link": "https://www.fimfiction.net/story/428991/8/how-the-tantabus-parses-sleep/frightening-foals-for-fun-and-finances",
        "date_modified": 1625766623
      },
      {
        "id": 1365466,
        "title": "Machine Teaching",
        "words": 6451,
        "views": 5855,
        "link": "https://www.fimfiction.net/story/428991/9/how-the-tantabus-parses-sleep/machine-teaching",
        "date_modified": 1592483985
      },
      {
        "id": 1375472,
        "title": "Source Incantations: Debugging",
        "words": 5321,
        "views": 5213,
        "link": "https://www.fimfiction.net/story/428991/10/how-the-tantabus-parses-sleep/source-incantations-debugging",
        "date_modified": 1598766861
      },
      {
        "id": 1375809,
        "title": "Source Incantations: Unit Testing",
        "words": 6871,
        "views": 5789,
        "link": "https://www.fimfiction.net/story/428991/11/how-the-tantabus-parses-sleep/source-incantations-unit-testing",
        "date_modified": 1578848620
      },
      {
        "id": 1399983,
        "title": "Stress Test",
        "words": 6612,
        "views": 5280,
        "link": "https://www.fimfiction.net/story/428991/12/how-the-tantabus-parses-sleep/stress-test",
        "date_modified": 1586799620
      },
      {
        "id": 1402312,
        "title": "Oracle Machines and Archmages",
        "words": 6170,
        "views": 5279,
        "link": "https://www.fimfiction.net/story/428991/13/how-the-tantabus-parses-sleep/oracle-machines-and-archmages",
        "date_modified": 1610912770
      },
      {
        "id": 1416030,
        "title": "Magfault",
        "words": 9263,
        "views": 4962,
        "link": "https://www.fimfiction.net/story/428991/14/how-the-tantabus-parses-sleep/magfault",
        "date_modified": 1640555276
      },
      {
        "id": 1426104,
        "title": "Rootkit",
        "words": 5029,
        "views": 4716,
        "link": "https://www.fimfiction.net/story/428991/15/how-the-tantabus-parses-sleep/rootkit",
        "date_modified": 1616131537
      },
      {
        "id": 1432623,
        "title": "Machine Teaching II: Oneiric Boogaloo",
        "words": 7057,
        "views": 4647,
        "link": "https://www.fimfiction.net/story/428991/16/how-the-tantabus-parses-sleep/machine-teaching-ii-oneiric-boogaloo",
        "date_modified": 1610340575
      },
      {
        "id": 1438608,
        "title": "Nightmarewall",
        "words": 7116,
        "views": 4645,
        "link": "https://www.fimfiction.net/story/428991/17/how-the-tantabus-parses-sleep/nightmarewall",
        "date_modified": 1669507251
      },
      {
        "id": 1449521,
        "title": "Deployment Environments: Sandbox",
        "words": 7394,
        "views": 4068,
        "link": "https://www.fimfiction.net/story/428991/18/how-the-tantabus-parses-sleep/deployment-environments-sandbox",
        "date_modified": 1627263960
      },
      {
        "id": 1451455,
        "title": "Deployment Environments: Release",
        "words": 8591,
        "views": 3898,
        "link": "https://www.fimfiction.net/story/428991/19/how-the-tantabus-parses-sleep/deployment-environments-release",
        "date_modified": 1602954630
      },
      {
        "id": 1453388,
        "title": "Caster Party",
        "words": 7790,
        "views": 4057,
        "link": "https://www.fimfiction.net/story/428991/20/how-the-tantabus-parses-sleep/caster-party",
        "date_modified": 1606610684
      },
      {
        "id": 1455478,
        "title": "Petrifying Pupils for Pleasure and Profit",
        "words": 4243,
        "views": 3869,
        "link": "https://www.fimfiction.net/story/428991/21/how-the-tantabus-parses-sleep/petrifying-pupils-for-pleasure-and-profit",
        "date_modified": 1604158079
      },
      {
        "id": 1457383,
        "title": "Quality Assurance",
        "words": 8800,
        "views": 4424,
        "link": "https://www.fimfiction.net/story/428991/22/how-the-tantabus-parses-sleep/quality-assurance",
        "date_modified": 1604767426
      },
      {
        "id": 1465017,
        "title": "Backend Testing",
        "words": 4599,
        "views": 3633,
        "link": "https://www.fimfiction.net/story/428991/23/how-the-tantabus-parses-sleep/backend-testing",
        "date_modified": 1612878316
      },
      {
        "id": 1469218,
        "title": "Open Source",
        "words": 5833,
        "views": 3690,
        "link": "https://www.fimfiction.net/story/428991/24/how-the-tantabus-parses-sleep/open-source",
        "date_modified": 1608563802
      },
      {
        "id": 1476898,
        "title": "Daydream Believers: Expectations",
        "words": 6924,
        "views": 3527,
        "link": "https://www.fimfiction.net/story/428991/25/how-the-tantabus-parses-sleep/daydream-believers-expectations",
        "date_modified": 1623734072
      },
      {
        "id": 1478712,
        "title": "Daydream Believers: Reality",
        "words": 6984,
        "views": 3628,
        "link": "https://www.fimfiction.net/story/428991/26/how-the-tantabus-parses-sleep/daydream-believers-reality",
        "date_modified": 1627071066
      },
      {
        "id": 1483565,
        "title": "Baby Don't Hurt Me",
        "words": 5791,
        "views": 3692,
        "link": "https://www.fimfiction.net/story/428991/27/how-the-tantabus-parses-sleep/baby-dont-hurt-me",
        "date_modified": 1613328939
      },
      {
        "id": 1491245,
        "title": "Bug Hunt",
        "words": 9261,
        "views": 3758,
        "link": "https://www.fimfiction.net/story/428991/28/how-the-tantabus-parses-sleep/bug-hunt",
        "date_modified": 1616261633
      },
      {
        "id": 1494810,
        "title": "The Battle of the Bell",
        "words": 6602,
        "views": 3723,
        "link": "https://www.fimfiction.net/story/428991/29/how-the-tantabus-parses-sleep/the-battle-of-the-bell",
        "date_modified": 1638802999
      },
      {
        "id": 1504708,
        "title": "The Galascene",
        "words": 10521,
        "views": 4081,
        "link": "https://www.fimfiction.net/story/428991/30/how-the-tantabus-parses-sleep/the-galascene",
        "date_modified": 1634268673
      },
      {
        "id": 1514941,
        "title": "Gone Gold",
        "words": 5231,
        "views": 3979,
        "link": "https://www.fimfiction.net/story/428991/31/how-the-tantabus-parses-sleep/gone-gold",
        "date_modified": 1635958250
      },
      {
        "id": 1533490,
        "title": "Logging System",
        "words": 5831,
        "views": 3493,
        "link": "https://www.fimfiction.net/story/428991/32/how-the-tantabus-parses-sleep/logging-system",
        "date_modified": 1628626556
      },
      {
        "id": 1549838,
        "title": "Day One Patch",
        "words": 4911,
        "views": 2839,
        "link": "https://www.fimfiction.net/story/428991/33/how-the-tantabus-parses-sleep/day-one-patch",
        "date_modified": 1634401563
      },
      {
        "id": 1553343,
        "title": "Scaring Servants for Sport and Sales",
        "words": 4414,
        "views": 3088,
        "link": "https://www.fimfiction.net/story/428991/34/how-the-tantabus-parses-sleep/scaring-servants-for-sport-and-sales",
        "date_modified": 1635691226
      },
      {
        "id": 1584499,
        "title": "Dream Message Access Protocol",
        "words": 10453,
        "views": 2835,
        "link": "https://www.fimfiction.net/story/428991/35/how-the-tantabus-parses-sleep/dream-message-access-protocol",
        "date_modified": 1647873486
      },
      {
        "id": 1600405,
        "title": "Arcane Postmare Interface",
        "words": 8991,
        "views": 2432,
        "link": "https://www.fimfiction.net/story/428991/36/how-the-tantabus-parses-sleep/arcane-postmare-interface",
        "date_modified": 1654282765
      },
      {
        "id": 1610775,
        "title": "Problem Exists Between Map and Assignment: Submission",
        "words": 9607,
        "views": 2021,
        "link": "https://www.fimfiction.net/story/428991/37/how-the-tantabus-parses-sleep/problem-exists-between-map-and-assignment-submission",
        "date_modified": 1658837597
      },
      {
        "id": 1612207,
        "title": "Problem Exists Between Map and Assignment: Escalation",
        "words": 13085,
        "views": 1936,
        "link": "https://www.fimfiction.net/story/428991/38/how-the-tantabus-parses-sleep/problem-exists-between-map-and-assignment-escalation",
        "date_modified": 1659533697
      },
      {
        "id": 1618997,
        "title": "Sailor Equine Transfer Protocol",
        "words": 4043,
        "views": 2006,
        "link": "https://www.fimfiction.net/story/428991/39/how-the-tantabus-parses-sleep/sailor-equine-transfer-protocol",
        "date_modified": 1673156378
      },
      {
        "id": 1641318,
        "title": "Data Entry Automation",
        "words": 6256,
        "views": 1807,
        "link": "https://www.fimfiction.net/story/428991/40/how-the-tantabus-parses-sleep/data-entry-automation",
        "date_modified": 1678750782
      }
    ]
  }
}"#;

    #[test]
    fn deserialize_story_response() {
        from_str(RESPONSE_SAMPLE).expect("response should be deserialized into a Story");
    }

    #[test]
    fn serialize_story_response() {
        let story =
            from_str(RESPONSE_SAMPLE).expect("response should be deserialized into a Story");
        to_string(story).expect("Story should be serializable as a String");
    }

    #[test]
    fn serde_reversible() {
        let value: Value = serde_json::from_str(RESPONSE_SAMPLE).unwrap();

        let story: Story =
            from_str(RESPONSE_SAMPLE).expect("response should be deserialized into a Story");
        let serialized_value = serde_json::to_value(Response::Story(story)).unwrap();

        assert_eq!(value, serialized_value);
    }

    #[test]
    fn deserialize_invalid_id_error_response() {
        let response = r#"{
    "error": "Invalid story id"
}"#;

        match from_str(response).unwrap_err() {
            StoryError::InvalidId => {}
            err => panic!("expected invalid ID error, got: {err:?}"),
        }
    }

    #[test]
    fn deserialize_error_response() {
        let response = r#"{
    "error": "Some other error message"
}"#;

        match from_str(response).unwrap_err() {
            StoryError::Api(msg) => assert_eq!(msg, "Some other error message"),
            err => panic!("expected API error, got: {err:?}"),
        }
    }

    #[test]
    fn deserialization_error() {
        let response = "{}";
        match from_str(response).unwrap_err() {
            StoryError::Json(_) => {}
            err => panic!("expected a deserialization error, got: {err:?}"),
        }
    }
}
