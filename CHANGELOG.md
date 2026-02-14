# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

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

[unreleased]: https://github.com/M0Rf30/opensubsonic-rs/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/M0Rf30/opensubsonic-rs/releases/tag/v0.1.0
