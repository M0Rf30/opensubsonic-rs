//! Internet Radio API endpoints.

use crate::Client;
use crate::data::InternetRadioStation;
use crate::error::Error;

impl Client {
    /// Get all internet radio stations.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getinternetradiostations/>
    pub async fn get_internet_radio_stations(&self) -> Result<Vec<InternetRadioStation>, Error> {
        let data = self.get_response("getInternetRadioStations", &[]).await?;
        let stations = data
            .get("internetRadioStations")
            .and_then(|v| v.get("internetRadioStation"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(stations)?)
    }

    /// Create a new internet radio station.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createinternetradiostation/>
    pub async fn create_internet_radio_station(
        &self,
        stream_url: &str,
        name: &str,
        home_page_url: Option<&str>,
    ) -> Result<(), Error> {
        let mut params = vec![("streamUrl", stream_url), ("name", name)];
        if let Some(hp) = home_page_url {
            params.push(("homepageUrl", hp));
        }
        self.get_response("createInternetRadioStation", &params)
            .await?;
        Ok(())
    }

    /// Update an existing internet radio station.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/updateinternetradiostation/>
    pub async fn update_internet_radio_station(
        &self,
        id: &str,
        stream_url: &str,
        name: &str,
        home_page_url: Option<&str>,
    ) -> Result<(), Error> {
        let mut params = vec![("id", id), ("streamUrl", stream_url), ("name", name)];
        if let Some(hp) = home_page_url {
            params.push(("homepageUrl", hp));
        }
        self.get_response("updateInternetRadioStation", &params)
            .await?;
        Ok(())
    }

    /// Delete an internet radio station.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteinternetradiostation/>
    pub async fn delete_internet_radio_station(&self, id: &str) -> Result<(), Error> {
        self.get_response("deleteInternetRadioStation", &[("id", id)])
            .await?;
        Ok(())
    }
}
