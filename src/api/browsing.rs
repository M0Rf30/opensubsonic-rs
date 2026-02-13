//! Browsing API endpoints.

use crate::data::{
    AlbumInfo, AlbumWithSongsId3, ArtistInfo, ArtistInfo2, ArtistWithAlbumsId3,
    ArtistsId3, Child, Directory, Genre, Indexes, MusicFolder,
};
use crate::error::Error;
use crate::Client;

impl Client {
    /// Get all configured music folders.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getmusicfolders/>
    pub async fn get_music_folders(&self) -> Result<Vec<MusicFolder>, Error> {
        let data = self.get_response("getMusicFolders", &[]).await?;
        let folders = data
            .get("musicFolders")
            .and_then(|v| v.get("musicFolder"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(folders)?)
    }

    /// Get an indexed structure of all artists (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getindexes/>
    pub async fn get_indexes(
        &self,
        music_folder_id: Option<&str>,
        if_modified_since: Option<i64>,
    ) -> Result<Indexes, Error> {
        let mut params = Vec::new();
        let folder_str;
        if let Some(id) = music_folder_id {
            folder_str = id.to_string();
            params.push(("musicFolderId", folder_str.as_str()));
        }
        let since_str;
        if let Some(since) = if_modified_since {
            since_str = since.to_string();
            params.push(("ifModifiedSince", since_str.as_str()));
        }
        let data = self.get_response("getIndexes", &params).await?;
        let indexes = data
            .get("indexes")
            .ok_or_else(|| Error::Parse("Missing 'indexes' in response".into()))?;
        Ok(serde_json::from_value(indexes.clone())?)
    }

    /// Get a directory listing (folder-based browsing).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getmusicdirectory/>
    pub async fn get_music_directory(&self, id: &str) -> Result<Directory, Error> {
        let data = self
            .get_response("getMusicDirectory", &[("id", id)])
            .await?;
        let dir = data
            .get("directory")
            .ok_or_else(|| Error::Parse("Missing 'directory' in response".into()))?;
        Ok(serde_json::from_value(dir.clone())?)
    }

    /// Get all genres.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getgenres/>
    pub async fn get_genres(&self) -> Result<Vec<Genre>, Error> {
        let data = self.get_response("getGenres", &[]).await?;
        let genres = data
            .get("genres")
            .and_then(|v| v.get("genre"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(genres)?)
    }

    /// Get all artists (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getartists/>
    pub async fn get_artists(
        &self,
        music_folder_id: Option<&str>,
    ) -> Result<ArtistsId3, Error> {
        let mut params = Vec::new();
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id));
        }
        let data = self.get_response("getArtists", &params).await?;
        let artists = data
            .get("artists")
            .ok_or_else(|| Error::Parse("Missing 'artists' in response".into()))?;
        Ok(serde_json::from_value(artists.clone())?)
    }

    /// Get details for an artist, including a list of albums (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getartist/>
    pub async fn get_artist(&self, id: &str) -> Result<ArtistWithAlbumsId3, Error> {
        let data = self.get_response("getArtist", &[("id", id)]).await?;
        let artist = data
            .get("artist")
            .ok_or_else(|| Error::Parse("Missing 'artist' in response".into()))?;
        Ok(serde_json::from_value(artist.clone())?)
    }

    /// Get details for an album, including a list of songs (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbum/>
    pub async fn get_album(&self, id: &str) -> Result<AlbumWithSongsId3, Error> {
        let data = self.get_response("getAlbum", &[("id", id)]).await?;
        let album = data
            .get("album")
            .ok_or_else(|| Error::Parse("Missing 'album' in response".into()))?;
        Ok(serde_json::from_value(album.clone())?)
    }

    /// Get details for a song.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsong/>
    pub async fn get_song(&self, id: &str) -> Result<Child, Error> {
        let data = self.get_response("getSong", &[("id", id)]).await?;
        let song = data
            .get("song")
            .ok_or_else(|| Error::Parse("Missing 'song' in response".into()))?;
        Ok(serde_json::from_value(song.clone())?)
    }

    /// Get all video files. Returns an empty list if the server has no videos.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getvideos/>
    pub async fn get_videos(&self) -> Result<Vec<Child>, Error> {
        let data = self.get_response("getVideos", &[]).await?;
        let videos = data
            .get("videos")
            .and_then(|v| v.get("video"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(videos)?)
    }

    /// Get artist info (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getartistinfo/>
    pub async fn get_artist_info(
        &self,
        id: &str,
        count: Option<i32>,
        include_not_present: Option<bool>,
    ) -> Result<ArtistInfo, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        if let Some(b) = include_not_present {
            params.push(("includeNotPresent", b.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getArtistInfo", &param_refs).await?;
        let info = data
            .get("artistInfo")
            .ok_or_else(|| Error::Parse("Missing 'artistInfo' in response".into()))?;
        Ok(serde_json::from_value(info.clone())?)
    }

    /// Get artist info (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getartistinfo2/>
    pub async fn get_artist_info2(
        &self,
        id: &str,
        count: Option<i32>,
        include_not_present: Option<bool>,
    ) -> Result<ArtistInfo2, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        if let Some(b) = include_not_present {
            params.push(("includeNotPresent", b.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getArtistInfo2", &param_refs).await?;
        let info = data
            .get("artistInfo2")
            .ok_or_else(|| Error::Parse("Missing 'artistInfo2' in response".into()))?;
        Ok(serde_json::from_value(info.clone())?)
    }

    /// Get album info (external metadata).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbuminfo/>
    pub async fn get_album_info(&self, id: &str) -> Result<AlbumInfo, Error> {
        let data = self.get_response("getAlbumInfo", &[("id", id)]).await?;
        let info = data
            .get("albumInfo")
            .ok_or_else(|| Error::Parse("Missing 'albumInfo' in response".into()))?;
        Ok(serde_json::from_value(info.clone())?)
    }

    /// Get album info (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbuminfo2/>
    pub async fn get_album_info2(&self, id: &str) -> Result<AlbumInfo, Error> {
        let data = self.get_response("getAlbumInfo2", &[("id", id)]).await?;
        let info = data
            .get("albumInfo")
            .ok_or_else(|| Error::Parse("Missing 'albumInfo' in response".into()))?;
        Ok(serde_json::from_value(info.clone())?)
    }

    /// Get similar songs (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsimilarsongs/>
    pub async fn get_similar_songs(
        &self,
        id: &str,
        count: Option<i32>,
    ) -> Result<Vec<Child>, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getSimilarSongs", &param_refs).await?;
        let songs = data
            .get("similarSongs")
            .and_then(|v| v.get("song"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(songs)?)
    }

    /// Get similar songs (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsimilarsongs2/>
    pub async fn get_similar_songs2(
        &self,
        id: &str,
        count: Option<i32>,
    ) -> Result<Vec<Child>, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getSimilarSongs2", &param_refs).await?;
        let songs = data
            .get("similarSongs2")
            .and_then(|v| v.get("song"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(songs)?)
    }

    /// Get top songs for an artist.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/gettopsongs/>
    pub async fn get_top_songs(
        &self,
        artist: &str,
        count: Option<i32>,
    ) -> Result<Vec<Child>, Error> {
        let mut params = vec![("artist", artist.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getTopSongs", &param_refs).await?;
        let songs = data
            .get("topSongs")
            .and_then(|v| v.get("song"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(songs)?)
    }
}
