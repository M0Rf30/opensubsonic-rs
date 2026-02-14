//! Media Annotation API endpoints.

use crate::Client;
use crate::error::Error;

impl Client {
    /// Star songs, albums, or artists.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/star/>
    pub async fn star(
        &self,
        ids: &[&str],
        album_ids: &[&str],
        artist_ids: &[&str],
    ) -> Result<(), Error> {
        let mut params = Vec::new();
        for id in ids {
            params.push(("id", id.to_string()));
        }
        for id in album_ids {
            params.push(("albumId", id.to_string()));
        }
        for id in artist_ids {
            params.push(("artistId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("star", &param_refs).await?;
        Ok(())
    }

    /// Unstar songs, albums, or artists.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/unstar/>
    pub async fn unstar(
        &self,
        ids: &[&str],
        album_ids: &[&str],
        artist_ids: &[&str],
    ) -> Result<(), Error> {
        let mut params = Vec::new();
        for id in ids {
            params.push(("id", id.to_string()));
        }
        for id in album_ids {
            params.push(("albumId", id.to_string()));
        }
        for id in artist_ids {
            params.push(("artistId", id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("unstar", &param_refs).await?;
        Ok(())
    }

    /// Set the rating of a song, album, or artist.
    ///
    /// A rating of 0 removes the rating.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/setrating/>
    pub async fn set_rating(&self, id: &str, rating: i32) -> Result<(), Error> {
        let rating_str = rating.to_string();
        self.get_response("setRating", &[("id", id), ("rating", &rating_str)])
            .await?;
        Ok(())
    }

    /// Register a song as played (scrobble).
    ///
    /// If `submission` is `false`, this is a "now playing" notification rather than a scrobble.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/scrobble/>
    pub async fn scrobble(
        &self,
        id: &str,
        time: Option<i64>,
        submission: Option<bool>,
    ) -> Result<(), Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(t) = time {
            params.push(("time", t.to_string()));
        }
        if let Some(s) = submission {
            params.push(("submission", s.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("scrobble", &param_refs).await?;
        Ok(())
    }
}
