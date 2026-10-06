// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Types for structured lyrics (OpenSubsonic extension).

use serde::{Deserialize, Serialize};

/// A singer/voice attribution in lyrics (OpenSubsonic, songLyrics v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Agent {
    /// Unique identifier of the agent, referenced by `agent_id` in cue lines.
    pub id: String,
    /// Role of the agent (e.g. "main", "voice", "bg").
    pub role: String,
    /// Display name of the agent (e.g. the singer).
    pub name: String,
}

/// An individual word/syllable timestamp within a cue line (OpenSubsonic, songLyrics v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Cue {
    /// Start time of the cue in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    /// End time of the cue in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    /// Text of the word/syllable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Byte offset in the line text where this cue starts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_start: Option<i32>,
    /// Byte offset in the line text where this cue ends.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub byte_end: Option<i32>,
}

/// A word/syllable-level timing line (OpenSubsonic, songLyrics v2).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CueLine {
    /// Index of the matching main line this cue line refers to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<i32>,
    /// ID of the [`Agent`] singing this line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub agent_id: Option<String>,
    /// Start time of the line in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<f64>,
    /// End time of the line in milliseconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<f64>,
    /// Full text of the line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Word/syllable-level cues within the line.
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
