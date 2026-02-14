//! Playlists API endpoints.

use crate::Client;
use crate::data::{Playlist, PlaylistWithSongs};
use crate::error::Error;

impl Client {
    /// Get all playlists.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplaylists/>
    pub async fn get_playlists(&self, username: Option<&str>) -> Result<Vec<Playlist>, Error> {
        let mut params = Vec::new();
        if let Some(u) = username {
            params.push(("username", u));
        }
        let data = self.get_response("getPlaylists", &params).await?;
        let playlists = data
            .get("playlists")
            .and_then(|v| v.get("playlist"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(playlists)?)
    }

    /// Get a playlist with its songs.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplaylist/>
    pub async fn get_playlist(&self, id: &str) -> Result<PlaylistWithSongs, Error> {
        let data = self.get_response("getPlaylist", &[("id", id)]).await?;
        let playlist = data
            .get("playlist")
            .ok_or_else(|| Error::Parse("Missing 'playlist' in response".into()))?;
        Ok(serde_json::from_value(playlist.clone())?)
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
        let mut params = Vec::new();
        if let Some(id) = playlist_id {
            params.push(("playlistId", id.to_string()));
        }
        if let Some(n) = name {
            params.push(("name", n.to_string()));
        }
        for song_id in song_ids {
            params.push(("songId", song_id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("createPlaylist", &param_refs).await?;
        let playlist = data
            .get("playlist")
            .ok_or_else(|| Error::Parse("Missing 'playlist' in response".into()))?;
        Ok(serde_json::from_value(playlist.clone())?)
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
        let mut params = vec![("playlistId", playlist_id.to_string())];
        if let Some(n) = name {
            params.push(("name", n.to_string()));
        }
        if let Some(c) = comment {
            params.push(("comment", c.to_string()));
        }
        if let Some(p) = public {
            params.push(("public", p.to_string()));
        }
        for id in song_ids_to_add {
            params.push(("songIdToAdd", id.to_string()));
        }
        for idx in song_indexes_to_remove {
            params.push(("songIndexToRemove", idx.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("updatePlaylist", &param_refs).await?;
        Ok(())
    }

    /// Delete a playlist.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteplaylist/>
    pub async fn delete_playlist(&self, id: &str) -> Result<(), Error> {
        self.get_response("deletePlaylist", &[("id", id)]).await?;
        Ok(())
    }
}
