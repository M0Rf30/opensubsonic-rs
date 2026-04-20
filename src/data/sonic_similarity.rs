//! Types for the Sonic Similarity API (OpenSubsonic extension).

use serde::{Deserialize, Serialize};

use super::common::Child;

/// A sonic similarity match — a [`Child`] with a similarity score.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SonicMatch {
    /// The matched track.
    #[serde(flatten)]
    pub entry: Child,
    /// Normalized similarity score (1.0 = exact, 0.0 = most different, -1 if unsupported).
    pub similarity: f64,
}
