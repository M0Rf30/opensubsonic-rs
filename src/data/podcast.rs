//! Types for the Podcast API section.

use serde::{Deserialize, Serialize};

use super::common::Child;

/// Podcast episode status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PodcastStatus {
    /// New episode.
    New,
    /// Currently downloading.
    Downloading,
    /// Download completed.
    Completed,
    /// Download error.
    Error,
    /// Episode deleted.
    Deleted,
    /// Episode skipped.
    Skipped,
}

/// A podcast channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastChannel {
    /// Channel ID.
    pub id: String,
    /// Podcast feed URL.
    pub url: String,
    /// Channel title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    /// Channel description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// Original image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_image_url: Option<String>,
    /// Channel status.
    pub status: PodcastStatus,
    /// Error message (if status is error).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_message: Option<String>,
    /// Episodes in this channel.
    #[serde(default)]
    pub episode: Vec<PodcastEpisode>,
}

/// A podcast episode (extends [`Child`]).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PodcastEpisode {
    /// All media fields from [`Child`].
    #[serde(flatten)]
    pub child: Child,
    /// Stream ID for streaming this episode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_id: Option<String>,
    /// Channel ID this episode belongs to.
    pub channel_id: String,
    /// Episode description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Episode status.
    pub status: PodcastStatus,
    /// Publish date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub publish_date: Option<String>,
}
