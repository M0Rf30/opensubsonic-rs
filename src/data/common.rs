//! Common/shared types used across multiple API sections.

use serde::{Deserialize, Serialize};

/// A genre.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Genre {
    /// Genre name.
    #[serde(rename = "value")]
    pub name: String,
    /// Number of songs in this genre.
    pub song_count: i64,
    /// Number of albums in this genre.
    pub album_count: i64,
}

/// A genre tag on a media item (simplified, just a name).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemGenre {
    /// Genre name.
    pub name: String,
}

/// A date for a media item that may be partial (year only, year-month, or full date).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemDate {
    /// The year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// The month (1–12).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub month: Option<i32>,
    /// The day (1–31).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub day: Option<i32>,
}

/// A disc title for an album.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiscTitle {
    /// The disc number.
    pub disc: i32,
    /// The name of the disc.
    pub title: String,
}

/// A record label for an album.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RecordLabel {
    /// Label name.
    pub name: String,
}

/// Replay gain data for a song.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReplayGain {
    /// Track replay gain value in dB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_gain: Option<f64>,
    /// Album replay gain value in dB.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_gain: Option<f64>,
    /// Track peak value (positive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track_peak: Option<f64>,
    /// Album peak value (positive).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_peak: Option<f64>,
    /// Base gain value in dB (e.g. Ogg Opus output gain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_gain: Option<f64>,
    /// Fallback gain for when track/album gain is missing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fallback_gain: Option<f64>,
}

/// A contributor artist for a song or album.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contributor {
    /// The contributor role (e.g. "composer", "performer").
    pub role: String,
    /// Sub-role for roles that require it (e.g. the instrument for "performer").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_role: Option<String>,
    /// The contributing artist.
    pub artist: ArtistId3,
}

/// A music folder.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MusicFolder {
    /// Folder ID.
    pub id: i64,
    /// Folder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// A supported OpenSubsonic extension.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenSubsonicExtension {
    /// Extension name.
    pub name: String,
    /// Supported version numbers of this extension.
    pub versions: Vec<i32>,
}

/// License information.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    /// Whether the license is valid.
    pub valid: bool,
    /// User email.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    /// License expiration date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license_expires: Option<String>,
    /// Trial expiration date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_expires: Option<String>,
}

/// Token info (OpenSubsonic extension).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    /// Username associated with the token.
    pub username: String,
}

// ── ID3-based artist ───────────────────────────────────────────────────────

/// An artist from ID3 tags.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistId3 {
    /// Artist ID.
    pub id: String,
    /// Artist name.
    pub name: String,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// External image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_image_url: Option<String>,
    /// Number of albums.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_count: Option<i64>,
    /// Date the artist was starred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<String>,
    /// MusicBrainz ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// Sort name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    /// Roles this artist has in the library (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
}

/// An artist with its albums (ID3-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistWithAlbumsId3 {
    /// Artist ID.
    pub id: String,
    /// Artist name.
    pub name: String,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// External image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_image_url: Option<String>,
    /// Number of albums.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_count: Option<i64>,
    /// Date the artist was starred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<String>,
    /// MusicBrainz ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// Sort name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    /// Roles this artist has in the library (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roles: Option<Vec<String>>,
    /// The artist's albums.
    #[serde(default)]
    pub album: Vec<AlbumId3>,
}

/// A list of indexed artists (ID3-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArtistsId3 {
    /// Ignored articles (space-separated).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignored_articles: Option<String>,
    /// Index list (each entry groups artists by first letter).
    #[serde(default)]
    pub index: Vec<IndexId3>,
}

/// A single index entry in the artist list (ID3-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexId3 {
    /// Index name (e.g. "A", "B", "#").
    pub name: String,
    /// Artists in this index.
    #[serde(default)]
    pub artist: Vec<ArtistId3>,
}

// ── ID3-based album ────────────────────────────────────────────────────────

/// An album from ID3 tags.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumId3 {
    /// Album ID.
    pub id: String,
    /// Album name.
    pub name: String,
    /// Album version (e.g. "Remastered", "Deluxe Edition").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Artist name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    /// Artist ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<String>,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// Number of songs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i64>,
    /// Total duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Play count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i64>,
    /// Date added (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Date starred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<String>,
    /// Album year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// Genre name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Date last played (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub played: Option<String>,
    /// User rating (1–5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<i32>,
    /// Record labels (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_labels: Option<Vec<RecordLabel>>,
    /// MusicBrainz ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// All genres (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<ItemGenre>>,
    /// All artists (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<ArtistId3>>,
    /// Display artist string (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_artist: Option<String>,
    /// All album releases (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub releases: Option<Vec<serde_json::Value>>,
    /// Release date (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_release_date: Option<ItemDate>,
    /// Release date (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<ItemDate>,
    /// Whether this is a compilation (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compilation: Option<bool>,
    /// Sort name (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    /// Disc titles (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_titles: Option<Vec<DiscTitle>>,
    /// Explicit status (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_status: Option<String>,
    /// Moods (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moods: Option<Vec<String>>,
}

/// An album with its songs (ID3-based).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AlbumWithSongsId3 {
    /// Album ID.
    pub id: String,
    /// Album name.
    pub name: String,
    /// Album version.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Artist name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    /// Artist ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<String>,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// Number of songs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i64>,
    /// Total duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Play count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i64>,
    /// Date added (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Date starred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<String>,
    /// Album year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// Genre name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Date last played (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub played: Option<String>,
    /// User rating (1–5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<i32>,
    /// Record labels (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record_labels: Option<Vec<RecordLabel>>,
    /// MusicBrainz ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// All genres (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<ItemGenre>>,
    /// All artists (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<ArtistId3>>,
    /// Display artist string (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_artist: Option<String>,
    /// Release date (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_release_date: Option<ItemDate>,
    /// Release date (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub release_date: Option<ItemDate>,
    /// Whether this is a compilation (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_compilation: Option<bool>,
    /// Sort name (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    /// Disc titles (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_titles: Option<Vec<DiscTitle>>,
    /// Explicit status (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_status: Option<String>,
    /// Moods (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moods: Option<Vec<String>>,
    /// The songs in this album.
    #[serde(default)]
    pub song: Vec<Child>,
}

// ── Folder-based artist (legacy) ───────────────────────────────────────────

/// An artist (folder-based / legacy).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Artist {
    /// Artist ID.
    pub id: String,
    /// Artist name.
    pub name: String,
    /// Artist image URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_image_url: Option<String>,
    /// Date starred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<String>,
    /// User rating (1–5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<i32>,
    /// Average rating (1.0–5.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_rating: Option<f64>,
}

// ── Child (song/media) ─────────────────────────────────────────────────────

/// A media item (song, video, or directory entry). This is the fundamental type returned by
/// most browsing, searching, and listing endpoints.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Child {
    /// Media ID.
    pub id: String,
    /// Parent folder/album ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,
    /// Whether this entry is a directory.
    #[serde(default)]
    pub is_dir: bool,
    /// Media title.
    pub title: String,
    /// Album name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    /// Artist name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist: Option<String>,
    /// Track number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub track: Option<i32>,
    /// Year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
    /// Genre name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genre: Option<String>,
    /// Cover art ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_art: Option<String>,
    /// File size in bytes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
    /// MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    /// File suffix (e.g. "mp3", "flac").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    /// Transcoded MIME type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcoded_content_type: Option<String>,
    /// Transcoded file suffix.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcoded_suffix: Option<String>,
    /// Duration in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<i64>,
    /// Bitrate in kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_rate: Option<i32>,
    /// Bit depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bit_depth: Option<i32>,
    /// Sampling rate in Hz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling_rate: Option<i32>,
    /// Number of audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub channel_count: Option<i32>,
    /// Full file path.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    /// Whether this is a video.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_video: Option<bool>,
    /// User rating (1–5).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_rating: Option<i32>,
    /// Average rating (1.0–5.0).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub average_rating: Option<f64>,
    /// Play count.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub play_count: Option<i64>,
    /// Disc number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disc_number: Option<i32>,
    /// Date created (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<String>,
    /// Date starred (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub starred: Option<String>,
    /// Album ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_id: Option<String>,
    /// Artist ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artist_id: Option<String>,
    /// Generic media type (music/podcast/audiobook/video).
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub media_type_generic: Option<String>,
    /// Media type (song/album/artist) — OpenSubsonic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub media_type: Option<String>,
    /// Bookmark position in seconds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bookmark_position: Option<i64>,
    /// Original video width.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_width: Option<i32>,
    /// Original video height.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_height: Option<i32>,
    /// Date last played (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub played: Option<String>,
    /// BPM (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bpm: Option<i32>,
    /// Comment tag (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comment: Option<String>,
    /// Sort name (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_name: Option<String>,
    /// MusicBrainz ID (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_brainz_id: Option<String>,
    /// ISRC codes (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isrc: Option<Vec<String>>,
    /// All genres (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub genres: Option<Vec<ItemGenre>>,
    /// All song artists (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<ArtistId3>>,
    /// Display artist string (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_artist: Option<String>,
    /// Album artists (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub album_artists: Option<Vec<ArtistId3>>,
    /// Display album artist (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_album_artist: Option<String>,
    /// Contributors (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contributors: Option<Vec<Contributor>>,
    /// Display composer (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_composer: Option<String>,
    /// Moods (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub moods: Option<Vec<String>>,
    /// Replay gain data (OpenSubsonic).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replay_gain: Option<ReplayGain>,
    /// Explicit status (OpenSubsonic): "explicit", "clean", or "".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explicit_status: Option<String>,
}

/// A "now playing" entry — a [`Child`] with additional playback metadata.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NowPlayingEntry {
    /// All fields from [`Child`].
    #[serde(flatten)]
    pub child: Child,
    /// Username of the listener.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Minutes since last update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minutes_ago: Option<i64>,
    /// Player ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_id: Option<i64>,
    /// Player name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub player_name: Option<String>,
}
