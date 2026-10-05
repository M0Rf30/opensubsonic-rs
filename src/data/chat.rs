// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Types for the Chat API section.

use serde::{Deserialize, Serialize};

/// A chat message.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    /// Username of the sender.
    pub username: String,
    /// Timestamp in milliseconds since epoch (Jan 1, 1970).
    pub time: i64,
    /// Message text.
    pub message: String,
}
