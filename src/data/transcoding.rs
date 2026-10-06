// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Types for the Transcoding API section (OpenSubsonic extension).

use serde::{Deserialize, Serialize};

/// Kind of media referred to by a transcode request's `mediaId`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TranscodeMediaType {
    /// A song.
    Song,
    /// A podcast episode.
    Podcast,
}

impl TranscodeMediaType {
    /// The wire representation (`"song"` or `"podcast"`).
    #[must_use]
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Song => "song",
            Self::Podcast => "podcast",
        }
    }
}

impl std::fmt::Display for TranscodeMediaType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Details about a media stream (`StreamDetails`). Bitrates are in bits per second.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
    /// Audio bitrate in bits per second (e.g. `3000000`).
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
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
    /// Opaque server value to pass verbatim to `getTranscodeStream` as `transcodeParams`.
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientInfo {
    /// Client name.
    pub name: String,
    /// Client platform.
    pub platform: String,
    /// Maximum audio bitrate the client can handle, in bits per second. `0` or absent means no limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_bitrate: Option<i32>,
    /// Maximum audio bitrate for transcoded content, in bits per second. `0` or absent means no limit.
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
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DirectPlayProfile {
    /// Supported containers (empty = any).
    #[serde(default)]
    pub containers: Vec<String>,
    /// Supported audio codecs (empty = any).
    #[serde(default)]
    pub audio_codecs: Vec<String>,
    /// Supported protocols (`http`, `hls`).
    #[serde(default)]
    pub protocols: Vec<String>,
    /// Max audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_channels: Option<i32>,
}

/// Transcoding profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TranscodingProfile {
    /// Container format.
    pub container: String,
    /// Audio codec.
    pub audio_codec: String,
    /// Protocol (`http` or `hls`).
    pub protocol: String,
    /// Max audio channels.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_audio_channels: Option<i32>,
}

/// Codec profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodecProfile {
    /// Profile type; currently only `AudioCodec`.
    #[serde(rename = "type")]
    pub profile_type: String,
    /// Codec name.
    pub name: String,
    /// Limitations.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub limitations: Vec<Limitation>,
}

/// A limitation on a codec profile.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Limitation {
    /// Limitation name: `audioChannels`, `audioBitrate`, `audioProfile`, `audioSamplerate` or `audioBitdepth`.
    pub name: String,
    /// Comparison operator (`Equals`, `NotEquals`, `LessThanEqual`, `GreaterThanEqual`).
    pub comparison: String,
    /// Values to compare against.
    pub values: Vec<String>,
    /// Whether this limitation is required.
    pub required: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    #[test]
    fn deserialize_spec_decision() {
        let v = json!({
            "canDirectPlay": false, "canTranscode": true,
            "transcodeReason": ["AudioCodecNotSupported"], "errorReason": "",
            "transcodeParams": "0001-0005-004",
            "sourceStream": {"protocol":"http","container":"flac","codec":"flac",
                "audioChannels":6,"audioBitrate":3000000,"audioProfile":"",
                "audioSamplerate":96000,"audioBitdepth":24},
            "transcodeStream": {"protocol":"hls","container":"mp4","codec":"aac",
                "audioChannels":2,"audioBitrate":256000,"audioProfile":"xHE-AAC",
                "audioSamplerate":48000,"audioBitdepth":16}
        });
        let d: TranscodeDecision = serde_json::from_value(v).unwrap();
        assert!(!d.can_direct_play && d.can_transcode);
        assert_eq!(d.transcode_reason, ["AudioCodecNotSupported"]);
        assert_eq!(d.transcode_params.as_deref(), Some("0001-0005-004"));
        assert_eq!(d.source_stream.unwrap().audio_bitrate, Some(3_000_000));
        assert_eq!(
            d.transcode_stream.unwrap().audio_profile.as_deref(),
            Some("xHE-AAC")
        );
    }

    #[test]
    fn client_info_round_trip() {
        let v = json!({
            "name": "Play:1", "platform": "Sonos",
            "maxAudioBitrate": 512000, "maxTranscodingAudioBitrate": 256000,
            "directPlayProfiles": [
                {"containers":["mp3"],"audioCodecs":["mp3"],"protocols":["http"],"maxAudioChannels":2},
                {"containers":["flac"],"audioCodecs":["flac"],"protocols":[],"maxAudioChannels":2},
                {"containers":["mp4"],"audioCodecs":["flac","aac","alac"],"protocols":[],"maxAudioChannels":2}
            ],
            "transcodingProfiles": [
                {"container":"mp3","audioCodec":"mp3","protocol":"http","maxAudioChannels":2},
                {"container":"flac","audioCodec":"flac","protocol":"hls","maxAudioChannels":2}
            ],
            "codecProfiles": [
                {"type":"AudioCodec","name":"mp3","limitations":[
                    {"name":"audioBitrate","comparison":"LessThanEqual","values":["320000"],"required":true}]},
                {"type":"AudioCodec","name":"flac","limitations":[
                    {"name":"audioSamplerate","comparison":"LessThanEqual","values":["192000"],"required":false},
                    {"name":"audioChannels","comparison":"Equals","values":["1","2"],"required":false}]}
            ]
        });
        let info: ClientInfo = serde_json::from_value(v.clone()).unwrap();
        let back: Value = serde_json::to_value(&info).unwrap();
        assert_eq!(back, v);
    }

    #[test]
    fn media_type_strings() {
        assert_eq!(TranscodeMediaType::Song.to_string(), "song");
        assert_eq!(TranscodeMediaType::Podcast.as_str(), "podcast");
    }
}
