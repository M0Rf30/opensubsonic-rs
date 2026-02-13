//! Types for the Playlists API section.

use serde::{Deserialize, Serialize};

use super::common::Child;

/// A playlist (without songs).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Playlist {
    /// Playlist ID.
    pub id: String,
    /// Playlist name.
    pub name: String,
    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Owner username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Whether the playlist is public.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Number of songs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i64>,
    /// Total duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Date created (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Date last changed (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// Allowed users.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_user: Vec<String>,
    /// Whether the playlist is read-only for the current user (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    /// Date until playlist contents are valid for caching (ISO 8601, OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valid_until: Option<String>,
}

/// A playlist with its songs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaylistWithSongs {
    /// Playlist ID.
    pub id: String,
    /// Playlist name.
    pub name: String,
    /// Comment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Owner username.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    /// Whether the playlist is public.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub public: Option<bool>,
    /// Number of songs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i64>,
    /// Total duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Date created (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Date last changed (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub changed: Option<String>,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// Allowed users.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub allowed_user: Vec<String>,
    /// Whether the playlist is read-only (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readonly: Option<bool>,
    /// The songs in this playlist.
    #[serde(default)]
    pub entry: Vec<Child>,
}
