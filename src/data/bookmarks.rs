//! Types for the Bookmarks API section.

use serde::{Deserialize, Serialize};

use super::common::Child;

/// A bookmark on a media file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Bookmark {
    /// Position in milliseconds.
    pub position: i64,
    /// Username.
    pub username: String,
    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Date created (ISO 8601).
    pub created: String,
    /// Date last changed (ISO 8601).
    pub changed: String,
    /// The bookmarked media item.
    pub entry: Child,
}

/// The play queue (current playlist with position).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayQueue {
    /// ID of the currently playing track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current: Option<String>,
    /// Position in milliseconds of the currently playing track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// Username.
    pub username: String,
    /// Date modified (ISO 8601).
    pub changed: String,
    /// Client app name that last modified this queue.
    pub changed_by: String,
    /// Songs in the queue.
    #[serde(default)]
    pub entry: Vec<Child>,
}

/// The play queue by index (OpenSubsonic extension).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlayQueueByIndex {
    /// Index of the currently playing track.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_index: Option<i32>,
    /// Position in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
    /// Username.
    pub username: String,
    /// Date modified (ISO 8601).
    pub changed: String,
    /// Client app name.
    pub changed_by: String,
    /// Songs in the queue.
    #[serde(default)]
    pub entry: Vec<Child>,
}
