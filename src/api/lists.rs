// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Lists API endpoints.

use crate::Client;
use crate::data::{AlbumId3, ArtistId3, Child, NowPlayingEntry};
use crate::error::Error;
use crate::params::Params;

use super::browsing::nested_list;

/// Album list ordering type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlbumListType {
    /// Random albums.
    Random,
    /// Most recently added albums.
    Newest,
    /// Highest rated albums.
    Highest,
    /// Most frequently played albums.
    Frequent,
    /// Most recently played albums.
    Recent,
    /// Albums sorted alphabetically by name.
    AlphabeticalByName,
    /// Albums sorted alphabetically by artist name.
    AlphabeticalByArtist,
    /// Starred albums.
    Starred,
    /// Albums in a given year range (`fromYear`/`toYear`).
    ByYear,
    /// Albums of a given genre (`genre`).
    ByGenre,
}

impl AlbumListType {
    fn as_str(self) -> &'static str {
        match self {
            Self::Random => "random",
            Self::Newest => "newest",
            Self::Highest => "highest",
            Self::Frequent => "frequent",
            Self::Recent => "recent",
            Self::AlphabeticalByName => "alphabeticalByName",
            Self::AlphabeticalByArtist => "alphabeticalByArtist",
            Self::Starred => "starred",
            Self::ByYear => "byYear",
            Self::ByGenre => "byGenre",
        }
    }
}

impl Client {
    /// Get a list of albums (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbumlist/>
    #[allow(clippy::too_many_arguments)]
    pub async fn get_album_list(
        &self,
        list_type: AlbumListType,
        size: Option<i32>,
        offset: Option<i32>,
        from_year: Option<i32>,
        to_year: Option<i32>,
        genre: Option<&str>,
        music_folder_id: Option<&str>,
    ) -> Result<Vec<Child>, Error> {
        let params = Params::new()
            .with("type", list_type.as_str())
            .with_opt("size", size)
            .with_opt("offset", offset)
            .with_opt("fromYear", from_year)
            .with_opt("toYear", to_year)
            .with_opt("genre", genre)
            .with_opt("musicFolderId", music_folder_id);
        let mut data = self.get_map("getAlbumList", &params).await?;
        nested_list(&mut data, "albumList", "album")
    }

    /// Get a list of albums (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbumlist2/>
    #[allow(clippy::too_many_arguments)]
    pub async fn get_album_list2(
        &self,
        list_type: AlbumListType,
        size: Option<i32>,
        offset: Option<i32>,
        from_year: Option<i32>,
        to_year: Option<i32>,
        genre: Option<&str>,
        music_folder_id: Option<&str>,
    ) -> Result<Vec<AlbumId3>, Error> {
        let params = Params::new()
            .with("type", list_type.as_str())
            .with_opt("size", size)
            .with_opt("offset", offset)
            .with_opt("fromYear", from_year)
            .with_opt("toYear", to_year)
            .with_opt("genre", genre)
            .with_opt("musicFolderId", music_folder_id);
        let mut data = self.get_map("getAlbumList2", &params).await?;
        nested_list(&mut data, "albumList2", "album")
    }

    /// Get random songs.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getrandomsongs/>
    pub async fn get_random_songs(
        &self,
        size: Option<i32>,
        genre: Option<&str>,
        from_year: Option<i32>,
        to_year: Option<i32>,
        music_folder_id: Option<&str>,
    ) -> Result<Vec<Child>, Error> {
        let params = Params::new()
            .with_opt("size", size)
            .with_opt("genre", genre)
            .with_opt("fromYear", from_year)
            .with_opt("toYear", to_year)
            .with_opt("musicFolderId", music_folder_id);
        let mut data = self.get_map("getRandomSongs", &params).await?;
        nested_list(&mut data, "randomSongs", "song")
    }

    /// Get songs by genre.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsongsbygenre/>
    pub async fn get_songs_by_genre(
        &self,
        genre: &str,
        count: Option<i32>,
        offset: Option<i32>,
        music_folder_id: Option<&str>,
    ) -> Result<Vec<Child>, Error> {
        let params = Params::new()
            .with("genre", genre)
            .with_opt("count", count)
            .with_opt("offset", offset)
            .with_opt("musicFolderId", music_folder_id);
        let mut data = self.get_map("getSongsByGenre", &params).await?;
        nested_list(&mut data, "songsByGenre", "song")
    }

    /// Get what is currently being played by all users.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getnowplaying/>
    pub async fn get_now_playing(&self) -> Result<Vec<NowPlayingEntry>, Error> {
        let mut data = self.get_map("getNowPlaying", &Params::new()).await?;
        nested_list(&mut data, "nowPlaying", "entry")
    }

    /// Get starred songs, albums and artists (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getstarred/>
    pub async fn get_starred(
        &self,
        music_folder_id: Option<&str>,
    ) -> Result<StarredContent, Error> {
        let params = Params::new().with_opt("musicFolderId", music_folder_id);
        self.get_field("getStarred", &params, "starred").await
    }

    /// Get starred songs, albums and artists (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getstarred2/>
    pub async fn get_starred2(
        &self,
        music_folder_id: Option<&str>,
    ) -> Result<Starred2Content, Error> {
        let params = Params::new().with_opt("musicFolderId", music_folder_id);
        self.get_field("getStarred2", &params, "starred2").await
    }
}

/// Starred content (folder-based).
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StarredContent {
    /// Starred artists.
    #[serde(default)]
    pub artist: Vec<crate::data::Artist>,
    /// Starred albums (as Child).
    #[serde(default)]
    pub album: Vec<Child>,
    /// Starred songs.
    #[serde(default)]
    pub song: Vec<Child>,
}

/// Starred content (ID3-based).
#[derive(Debug, Clone, PartialEq, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Starred2Content {
    /// Starred artists (ID3).
    #[serde(default)]
    pub artist: Vec<ArtistId3>,
    /// Starred albums (ID3).
    #[serde(default)]
    pub album: Vec<AlbumId3>,
    /// Starred songs.
    #[serde(default)]
    pub song: Vec<Child>,
}
