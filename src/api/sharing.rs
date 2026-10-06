// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Sharing API endpoints.

use crate::Client;
use crate::data::Share;
use crate::error::Error;
use crate::params::Params;
use serde::Deserialize;

/// Wrapper for the nested `shares.share` response shape.
#[derive(Deserialize, Default)]
struct SharesWrapper {
    #[serde(default)]
    share: Vec<Share>,
}

impl Client {
    /// Get all shares.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getshares/>
    pub async fn get_shares(&self) -> Result<Vec<Share>, Error> {
        let wrapper: Option<SharesWrapper> = self
            .get_field_or_default("getShares", &Params::new(), "shares")
            .await?;
        Ok(wrapper.map(|w| w.share).unwrap_or_default())
    }

    /// Create a new share.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createshare/>
    pub async fn create_share(
        &self,
        ids: &[&str],
        description: Option<&str>,
        expires: Option<i64>,
    ) -> Result<Vec<Share>, Error> {
        let params = Params::new()
            .with_all("id", ids)
            .with_opt("description", description)
            .with_opt("expires", expires);
        let wrapper: Option<SharesWrapper> = self
            .get_field_or_default("createShare", &params, "shares")
            .await?;
        Ok(wrapper.map(|w| w.share).unwrap_or_default())
    }

    /// Update an existing share.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/updateshare/>
    pub async fn update_share(
        &self,
        id: &str,
        description: Option<&str>,
        expires: Option<i64>,
    ) -> Result<(), Error> {
        let params = Params::new()
            .with("id", id)
            .with_opt("description", description)
            .with_opt("expires", expires);
        self.get_unit("updateShare", &params).await
    }

    /// Delete an existing share.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteshare/>
    pub async fn delete_share(&self, id: &str) -> Result<(), Error> {
        self.get_unit("deleteShare", &Params::new().with("id", id))
            .await
    }
}
