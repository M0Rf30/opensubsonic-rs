// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integration tests for chunked streaming endpoints.

mod common;

use common::{failed_envelope, token_client};
use futures_util::StreamExt;
use opensubsonic::{Error, SubsonicErrorCode};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn stream_chunked_yields_served_body() {
    let server = MockServer::start().await;
    let body: Vec<u8> = (0..200_000u32).map(|i| (i % 251) as u8).collect();
    Mock::given(method("GET"))
        .and(path("/rest/stream"))
        .and(query_param("id", "song-1"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "audio/mpeg")
                .set_body_bytes(body.clone()),
        )
        .expect(1)
        .mount(&server)
        .await;

    let client = token_client(&server);
    let mut stream = client
        .stream_chunked("song-1", &opensubsonic::StreamOptions::default())
        .await
        .unwrap();
    let mut collected = Vec::new();
    while let Some(chunk) = stream.next().await {
        collected.extend_from_slice(&chunk.unwrap());
    }
    assert_eq!(collected, body);
}

#[tokio::test]
async fn stream_chunked_json_error_is_api_error() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/stream"))
        .respond_with(ResponseTemplate::new(200).set_body_json(failed_envelope(
            70,
            "Song not found",
            None,
        )))
        .mount(&server)
        .await;

    let client = token_client(&server);
    let err = match client
        .stream_chunked("nope", &opensubsonic::StreamOptions::default())
        .await
    {
        Ok(_) => panic!("expected error"),
        Err(e) => e,
    };
    assert!(matches!(err, Error::Api(_)), "got {err:?}");
    assert_eq!(err.api_error_code(), Some(SubsonicErrorCode::NotFound));
}
