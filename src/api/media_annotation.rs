// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Media Annotation API endpoints.

use crate::Client;
use crate::error::Error;
use crate::params::Params;

impl Client {
    /// Star songs, albums, or artists.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/star/>
    pub async fn star(
        &self,
        ids: &[&str],
        album_ids: &[&str],
        artist_ids: &[&str],
    ) -> Result<(), Error> {
        let params = Params::new()
            .with_all("id", ids.iter().copied())
            .with_all("albumId", album_ids.iter().copied())
            .with_all("artistId", artist_ids.iter().copied());
        self.get_unit("star", &params).await
    }

    /// Unstar songs, albums, or artists.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/unstar/>
    pub async fn unstar(
        &self,
        ids: &[&str],
        album_ids: &[&str],
        artist_ids: &[&str],
    ) -> Result<(), Error> {
        let params = Params::new()
            .with_all("id", ids.iter().copied())
            .with_all("albumId", album_ids.iter().copied())
            .with_all("artistId", artist_ids.iter().copied());
        self.get_unit("unstar", &params).await
    }

    /// Set the rating of a song, album, or artist.
    ///
    /// A rating of 0 removes the rating.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/setrating/>
    pub async fn set_rating(&self, id: &str, rating: i32) -> Result<(), Error> {
        let params = Params::new().with("id", id).with("rating", rating);
        self.get_unit("setRating", &params).await
    }

    /// Register a song as played (scrobble).
    ///
    /// If `submission` is `false`, this is a "now playing" notification rather than a scrobble.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/scrobble/>
    pub async fn scrobble(
        &self,
        id: &str,
        time: Option<i64>,
        submission: Option<bool>,
    ) -> Result<(), Error> {
        let params = Params::new()
            .with("id", id)
            .with_opt("time", time)
            .with_opt("submission", submission);
        self.get_unit("scrobble", &params).await
    }

    /// Report playback state to the server (OpenSubsonic, playbackReport extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/reportplayback/>
    pub async fn report_playback(
        &self,
        media_id: &str,
        media_type: &str,
        position_ms: i64,
        state: &str,
        playback_rate: Option<f64>,
        ignore_scrobble: Option<bool>,
    ) -> Result<(), Error> {
        let params = Params::new()
            .with("mediaId", media_id)
            .with("mediaType", media_type)
            .with("positionMs", position_ms)
            .with("state", state)
            .with_opt("playbackRate", playback_rate)
            .with_opt("ignoreScrobble", ignore_scrobble);
        self.get_unit("reportPlayback", &params).await
    }
}
