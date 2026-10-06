// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Internet Radio API endpoints.

use crate::Client;
use crate::data::InternetRadioStation;
use crate::error::Error;
use crate::params::Params;
use serde::Deserialize;

/// Wrapper for the nested `internetRadioStations.internetRadioStation` shape.
#[derive(Deserialize, Default)]
struct StationsWrapper {
    #[serde(default, rename = "internetRadioStation")]
    internet_radio_station: Vec<InternetRadioStation>,
}

impl Client {
    /// Get all internet radio stations.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getinternetradiostations/>
    pub async fn get_internet_radio_stations(&self) -> Result<Vec<InternetRadioStation>, Error> {
        let wrapper: Option<StationsWrapper> = self
            .get_field_or_default(
                "getInternetRadioStations",
                &Params::new(),
                "internetRadioStations",
            )
            .await?;
        Ok(wrapper
            .map(|w| w.internet_radio_station)
            .unwrap_or_default())
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
        let params = Params::new()
            .with("streamUrl", stream_url)
            .with("name", name)
            .with_opt("homepageUrl", home_page_url);
        self.get_unit("createInternetRadioStation", &params).await
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
        let params = Params::new()
            .with("id", id)
            .with("streamUrl", stream_url)
            .with("name", name)
            .with_opt("homepageUrl", home_page_url);
        self.get_unit("updateInternetRadioStation", &params).await
    }

    /// Delete an internet radio station.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteinternetradiostation/>
    pub async fn delete_internet_radio_station(&self, id: &str) -> Result<(), Error> {
        self.get_unit("deleteInternetRadioStation", &Params::new().with("id", id))
            .await
    }
}
