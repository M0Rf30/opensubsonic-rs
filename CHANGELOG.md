# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.0] - 2026-10-06

### Added

- `Client::stream_chunked`, `Client::download_chunked` and
  `Client::get_transcode_stream_chunked` return a `ByteStream` that yields chunks as they
  arrive instead of buffering the whole file in memory.
- `Client::server_info()` exposes the response envelope metadata (`version`, `type`,
  `serverVersion`, `openSubsonic`) as `ServerInfo`.
- `SubsonicErrorCode::code()` and `Error::api_error_code()` convenience accessors.
- `TranscodeMediaType` enum; `Default` derives on transcoding payload types and `Lyrics`.
- Integration test suite running the client against a local mock server (`wiremock`).
- CI: MSRV (1.85) job, doc tests, Dependabot, GitHub release creation on tag.

### Changed

- **Breaking:** transcoding endpoints now follow the OpenSubsonic spec.
  `get_transcode_decision(media_id, media_type, &client_info)` always POSTs the `ClientInfo`
  body with `mediaId`/`mediaType`; `get_transcode_stream*` take
  `(media_id, media_type, transcode_params, offset)`.
- **Breaking:** `Error` and `SubsonicErrorCode` are `#[non_exhaustive]`.
- `Auth`'s `Debug` output redacts passwords and API keys. Request URLs in debug logs and in
  `Error::Http` (reqwest) errors have `p`, `t`, `s`, `apiKey`, `password` and URL userinfo
  passwords redacted.
- Parse errors include at most 256 characters of the offending response body.
- Internal refactor: a single `Params` builder and shared envelope handling replace the
  per-endpoint boilerplate; response fields are moved out of the JSON map instead of cloned.
- Dependencies: `md-5` 0.11, `rand` 0.10; lockfile refreshed.
- CI: `actions/checkout@v7`, `Swatinem/rust-cache`, `--locked` builds, publishing via
  `CARGO_REGISTRY_TOKEN`.

### Removed

- Unused `chrono` dependency.

### Fixed

- POST error responses (`getTranscodeDecision`) now keep `helpUrl` and no longer truncate
  the error code through an `i64` → `i32` cast.
- Missing documentation on public enum variants and lyrics fields; `missing_docs` is now
  linted.

## [0.4.0] - 2026-07-04

### Added

- `getVideoInfo` endpoint with `VideoInfo`, `Captions`, `AudioTrack`, `VideoConversion`.
- `DiscTitle.coverArt`, `Child.groupings`, `AlbumWithSongsId3.releaseTypes`.

### Changed

- Dependencies updated to the latest Rust 1.85-compatible versions.

## [0.3.0] - 2026-04-20

### Changed

- Dependencies updated.

## [0.2.0] - 2026-04-20

### Added

- API key authentication (OpenSubsonic `apiKeyAuthentication` extension) and error codes
  42–44.
- `reportPlayback`, `getSonicSimilarTracks`, `findSonicPath` endpoints.
- `Work`, `Movement`, `SonicMatch`, `Agent`, `Cue`, `CueLine` types; songLyrics v2 fields
  and `enhanced` parameter for `getLyricsBySongId`.
- `NowPlayingEntry` playback state fields, `PlaylistWithSongs.validUntil`,
  `SubsonicApiError.help_url`.

### Fixed

- `AlbumId3.releaseTypes` field name and type.

## [0.1.0] - 2026-02-14

### Added

- Initial release of opensubsonic
- Complete async Rust client for the OpenSubsonic/Subsonic REST API
- Support for Subsonic API v1.16.1 and OpenSubsonic extensions
- Authentication via token-based (MD5 + salt) and plain text methods
- All ~80 endpoints implemented:
  - System: ping, getLicense, getOpenSubsonicExtensions, tokenInfo
  - Browsing: getMusicFolders, getIndexes, getMusicDirectory, getGenres, getArtists, getArtist, getAlbum, getSong, getVideos, getArtistInfo/2, getAlbumInfo/2, getSimilarSongs/2, getTopSongs
  - Lists: getAlbumList/2, getRandomSongs, getSongsByGenre, getNowPlaying, getStarred/2
  - Searching: search, search2, search3
  - Playlists: getPlaylists, getPlaylist, createPlaylist, updatePlaylist, deletePlaylist
  - Media Retrieval: stream, download, hls, getCaptions, getCoverArt, getLyrics, getLyricsBySongId, getAvatar
  - Media Annotation: star, unstar, setRating, scrobble
  - Sharing: getShares, createShare, updateShare, deleteShare
  - Podcast: getPodcasts, getNewestPodcasts, getPodcastEpisode, refreshPodcasts, createPodcastChannel, deletePodcastChannel, deletePodcastEpisode, downloadPodcastEpisode
  - Jukebox: jukeboxControl
  - Internet Radio: getInternetRadioStations, createInternetRadioStation, updateInternetRadioStation, deleteInternetRadioStation
  - Chat: getChatMessages, addChatMessage
  - User Management: getUser, getUsers, createUser, updateUser, deleteUser, changePassword
  - Bookmarks: getBookmarks, createBookmark, deleteBookmark, getPlayQueue, savePlayQueue, getPlayQueueByIndex, savePlayQueueByIndex
  - Scanning: getScanStatus, startScan
  - OpenSubsonic extensions: getTranscodeDecision, getTranscodeStream
- URL builder methods for stream, cover art, and HLS URLs
- Builder pattern for client configuration
- Comprehensive error handling
- Full type definitions for all API responses

[unreleased]: https://github.com/M0Rf30/opensubsonic-rs/compare/v0.5.0...HEAD
[0.5.0]: https://github.com/M0Rf30/opensubsonic-rs/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/M0Rf30/opensubsonic-rs/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/M0Rf30/opensubsonic-rs/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/M0Rf30/opensubsonic-rs/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/M0Rf30/opensubsonic-rs/releases/tag/v0.1.0
