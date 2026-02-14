//! Lists API endpoints.

use crate::Client;
use crate::data::{AlbumId3, ArtistId3, Child, NowPlayingEntry};
use crate::error::Error;

/// Album list ordering type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlbumListType {
    Random,
    Newest,
    Highest,
    Frequent,
    Recent,
    AlphabeticalByName,
    AlphabeticalByArtist,
    Starred,
    ByYear,
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
        let mut params = vec![("type", list_type.as_str().to_string())];
        if let Some(s) = size {
            params.push(("size", s.to_string()));
        }
        if let Some(o) = offset {
            params.push(("offset", o.to_string()));
        }
        if let Some(y) = from_year {
            params.push(("fromYear", y.to_string()));
        }
        if let Some(y) = to_year {
            params.push(("toYear", y.to_string()));
        }
        if let Some(g) = genre {
            params.push(("genre", g.to_string()));
        }
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getAlbumList", &param_refs).await?;
        let albums = data
            .get("albumList")
            .and_then(|v| v.get("album"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(albums)?)
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
        let mut params = vec![("type", list_type.as_str().to_string())];
        if let Some(s) = size {
            params.push(("size", s.to_string()));
        }
        if let Some(o) = offset {
            params.push(("offset", o.to_string()));
        }
        if let Some(y) = from_year {
            params.push(("fromYear", y.to_string()));
        }
        if let Some(y) = to_year {
            params.push(("toYear", y.to_string()));
        }
        if let Some(g) = genre {
            params.push(("genre", g.to_string()));
        }
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getAlbumList2", &param_refs).await?;
        let albums = data
            .get("albumList2")
            .and_then(|v| v.get("album"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(albums)?)
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
        let mut params = Vec::new();
        if let Some(s) = size {
            params.push(("size", s.to_string()));
        }
        if let Some(g) = genre {
            params.push(("genre", g.to_string()));
        }
        if let Some(y) = from_year {
            params.push(("fromYear", y.to_string()));
        }
        if let Some(y) = to_year {
            params.push(("toYear", y.to_string()));
        }
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getRandomSongs", &param_refs).await?;
        let songs = data
            .get("randomSongs")
            .and_then(|v| v.get("song"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(songs)?)
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
        let mut params = vec![("genre", genre.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        if let Some(o) = offset {
            params.push(("offset", o.to_string()));
        }
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getSongsByGenre", &param_refs).await?;
        let songs = data
            .get("songsByGenre")
            .and_then(|v| v.get("song"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(songs)?)
    }

    /// Get what is currently being played by all users.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getnowplaying/>
    pub async fn get_now_playing(&self) -> Result<Vec<NowPlayingEntry>, Error> {
        let data = self.get_response("getNowPlaying", &[]).await?;
        let entries = data
            .get("nowPlaying")
            .and_then(|v| v.get("entry"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(entries)?)
    }

    /// Get starred songs, albums and artists (folder-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getstarred/>
    pub async fn get_starred(
        &self,
        music_folder_id: Option<&str>,
    ) -> Result<StarredContent, Error> {
        let mut params = Vec::new();
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id));
        }
        let data = self.get_response("getStarred", &params).await?;
        let starred = data
            .get("starred")
            .ok_or_else(|| Error::Parse("Missing 'starred' in response".into()))?;
        Ok(serde_json::from_value(starred.clone())?)
    }

    /// Get starred songs, albums and artists (ID3-based).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getstarred2/>
    pub async fn get_starred2(
        &self,
        music_folder_id: Option<&str>,
    ) -> Result<Starred2Content, Error> {
        let mut params = Vec::new();
        if let Some(id) = music_folder_id {
            params.push(("musicFolderId", id));
        }
        let data = self.get_response("getStarred2", &params).await?;
        let starred = data
            .get("starred2")
            .ok_or_else(|| Error::Parse("Missing 'starred2' in response".into()))?;
        Ok(serde_json::from_value(starred.clone())?)
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
