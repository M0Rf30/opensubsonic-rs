//! Types for structured lyrics (OpenSubsonic extension).

use serde::{Deserialize, Serialize};

/// A single line of lyrics.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Line {
    /// The text of this line.
    pub value: String,
    /// Start time in milliseconds (present only for synced lyrics).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
}

/// Structured lyrics for a song.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StructuredLyrics {
    /// Language code (ideally ISO 639; "und" or "xxx" for unknown).
    pub lang: String,
    /// Whether the lyrics are time-synced.
    pub synced: bool,
    /// The lyrics lines.
    pub line: Vec<Line>,
    /// Display artist name (may be localized).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_artist: Option<String>,
    /// Display title (may be localized).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_title: Option<String>,
    /// Time offset in milliseconds to apply to all lines.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<f64>,
}

/// A list of structured lyrics entries for a song.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsList {
    /// Structured lyrics entries (may have multiple per language).
    #[serde(default)]
    pub structured_lyrics: Vec<StructuredLyrics>,
}
