// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Integration tests running the public `Client` against a mock HTTP server.

mod common;

use common::*;
use opensubsonic::{Auth, Client, Error, SubsonicErrorCode};
use serde_json::json;
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

// ── System / envelope ───────────────────────────────────────────────────────

#[tokio::test]
async fn ping_ok() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/ping"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .expect(1)
        .mount(&server)
        .await;
    token_client(&server).ping().await.unwrap();
}

#[tokio::test]
async fn server_info_from_envelope() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/ping"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .mount(&server)
        .await;
    let info = token_client(&server).server_info().await.unwrap();
    assert_eq!(info.version.as_deref(), Some("1.16.1"));
    assert_eq!(info.server_type.as_deref(), Some("navidrome"));
    assert_eq!(info.server_version.as_deref(), Some("0.54.0"));
    assert!(info.open_subsonic);
}

#[tokio::test]
async fn server_info_plain_subsonic_defaults() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/ping"))
        .respond_with(json_response(
            &json!({"subsonic-response": {"status": "ok", "version": "1.16.1"}}),
        ))
        .mount(&server)
        .await;
    let info = token_client(&server).server_info().await.unwrap();
    assert_eq!(info.version.as_deref(), Some("1.16.1"));
    assert_eq!(info.server_type, None);
    assert_eq!(info.server_version, None);
    assert!(!info.open_subsonic);
}

#[tokio::test]
async fn api_error_envelope() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/ping"))
        .respond_with(json_response(&failed_envelope(
            40,
            "Wrong username or password",
            Some("https://example.com/help"),
        )))
        .mount(&server)
        .await;
    let err = token_client(&server).ping().await.unwrap_err();
    assert_eq!(
        err.api_error_code(),
        Some(SubsonicErrorCode::WrongCredentials)
    );
    match err {
        Error::Api(api) => {
            assert_eq!(api.code, 40);
            assert_eq!(api.message, "Wrong username or password");
            assert_eq!(api.help_url.as_deref(), Some("https://example.com/help"));
            assert_eq!(api.error_code(), Some(SubsonicErrorCode::WrongCredentials));
        }
        other => panic!("expected Error::Api, got {other:?}"),
    }
}

#[tokio::test]
async fn unknown_api_error_code_has_no_known_variant() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/ping"))
        .respond_with(json_response(&failed_envelope(999, "weird", None)))
        .mount(&server)
        .await;
    let err = token_client(&server).ping().await.unwrap_err();
    assert!(matches!(err, Error::Api(_)));
    assert_eq!(err.api_error_code(), None);
}

#[tokio::test]
async fn http_500_is_http_error() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/ping"))
        .respond_with(ResponseTemplate::new(500))
        .mount(&server)
        .await;
    let err = token_client(&server).ping().await.unwrap_err();
    assert!(matches!(err, Error::Http(_)), "got {err:?}");
}

#[tokio::test]
async fn malformed_json_is_truncated_parse_error() {
    let server = MockServer::start().await;
    let body = format!("{{\"subsonic-response\": {}", "x".repeat(8192));
    Mock::given(path("/rest/ping"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;
    let err = token_client(&server).ping().await.unwrap_err();
    match err {
        Error::Parse(msg) => {
            assert!(
                msg.len() < 1024,
                "message not truncated: {} bytes",
                msg.len()
            );
            assert!(msg.ends_with('…'), "missing truncation marker: {msg}");
        }
        other => panic!("expected Error::Parse, got {other:?}"),
    }
}

// ── Request shape ───────────────────────────────────────────────────────────

#[tokio::test]
async fn token_auth_request_shape() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/ping"))
        .and(query_param("u", "alice"))
        .and(query_param("v", "1.16.1"))
        .and(query_param("c", "opensubsonic-rs"))
        .and(query_param("f", "json"))
        .and(NoQuery("apiKey"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .mount(&server)
        .await;
    token_client(&server).ping().await.unwrap();

    let requests = server.received_requests().await.unwrap();
    assert_eq!(requests.len(), 1);
    let pairs: std::collections::HashMap<_, _> = requests[0].url.query_pairs().collect();
    let token = pairs.get("t").expect("token present");
    let salt = pairs.get("s").expect("salt present");
    assert_eq!(token.len(), 32);
    assert!(!salt.is_empty());
    assert!(!pairs.contains_key("p"));
}

#[tokio::test]
async fn custom_client_name_and_version() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/ping"))
        .and(query_param("c", "my-app"))
        .and(query_param("v", "1.15.0"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .expect(1)
        .mount(&server)
        .await;
    token_client(&server)
        .with_client_name("my-app")
        .with_api_version("1.15.0")
        .ping()
        .await
        .unwrap();
}

#[tokio::test]
async fn api_key_auth_has_no_username() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/rest/ping"))
        .and(query_param("apiKey", "my-key"))
        .and(query_param("f", "json"))
        .and(NoQuery("u"))
        .and(NoQuery("t"))
        .and(NoQuery("s"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .expect(1)
        .mount(&server)
        .await;
    api_key_client(&server).ping().await.unwrap();
}

#[tokio::test]
async fn base_url_sub_path_is_preserved() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/music/rest/ping"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .expect(2)
        .mount(&server)
        .await;
    for suffix in ["/music", "/music/"] {
        let client = Client::new(
            &format!("{}{suffix}", server.uri()),
            Auth::token("alice", "secret"),
        )
        .unwrap();
        client.ping().await.unwrap();
    }
    assert_eq!(server.received_requests().await.unwrap().len(), 2);
}

// ── Typed endpoints ─────────────────────────────────────────────────────────

#[tokio::test]
async fn get_artists_typed() {
    let server = MockServer::start().await;
    let body = ok_envelope(json!({
        "artists": {
            "ignoredArticles": "The El La Los Las Le Les",
            "index": [
                {"name": "A", "artist": [
                    {"id": "100000047", "name": "A Perfect Circle", "albumCount": 3},
                    {"id": "100000048", "name": "Aphex Twin", "albumCount": 5}
                ]},
                {"name": "B", "artist": [
                    {"id": "100000049", "name": "Beck", "albumCount": 12}
                ]}
            ]
        }
    }));
    Mock::given(path("/rest/getArtists"))
        .and(query_param("musicFolderId", "7"))
        .respond_with(json_response(&body))
        .expect(1)
        .mount(&server)
        .await;
    let artists = token_client(&server).get_artists(Some("7")).await.unwrap();
    assert_eq!(
        artists.ignored_articles.as_deref(),
        Some("The El La Los Las Le Les")
    );
    assert_eq!(artists.index.len(), 2);
    assert_eq!(artists.index[0].name, "A");
    assert_eq!(artists.index[0].artist[1].name, "Aphex Twin");
    assert_eq!(artists.index[1].artist[0].id, "100000049");
}

#[tokio::test]
async fn get_album_typed() {
    let server = MockServer::start().await;
    let body = ok_envelope(json!({
        "album": {
            "id": "200000047",
            "name": "Mer de Noms",
            "artist": "A Perfect Circle",
            "artistId": "100000047",
            "songCount": 2,
            "duration": 480,
            "year": 2000,
            "song": [
                {"id": "300000001", "isDir": false, "title": "The Hollow", "track": 1,
                 "album": "Mer de Noms", "artist": "A Perfect Circle", "duration": 243},
                {"id": "300000002", "isDir": false, "title": "Magdalena", "track": 2,
                 "album": "Mer de Noms", "artist": "A Perfect Circle", "duration": 237}
            ]
        }
    }));
    Mock::given(path("/rest/getAlbum"))
        .and(query_param("id", "200000047"))
        .respond_with(json_response(&body))
        .expect(1)
        .mount(&server)
        .await;
    let album = token_client(&server).get_album("200000047").await.unwrap();
    assert_eq!(album.name, "Mer de Noms");
    assert_eq!(album.year, Some(2000));
    assert_eq!(album.song.len(), 2);
    assert_eq!(album.song[1].title, "Magdalena");
}

#[tokio::test]
async fn get_album_missing_key_is_parse_error() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/getAlbum"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .mount(&server)
        .await;
    let err = token_client(&server).get_album("x").await.unwrap_err();
    assert!(matches!(err, Error::Parse(_)), "got {err:?}");
}

#[tokio::test]
async fn search3_typed_and_params() {
    let server = MockServer::start().await;
    let body = ok_envelope(json!({
        "searchResult3": {
            "artist": [{"id": "1", "name": "Queen"}],
            "album": [{"id": "2", "name": "A Night at the Opera"}],
            "song": [{"id": "3", "isDir": false, "title": "Bohemian Rhapsody"}]
        }
    }));
    Mock::given(path("/rest/search3"))
        .and(query_param("query", "bohemian"))
        .and(query_param("artistCount", "2"))
        .and(query_param("songCount", "10"))
        .and(NoQuery("albumCount"))
        .respond_with(json_response(&body))
        .expect(1)
        .mount(&server)
        .await;
    let res = token_client(&server)
        .search3("bohemian", Some(2), None, None, None, Some(10), None, None)
        .await
        .unwrap();
    assert_eq!(res.artist[0].name, "Queen");
    assert_eq!(res.album[0].name, "A Night at the Opera");
    assert_eq!(res.song[0].title, "Bohemian Rhapsody");
}

#[tokio::test]
async fn get_playlists_typed() {
    let server = MockServer::start().await;
    let body = ok_envelope(json!({
        "playlists": {"playlist": [
            {"id": "800000011", "name": "Road trip", "songCount": 12, "duration": 2700,
             "public": true, "owner": "alice"},
            {"id": "800000012", "name": "Focus", "songCount": 3}
        ]}
    }));
    Mock::given(path("/rest/getPlaylists"))
        .respond_with(json_response(&body))
        .mount(&server)
        .await;
    let playlists = token_client(&server).get_playlists(None).await.unwrap();
    assert_eq!(playlists.len(), 2);
    assert_eq!(playlists[0].name, "Road trip");
    assert_eq!(playlists[0].song_count, Some(12));
    assert_eq!(playlists[1].id, "800000012");
}

#[tokio::test]
async fn get_playlists_empty_when_inner_key_missing() {
    for extra in [json!({"playlists": {}}), json!({})] {
        let server = MockServer::start().await;
        Mock::given(path("/rest/getPlaylists"))
            .respond_with(json_response(&ok_envelope(extra)))
            .mount(&server)
            .await;
        let playlists = token_client(&server).get_playlists(None).await.unwrap();
        assert!(playlists.is_empty());
    }
}

#[tokio::test]
async fn get_open_subsonic_extensions_typed() {
    let server = MockServer::start().await;
    let body = ok_envelope(json!({
        "openSubsonicExtensions": [
            {"name": "template", "versions": [1, 2]},
            {"name": "transcodeOffset", "versions": [1]}
        ]
    }));
    Mock::given(path("/rest/getOpenSubsonicExtensions"))
        .respond_with(json_response(&body))
        .mount(&server)
        .await;
    let ext = token_client(&server)
        .get_open_subsonic_extensions()
        .await
        .unwrap();
    assert_eq!(ext.len(), 2);
    assert_eq!(ext[0].name, "template");
    assert_eq!(ext[0].versions, vec![1, 2]);
    assert_eq!(ext[1].name, "transcodeOffset");
}

#[tokio::test]
async fn star_unit_with_repeated_keys() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/star"))
        .and(RepeatedQuery::new("id", &["s1", "s2"]))
        .and(RepeatedQuery::new("albumId", &["a1"]))
        .and(NoQuery("artistId"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .expect(1)
        .mount(&server)
        .await;
    token_client(&server)
        .star(&["s1", "s2"], &["a1"], &[])
        .await
        .unwrap();
}

#[tokio::test]
async fn update_playlist_repeated_keys_in_order() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/updatePlaylist"))
        .and(query_param("playlistId", "p1"))
        .and(query_param("name", "New name"))
        .and(query_param("public", "true"))
        .and(RepeatedQuery::new("songIdToAdd", &["c", "a", "b"]))
        .and(RepeatedQuery::new("songIndexToRemove", &["4", "0"]))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .expect(1)
        .mount(&server)
        .await;
    token_client(&server)
        .update_playlist(
            "p1",
            Some("New name"),
            None,
            Some(true),
            &["c", "a", "b"],
            &[4, 0],
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn save_play_queue_repeated_ids() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/savePlayQueue"))
        .and(RepeatedQuery::new("id", &["z", "y", "x"]))
        .and(query_param("current", "y"))
        .and(query_param("position", "12345"))
        .respond_with(json_response(&ok_envelope(json!({}))))
        .expect(1)
        .mount(&server)
        .await;
    token_client(&server)
        .save_play_queue(&["z", "y", "x"], Some("y"), Some(12345))
        .await
        .unwrap();
}

// ── Binary endpoints ────────────────────────────────────────────────────────

#[tokio::test]
async fn get_cover_art_returns_bytes() {
    let server = MockServer::start().await;
    let png: Vec<u8> = vec![0x89, b'P', b'N', b'G', 0x0d, 0x0a, 0x1a, 0x0a, 0, 1, 2, 3];
    Mock::given(path("/rest/getCoverArt"))
        .and(query_param("id", "al-1"))
        .and(query_param("size", "300"))
        .respond_with(
            ResponseTemplate::new(200)
                .insert_header("content-type", "image/png")
                .set_body_bytes(png.clone()),
        )
        .expect(1)
        .mount(&server)
        .await;
    let bytes = token_client(&server)
        .get_cover_art("al-1", Some(300))
        .await
        .unwrap();
    assert_eq!(bytes.as_ref(), png.as_slice());
}

#[tokio::test]
async fn binary_endpoint_json_failure_is_api_error() {
    let server = MockServer::start().await;
    Mock::given(path("/rest/getCoverArt"))
        .respond_with(ResponseTemplate::new(200).set_body_json(failed_envelope(
            70,
            "Cover art not found",
            None,
        )))
        .mount(&server)
        .await;
    let err = token_client(&server)
        .get_cover_art("missing", None)
        .await
        .unwrap_err();
    assert!(matches!(err, Error::Api(_)), "got {err:?}");
    assert_eq!(err.api_error_code(), Some(SubsonicErrorCode::NotFound));
}
