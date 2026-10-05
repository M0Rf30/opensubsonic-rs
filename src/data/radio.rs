// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Types for the Internet Radio API section.

use serde::{Deserialize, Serialize};

/// An internet radio station.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternetRadioStation {
    /// Station ID.
    pub id: String,
    /// Station name.
    pub name: String,
    /// Stream URL.
    pub stream_url: String,
    /// Home page URL.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub home_page_url: Option<String>,
}
