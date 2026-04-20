//! Types for structured lyrics (OpenSubsonic extension).

use serde::{Deserialize, Serialize};

/// A singer/voice attribution in lyrics (OpenSubsonic, songLyrics v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    pub id: String,
    pub role: String,
    pub name: String,
}

/// An individual word/syllable timestamp within a cue line (OpenSubsonic, songLyrics v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cue {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_start: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_end: Option<i32>,
}

/// A word/syllable-level timing line (OpenSubsonic, songLyrics v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CueLine {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub cue: Vec<Cue>,
}

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
    /// Lyrics kind: "main", "translation", or "pronunciation" (OpenSubsonic, songLyrics v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<String>,
    /// Singer/voice attributions (OpenSubsonic, songLyrics v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agents: Option<Vec<Agent>>,
    /// Word/syllable-level timing lines (OpenSubsonic, songLyrics v2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cue_line: Option<Vec<CueLine>>,
}

/// A list of structured lyrics entries for a song.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LyricsList {
    /// Structured lyrics entries (may have multiple per language).
    #[serde(default)]
    pub structured_lyrics: Vec<StructuredLyrics>,
}
