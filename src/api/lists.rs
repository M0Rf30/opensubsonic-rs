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

/// Optional parameters for [`Client::get_album_list`] and [`Client::get_album_list2`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct AlbumListOptions {
    /// Number of items to return (`size`).
    pub size: Option<i32>,
    /// Result offset (`offset`).
    pub offset: Option<i32>,
    /// First year (for `byYear`) (`fromYear`).
    pub from_year: Option<i32>,
    /// Last year (for `byYear`) (`toYear`).
    pub to_year: Option<i32>,
    /// Genre (for `byGenre`) (`genre`).
    pub genre: Option<String>,
    /// Restrict to a music folder (`musicFolderId`).
    pub music_folder_id: Option<String>,
}

impl AlbumListOptions {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `size`: number of items to return.
    #[must_use]
    pub fn size(mut self, v: i32) -> Self {
        self.size = Some(v);
        self
    }

    /// Set `offset`: result offset.
    #[must_use]
    pub fn offset(mut self, v: i32) -> Self {
        self.offset = Some(v);
        self
    }

    /// Set `fromYear`: first year (for `byYear`).
    #[must_use]
    pub fn from_year(mut self, v: i32) -> Self {
        self.from_year = Some(v);
        self
    }

    /// Set `toYear`: last year (for `byYear`).
    #[must_use]
    pub fn to_year(mut self, v: i32) -> Self {
        self.to_year = Some(v);
        self
    }

    /// Set `genre`: genre (for `byGenre`).
    #[must_use]
    pub fn genre(mut self, v: impl Into<String>) -> Self {
        self.genre = Some(v.into());
        self
    }

    /// Set `musicFolderId`: restrict to a music folder.
    #[must_use]
    pub fn music_folder_id(mut self, v: impl Into<String>) -> Self {
        self.music_folder_id = Some(v.into());
        self
    }
}

/// Optional parameters for [`Client::get_random_songs`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct RandomSongsOptions {
    /// Number of songs to return (`size`).
    pub size: Option<i32>,
    /// Only songs of this genre (`genre`).
    pub genre: Option<String>,
    /// Earliest release year (`fromYear`).
    pub from_year: Option<i32>,
    /// Latest release year (`toYear`).
    pub to_year: Option<i32>,
    /// Restrict to a music folder (`musicFolderId`).
    pub music_folder_id: Option<String>,
}

impl RandomSongsOptions {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `size`: number of songs to return.
    #[must_use]
    pub fn size(mut self, v: i32) -> Self {
        self.size = Some(v);
        self
    }

    /// Set `genre`: only songs of this genre.
    #[must_use]
    pub fn genre(mut self, v: impl Into<String>) -> Self {
        self.genre = Some(v.into());
        self
    }

    /// Set `fromYear`: earliest release year.
    #[must_use]
    pub fn from_year(mut self, v: i32) -> Self {
        self.from_year = Some(v);
        self
    }

    /// Set `toYear`: latest release year.
    #[must_use]
    pub fn to_year(mut self, v: i32) -> Self {
        self.to_year = Some(v);
        self
    }

    /// Set `musicFolderId`: restrict to a music folder.
    #[must_use]
    pub fn music_folder_id(mut self, v: impl Into<String>) -> Self {
        self.music_folder_id = Some(v.into());
        self
    }
}

/// Optional parameters for [`Client::get_songs_by_genre`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SongsByGenreOptions {
    /// Maximum number of songs (`count`).
    pub count: Option<i32>,
    /// Result offset (`offset`).
    pub offset: Option<i32>,
    /// Restrict to a music folder (`musicFolderId`).
    pub music_folder_id: Option<String>,
}

impl SongsByGenreOptions {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `count`: maximum number of songs.
    #[must_use]
    pub fn count(mut self, v: i32) -> Self {
        self.count = Some(v);
        self
    }

    /// Set `offset`: result offset.
    #[must_use]
    pub fn offset(mut self, v: i32) -> Self {
        self.offset = Some(v);
        self
    }

    /// Set `musicFolderId`: restrict to a music folder.
    #[must_use]
    pub fn music_folder_id(mut self, v: impl Into<String>) -> Self {
        self.music_folder_id = Some(v.into());
        self
    }
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

fn album_list_params(list_type: AlbumListType, o: &AlbumListOptions) -> Params {
    Params::new()
        .with("type", list_type.as_str())
        .with_opt("size", o.size)
        .with_opt("offset", o.offset)
        .with_opt("fromYear", o.from_year)
        .with_opt("toYear", o.to_year)
        .with_opt("genre", o.genre.as_deref())
        .with_opt("musicFolderId", o.music_folder_id.as_deref())
}

fn random_songs_params(o: &RandomSongsOptions) -> Params {
    Params::new()
        .with_opt("size", o.size)
        .with_opt("genre", o.genre.as_deref())
        .with_opt("fromYear", o.from_year)
        .with_opt("toYear", o.to_year)
        .with_opt("musicFolderId", o.music_folder_id.as_deref())
}

fn songs_by_genre_params(genre: &str, o: &SongsByGenreOptions) -> Params {
    Params::new()
        .with("genre", genre)
        .with_opt("count", o.count)
        .with_opt("offset", o.offset)
        .with_opt("musicFolderId", o.music_folder_id.as_deref())
}

impl Client {
    /// Get a list of albums (folder-based).
    ///
    /// Optional parameters are given via [`AlbumListOptions`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbumlist/>
    pub async fn get_album_list(
        &self,
        list_type: AlbumListType,
        options: &AlbumListOptions,
    ) -> Result<Vec<Child>, Error> {
        let mut data = self
            .get_map("getAlbumList", &album_list_params(list_type, options))
            .await?;
        nested_list(&mut data, "albumList", "album")
    }

    /// Get a list of albums (ID3-based).
    ///
    /// Optional parameters are given via [`AlbumListOptions`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getalbumlist2/>
    pub async fn get_album_list2(
        &self,
        list_type: AlbumListType,
        options: &AlbumListOptions,
    ) -> Result<Vec<AlbumId3>, Error> {
        let mut data = self
            .get_map("getAlbumList2", &album_list_params(list_type, options))
            .await?;
        nested_list(&mut data, "albumList2", "album")
    }

    /// Get random songs.
    ///
    /// Optional parameters are given via [`RandomSongsOptions`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getrandomsongs/>
    pub async fn get_random_songs(
        &self,
        options: &RandomSongsOptions,
    ) -> Result<Vec<Child>, Error> {
        let mut data = self
            .get_map("getRandomSongs", &random_songs_params(options))
            .await?;
        nested_list(&mut data, "randomSongs", "song")
    }

    /// Get songs by genre.
    ///
    /// Optional parameters are given via [`SongsByGenreOptions`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsongsbygenre/>
    pub async fn get_songs_by_genre(
        &self,
        genre: &str,
        options: &SongsByGenreOptions,
    ) -> Result<Vec<Child>, Error> {
        let mut data = self
            .get_map("getSongsByGenre", &songs_by_genre_params(genre, options))
            .await?;
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

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(p: &Params) -> Vec<(&str, &str)> {
        p.iter().collect()
    }

    #[test]
    fn album_list_full() {
        let o = AlbumListOptions::new()
            .size(1)
            .offset(2)
            .from_year(3)
            .to_year(4)
            .genre("x4")
            .music_folder_id("x5");
        let p = album_list_params(AlbumListType::ByYear, &o);
        assert_eq!(
            collect(&p),
            vec![
                ("type", "byYear"),
                ("size", "1"),
                ("offset", "2"),
                ("fromYear", "3"),
                ("toYear", "4"),
                ("genre", "x4"),
                ("musicFolderId", "x5")
            ]
        );
    }

    #[test]
    fn random_songs_full() {
        let o = RandomSongsOptions::new()
            .size(1)
            .genre("x1")
            .from_year(3)
            .to_year(4)
            .music_folder_id("x4");
        let p = random_songs_params(&o);
        assert_eq!(
            collect(&p),
            vec![
                ("size", "1"),
                ("genre", "x1"),
                ("fromYear", "3"),
                ("toYear", "4"),
                ("musicFolderId", "x4")
            ]
        );
    }

    #[test]
    fn songs_by_genre_full() {
        let o = SongsByGenreOptions::new()
            .count(1)
            .offset(2)
            .music_folder_id("x2");
        let p = songs_by_genre_params("g", &o);
        assert_eq!(
            collect(&p),
            vec![
                ("genre", "g"),
                ("count", "1"),
                ("offset", "2"),
                ("musicFolderId", "x2")
            ]
        );
    }
}
