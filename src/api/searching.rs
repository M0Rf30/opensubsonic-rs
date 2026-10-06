// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Searching API endpoints.

use crate::Client;
use crate::data::{SearchResult, SearchResult2, SearchResult3};
use crate::error::Error;
use crate::params::Params;

impl Client {
    /// Search (legacy, pre-1.4.0).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/search/>
    #[allow(clippy::too_many_arguments)]
    pub async fn search(
        &self,
        artist: Option<&str>,
        album: Option<&str>,
        title: Option<&str>,
        any: Option<&str>,
        count: Option<i32>,
        offset: Option<i32>,
        newer_than: Option<i64>,
    ) -> Result<SearchResult, Error> {
        let params = Params::new()
            .with_opt("artist", artist)
            .with_opt("album", album)
            .with_opt("title", title)
            .with_opt("any", any)
            .with_opt("count", count)
            .with_opt("offset", offset)
            .with_opt("newerThan", newer_than);
        self.get_field_or_default("search", &params, "searchResult")
            .await
    }

    /// Search (folder-based, search2).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/search2/>
    #[allow(clippy::too_many_arguments)]
    pub async fn search2(
        &self,
        query: &str,
        artist_count: Option<i32>,
        artist_offset: Option<i32>,
        album_count: Option<i32>,
        album_offset: Option<i32>,
        song_count: Option<i32>,
        song_offset: Option<i32>,
        music_folder_id: Option<&str>,
    ) -> Result<SearchResult2, Error> {
        let params = Params::new()
            .with("query", query)
            .with_opt("artistCount", artist_count)
            .with_opt("artistOffset", artist_offset)
            .with_opt("albumCount", album_count)
            .with_opt("albumOffset", album_offset)
            .with_opt("songCount", song_count)
            .with_opt("songOffset", song_offset)
            .with_opt("musicFolderId", music_folder_id);
        self.get_field("search2", &params, "searchResult2").await
    }

    /// Search (ID3-based, search3).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/search3/>
    #[allow(clippy::too_many_arguments)]
    pub async fn search3(
        &self,
        query: &str,
        artist_count: Option<i32>,
        artist_offset: Option<i32>,
        album_count: Option<i32>,
        album_offset: Option<i32>,
        song_count: Option<i32>,
        song_offset: Option<i32>,
        music_folder_id: Option<&str>,
    ) -> Result<SearchResult3, Error> {
        let params = Params::new()
            .with("query", query)
            .with_opt("artistCount", artist_count)
            .with_opt("artistOffset", artist_offset)
            .with_opt("albumCount", album_count)
            .with_opt("albumOffset", album_offset)
            .with_opt("songCount", song_count)
            .with_opt("songOffset", song_offset)
            .with_opt("musicFolderId", music_folder_id);
        self.get_field("search3", &params, "searchResult3").await
    }
}
