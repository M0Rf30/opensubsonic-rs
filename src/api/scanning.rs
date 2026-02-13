//! Media Library Scanning API endpoints.

use crate::data::ScanStatus;
use crate::error::Error;
use crate::Client;

impl Client {
    /// Get the current scan status.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getscanstatus/>
    pub async fn get_scan_status(&self) -> Result<ScanStatus, Error> {
        let data = self.get_response("getScanStatus", &[]).await?;
        let status = data
            .get("scanStatus")
            .ok_or_else(|| Error::Parse("Missing 'scanStatus' in response".into()))?;
        Ok(serde_json::from_value(status.clone())?)
    }

    /// Start a media library scan.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/startscan/>
    pub async fn start_scan(&self) -> Result<ScanStatus, Error> {
        let data = self.get_response("startScan", &[]).await?;
        let status = data
            .get("scanStatus")
            .ok_or_else(|| Error::Parse("Missing 'scanStatus' in response".into()))?;
        Ok(serde_json::from_value(status.clone())?)
    }
}
