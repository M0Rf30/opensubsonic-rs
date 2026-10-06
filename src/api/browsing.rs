// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Browsing API endpoints.

use crate::Client;
use crate::data::{
    AlbumInfo, AlbumWithSongsId3, ArtistInfo, ArtistInfo2, ArtistWithAlbumsId3, ArtistsId3, Child,
    Directory, Genre, Indexes, MusicFolder, VideoInfo,
};
use crate::error::Error;
use crate::params::Params;
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

/// Extract `outer.inner` as a list, defaulting to empty when absent.
pub(super) fn nested_list<T: DeserializeOwned>(
    map: &mut Map<String, Value>,
    outer: &str,
    inner: &str,
) -> Result<Vec<T>, Error> {
    let v = map
        .remove(outer)
        .and_then(|mut o| o.get_mut(inner).map(Value::take))
        .unwrap_or_else(|| Value::Array(vec![]));
    serde_json::from_value(v).map_err(|e| Error::Parse(format!("Invalid '{outer}.{inner}': {e}")))
}

impl Client {
    /// Get all configured music folders.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getmusicfolders/>
    pub async fn get_music_folders(&self) -> Result<Vec<MusicFolder>, Error> {
        let mut data = self.get_map("getMusicFolders", &Params::new()).await?;
        nested_list(&mut data, "musicFolders", "musicFolder")
    }

    /// Get an indexed structure of all artists (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getindexes/>
    pub async fn get_indexes(
        &self,
        music_folder_id: Option<&str>,
        if_modified_since: Option<i64>,
    ) -> Result<Indexes, Error> {
        let params = Params::new()
            .with_opt("musicFolderId", music_folder_id)
            .with_opt("ifModifiedSince", if_modified_since);
        self.get_field("getIndexes", &params, "indexes").await
    }

    /// Get a directory listing (folder-based browsing).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getmusicdirectory/>
    pub async fn get_music_directory(&self, id: &str) -> Result<Directory, Error> {
        self.get_field(
            "getMusicDirectory",
            &Params::new().with("id", id),
            "directory",
        )
        .await
    }

    /// Get all genres.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getgenres/>
    pub async fn get_genres(&self) -> Result<Vec<Genre>, Error> {
        let mut data = self.get_map("getGenres", &Params::new()).await?;
        nested_list(&mut data, "genres", "genre")
    }

    /// Get all artists (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getartists/>
    pub async fn get_artists(&self, music_folder_id: Option<&str>) -> Result<ArtistsId3, Error> {
        let params = Params::new().with_opt("musicFolderId", music_folder_id);
        self.get_field("getArtists", &params, "artists").await
    }

    /// Get details for an artist, including a list of albums (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getartist/>
    pub async fn get_artist(&self, id: &str) -> Result<ArtistWithAlbumsId3, Error> {
        self.get_field("getArtist", &Params::new().with("id", id), "artist")
            .await
    }

    /// Get details for an album, including a list of songs (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbum/>
    pub async fn get_album(&self, id: &str) -> Result<AlbumWithSongsId3, Error> {
        self.get_field("getAlbum", &Params::new().with("id", id), "album")
            .await
    }

    /// Get details for a song.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsong/>
    pub async fn get_song(&self, id: &str) -> Result<Child, Error> {
        self.get_field("getSong", &Params::new().with("id", id), "song")
            .await
    }

    /// Get additional info for a video: captions, audio tracks, conversions.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getvideoinfo/>
    pub async fn get_video_info(&self, id: &str) -> Result<VideoInfo, Error> {
        self.get_field("getVideoInfo", &Params::new().with("id", id), "videoInfo")
            .await
    }

    /// Get album info (external metadata).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbuminfo/>
    pub async fn get_album_info(&self, id: &str) -> Result<AlbumInfo, Error> {
        self.get_field("getAlbumInfo", &Params::new().with("id", id), "albumInfo")
            .await
    }

    /// Get album info (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbuminfo2/>
    pub async fn get_album_info2(&self, id: &str) -> Result<AlbumInfo, Error> {
        self.get_field("getAlbumInfo2", &Params::new().with("id", id), "albumInfo")
            .await
    }

    /// Get all video files. Returns an empty list if the server has no videos.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getvideos/>
    pub async fn get_videos(&self) -> Result<Vec<Child>, Error> {
        let mut data = self.get_map("getVideos", &Params::new()).await?;
        nested_list(&mut data, "videos", "video")
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
        let params = Params::new()
            .with("id", id)
            .with_opt("count", count)
            .with_opt("includeNotPresent", include_not_present);
        self.get_field("getArtistInfo", &params, "artistInfo").await
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
        let params = Params::new()
            .with("id", id)
            .with_opt("count", count)
            .with_opt("includeNotPresent", include_not_present);
        self.get_field("getArtistInfo2", &params, "artistInfo2")
            .await
    }

    /// Get similar songs (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsimilarsongs/>
    pub async fn get_similar_songs(
        &self,
        id: &str,
        count: Option<i32>,
    ) -> Result<Vec<Child>, Error> {
        let params = Params::new().with("id", id).with_opt("count", count);
        let mut data = self.get_map("getSimilarSongs", &params).await?;
        nested_list(&mut data, "similarSongs", "song")
    }

    /// Get similar songs (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsimilarsongs2/>
    pub async fn get_similar_songs2(
        &self,
        id: &str,
        count: Option<i32>,
    ) -> Result<Vec<Child>, Error> {
        let params = Params::new().with("id", id).with_opt("count", count);
        let mut data = self.get_map("getSimilarSongs2", &params).await?;
        nested_list(&mut data, "similarSongs2", "song")
    }

    /// Get top songs for an artist.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/gettopsongs/>
    pub async fn get_top_songs(
        &self,
        artist: &str,
        count: Option<i32>,
    ) -> Result<Vec<Child>, Error> {
        let params = Params::new()
            .with("artist", artist)
            .with_opt("count", count);
        let mut data = self.get_map("getTopSongs", &params).await?;
        nested_list(&mut data, "topSongs", "song")
    }
}
