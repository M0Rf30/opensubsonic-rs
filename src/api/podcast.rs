// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Podcast API endpoints.

use crate::Client;
use crate::data::{PodcastChannel, PodcastEpisode};
use crate::error::Error;
use crate::params::Params;
use serde::Deserialize;

/// Wrapper for the nested `podcasts.channel` response shape.
#[derive(Deserialize, Default)]
struct ChannelsWrapper {
    #[serde(default)]
    channel: Vec<PodcastChannel>,
}

/// Wrapper for the nested `newestPodcasts.episode` response shape.
#[derive(Deserialize, Default)]
struct NewestWrapper {
    #[serde(default)]
    episode: Vec<PodcastEpisode>,
}

impl Client {
    /// Get all podcast channels.
    ///
    /// If `include_episodes` is `false`, episodes are not included in the response.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getpodcasts/>
    pub async fn get_podcasts(
        &self,
        include_episodes: Option<bool>,
        id: Option<&str>,
    ) -> Result<Vec<PodcastChannel>, Error> {
        let params = Params::new()
            .with_opt("includeEpisodes", include_episodes)
            .with_opt("id", id);
        let wrapper: Option<ChannelsWrapper> = self
            .get_field_or_default("getPodcasts", &params, "podcasts")
            .await?;
        Ok(wrapper.map(|w| w.channel).unwrap_or_default())
    }

    /// Get the newest podcast episodes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getnewestpodcasts/>
    pub async fn get_newest_podcasts(
        &self,
        count: Option<i32>,
    ) -> Result<Vec<PodcastEpisode>, Error> {
        let params = Params::new().with_opt("count", count);
        let wrapper: Option<NewestWrapper> = self
            .get_field_or_default("getNewestPodcasts", &params, "newestPodcasts")
            .await?;
        Ok(wrapper.map(|w| w.episode).unwrap_or_default())
    }

    /// Get a specific podcast episode (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getpodcastepisode/>
    pub async fn get_podcast_episode(&self, id: &str) -> Result<PodcastEpisode, Error> {
        self.get_field(
            "getPodcastEpisode",
            &Params::new().with("id", id),
            "podcastEpisode",
        )
        .await
    }

    /// Tell the server to check for new podcast episodes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/refreshpodcasts/>
    pub async fn refresh_podcasts(&self) -> Result<(), Error> {
        self.get_unit("refreshPodcasts", &Params::new()).await
    }

    /// Add a new podcast channel (by feed URL).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createpodcastchannel/>
    pub async fn create_podcast_channel(&self, url: &str) -> Result<(), Error> {
        self.get_unit("createPodcastChannel", &Params::new().with("url", url))
            .await
    }

    /// Delete a podcast channel.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deletepodcastchannel/>
    pub async fn delete_podcast_channel(&self, id: &str) -> Result<(), Error> {
        self.get_unit("deletePodcastChannel", &Params::new().with("id", id))
            .await
    }

    /// Delete a podcast episode.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deletepodcastepisode/>
    pub async fn delete_podcast_episode(&self, id: &str) -> Result<(), Error> {
        self.get_unit("deletePodcastEpisode", &Params::new().with("id", id))
            .await
    }

    /// Tell the server to download a podcast episode.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/downloadpodcastepisode/>
    pub async fn download_podcast_episode(&self, id: &str) -> Result<(), Error> {
        self.get_unit("downloadPodcastEpisode", &Params::new().with("id", id))
            .await
    }
}
