//! Types for the Transcoding API section (OpenSubsonic extension).

use serde::{Deserialize, Serialize};

/// Stream details for a media file.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StreamDetails {
    /// Protocol (e.g. "http", "hls").
    pub protocol: String,
    /// Container format.
    pub container: String,
    /// Codec.
    pub codec: String,
    /// Number of audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_channels: Option<i32>,
    /// Audio bitrate in kbps.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_bitrate: Option<i32>,
    /// Audio profile.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_profile: Option<String>,
    /// Audio sample rate in Hz.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_samplerate: Option<i32>,
    /// Audio bit depth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audio_bitdepth: Option<i32>,
}

/// Transcode decision response.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscodeDecision {
    /// Whether direct play is possible.
    pub can_direct_play: bool,
    /// Whether transcoding is possible.
    pub can_transcode: bool,
    /// Reasons for transcoding (if any).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transcode_reason: Vec<String>,
    /// Error reason (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_reason: Option<String>,
    /// Transcoding parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_params: Option<String>,
    /// Source stream details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_stream: Option<StreamDetails>,
    /// Transcoded stream details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transcode_stream: Option<StreamDetails>,
}

/// Client info for transcode decision request.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    /// Client name.
    pub name: String,
    /// Client platform.
    pub platform: String,
    /// Max audio bitrate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_bitrate: Option<i32>,
    /// Max transcoding audio bitrate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_transcoding_audio_bitrate: Option<i32>,
    /// Direct play profiles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub direct_play_profiles: Vec<DirectPlayProfile>,
    /// Transcoding profiles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub transcoding_profiles: Vec<TranscodingProfile>,
    /// Codec profiles.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub codec_profiles: Vec<CodecProfile>,
}

/// Direct play profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectPlayProfile {
    /// Supported containers (empty = any).
    #[serde(default)]
    pub containers: Vec<String>,
    /// Supported audio codecs (empty = any).
    #[serde(default)]
    pub audio_codecs: Vec<String>,
    /// Supported protocols (empty = any).
    #[serde(default)]
    pub protocols: Vec<String>,
    /// Max audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_channels: Option<i32>,
}

/// Transcoding profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscodingProfile {
    /// Container format.
    pub container: String,
    /// Audio codec.
    pub audio_codec: String,
    /// Protocol ("http" or "hls").
    pub protocol: String,
    /// Max audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_channels: Option<i32>,
}

/// Codec profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodecProfile {
    /// Type (e.g. "AudioCodec").
    #[serde(rename = "type")]
    pub profile_type: String,
    /// Codec name.
    pub name: String,
    /// Limitations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitations: Vec<Limitation>,
}

/// A limitation on a codec profile.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Limitation {
    /// Limitation name (e.g. "audioChannels", "audioBitrate").
    pub name: String,
    /// Comparison operator.
    pub comparison: String,
    /// Values to compare against.
    pub values: Vec<String>,
    /// Whether this limitation is required.
    pub required: bool,
}
