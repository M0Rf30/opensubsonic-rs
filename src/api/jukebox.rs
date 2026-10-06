// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Jukebox API endpoint.

use crate::Client;
use crate::data::{JukeboxPlaylist, JukeboxStatus};
use crate::error::Error;
use crate::params::Params;

/// Jukebox control action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JukeboxAction {
    /// Return the full jukebox playlist (`get`).
    Get,
    /// Return the current jukebox status (`status`).
    Status,
    /// Replace the playlist with the given song ids (`set`).
    Set,
    /// Start playback (`start`).
    Start,
    /// Stop playback (`stop`).
    Stop,
    /// Skip to the track at a given index/offset (`skip`).
    Skip,
    /// Append the given song ids to the playlist (`add`).
    Add,
    /// Clear the playlist (`clear`).
    Clear,
    /// Remove the track at a given index (`remove`).
    Remove,
    /// Shuffle the playlist (`shuffle`).
    Shuffle,
    /// Set the playback gain (`setGain`).
    SetGain,
}

impl JukeboxAction {
    fn as_str(self) -> &'static str {
        match self {
            Self::Get => "get",
            Self::Status => "status",
            Self::Set => "set",
            Self::Start => "start",
            Self::Stop => "stop",
            Self::Skip => "skip",
            Self::Add => "add",
            Self::Clear => "clear",
            Self::Remove => "remove",
            Self::Shuffle => "shuffle",
            Self::SetGain => "setGain",
        }
    }
}

/// Jukebox control result — either a status or a full playlist.
#[derive(Debug, Clone, PartialEq)]
pub enum JukeboxResult {
    /// Returned for most actions.
    Status(JukeboxStatus),
    /// Returned for the `get` action.
    Playlist(JukeboxPlaylist),
}

impl Client {
    /// Control the jukebox (server-side playback).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/jukeboxcontrol/>
    pub async fn jukebox_control(
        &self,
        action: JukeboxAction,
        index: Option<i32>,
        offset: Option<i32>,
        ids: &[&str],
        gain: Option<f64>,
    ) -> Result<JukeboxResult, Error> {
        let params = Params::new()
            .with("action", action.as_str())
            .with_opt("index", index)
            .with_opt("offset", offset)
            .with_all("id", ids)
            .with_opt("gain", gain);
        // The "get" action returns jukeboxPlaylist; all others return jukeboxStatus.
        if action == JukeboxAction::Get {
            Ok(JukeboxResult::Playlist(
                self.get_field("jukeboxControl", &params, "jukeboxPlaylist")
                    .await?,
            ))
        } else {
            Ok(JukeboxResult::Status(
                self.get_field("jukeboxControl", &params, "jukeboxStatus")
                    .await?,
            ))
        }
    }
}
