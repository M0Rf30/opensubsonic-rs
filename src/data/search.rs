//! Types for the Searching API section.

use serde::{Deserialize, Serialize};

use super::common::{AlbumId3, Artist, ArtistId3, Child};

/// Legacy search result (search endpoint).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult {
    /// Matching entries.
    #[serde(default, rename = "match")]
    pub matches: Vec<Child>,
    /// Offset used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<i64>,
    /// Total hits.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_hits: Option<i64>,
}

/// Search result from `search2` (folder-based).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult2 {
    /// Matching artists.
    #[serde(default)]
    pub artist: Vec<Artist>,
    /// Matching albums (as Child entries).
    #[serde(default)]
    pub album: Vec<Child>,
    /// Matching songs.
    #[serde(default)]
    pub song: Vec<Child>,
}

/// Search result from `search3` (ID3-based).
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SearchResult3 {
    /// Matching artists (ID3).
    #[serde(default)]
    pub artist: Vec<ArtistId3>,
    /// Matching albums (ID3).
    #[serde(default)]
    pub album: Vec<AlbumId3>,
    /// Matching songs.
    #[serde(default)]
    pub song: Vec<Child>,
}
