//! Podcast API endpoints.

use crate::Client;
use crate::data::{PodcastChannel, PodcastEpisode};
use crate::error::Error;

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
        let mut params = Vec::new();
        if let Some(ie) = include_episodes {
            params.push(("includeEpisodes", ie.to_string()));
        }
        if let Some(id_val) = id {
            params.push(("id", id_val.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getPodcasts", &param_refs).await?;
        let channels = data
            .get("podcasts")
            .and_then(|v| v.get("channel"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(channels)?)
    }

    /// Get the newest podcast episodes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getnewestpodcasts/>
    pub async fn get_newest_podcasts(
        &self,
        count: Option<i32>,
    ) -> Result<Vec<PodcastEpisode>, Error> {
        let mut params = Vec::new();
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("getNewestPodcasts", &param_refs).await?;
        let episodes = data
            .get("newestPodcasts")
            .and_then(|v| v.get("episode"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(episodes)?)
    }

    /// Get a specific podcast episode (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getpodcastepisode/>
    pub async fn get_podcast_episode(&self, id: &str) -> Result<PodcastEpisode, Error> {
        let data = self
            .get_response("getPodcastEpisode", &[("id", id)])
            .await?;
        let episode = data
            .get("podcastEpisode")
            .ok_or_else(|| Error::Parse("Missing 'podcastEpisode' in response".into()))?;
        Ok(serde_json::from_value(episode.clone())?)
    }

    /// Tell the server to check for new podcast episodes.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/refreshpodcasts/>
    pub async fn refresh_podcasts(&self) -> Result<(), Error> {
        self.get_response("refreshPodcasts", &[]).await?;
        Ok(())
    }

    /// Add a new podcast channel (by feed URL).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createpodcastchannel/>
    pub async fn create_podcast_channel(&self, url: &str) -> Result<(), Error> {
        self.get_response("createPodcastChannel", &[("url", url)])
            .await?;
        Ok(())
    }

    /// Delete a podcast channel.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deletepodcastchannel/>
    pub async fn delete_podcast_channel(&self, id: &str) -> Result<(), Error> {
        self.get_response("deletePodcastChannel", &[("id", id)])
            .await?;
        Ok(())
    }

    /// Delete a podcast episode.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deletepodcastepisode/>
    pub async fn delete_podcast_episode(&self, id: &str) -> Result<(), Error> {
        self.get_response("deletePodcastEpisode", &[("id", id)])
            .await?;
        Ok(())
    }

    /// Tell the server to download a podcast episode.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/downloadpodcastepisode/>
    pub async fn download_podcast_episode(&self, id: &str) -> Result<(), Error> {
        self.get_response("downloadPodcastEpisode", &[("id", id)])
            .await?;
        Ok(())
    }
}
