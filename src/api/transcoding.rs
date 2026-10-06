// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Transcoding API endpoints (OpenSubsonic extension).
//!
//! The flow is two steps: call [`Client::get_transcode_decision`] with the
//! client's capabilities ([`ClientInfo`]) to learn whether the media can be
//! direct-played or must be transcoded, then pass the returned
//! `transcodeParams` to one of the `get_transcode_stream*` methods.

use bytes::Bytes;
use url::Url;

use crate::ByteStream;
use crate::Client;
use crate::data::{ClientInfo, TranscodeDecision, TranscodeMediaType};
use crate::error::Error;
use crate::params::Params;

/// Shared query parameters for `getTranscodeStream` (order: mediaId, mediaType, offset, transcodeParams).
fn stream_params(
    media_id: &str,
    media_type: TranscodeMediaType,
    transcode_params: &str,
    offset: Option<i32>,
) -> Params {
    Params::new()
        .with("mediaId", media_id)
        .with("mediaType", media_type)
        .with_opt("offset", offset)
        .with("transcodeParams", transcode_params)
}

impl Client {
    /// Get a transcode decision for a media item (OpenSubsonic extension).
    ///
    /// Sends a `POST` with the `mediaId`/`mediaType` query parameters and the
    /// [`ClientInfo`] as a JSON body. Bitrates in `client_info` are in bits
    /// per second. If the decision says transcoding is needed, pass its
    /// `transcode_params` unchanged to [`Client::get_transcode_stream`] (or the
    /// URL/chunked variants).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/gettranscodedecision/>
    pub async fn get_transcode_decision(
        &self,
        media_id: &str,
        media_type: TranscodeMediaType,
        client_info: &ClientInfo,
    ) -> Result<TranscodeDecision, Error> {
        let params = Params::new()
            .with("mediaId", media_id)
            .with("mediaType", media_type);
        self.post_field(
            "getTranscodeDecision",
            &params,
            client_info,
            "transcodeDecision",
        )
        .await
    }

    /// Build the URL of a transcoded stream (OpenSubsonic extension).
    ///
    /// Does not make an HTTP request. `transcode_params` must be the opaque
    /// value from [`TranscodeDecision::transcode_params`], passed verbatim;
    /// never construct it yourself. `offset` is the start time in seconds.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/gettranscodestream/>
    pub fn get_transcode_stream_url(
        &self,
        media_id: &str,
        media_type: TranscodeMediaType,
        transcode_params: &str,
        offset: Option<i32>,
    ) -> Result<Url, Error> {
        let params = stream_params(media_id, media_type, transcode_params, offset);
        self.endpoint_url("getTranscodeStream", &params)
    }

    /// Get a transcoded stream as raw bytes (OpenSubsonic extension).
    ///
    /// See [`Client::get_transcode_stream_url`] for argument semantics.
    pub async fn get_transcode_stream(
        &self,
        media_id: &str,
        media_type: TranscodeMediaType,
        transcode_params: &str,
        offset: Option<i32>,
    ) -> Result<Bytes, Error> {
        let params = stream_params(media_id, media_type, transcode_params, offset);
        self.get_binary("getTranscodeStream", &params).await
    }

    /// Get a transcoded stream as a chunked byte stream (OpenSubsonic extension).
    ///
    /// See [`Client::get_transcode_stream_url`] for argument semantics.
    pub async fn get_transcode_stream_chunked(
        &self,
        media_id: &str,
        media_type: TranscodeMediaType,
        transcode_params: &str,
        offset: Option<i32>,
    ) -> Result<ByteStream, Error> {
        let params = stream_params(media_id, media_type, transcode_params, offset);
        self.get_binary_stream("getTranscodeStream", &params).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Auth;

    #[test]
    fn stream_url_params_in_order() {
        let c = Client::new("http://localhost:4533", Auth::token("u", "p")).unwrap();
        let url = c
            .get_transcode_stream_url("123", TranscodeMediaType::Song, "0001-0005-004", Some(30))
            .unwrap();
        let q = url.query().unwrap();
        let pos = |k: &str| q.find(k).unwrap_or_else(|| panic!("missing {k} in {q}"));
        assert!(q.contains("mediaType=song"));
        assert!(pos("mediaId=123") < pos("mediaType="));
        assert!(pos("mediaType=") < pos("offset=30"));
        assert!(pos("offset=30") < pos("transcodeParams=0001-0005-004"));
        let url = c
            .get_transcode_stream_url("1", TranscodeMediaType::Podcast, "x", None)
            .unwrap();
        assert!(!url.query().unwrap().contains("offset"));
    }
}
