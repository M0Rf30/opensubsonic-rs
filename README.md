# opensubsonic-rs

[![Crates.io](https://img.shields.io/crates/v/opensubsonic.svg)](https://crates.io/crates/opensubsonic)
[![Docs.rs](https://docs.rs/opensubsonic/badge.svg)](https://docs.rs/opensubsonic)
[![CI](https://github.com/M0Rf30/opensubsonic-rs/workflows/CI/badge.svg)](https://github.com/M0Rf30/opensubsonic-rs/actions)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)

Complete async Rust client for the [OpenSubsonic](https://opensubsonic.netlify.app/) / Subsonic REST API.

Supports **Subsonic API v1.16.1** and **OpenSubsonic extensions**. Works with [Navidrome](https://www.navidrome.org/), [Gonic](https://github.com/sentriz/gonic), [Ampache](https://ampache.org/), and any other Subsonic-compatible server.

## Quick start

```rust
use opensubsonic::{Client, Auth};

#[tokio::main]
async fn main() -> Result<(), opensubsonic::Error> {
    let client = Client::new(
        "https://music.example.com",
        "admin",
        Auth::token("password"),
    )?;

    // Verify connectivity.
    client.ping().await?;

    // Browse artists.
    let artists = client.get_artists(None).await?;
    for index in &artists.index {
        for artist in &index.artist {
            println!("{}", artist.name);
        }
    }

    // Search for songs.
    let results = client.search3("bohemian", None, None, None, None, None, None, None).await?;
    for song in &results.song {
        println!("{} - {}", song.artist.as_deref().unwrap_or("?"), song.title);
    }

    // Get a streaming URL (no HTTP request, just builds the URL).
    let url = client.stream_url("song-id-123", None, None)?;
    println!("Stream: {url}");

    Ok(())
}
```

## Authentication

Two methods are supported:

```rust
// Token-based (recommended) — sends MD5(password + salt) + salt
let auth = Auth::token("my-password");

// Plain text (legacy) — sends hex-encoded password
let auth = Auth::plain("my-password");
```

## API coverage

All ~80 endpoints from Subsonic API v1.16.1 are implemented, plus OpenSubsonic extensions:

| Category | Endpoints |
|---|---|
| **System** | `ping`, `getLicense`, `getOpenSubsonicExtensions`, `tokenInfo` |
| **Browsing** | `getMusicFolders`, `getIndexes`, `getMusicDirectory`, `getGenres`, `getArtists`, `getArtist`, `getAlbum`, `getSong`, `getVideos`, `getArtistInfo`/`2`, `getAlbumInfo`/`2`, `getSimilarSongs`/`2`, `getTopSongs` |
| **Lists** | `getAlbumList`/`2`, `getRandomSongs`, `getSongsByGenre`, `getNowPlaying`, `getStarred`/`2` |
| **Searching** | `search`, `search2`, `search3` |
| **Playlists** | `getPlaylists`, `getPlaylist`, `createPlaylist`, `updatePlaylist`, `deletePlaylist` |
| **Media Retrieval** | `stream`, `download`, `hls`, `getCaptions`, `getCoverArt`, `getLyrics`, `getLyricsBySongId`, `getAvatar` |
| **Media Annotation** | `star`, `unstar`, `setRating`, `scrobble` |
| **Sharing** | `getShares`, `createShare`, `updateShare`, `deleteShare` |
| **Podcast** | `getPodcasts`, `getNewestPodcasts`, `getPodcastEpisode`, `refreshPodcasts`, `createPodcastChannel`, `deletePodcastChannel`, `deletePodcastEpisode`, `downloadPodcastEpisode` |
| **Jukebox** | `jukeboxControl` |
| **Internet Radio** | `getInternetRadioStations`, `createInternetRadioStation`, `updateInternetRadioStation`, `deleteInternetRadioStation` |
| **Chat** | `getChatMessages`, `addChatMessage` |
| **User Management** | `getUser`, `getUsers`, `createUser`, `updateUser`, `deleteUser`, `changePassword` |
| **Bookmarks** | `getBookmarks`, `createBookmark`, `deleteBookmark`, `getPlayQueue`, `savePlayQueue`, `getPlayQueueByIndex`, `savePlayQueueByIndex` |
| **Scanning** | `getScanStatus`, `startScan` |
| **Transcoding** | `getTranscodeDecision`, `getTranscodeStream` *(OpenSubsonic)* |

## Builder options

```rust
let client = Client::new("https://music.example.com", "admin", Auth::token("pass"))?
    .with_client_name("my-app")        // Custom client identifier (default: "opensubsonic-rs")
    .with_api_version("1.15.0")        // Override protocol version (default: "1.16.1")
    .with_http_client(custom_reqwest);  // Inject a custom reqwest::Client
```

## URL builders

Some methods build URLs without making HTTP requests, useful for passing to audio players:

```rust
let stream_url = client.stream_url("song-id", None, None)?;
let cover_url = client.cover_art_url("cover-id", Some(300))?;
let hls_url = client.hls_url("video-id", None, None)?;
```

## Dependencies

- [reqwest](https://crates.io/crates/reqwest) 0.13 (async HTTP, rustls TLS)
- [serde](https://crates.io/crates/serde) / serde_json (JSON serialization)
- [tokio](https://crates.io/crates/tokio) (async runtime, dev-dependency)

Minimum supported Rust version: **1.85** (edition 2024).

## License

Licensed under either of [Apache License, Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or [MIT License](http://opensource.org/licenses/MIT) at your option.
