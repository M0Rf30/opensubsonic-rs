//! Jukebox API endpoint.

use crate::data::{JukeboxPlaylist, JukeboxStatus};
use crate::error::Error;
use crate::Client;

/// Jukebox control action.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JukeboxAction {
    Get,
    Status,
    Set,
    Start,
    Stop,
    Skip,
    Add,
    Clear,
    Remove,
    Shuffle,
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
        let mut params = vec![("action", action.as_str().to_string())];
        if let Some(idx) = index {
            params.push(("index", idx.to_string()));
        }
        if let Some(off) = offset {
            params.push(("offset", off.to_string()));
        }
        for id in ids {
            params.push(("id", id.to_string()));
        }
        if let Some(g) = gain {
            params.push(("gain", g.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("jukeboxControl", &param_refs).await?;

        // The "get" action returns jukeboxPlaylist; all others return jukeboxStatus.
        if action == JukeboxAction::Get {
            let playlist = data
                .get("jukeboxPlaylist")
                .ok_or_else(|| {
                    Error::Parse("Missing 'jukeboxPlaylist' in response".into())
                })?;
            Ok(JukeboxResult::Playlist(serde_json::from_value(
                playlist.clone(),
            )?))
        } else {
            let status = data
                .get("jukeboxStatus")
                .ok_or_else(|| Error::Parse("Missing 'jukeboxStatus' in response".into()))?;
            Ok(JukeboxResult::Status(serde_json::from_value(
                status.clone(),
            )?))
        }
    }
}
