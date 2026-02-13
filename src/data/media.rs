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
