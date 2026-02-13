//! Complete async Rust client for the OpenSubsonic / Subsonic REST API.
//!
//! Supports Subsonic API v1.16.1 and OpenSubsonic extensions.
//!
//! # Quick start
//!
//! ```no_run
//! use opensubsonic::{Client, Auth};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), opensubsonic::Error> {
//!     let client = Client::new(
//!         "https://music.example.com",
//!         "admin",
//!         Auth::token("password"),
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
//!     let results = client.search3("bohemian", None, None, None, None, None, None, None).await?;
//!     for song in &results.song {
//!         println!("{} - {}", song.artist.as_deref().unwrap_or("?"), song.title);
//!     }
//!
//!     // Get a streaming URL.
//!     let url = client.stream_url("song-id-123", None, None)?;
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
//!   `getArtists`, `getArtist`, `getAlbum`, `getSong`, `getVideos`, `getArtistInfo`/`2`,
//!   `getAlbumInfo`/`2`, `getSimilarSongs`/`2`, `getTopSongs`
//! - **Lists**: `getAlbumList`/`2`, `getRandomSongs`, `getSongsByGenre`, `getNowPlaying`,
//!   `getStarred`/`2`
//! - **Searching**: `search`, `search2`, `search3`
//! - **Playlists**: `getPlaylists`, `getPlaylist`, `createPlaylist`, `updatePlaylist`,
//!   `deletePlaylist`
//! - **Media Retrieval**: `stream`, `download`, `hls`, `getCaptions`, `getCoverArt`,
//!   `getLyrics`, `getLyricsBySongId`, `getAvatar`
//! - **Media Annotation**: `star`, `unstar`, `setRating`, `scrobble`
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

mod auth;
mod client;
mod error;
pub mod api;
pub mod data;

pub use auth::Auth;
pub use client::Client;
pub use error::{Error, SubsonicApiError, SubsonicErrorCode};

// Re-export commonly used API types that live in api modules.
pub use api::jukebox::{JukeboxAction, JukeboxResult};
pub use api::lists::{AlbumListType, Starred2Content, StarredContent};
