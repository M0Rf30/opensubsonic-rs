// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Types for the Media Library Scanning API section.

use serde::{Deserialize, Serialize};

/// Library scan status.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScanStatus {
    /// Whether a scan is currently in progress.
    pub scanning: bool,
    /// Number of items scanned so far.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
}
