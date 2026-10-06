// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Media Retrieval API endpoints.

use bytes::Bytes;
use url::Url;

use crate::Client;
use crate::data::{Lyrics, LyricsList};
use crate::error::Error;
use crate::params::Params;

/// Shared query parameters for `stream` and `stream_chunked`.
fn stream_params(
    id: &str,
    max_bit_rate: Option<i32>,
    format: Option<&str>,
    time_offset: Option<i32>,
    estimated_content_length: Option<bool>,
) -> Params {
    Params::new()
        .with("id", id)
        .with_opt("maxBitRate", max_bit_rate)
        .with_opt("format", format)
        .with_opt("timeOffset", time_offset)
        .with_opt("estimateContentLength", estimated_content_length)
}

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
        self.get_binary(
            "stream",
            &stream_params(
                id,
                max_bit_rate,
                format,
                time_offset,
                estimated_content_length,
            ),
        )
        .await
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
    /// let mut stream = client.stream_chunked("song-id", None, None, None, None).await?;
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
        max_bit_rate: Option<i32>,
        format: Option<&str>,
        time_offset: Option<i32>,
        estimated_content_length: Option<bool>,
    ) -> Result<crate::ByteStream, Error> {
        self.get_binary_stream(
            "stream",
            &stream_params(
                id,
                max_bit_rate,
                format,
                time_offset,
                estimated_content_length,
            ),
        )
        .await
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
        let params = Params::new()
            .with("id", id)
            .with_opt("maxBitRate", max_bit_rate)
            .with_opt("format", format);
        self.endpoint_url("stream", &params)
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
