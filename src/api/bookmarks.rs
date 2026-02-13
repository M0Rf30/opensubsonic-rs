//! Bookmarks API endpoints.

use crate::data::{Bookmark, PlayQueue, PlayQueueByIndex};
use crate::error::Error;
use crate::Client;

impl Client {
    /// Get all bookmarks.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getbookmarks/>
    pub async fn get_bookmarks(&self) -> Result<Vec<Bookmark>, Error> {
        let data = self.get_response("getBookmarks", &[]).await?;
        let bookmarks = data
            .get("bookmarks")
            .and_then(|v| v.get("bookmark"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(bookmarks)?)
    }

    /// Create or update a bookmark.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createbookmark/>
    pub async fn create_bookmark(
        &self,
        id: &str,
        position: i64,
        comment: Option<&str>,
    ) -> Result<(), Error> {
        let pos_str = position.to_string();
        let mut params = vec![("id", id), ("position", &pos_str)];
        if let Some(c) = comment {
            params.push(("comment", c));
        }
        self.get_response("createBookmark", &params).await?;
        Ok(())
    }

    /// Delete a bookmark.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deletebookmark/>
    pub async fn delete_bookmark(&self, id: &str) -> Result<(), Error> {
        self.get_response("deleteBookmark", &[("id", id)]).await?;
        Ok(())
    }

    /// Get the play queue (current playlist/position saved by a client).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplayqueue/>
    pub async fn get_play_queue(&self) -> Result<PlayQueue, Error> {
        let data = self.get_response("getPlayQueue", &[]).await?;
        let queue = data
            .get("playQueue")
            .ok_or_else(|| Error::Parse("Missing 'playQueue' in response".into()))?;
        Ok(serde_json::from_value(queue.clone())?)
    }

    /// Save the play queue.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/saveplayqueue/>
    pub async fn save_play_queue(
        &self,
        ids: &[&str],
        current: Option<&str>,
        position: Option<i64>,
    ) -> Result<(), Error> {
        let mut params = Vec::new();
        for id in ids {
            params.push(("id", id.to_string()));
        }
        if let Some(c) = current {
            params.push(("current", c.to_string()));
        }
        if let Some(p) = position {
            params.push(("position", p.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("savePlayQueue", &param_refs).await?;
        Ok(())
    }

    /// Get the play queue by index (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplayqueuebyindex/>
    pub async fn get_play_queue_by_index(&self) -> Result<PlayQueueByIndex, Error> {
        let data = self.get_response("getPlayQueueByIndex", &[]).await?;
        let queue = data
            .get("playQueueByIndex")
            .ok_or_else(|| {
                Error::Parse("Missing 'playQueueByIndex' in response".into())
            })?;
        Ok(serde_json::from_value(queue.clone())?)
    }

    /// Save the play queue by index (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/saveplayqueuebyindex/>
    pub async fn save_play_queue_by_index(
        &self,
        ids: &[&str],
        current_index: Option<i32>,
        position: Option<i64>,
    ) -> Result<(), Error> {
        let mut params = Vec::new();
        for id in ids {
            params.push(("id", id.to_string()));
        }
        if let Some(ci) = current_index {
            params.push(("currentIndex", ci.to_string()));
        }
        if let Some(p) = position {
            params.push(("position", p.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("savePlayQueueByIndex", &param_refs)
            .await?;
        Ok(())
    }
}
