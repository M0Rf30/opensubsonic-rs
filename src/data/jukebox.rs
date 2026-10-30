// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

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
    /// Volume level, between 0.0 and 1.0.
    pub gain: f64,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_deserializes_from_spec_json() {
        let json = r#"{"currentIndex":2,"playing":true,"gain":0.5,"position":41}"#;
        let status: JukeboxStatus = serde_json::from_str(json).unwrap();
        assert_eq!(
            status,
            JukeboxStatus {
                current_index: 2,
                playing: true,
                gain: 0.5,
                position: Some(41),
            }
        );
    }

    #[test]
    fn playlist_deserializes_from_spec_json() {
        let json = r#"{"currentIndex":0,"playing":false,"gain":1,"entry":[]}"#;
        let playlist: JukeboxPlaylist = serde_json::from_str(json).unwrap();
        assert_eq!(playlist.status.gain, 1.0);
        assert_eq!(playlist.status.position, None);
        assert!(playlist.entry.is_empty());
    }
}
