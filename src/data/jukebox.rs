//! Types for the Jukebox API section.

use serde::{Deserialize, Serialize};

use super::common::Child;

/// Jukebox playback status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JukeboxStatus {
    /// Index of the currently playing song in the playlist.
    pub current_index: i32,
    /// Whether the jukebox is currently playing.
    pub playing: bool,
    /// Volume level (0.0–1.0, encoded as integer by some servers).
    pub volume: f64,
    /// Current position in the track (seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<i64>,
}

/// Jukebox playlist (status + entries).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JukeboxPlaylist {
    /// Jukebox status fields.
    #[serde(flatten)]
    pub status: JukeboxStatus,
    /// Songs currently enqueued.
    #[serde(default)]
    pub entry: Vec<Child>,
}
