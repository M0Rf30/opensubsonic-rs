//! Media Retrieval API endpoints.

use bytes::Bytes;
use url::Url;

use crate::data::{Lyrics, LyricsList};
use crate::error::Error;
use crate::Client;

impl Client {
    /// Stream a song or video. Returns the raw bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/stream/>
    pub async fn stream(
        &self,
        id: &str,
        max_bit_rate: Option<i32>,
        format: Option<&str>,
        time_offset: Option<i32>,
        estimated_content_length: Option<bool>,
    ) -> Result<Bytes, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(br) = max_bit_rate {
            params.push(("maxBitRate", br.to_string()));
        }
        if let Some(f) = format {
            params.push(("format", f.to_string()));
        }
        if let Some(t) = time_offset {
            params.push(("timeOffset", t.to_string()));
        }
        if let Some(e) = estimated_content_length {
            params.push(("estimateContentLength", e.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_bytes("stream", &param_refs).await
    }

    /// Build a streaming URL for a song without making an HTTP request.
    ///
    /// Useful for passing to external audio players or download managers.
    pub fn stream_url(
        &self,
        id: &str,
        max_bit_rate: Option<i32>,
        format: Option<&str>,
    ) -> Result<Url, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(br) = max_bit_rate {
            params.push(("maxBitRate", br.to_string()));
        }
        if let Some(f) = format {
            params.push(("format", f.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.build_url("stream", &param_refs)
    }

    /// Download a song or video. Returns raw bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/download/>
    pub async fn download(&self, id: &str) -> Result<Bytes, Error> {
        self.get_bytes("download", &[("id", id)]).await
    }

    /// Get an HLS playlist URL for a video or song.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/hls/>
    pub fn hls_url(
        &self,
        id: &str,
        bit_rate: Option<&str>,
        audio_track: Option<&str>,
    ) -> Result<Url, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(br) = bit_rate {
            params.push(("bitRate", br.to_string()));
        }
        if let Some(at) = audio_track {
            params.push(("audioTrack", at.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.build_url("hls.m3u8", &param_refs)
    }

    /// Get captions (subtitles) for a video. Returns raw bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getcaptions/>
    pub async fn get_captions(
        &self,
        id: &str,
        format: Option<&str>,
    ) -> Result<Bytes, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(f) = format {
            params.push(("format", f.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_bytes("getCaptions", &param_refs).await
    }

    /// Get cover art for an album or artist. Returns raw image bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getcoverart/>
    pub async fn get_cover_art(
        &self,
        id: &str,
        size: Option<i32>,
    ) -> Result<Bytes, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(s) = size {
            params.push(("size", s.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_bytes("getCoverArt", &param_refs).await
    }

    /// Build a cover art URL without making an HTTP request.
    pub fn cover_art_url(&self, id: &str, size: Option<i32>) -> Result<Url, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(s) = size {
            params.push(("size", s.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.build_url("getCoverArt", &param_refs)
    }

    /// Get lyrics for a song (legacy, unstructured).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getlyrics/>
    pub async fn get_lyrics(
        &self,
        artist: Option<&str>,
        title: Option<&str>,
    ) -> Result<Lyrics, Error> {
        let mut params = Vec::new();
        if let Some(a) = artist {
            params.push(("artist", a));
        }
        if let Some(t) = title {
            params.push(("title", t));
        }
        let data = self.get_response("getLyrics", &params).await?;
        let lyrics = data
            .get("lyrics")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));
        Ok(serde_json::from_value(lyrics)?)
    }

    /// Get structured lyrics for a song by ID (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getlyricsbysongid/>
    pub async fn get_lyrics_by_song_id(&self, id: &str) -> Result<LyricsList, Error> {
        let data = self
            .get_response("getLyricsBySongId", &[("id", id)])
            .await?;
        let lyrics = data
            .get("lyricsList")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Object(serde_json::Map::new()));
        Ok(serde_json::from_value(lyrics)?)
    }

    /// Get a user's avatar image. Returns raw image bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getavatar/>
    pub async fn get_avatar(&self, username: &str) -> Result<Bytes, Error> {
        self.get_bytes("getAvatar", &[("username", username)]).await
    }
}
