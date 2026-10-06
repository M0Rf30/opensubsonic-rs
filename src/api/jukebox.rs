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

/// Optional parameters for [`Client::jukebox_control`].
///
/// Which fields apply depends on the [`JukeboxAction`]: `index` and `offset` for
/// `Skip`, `index` for `Remove`, `ids` for `Set` and `Add`, `gain` for `SetGain`.
/// Fields not relevant to the action are still sent if set.
#[derive(Debug, Clone, Default, PartialEq)]
#[non_exhaustive]
pub struct JukeboxOptions {
    /// Track index (`index`); used by `Skip` and `Remove`.
    pub index: Option<i32>,
    /// Offset in seconds within the track (`offset`); used by `Skip`.
    pub offset: Option<i32>,
    /// Song ids (`id`); used by `Set` and `Add`.
    pub ids: Vec<String>,
    /// Gain between 0.0 and 1.0 (`gain`); used by `SetGain`.
    pub gain: Option<f64>,
}

impl JukeboxOptions {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `index` (used by `Skip` and `Remove`).
    #[must_use]
    pub fn index(mut self, v: i32) -> Self {
        self.index = Some(v);
        self
    }

    /// Set `offset` in seconds (used by `Skip`).
    #[must_use]
    pub fn offset(mut self, v: i32) -> Self {
        self.offset = Some(v);
        self
    }

    /// Append one song id (`id`; used by `Set` and `Add`).
    #[must_use]
    pub fn id(mut self, v: impl Into<String>) -> Self {
        self.ids.push(v.into());
        self
    }

    /// Append several song ids (`id`; used by `Set` and `Add`).
    #[must_use]
    pub fn ids(mut self, v: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ids.extend(v.into_iter().map(Into::into));
        self
    }

    /// Set `gain` (0.0 to 1.0; used by `SetGain`).
    #[must_use]
    pub fn gain(mut self, v: f64) -> Self {
        self.gain = Some(v);
        self
    }
}

fn jukebox_params(action: JukeboxAction, options: &JukeboxOptions) -> Params {
    Params::new()
        .with("action", action.as_str())
        .with_opt("index", options.index)
        .with_opt("offset", options.offset)
        .with_all("id", &options.ids)
        .with_opt("gain", options.gain)
}

impl Client {
    /// Control the jukebox (server-side playback).
    ///
    /// See [`JukeboxOptions`] for the optional parameters.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/jukeboxcontrol/>
    pub async fn jukebox_control(
        &self,
        action: JukeboxAction,
        options: &JukeboxOptions,
    ) -> Result<JukeboxResult, Error> {
        let params = jukebox_params(action, options);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jukebox_params_full() {
        let o = JukeboxOptions::new()
            .index(2)
            .offset(10)
            .id("a")
            .ids(["b"])
            .gain(0.5);
        let p = jukebox_params(JukeboxAction::Set, &o);
        assert_eq!(
            p.iter().collect::<Vec<_>>(),
            vec![
                ("action", "set"),
                ("index", "2"),
                ("offset", "10"),
                ("id", "a"),
                ("id", "b"),
                ("gain", "0.5"),
            ]
        );
    }
}
