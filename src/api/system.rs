// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! System API endpoints: `ping`, `getLicense`, `getOpenSubsonicExtensions`, `tokenInfo`.

use crate::Client;
use crate::data::{License, OpenSubsonicExtension, TokenInfo};
use crate::error::Error;
use crate::params::Params;

impl Client {
    /// Test connectivity with the server. Returns `Ok(())` on success.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/ping/>
    pub async fn ping(&self) -> Result<(), Error> {
        self.get_unit("ping", &Params::new()).await
    }

    /// Get details about the software license.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getlicense/>
    pub async fn get_license(&self) -> Result<License, Error> {
        self.get_field("getLicense", &Params::new(), "license")
            .await
    }

    /// Get the list of OpenSubsonic API extensions supported by the server.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getopensubsonicextensions/>
    pub async fn get_open_subsonic_extensions(&self) -> Result<Vec<OpenSubsonicExtension>, Error> {
        self.get_field_or_default(
            "getOpenSubsonicExtensions",
            &Params::new(),
            "openSubsonicExtensions",
        )
        .await
    }

    /// Get information about the API token (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/tokeninfo/>
    pub async fn token_info(&self) -> Result<TokenInfo, Error> {
        self.get_field("tokenInfo", &Params::new(), "tokenInfo")
            .await
    }
}
