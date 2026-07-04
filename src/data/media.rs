//! Types for the Media Retrieval API section.
//!
//! Most media retrieval endpoints return binary data (bytes) rather than JSON.
//! The types here are for the few endpoints that return structured data
//! (e.g. `getLyrics`).

use serde::{Deserialize, Serialize};

// Note: `stream`, `download`, `getCoverArt`, `getAvatar`, `hls`, `getCaptions`
// all return binary data — no data types needed beyond bytes::Bytes.

/// Lyrics for a song (legacy, unstructured).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Lyrics {
    /// The lyrics text.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    /// Artist name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    /// Song title.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
}

/// Additional information about a video (captions, audio tracks, conversions).
///
/// Returned by `getVideoInfo`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoInfo {
    /// Video ID.
    pub id: String,
    /// Available caption/subtitle tracks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub captions: Vec<Captions>,
    /// Available audio tracks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub audio_track: Vec<AudioTrack>,
    /// Available pre-computed conversions.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub conversion: Vec<VideoConversion>,
}

/// A caption / subtitle track for a video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Captions {
    /// Caption track ID.
    pub id: String,
    /// Caption track name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// An audio track for a video.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AudioTrack {
    /// Audio track ID.
    pub id: String,
    /// Audio track name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Language code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language_code: Option<String>,
}

/// A pre-computed video conversion.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VideoConversion {
    /// Conversion ID.
    pub id: String,
    /// Bit rate in kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<i32>,
    /// Associated audio track ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_track_id: Option<String>,
}
