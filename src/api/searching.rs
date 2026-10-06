// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Searching API endpoints.

use crate::Client;
use crate::data::{SearchResult, SearchResult2, SearchResult3};
use crate::error::Error;
use crate::params::Params;

/// Optional parameters for [`Client::search`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct SearchOptions {
    /// Artist to search for (`artist`).
    pub artist: Option<String>,
    /// Album to search for (`album`).
    pub album: Option<String>,
    /// Title to search for (`title`).
    pub title: Option<String>,
    /// Search across all fields (`any`).
    pub any: Option<String>,
    /// Maximum number of results (`count`).
    pub count: Option<i32>,
    /// Result offset (`offset`).
    pub offset: Option<i32>,
    /// Only return results newer than this timestamp (ms since epoch) (`newerThan`).
    pub newer_than: Option<i64>,
}

impl SearchOptions {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `artist`: artist to search for.
    #[must_use]
    pub fn artist(mut self, v: impl Into<String>) -> Self {
        self.artist = Some(v.into());
        self
    }

    /// Set `album`: album to search for.
    #[must_use]
    pub fn album(mut self, v: impl Into<String>) -> Self {
        self.album = Some(v.into());
        self
    }

    /// Set `title`: title to search for.
    #[must_use]
    pub fn title(mut self, v: impl Into<String>) -> Self {
        self.title = Some(v.into());
        self
    }

    /// Set `any`: search across all fields.
    #[must_use]
    pub fn any(mut self, v: impl Into<String>) -> Self {
        self.any = Some(v.into());
        self
    }

    /// Set `count`: maximum number of results.
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

    /// Set `newerThan`: only return results newer than this timestamp (ms since epoch).
    #[must_use]
    pub fn newer_than(mut self, v: i64) -> Self {
        self.newer_than = Some(v);
        self
    }
}

/// Optional parameters for [`Client::search2`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Search2Options {
    /// Maximum number of artists (`artistCount`).
    pub artist_count: Option<i32>,
    /// Artist result offset (`artistOffset`).
    pub artist_offset: Option<i32>,
    /// Maximum number of albums (`albumCount`).
    pub album_count: Option<i32>,
    /// Album result offset (`albumOffset`).
    pub album_offset: Option<i32>,
    /// Maximum number of songs (`songCount`).
    pub song_count: Option<i32>,
    /// Song result offset (`songOffset`).
    pub song_offset: Option<i32>,
    /// Restrict to a music folder (`musicFolderId`).
    pub music_folder_id: Option<String>,
}

impl Search2Options {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `artistCount`: maximum number of artists.
    #[must_use]
    pub fn artist_count(mut self, v: i32) -> Self {
        self.artist_count = Some(v);
        self
    }

    /// Set `artistOffset`: artist result offset.
    #[must_use]
    pub fn artist_offset(mut self, v: i32) -> Self {
        self.artist_offset = Some(v);
        self
    }

    /// Set `albumCount`: maximum number of albums.
    #[must_use]
    pub fn album_count(mut self, v: i32) -> Self {
        self.album_count = Some(v);
        self
    }

    /// Set `albumOffset`: album result offset.
    #[must_use]
    pub fn album_offset(mut self, v: i32) -> Self {
        self.album_offset = Some(v);
        self
    }

    /// Set `songCount`: maximum number of songs.
    #[must_use]
    pub fn song_count(mut self, v: i32) -> Self {
        self.song_count = Some(v);
        self
    }

    /// Set `songOffset`: song result offset.
    #[must_use]
    pub fn song_offset(mut self, v: i32) -> Self {
        self.song_offset = Some(v);
        self
    }

    /// Set `musicFolderId`: restrict to a music folder.
    #[must_use]
    pub fn music_folder_id(mut self, v: impl Into<String>) -> Self {
        self.music_folder_id = Some(v.into());
        self
    }
}

/// Optional parameters for [`Client::search3`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct Search3Options {
    /// Maximum number of artists (`artistCount`).
    pub artist_count: Option<i32>,
    /// Artist result offset (`artistOffset`).
    pub artist_offset: Option<i32>,
    /// Maximum number of albums (`albumCount`).
    pub album_count: Option<i32>,
    /// Album result offset (`albumOffset`).
    pub album_offset: Option<i32>,
    /// Maximum number of songs (`songCount`).
    pub song_count: Option<i32>,
    /// Song result offset (`songOffset`).
    pub song_offset: Option<i32>,
    /// Restrict to a music folder (`musicFolderId`).
    pub music_folder_id: Option<String>,
}

impl Search3Options {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `artistCount`: maximum number of artists.
    #[must_use]
    pub fn artist_count(mut self, v: i32) -> Self {
        self.artist_count = Some(v);
        self
    }

    /// Set `artistOffset`: artist result offset.
    #[must_use]
    pub fn artist_offset(mut self, v: i32) -> Self {
        self.artist_offset = Some(v);
        self
    }

    /// Set `albumCount`: maximum number of albums.
    #[must_use]
    pub fn album_count(mut self, v: i32) -> Self {
        self.album_count = Some(v);
        self
    }

    /// Set `albumOffset`: album result offset.
    #[must_use]
    pub fn album_offset(mut self, v: i32) -> Self {
        self.album_offset = Some(v);
        self
    }

    /// Set `songCount`: maximum number of songs.
    #[must_use]
    pub fn song_count(mut self, v: i32) -> Self {
        self.song_count = Some(v);
        self
    }

    /// Set `songOffset`: song result offset.
    #[must_use]
    pub fn song_offset(mut self, v: i32) -> Self {
        self.song_offset = Some(v);
        self
    }

    /// Set `musicFolderId`: restrict to a music folder.
    #[must_use]
    pub fn music_folder_id(mut self, v: impl Into<String>) -> Self {
        self.music_folder_id = Some(v.into());
        self
    }
}

fn search_params(o: &SearchOptions) -> Params {
    Params::new()
        .with_opt("artist", o.artist.as_deref())
        .with_opt("album", o.album.as_deref())
        .with_opt("title", o.title.as_deref())
        .with_opt("any", o.any.as_deref())
        .with_opt("count", o.count)
        .with_opt("offset", o.offset)
        .with_opt("newerThan", o.newer_than)
}

fn search2_params(query: &str, o: &Search2Options) -> Params {
    Params::new()
        .with("query", query)
        .with_opt("artistCount", o.artist_count)
        .with_opt("artistOffset", o.artist_offset)
        .with_opt("albumCount", o.album_count)
        .with_opt("albumOffset", o.album_offset)
        .with_opt("songCount", o.song_count)
        .with_opt("songOffset", o.song_offset)
        .with_opt("musicFolderId", o.music_folder_id.as_deref())
}

fn search3_params(query: &str, o: &Search3Options) -> Params {
    Params::new()
        .with("query", query)
        .with_opt("artistCount", o.artist_count)
        .with_opt("artistOffset", o.artist_offset)
        .with_opt("albumCount", o.album_count)
        .with_opt("albumOffset", o.album_offset)
        .with_opt("songCount", o.song_count)
        .with_opt("songOffset", o.song_offset)
        .with_opt("musicFolderId", o.music_folder_id.as_deref())
}

impl Client {
    /// Search (legacy, pre-1.4.0).
    ///
    /// Optional parameters are given via [`SearchOptions`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/search/>
    pub async fn search(&self, options: &SearchOptions) -> Result<SearchResult, Error> {
        self.get_field_or_default("search", &search_params(options), "searchResult")
            .await
    }

    /// Search (folder-based, search2).
    ///
    /// Optional parameters are given via [`Search2Options`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/search2/>
    pub async fn search2(
        &self,
        query: &str,
        options: &Search2Options,
    ) -> Result<SearchResult2, Error> {
        self.get_field("search2", &search2_params(query, options), "searchResult2")
            .await
    }

    /// Search (ID3-based, search3).
    ///
    /// Optional parameters are given via [`Search3Options`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/search3/>
    pub async fn search3(
        &self,
        query: &str,
        options: &Search3Options,
    ) -> Result<SearchResult3, Error> {
        self.get_field("search3", &search3_params(query, options), "searchResult3")
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn collect(p: &Params) -> Vec<(&str, &str)> {
        p.iter().collect()
    }

    #[test]
    fn search_params_full() {
        let o = SearchOptions::new()
            .artist("x0")
            .album("x1")
            .title("x2")
            .any("x3")
            .count(5)
            .offset(6)
            .newer_than(7);
        let p = search_params(&o);
        assert_eq!(
            collect(&p),
            vec![
                ("artist", "x0"),
                ("album", "x1"),
                ("title", "x2"),
                ("any", "x3"),
                ("count", "5"),
                ("offset", "6"),
                ("newerThan", "7")
            ]
        );
    }

    #[test]
    fn search_params_empty() {
        let p = search_params(&SearchOptions::new());
        assert_eq!(collect(&p), vec![]);
    }

    #[test]
    fn search2_params_full() {
        let o = Search2Options::new()
            .artist_count(1)
            .artist_offset(2)
            .album_count(3)
            .album_offset(4)
            .song_count(5)
            .song_offset(6)
            .music_folder_id("x6");
        let p = search2_params("q", &o);
        assert_eq!(
            collect(&p),
            vec![
                ("query", "q"),
                ("artistCount", "1"),
                ("artistOffset", "2"),
                ("albumCount", "3"),
                ("albumOffset", "4"),
                ("songCount", "5"),
                ("songOffset", "6"),
                ("musicFolderId", "x6")
            ]
        );
    }

    #[test]
    fn search2_params_empty() {
        let p = search2_params("q", &Search2Options::new());
        assert_eq!(collect(&p), vec![("query", "q")]);
    }

    #[test]
    fn search3_params_full() {
        let o = Search3Options::new()
            .artist_count(1)
            .artist_offset(2)
            .album_count(3)
            .album_offset(4)
            .song_count(5)
            .song_offset(6)
            .music_folder_id("x6");
        let p = search3_params("q", &o);
        assert_eq!(
            collect(&p),
            vec![
                ("query", "q"),
                ("artistCount", "1"),
                ("artistOffset", "2"),
                ("albumCount", "3"),
                ("albumOffset", "4"),
                ("songCount", "5"),
                ("songOffset", "6"),
                ("musicFolderId", "x6")
            ]
        );
    }

    #[test]
    fn search3_params_empty() {
        let p = search3_params("q", &Search3Options::new());
        assert_eq!(collect(&p), vec![("query", "q")]);
    }
}
