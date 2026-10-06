// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! User Management API endpoints.

use crate::Client;
use crate::data::User;
use crate::error::Error;
use crate::params::Params;

/// Private wrapper for `{ "user": [...] }` containers.
#[derive(Default, serde::Deserialize)]
#[serde(default)]
struct UserList {
    user: Vec<User>,
}

impl Client {
    /// Get details about a specific user.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getuser/>
    pub async fn get_user(&self, username: &str) -> Result<User, Error> {
        let params = Params::new().with("username", username);
        self.get_field("getUser", &params, "user").await
    }

    /// Get details about all users (admin only).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getusers/>
    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let w: UserList = self
            .get_field_or_default("getUsers", &Params::new(), "users")
            .await?;
        Ok(w.user)
    }

    /// Create a new user (admin only).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createuser/>
    #[allow(clippy::too_many_arguments)]
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
        ldap_authenticated: Option<bool>,
        admin_role: Option<bool>,
        settings_role: Option<bool>,
        stream_role: Option<bool>,
        jukebox_role: Option<bool>,
        download_role: Option<bool>,
        upload_role: Option<bool>,
        playlist_role: Option<bool>,
        cover_art_role: Option<bool>,
        comment_role: Option<bool>,
        podcast_role: Option<bool>,
        share_role: Option<bool>,
        video_conversion_role: Option<bool>,
        music_folder_ids: &[i64],
    ) -> Result<(), Error> {
        let params = Params::new()
            .with("username", username)
            .with("password", password)
            .with("email", email)
            .with_opt("ldapAuthenticated", ldap_authenticated)
            .with_opt("adminRole", admin_role)
            .with_opt("settingsRole", settings_role)
            .with_opt("streamRole", stream_role)
            .with_opt("jukeboxRole", jukebox_role)
            .with_opt("downloadRole", download_role)
            .with_opt("uploadRole", upload_role)
            .with_opt("playlistRole", playlist_role)
            .with_opt("coverArtRole", cover_art_role)
            .with_opt("commentRole", comment_role)
            .with_opt("podcastRole", podcast_role)
            .with_opt("shareRole", share_role)
            .with_opt("videoConversionRole", video_conversion_role)
            .with_all("musicFolderId", music_folder_ids.iter().copied());
        self.get_unit("createUser", &params).await
    }

    /// Update an existing user (admin only).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/updateuser/>
    #[allow(clippy::too_many_arguments)]
    pub async fn update_user(
        &self,
        username: &str,
        password: Option<&str>,
        email: Option<&str>,
        ldap_authenticated: Option<bool>,
        admin_role: Option<bool>,
        settings_role: Option<bool>,
        stream_role: Option<bool>,
        jukebox_role: Option<bool>,
        download_role: Option<bool>,
        upload_role: Option<bool>,
        playlist_role: Option<bool>,
        cover_art_role: Option<bool>,
        comment_role: Option<bool>,
        podcast_role: Option<bool>,
        share_role: Option<bool>,
        video_conversion_role: Option<bool>,
        max_bit_rate: Option<i32>,
        music_folder_ids: &[i64],
    ) -> Result<(), Error> {
        let params = Params::new()
            .with("username", username)
            .with_opt("password", password)
            .with_opt("email", email)
            .with_opt("ldapAuthenticated", ldap_authenticated)
            .with_opt("adminRole", admin_role)
            .with_opt("settingsRole", settings_role)
            .with_opt("streamRole", stream_role)
            .with_opt("jukeboxRole", jukebox_role)
            .with_opt("downloadRole", download_role)
            .with_opt("uploadRole", upload_role)
            .with_opt("playlistRole", playlist_role)
            .with_opt("coverArtRole", cover_art_role)
            .with_opt("commentRole", comment_role)
            .with_opt("podcastRole", podcast_role)
            .with_opt("shareRole", share_role)
            .with_opt("videoConversionRole", video_conversion_role)
            .with_opt("maxBitRate", max_bit_rate)
            .with_all("musicFolderId", music_folder_ids.iter().copied());
        self.get_unit("updateUser", &params).await
    }

    /// Delete a user (admin only).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteuser/>
    pub async fn delete_user(&self, username: &str) -> Result<(), Error> {
        let params = Params::new().with("username", username);
        self.get_unit("deleteUser", &params).await
    }

    /// Change a user's password.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/changepassword/>
    pub async fn change_password(&self, username: &str, password: &str) -> Result<(), Error> {
        let params = Params::new()
            .with("username", username)
            .with("password", password);
        self.get_unit("changePassword", &params).await
    }
}
