//! Searching API endpoints.

use crate::data::{SearchResult, SearchResult2, SearchResult3};
use crate::error::Error;
use crate::Client;

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
        let mut params = Vec::new();
        if let Some(v) = artist {
            params.push(("artist", v.to_string()));
        }
        if let Some(v) = album {
            params.push(("album", v.to_string()));
        }
        if let Some(v) = title {
            params.push(("title", v.to_string()));
        }
        if let Some(v) = any {
            params.push(("any", v.to_string()));
        }
        if let Some(v) = count {
            params.push(("count", v.to_string()));
        }
        if let Some(v) = offset {
            params.push(("offset", v.to_string()));
        }
        if let Some(v) = newer_than {
            params.push(("newerThan", v.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("search", &param_refs).await?;
        let result = data
            .get("searchResult")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));
        Ok(serde_json::from_value(result)?)
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
        let mut params = vec![("query", query.to_string())];
        if let Some(v) = artist_count {
            params.push(("artistCount", v.to_string()));
        }
        if let Some(v) = artist_offset {
            params.push(("artistOffset", v.to_string()));
        }
        if let Some(v) = album_count {
            params.push(("albumCount", v.to_string()));
        }
        if let Some(v) = album_offset {
            params.push(("albumOffset", v.to_string()));
        }
        if let Some(v) = song_count {
            params.push(("songCount", v.to_string()));
        }
        if let Some(v) = song_offset {
            params.push(("songOffset", v.to_string()));
        }
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("search2", &param_refs).await?;
        let result = data
            .get("searchResult2")
            .ok_or_else(|| Error::Parse("Missing 'searchResult2' in response".into()))?;
        Ok(serde_json::from_value(result.clone())?)
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
        let mut params = vec![("query", query.to_string())];
        if let Some(v) = artist_count {
            params.push(("artistCount", v.to_string()));
        }
        if let Some(v) = artist_offset {
            params.push(("artistOffset", v.to_string()));
        }
        if let Some(v) = album_count {
            params.push(("albumCount", v.to_string()));
        }
        if let Some(v) = album_offset {
            params.push(("albumOffset", v.to_string()));
        }
        if let Some(v) = song_count {
            params.push(("songCount", v.to_string()));
        }
        if let Some(v) = song_offset {
            params.push(("songOffset", v.to_string()));
        }
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("search3", &param_refs).await?;
        let result = data
            .get("searchResult3")
            .ok_or_else(|| Error::Parse("Missing 'searchResult3' in response".into()))?;
        Ok(serde_json::from_value(result.clone())?)
    }
}
