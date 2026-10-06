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

/// Optional parameters for [`Client::update_playlist`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct UpdatePlaylistOptions {
    /// New playlist name (`name`).
    pub name: Option<String>,
    /// New playlist comment (`comment`).
    pub comment: Option<String>,
    /// Whether the playlist is public (`public`).
    pub public: Option<bool>,
    /// Song ids to append (`songIdToAdd`).
    pub song_ids_to_add: Vec<String>,
    /// Zero-based indexes of songs to remove (`songIndexToRemove`).
    pub song_indexes_to_remove: Vec<i32>,
}

impl UpdatePlaylistOptions {
    /// Create empty options (no changes).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the new playlist name (`name`).
    #[must_use]
    pub fn name(mut self, v: impl Into<String>) -> Self {
        self.name = Some(v.into());
        self
    }

    /// Set the new playlist comment (`comment`).
    #[must_use]
    pub fn comment(mut self, v: impl Into<String>) -> Self {
        self.comment = Some(v.into());
        self
    }

    /// Set whether the playlist is public (`public`).
    #[must_use]
    pub fn public(mut self, v: bool) -> Self {
        self.public = Some(v);
        self
    }

    /// Append one song id to add (`songIdToAdd`).
    #[must_use]
    pub fn add_song(mut self, id: impl Into<String>) -> Self {
        self.song_ids_to_add.push(id.into());
        self
    }

    /// Append several song ids to add (`songIdToAdd`).
    #[must_use]
    pub fn add_songs(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.song_ids_to_add.extend(ids.into_iter().map(Into::into));
        self
    }

    /// Add one index to remove (`songIndexToRemove`).
    #[must_use]
    pub fn remove_index(mut self, index: i32) -> Self {
        self.song_indexes_to_remove.push(index);
        self
    }

    /// Add several indexes to remove (`songIndexToRemove`).
    #[must_use]
    pub fn remove_indexes(mut self, indexes: impl IntoIterator<Item = i32>) -> Self {
        self.song_indexes_to_remove.extend(indexes);
        self
    }
}

/// Build the query for `updatePlaylist`.
fn update_playlist_params(playlist_id: &str, options: &UpdatePlaylistOptions) -> Params {
    Params::new()
        .with("playlistId", playlist_id)
        .with_opt("name", options.name.as_deref())
        .with_opt("comment", options.comment.as_deref())
        .with_opt("public", options.public)
        .with_all("songIdToAdd", &options.song_ids_to_add)
        .with_all("songIndexToRemove", &options.song_indexes_to_remove)
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
    /// See [`UpdatePlaylistOptions`] for the optional parameters.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/updateplaylist/>
    pub async fn update_playlist(
        &self,
        playlist_id: &str,
        options: &UpdatePlaylistOptions,
    ) -> Result<(), Error> {
        self.get_unit(
            "updatePlaylist",
            &update_playlist_params(playlist_id, options),
        )
        .await
    }

    /// Delete a playlist.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteplaylist/>
    pub async fn delete_playlist(&self, id: &str) -> Result<(), Error> {
        self.get_unit("deletePlaylist", &Params::new().with("id", id))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn update_playlist_params_full() {
        let o = UpdatePlaylistOptions::new()
            .name("n")
            .comment("c")
            .public(true)
            .add_song("a")
            .add_songs(["b", "c"])
            .remove_index(1)
            .remove_indexes([4, 5]);
        let p = update_playlist_params("p1", &o);
        assert_eq!(
            p.iter().collect::<Vec<_>>(),
            vec![
                ("playlistId", "p1"),
                ("name", "n"),
                ("comment", "c"),
                ("public", "true"),
                ("songIdToAdd", "a"),
                ("songIdToAdd", "b"),
                ("songIdToAdd", "c"),
                ("songIndexToRemove", "1"),
                ("songIndexToRemove", "4"),
                ("songIndexToRemove", "5"),
            ]
        );
    }
}
