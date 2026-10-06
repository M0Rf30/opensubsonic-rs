// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Sonic Similarity API endpoints (OpenSubsonic extension).

use crate::Client;
use crate::data::SonicMatch;
use crate::error::Error;
use crate::params::Params;

/// Private wrapper for `{ "sonicMatch": [...] }` containers.
#[derive(Default, serde::Deserialize)]
#[serde(default, rename_all = "camelCase")]
struct Wrapper {
    sonic_match: Vec<SonicMatch>,
}

impl Client {
    /// Get tracks sonically similar to the given song (OpenSubsonic, sonicSimilarity extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsonicsimilartracks/>
    pub async fn get_sonic_similar_tracks(
        &self,
        id: &str,
        count: Option<i32>,
    ) -> Result<Vec<SonicMatch>, Error> {
        let params = Params::new().with("id", id).with_opt("count", count);
        let w: Wrapper = self
            .get_field_or_default("getSonicSimilarTracks", &params, "sonicSimilarTracks")
            .await?;
        Ok(w.sonic_match)
    }

    /// Find a path of sonically similar tracks between two songs
    /// (OpenSubsonic, sonicSimilarity extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/findsonicpath/>
    pub async fn find_sonic_path(
        &self,
        start_song_id: &str,
        end_song_id: &str,
        count: Option<i32>,
    ) -> Result<Vec<SonicMatch>, Error> {
        let params = Params::new()
            .with("startSongId", start_song_id)
            .with("endSongId", end_song_id)
            .with_opt("count", count);
        let w: Wrapper = self
            .get_field_or_default("findSonicPath", &params, "sonicPath")
            .await?;
        Ok(w.sonic_match)
    }
}
