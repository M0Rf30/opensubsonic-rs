// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Media Retrieval API endpoints.

use bytes::Bytes;
use url::Url;

use crate::Client;
use crate::data::{Lyrics, LyricsList};
use crate::error::Error;
use crate::params::Params;

/// Optional parameters for [`Client::stream`], [`Client::stream_chunked`] and
/// [`Client::stream_url`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct StreamOptions {
    /// Maximum bit rate in kbps (`maxBitRate`).
    pub max_bit_rate: Option<i32>,
    /// Preferred target format (`format`).
    pub format: Option<String>,
    /// Start offset in seconds, for video (`timeOffset`).
    pub time_offset: Option<i32>,
    /// Whether to estimate the content length (`estimateContentLength`).
    pub estimate_content_length: Option<bool>,
}

impl StreamOptions {
    /// Create empty options (all server defaults).
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `maxBitRate` (kbps; 0 means no limit).
    #[must_use]
    pub fn max_bit_rate(mut self, v: i32) -> Self {
        self.max_bit_rate = Some(v);
        self
    }

    /// Set `format` (e.g. `mp3`, `opus`, or `raw`).
    #[must_use]
    pub fn format(mut self, v: impl Into<String>) -> Self {
        self.format = Some(v.into());
        self
    }

    /// Set `timeOffset` (seconds).
    #[must_use]
    pub fn time_offset(mut self, v: i32) -> Self {
        self.time_offset = Some(v);
        self
    }

    /// Set `estimateContentLength` (server estimates `Content-Length` for transcoded streams).
    #[must_use]
    pub fn estimate_content_length(mut self, v: bool) -> Self {
        self.estimate_content_length = Some(v);
        self
    }
}

/// Shared query parameters for `stream`, `stream_chunked` and `stream_url`.
fn stream_params(id: &str, options: &StreamOptions) -> Params {
    Params::new()
        .with("id", id)
        .with_opt("maxBitRate", options.max_bit_rate)
        .with_opt("format", options.format.as_deref())
        .with_opt("timeOffset", options.time_offset)
        .with_opt("estimateContentLength", options.estimate_content_length)
}

impl Client {
    /// Stream a song or video. Returns the raw bytes.
    ///
    /// See [`StreamOptions`] for the optional parameters.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/stream/>
    pub async fn stream(&self, id: &str, options: &StreamOptions) -> Result<Bytes, Error> {
        self.get_binary("stream", &stream_params(id, options)).await
    }

    /// Stream a song or video as a chunked byte stream.
    ///
    /// Unlike [`stream`](Self::stream), this does not buffer the whole file in memory:
    /// chunks are yielded as they arrive from the server.
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    ///
    /// # async fn run(client: opensubsonic::Client) -> Result<(), opensubsonic::Error> {
    /// let opts = opensubsonic::StreamOptions::new();
    /// let mut stream = client.stream_chunked("song-id", &opts).await?;
    /// while let Some(chunk) = stream.next().await {
    ///     let chunk = chunk?;
    ///     println!("got {} bytes", chunk.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/stream/>
    pub async fn stream_chunked(
        &self,
        id: &str,
        options: &StreamOptions,
    ) -> Result<crate::ByteStream, Error> {
        self.get_binary_stream("stream", &stream_params(id, options))
            .await
    }

    /// Build a streaming URL for a song without making an HTTP request.
    ///
    /// Useful for passing to external audio players or download managers.
    /// See [`StreamOptions`] for the supported query parameters.
    pub fn stream_url(&self, id: &str, options: &StreamOptions) -> Result<Url, Error> {
        self.endpoint_url("stream", &stream_params(id, options))
    }

    /// Download a song or video. Returns raw bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/download/>
    pub async fn download(&self, id: &str) -> Result<Bytes, Error> {
        self.get_binary("download", &Params::new().with("id", id))
            .await
    }

    /// Download a song or video as a chunked byte stream.
    ///
    /// Unlike [`download`](Self::download), this does not buffer the whole file in memory:
    /// chunks are yielded as they arrive from the server.
    ///
    /// ```no_run
    /// use futures_util::StreamExt;
    ///
    /// # async fn run(client: opensubsonic::Client) -> Result<(), opensubsonic::Error> {
    /// let mut stream = client.download_chunked("song-id").await?;
    /// while let Some(chunk) = stream.next().await {
    ///     let chunk = chunk?;
    ///     println!("got {} bytes", chunk.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/download/>
    pub async fn download_chunked(&self, id: &str) -> Result<crate::ByteStream, Error> {
        self.get_binary_stream("download", &Params::new().with("id", id))
            .await
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
        let params = Params::new()
            .with("id", id)
            .with_opt("bitRate", bit_rate)
            .with_opt("audioTrack", audio_track);
        self.endpoint_url("hls.m3u8", &params)
    }

    /// Get captions (subtitles) for a video. Returns raw bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getcaptions/>
    pub async fn get_captions(&self, id: &str, format: Option<&str>) -> Result<Bytes, Error> {
        let params = Params::new().with("id", id).with_opt("format", format);
        self.get_binary("getCaptions", &params).await
    }

    /// Get cover art for an album or artist. Returns raw image bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getcoverart/>
    pub async fn get_cover_art(&self, id: &str, size: Option<i32>) -> Result<Bytes, Error> {
        let params = Params::new().with("id", id).with_opt("size", size);
        self.get_binary("getCoverArt", &params).await
    }

    /// Build a cover art URL without making an HTTP request.
    pub fn cover_art_url(&self, id: &str, size: Option<i32>) -> Result<Url, Error> {
        let params = Params::new().with("id", id).with_opt("size", size);
        self.endpoint_url("getCoverArt", &params)
    }

    /// Get lyrics for a song (legacy, unstructured).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getlyrics/>
    pub async fn get_lyrics(
        &self,
        artist: Option<&str>,
        title: Option<&str>,
    ) -> Result<Lyrics, Error> {
        let params = Params::new()
            .with_opt("artist", artist)
            .with_opt("title", title);
        self.get_field_or_default("getLyrics", &params, "lyrics")
            .await
    }

    /// Get structured lyrics for a song by ID (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getlyricsbysongid/>
    pub async fn get_lyrics_by_song_id(
        &self,
        id: &str,
        enhanced: Option<bool>,
    ) -> Result<LyricsList, Error> {
        let params = Params::new().with("id", id).with_opt("enhanced", enhanced);
        self.get_field_or_default("getLyricsBySongId", &params, "lyricsList")
            .await
    }

    /// Get a user's avatar image. Returns raw image bytes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getavatar/>
    pub async fn get_avatar(&self, username: &str) -> Result<Bytes, Error> {
        self.get_binary("getAvatar", &Params::new().with("username", username))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stream_params_full() {
        let o = StreamOptions::new()
            .max_bit_rate(128)
            .format("mp3")
            .time_offset(30)
            .estimate_content_length(true);
        let p = stream_params("s1", &o);
        assert_eq!(
            p.iter().collect::<Vec<_>>(),
            vec![
                ("id", "s1"),
                ("maxBitRate", "128"),
                ("format", "mp3"),
                ("timeOffset", "30"),
                ("estimateContentLength", "true"),
            ]
        );
    }

    #[test]
    fn stream_params_default() {
        let p = stream_params("s1", &StreamOptions::new());
        assert_eq!(p.iter().collect::<Vec<_>>(), vec![("id", "s1")]);
    }
}
