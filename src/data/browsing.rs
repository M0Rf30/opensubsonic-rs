//! Types for the Browsing API section.

use serde::{Deserialize, Serialize};

use super::common::{Artist, Child};

/// A directory in the music library (folder-based browsing).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Directory {
    /// Directory ID.
    pub id: String,
    /// Parent directory ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Directory name.
    pub name: String,
    /// Date starred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<String>,
    /// User rating (1–5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<i32>,
    /// Average rating (1.0–5.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_rating: Option<f64>,
    /// Play count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i64>,
    /// Child entries in this directory.
    #[serde(default)]
    pub child: Vec<Child>,
}

/// An index entry grouping artists by first letter (folder-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Index {
    /// Index name (e.g. "A", "B", "#").
    pub name: String,
    /// Artists in this index.
    #[serde(default)]
    pub artist: Vec<Artist>,
}

/// The full indexes response (folder-based artist listing).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Indexes {
    /// Ignored articles (space-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_articles: Option<String>,
    /// Last modified timestamp (millis since epoch).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<i64>,
    /// Shortcut artists.
    #[serde(default)]
    pub shortcut: Vec<Artist>,
    /// Child entries.
    #[serde(default)]
    pub child: Vec<Child>,
    /// Index list.
    #[serde(default)]
    pub index: Vec<Index>,
}

/// Album info (external metadata).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumInfo {
    /// Album notes/biography.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
    /// MusicBrainz ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// Last.fm URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fm_url: Option<String>,
    /// Small image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    /// Medium image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_image_url: Option<String>,
    /// Large image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
}

/// Artist info with similar artists (folder-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistInfo {
    /// Artist biography.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biography: Option<String>,
    /// MusicBrainz ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// Last.fm URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fm_url: Option<String>,
    /// Small image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    /// Medium image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_image_url: Option<String>,
    /// Large image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
    /// Similar artists.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub similar_artist: Vec<Artist>,
}

/// Artist info with similar artists (ID3-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistInfo2 {
    /// Artist biography.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub biography: Option<String>,
    /// MusicBrainz ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// Last.fm URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_fm_url: Option<String>,
    /// Small image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub small_image_url: Option<String>,
    /// Medium image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub medium_image_url: Option<String>,
    /// Large image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub large_image_url: Option<String>,
    /// Similar artists (ID3).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub similar_artist: Vec<super::common::ArtistId3>,
}
