// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Media Library Scanning API endpoints.

use crate::Client;
use crate::data::ScanStatus;
use crate::error::Error;
use crate::params::Params;

impl Client {
    /// Get the current scan status.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getscanstatus/>
    pub async fn get_scan_status(&self) -> Result<ScanStatus, Error> {
        self.get_field("getScanStatus", &Params::new(), "scanStatus")
            .await
    }

    /// Start a media library scan.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/startscan/>
    pub async fn start_scan(&self) -> Result<ScanStatus, Error> {
        self.get_field("startScan", &Params::new(), "scanStatus")
            .await
    }
}
