//! Sharing API endpoints.

use crate::Client;
use crate::data::Share;
use crate::error::Error;

impl Client {
    /// Get all shares.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getshares/>
    pub async fn get_shares(&self) -> Result<Vec<Share>, Error> {
        let data = self.get_response("getShares", &[]).await?;
        let shares = data
            .get("shares")
            .and_then(|v| v.get("share"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(shares)?)
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
        let mut params = Vec::new();
        for id in ids {
            params.push(("id", id.to_string()));
        }
        if let Some(d) = description {
            params.push(("description", d.to_string()));
        }
        if let Some(e) = expires {
            params.push(("expires", e.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("createShare", &param_refs).await?;
        let shares = data
            .get("shares")
            .and_then(|v| v.get("share"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(shares)?)
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
        let mut params = vec![("id", id.to_string())];
        if let Some(d) = description {
            params.push(("description", d.to_string()));
        }
        if let Some(e) = expires {
            params.push(("expires", e.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("updateShare", &param_refs).await?;
        Ok(())
    }

    /// Delete an existing share.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteshare/>
    pub async fn delete_share(&self, id: &str) -> Result<(), Error> {
        self.get_response("deleteShare", &[("id", id)]).await?;
        Ok(())
    }
}
