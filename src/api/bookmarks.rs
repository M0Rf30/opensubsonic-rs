// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Bookmarks API endpoints.

use crate::Client;
use crate::data::{Bookmark, PlayQueue, PlayQueueByIndex};
use crate::error::Error;
use crate::params::Params;
use serde::Deserialize;

/// Wrapper for the nested `bookmarks.bookmark` response shape.
#[derive(Deserialize, Default)]
struct BookmarksWrapper {
    #[serde(default)]
    bookmark: Vec<Bookmark>,
}

impl Client {
    /// Get all bookmarks.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getbookmarks/>
    pub async fn get_bookmarks(&self) -> Result<Vec<Bookmark>, Error> {
        let wrapper: Option<BookmarksWrapper> = self
            .get_field_or_default("getBookmarks", &Params::new(), "bookmarks")
            .await?;
        Ok(wrapper.map(|w| w.bookmark).unwrap_or_default())
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
        let params = Params::new()
            .with("id", id)
            .with("position", position)
            .with_opt("comment", comment);
        self.get_unit("createBookmark", &params).await
    }

    /// Delete a bookmark.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deletebookmark/>
    pub async fn delete_bookmark(&self, id: &str) -> Result<(), Error> {
        self.get_unit("deleteBookmark", &Params::new().with("id", id))
            .await
    }

    /// Get the play queue (current playlist/position saved by a client).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplayqueue/>
    pub async fn get_play_queue(&self) -> Result<PlayQueue, Error> {
        self.get_field("getPlayQueue", &Params::new(), "playQueue")
            .await
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
        let params = Params::new()
            .with_all("id", ids)
            .with_opt("current", current)
            .with_opt("position", position);
        self.get_unit("savePlayQueue", &params).await
    }

    /// Get the play queue by index (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getplayqueuebyindex/>
    pub async fn get_play_queue_by_index(&self) -> Result<PlayQueueByIndex, Error> {
        self.get_field("getPlayQueueByIndex", &Params::new(), "playQueueByIndex")
            .await
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
        let params = Params::new()
            .with_all("id", ids)
            .with_opt("currentIndex", current_index)
            .with_opt("position", position);
        self.get_unit("savePlayQueueByIndex", &params).await
    }
}
