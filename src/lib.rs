// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Complete async Rust client for the OpenSubsonic / Subsonic REST API.
//!
//! Supports Subsonic API v1.16.1 and OpenSubsonic extensions.
//!
//! # Quick start
//!
//! ```no_run
//! use opensubsonic::{Auth, Client, Search3Options, StreamOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), opensubsonic::Error> {
//!     // Token auth (username + password):
//!     let client = Client::new(
//!         "https://music.example.com",
//!         Auth::token("admin", "password"),
//!     )?;
//!
//!     // Or API key auth (OpenSubsonic extension):
//!     let client = Client::new(
//!         "https://music.example.com",
//!         Auth::api_key("your-api-key"),
//!     )?;
//!
//!     // Verify connectivity.
//!     client.ping().await?;
//!
//!     // Browse artists.
//!     let artists = client.get_artists(None).await?;
//!     for index in &artists.index {
//!         for artist in &index.artist {
//!             println!("{}", artist.name);
//!         }
//!     }
//!
//!     // Search for songs.
//!     let results = client
//!         .search3("bohemian", &Search3Options::new().song_count(20))
//!         .await?;
//!     for song in &results.song {
//!         println!("{} - {}", song.artist.as_deref().unwrap_or("?"), song.title);
//!     }
//!
//!     // Get a streaming URL.
//!     let url = client.stream_url("song-id-123", &StreamOptions::new().max_bit_rate(320))?;
//!     println!("Stream: {url}");
//!
//!     Ok(())
//! }
//! ```
//!
//! # API coverage
//!
//! All Subsonic API v1.16.1 endpoints are implemented, plus OpenSubsonic extensions:
//!
//! - **System**: `ping`, `getLicense`, `getOpenSubsonicExtensions`, `tokenInfo`
//! - **Browsing**: `getMusicFolders`, `getIndexes`, `getMusicDirectory`, `getGenres`,
//!   `getArtists`, `getArtist`, `getAlbum`, `getSong`, `getVideos`, `getVideoInfo`, `getArtistInfo`/`2`,
//!   `getAlbumInfo`/`2`, `getSimilarSongs`/`2`, `getTopSongs`
//! - **Lists**: `getAlbumList`/`2`, `getRandomSongs`, `getSongsByGenre`, `getNowPlaying`,
//!   `getStarred`/`2`
//! - **Searching**: `search`, `search2`, `search3`
//! - **Playlists**: `getPlaylists`, `getPlaylist`, `createPlaylist`, `updatePlaylist`,
//!   `deletePlaylist`
//! - **Media Retrieval**: `stream`, `download`, `hls`, `getCaptions`, `getCoverArt`,
//!   `getLyrics`, `getLyricsBySongId`, `getAvatar`
//! - **Media Annotation**: `star`, `unstar`, `setRating`, `scrobble`, `reportPlayback`
//! - **Sharing**: `getShares`, `createShare`, `updateShare`, `deleteShare`
//! - **Podcast**: `getPodcasts`, `getNewestPodcasts`, `getPodcastEpisode`, `refreshPodcasts`,
//!   `createPodcastChannel`, `deletePodcastChannel`, `deletePodcastEpisode`,
//!   `downloadPodcastEpisode`
//! - **Jukebox**: `jukeboxControl`
//! - **Internet Radio**: `getInternetRadioStations`, `createInternetRadioStation`,
//!   `updateInternetRadioStation`, `deleteInternetRadioStation`
//! - **Chat**: `getChatMessages`, `addChatMessage`
//! - **User Management**: `getUser`, `getUsers`, `createUser`, `updateUser`, `deleteUser`,
//!   `changePassword`
//! - **Bookmarks**: `getBookmarks`, `createBookmark`, `deleteBookmark`, `getPlayQueue`,
//!   `savePlayQueue`, `getPlayQueueByIndex`, `savePlayQueueByIndex`
//! - **Scanning**: `getScanStatus`, `startScan`
//! - **Transcoding** (OpenSubsonic): `getTranscodeDecision`, `getTranscodeStream`
//! - **Sonic Similarity** (OpenSubsonic): `getSonicSimilarTracks`, `findSonicPath`

pub mod api;
mod auth;
mod client;
pub mod data;
mod error;
mod params;

pub use auth::Auth;
pub use client::{ByteStream, Client};
pub use error::{Error, SubsonicApiError, SubsonicErrorCode};

// Re-export commonly used API types that live in api modules.
pub use api::jukebox::{JukeboxAction, JukeboxResult};
pub use api::lists::{AlbumListType, Starred2Content, StarredContent};
pub use api::{
    AlbumListOptions, CreateUserOptions, JukeboxOptions, RandomSongsOptions, Search2Options,
    Search3Options, SearchOptions, SongsByGenreOptions, StreamOptions, UpdatePlaylistOptions,
    UpdateUserOptions,
};
