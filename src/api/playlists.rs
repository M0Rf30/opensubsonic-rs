// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Playlists API endpoints.

use crate::Client;
use crate::data::{Playlist, PlaylistWithSongs};
use crate::error::Error;
use crate::params::Params;
use serde::Deserialize;

/// Wrapper for the nested `playlists.playlist` response shape.
#[derive(Deserialize, Default)]
struct PlaylistsWrapper {
    #[serde(default)]
    playlist: Vec<Playlist>,
}

impl Client {
    /// Get all playlists.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplaylists/>
    pub async fn get_playlists(&self, username: Option<&str>) -> Result<Vec<Playlist>, Error> {
        let params = Params::new().with_opt("username", username);
        let wrapper: Option<PlaylistsWrapper> = self
            .get_field_or_default("getPlaylists", &params, "playlists")
            .await?;
        Ok(wrapper.map(|w| w.playlist).unwrap_or_default())
    }

    /// Get a playlist with its songs.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplaylist/>
    pub async fn get_playlist(&self, id: &str) -> Result<PlaylistWithSongs, Error> {
        self.get_field("getPlaylist", &Params::new().with("id", id), "playlist")
            .await
    }

    /// Create or update a playlist.
    ///
    /// If `playlist_id` is provided, the existing playlist is updated.
    /// Otherwise, a new playlist is created with the given `name`.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createplaylist/>
    pub async fn create_playlist(
        &self,
        playlist_id: Option<&str>,
        name: Option<&str>,
        song_ids: &[&str],
    ) -> Result<PlaylistWithSongs, Error> {
        let params = Params::new()
            .with_opt("playlistId", playlist_id)
            .with_opt("name", name)
            .with_all("songId", song_ids);
        self.get_field("createPlaylist", &params, "playlist").await
    }

    /// Update a playlist (name, comment, public status, add/remove songs).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/updateplaylist/>
    pub async fn update_playlist(
        &self,
        playlist_id: &str,
        name: Option<&str>,
        comment: Option<&str>,
        public: Option<bool>,
        song_ids_to_add: &[&str],
        song_indexes_to_remove: &[i32],
    ) -> Result<(), Error> {
        let params = Params::new()
            .with("playlistId", playlist_id)
            .with_opt("name", name)
            .with_opt("comment", comment)
            .with_opt("public", public)
            .with_all("songIdToAdd", song_ids_to_add)
            .with_all("songIndexToRemove", song_indexes_to_remove);
        self.get_unit("updatePlaylist", &params).await
    }

    /// Delete a playlist.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteplaylist/>
    pub async fn delete_playlist(&self, id: &str) -> Result<(), Error> {
        self.get_unit("deletePlaylist", &Params::new().with("id", id))
            .await
    }
}
