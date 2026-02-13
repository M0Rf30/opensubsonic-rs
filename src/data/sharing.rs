//! Types for the Sharing API section.

use serde::{Deserialize, Serialize};

use super::common::Child;

/// A share (publicly accessible link).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Share {
    /// Share ID.
    pub id: String,
    /// Share URL.
    pub url: String,
    /// Description.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Username of the creator.
    pub username: String,
    /// Date created (ISO 8601).
    pub created: String,
    /// Expiration date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Last visited date (ISO 8601).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_visited: Option<String>,
    /// Visit count.
    pub visit_count: i64,
    /// Shared entries.
    #[serde(default)]
    pub entry: Vec<Child>,
}
