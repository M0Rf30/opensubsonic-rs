//! Types for the User Management API section.

use serde::{Deserialize, Serialize};

/// A Subsonic user.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Username.
    pub username: String,
    /// Whether scrobbling is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scrobbling_enabled: Option<bool>,
    /// Max bitrate (kbps).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_bit_rate: Option<i32>,
    /// Admin role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_role: Option<bool>,
    /// Settings role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings_role: Option<bool>,
    /// Download role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub download_role: Option<bool>,
    /// Upload role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upload_role: Option<bool>,
    /// Playlist role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub playlist_role: Option<bool>,
    /// Cover art role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art_role: Option<bool>,
    /// Comment role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment_role: Option<bool>,
    /// Podcast role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub podcast_role: Option<bool>,
    /// Stream role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_role: Option<bool>,
    /// Jukebox role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jukebox_role: Option<bool>,
    /// Share role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub share_role: Option<bool>,
    /// Video conversion role.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_conversion_role: Option<bool>,
    /// Date avatar was last changed (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub avatar_last_changed: Option<String>,
    /// Accessible music folder IDs.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub folder: Vec<i64>,
    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
