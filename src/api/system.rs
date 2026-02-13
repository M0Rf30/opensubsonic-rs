//! System API endpoints: `ping`, `getLicense`, `getOpenSubsonicExtensions`, `tokenInfo`.

use crate::data::{License, OpenSubsonicExtension, TokenInfo};
use crate::error::Error;
use crate::Client;

impl Client {
    /// Test connectivity with the server. Returns `Ok(())` on success.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/ping/>
    pub async fn ping(&self) -> Result<(), Error> {
        self.get_response("ping", &[]).await?;
        Ok(())
    }

    /// Get details about the software license.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getlicense/>
    pub async fn get_license(&self) -> Result<License, Error> {
        let data = self.get_response("getLicense", &[]).await?;
        let license = data
            .get("license")
            .ok_or_else(|| Error::Parse("Missing 'license' in response".into()))?;
        Ok(serde_json::from_value(license.clone())?)
    }

    /// Get the list of OpenSubsonic API extensions supported by the server.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getopensubsonicextensions/>
    pub async fn get_open_subsonic_extensions(
        &self,
    ) -> Result<Vec<OpenSubsonicExtension>, Error> {
        let data = self
            .get_response("getOpenSubsonicExtensions", &[])
            .await?;
        let extensions = data
            .get("openSubsonicExtensions")
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(extensions)?)
    }

    /// Get information about the API token (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/tokeninfo/>
    pub async fn token_info(&self) -> Result<TokenInfo, Error> {
        let data = self.get_response("tokenInfo", &[]).await?;
        let info = data
            .get("tokenInfo")
            .ok_or_else(|| Error::Parse("Missing 'tokenInfo' in response".into()))?;
        Ok(serde_json::from_value(info.clone())?)
    }
}
