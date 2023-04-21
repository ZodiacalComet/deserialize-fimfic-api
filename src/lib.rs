//! Deserialization for the JSON responses of the [Fimfiction][fimfiction] story API
//! (`https://www.fimfiction.net/api/story.php?story={ID}`).
//!
//! ```no_run
//! # use fimfiction_api::Story;
//! # let response = String::new();
//! let story: Story = fimfiction_api::from_str(&response)?;
//! # Ok::<(), serde_json::Error>(())
//! ```
//!
//! # The `chrono` feature
//!
//! Changes all date fields to use `DateTime<Utc>` instead of an `i64`.
//!
//! [fimfiction]: https://www.fimfiction.net/
#[cfg(feature = "chrono")]
use chrono::{offset::Utc, serde::ts_seconds::deserialize as deserialize_date, DateTime};
use serde::Deserialize;

mod rating;
mod status;
mod vote;

pub use rating::StoryRating;
pub use status::StoryStatus;
use vote::deserialize_vote;

/// Container struct of the author response given by the Fimfiction story API.
#[derive(Deserialize, Debug)]
pub struct Author {
    /// Author's ID.
    pub id: u32,
    /// Username of the author.
    pub name: String,
}

/// Container struct for all chapter response data given by the Fimfiction story API.
#[derive(Deserialize, Debug)]
pub struct Chapter {
    /// Chapter's ID.
    pub id: u32,
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
    #[serde(deserialize_with = "deserialize_date")]
    /// Last chapter update datetime.
    pub date_modified: DateTime<Utc>,
}

/// Container struct for all relevant story response data given by the Fimfiction story API.
#[derive(Deserialize, Debug)]
pub struct Story {
    /// Unique story ID.
    pub id: u32,
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
    #[serde(deserialize_with = "deserialize_date")]
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
    /// Rating given to the story.
    pub content_rating: StoryRating,
    /// The amount of likes the story has, if not disabled.
    #[serde(deserialize_with = "deserialize_vote")]
    pub likes: Option<u32>,
    /// The amount of dislikes the story has, if not disabled.
    #[serde(deserialize_with = "deserialize_vote")]
    pub dislikes: Option<u32>,
    /// Chapters of the story.
    pub chapters: Vec<Chapter>,
}

#[derive(Deserialize)]
struct FimfictionResponse {
    /// Story data.
    pub story: Story,
}

/// Deserialize an instance of [`Story`] from a string of JSON text.
pub fn from_str(input: &str) -> Result<Story, serde_json::Error> {
    serde_json::from_str::<FimfictionResponse>(input).map(|response| response.story)
}
