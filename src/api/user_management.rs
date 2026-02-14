//! User Management API endpoints.

use crate::Client;
use crate::data::User;
use crate::error::Error;

impl Client {
    /// Get details about a specific user.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getuser/>
    pub async fn get_user(&self, username: &str) -> Result<User, Error> {
        let data = self
            .get_response("getUser", &[("username", username)])
            .await?;
        let user = data
            .get("user")
            .ok_or_else(|| Error::Parse("Missing 'user' in response".into()))?;
        Ok(serde_json::from_value(user.clone())?)
    }

    /// Get details about all users (admin only).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getusers/>
    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let data = self.get_response("getUsers", &[]).await?;
        let users = data
            .get("users")
            .and_then(|v| v.get("user"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(users)?)
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
        let mut params = vec![
            ("username", username.to_string()),
            ("password", password.to_string()),
            ("email", email.to_string()),
        ];
        if let Some(v) = ldap_authenticated {
            params.push(("ldapAuthenticated", v.to_string()));
        }
        if let Some(v) = admin_role {
            params.push(("adminRole", v.to_string()));
        }
        if let Some(v) = settings_role {
            params.push(("settingsRole", v.to_string()));
        }
        if let Some(v) = stream_role {
            params.push(("streamRole", v.to_string()));
        }
        if let Some(v) = jukebox_role {
            params.push(("jukeboxRole", v.to_string()));
        }
        if let Some(v) = download_role {
            params.push(("downloadRole", v.to_string()));
        }
        if let Some(v) = upload_role {
            params.push(("uploadRole", v.to_string()));
        }
        if let Some(v) = playlist_role {
            params.push(("playlistRole", v.to_string()));
        }
        if let Some(v) = cover_art_role {
            params.push(("coverArtRole", v.to_string()));
        }
        if let Some(v) = comment_role {
            params.push(("commentRole", v.to_string()));
        }
        if let Some(v) = podcast_role {
            params.push(("podcastRole", v.to_string()));
        }
        if let Some(v) = share_role {
            params.push(("shareRole", v.to_string()));
        }
        if let Some(v) = video_conversion_role {
            params.push(("videoConversionRole", v.to_string()));
        }
        for folder_id in music_folder_ids {
            params.push(("musicFolderId", folder_id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("createUser", &param_refs).await?;
        Ok(())
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
        let mut params = vec![("username", username.to_string())];
        if let Some(v) = password {
            params.push(("password", v.to_string()));
        }
        if let Some(v) = email {
            params.push(("email", v.to_string()));
        }
        if let Some(v) = ldap_authenticated {
            params.push(("ldapAuthenticated", v.to_string()));
        }
        if let Some(v) = admin_role {
            params.push(("adminRole", v.to_string()));
        }
        if let Some(v) = settings_role {
            params.push(("settingsRole", v.to_string()));
        }
        if let Some(v) = stream_role {
            params.push(("streamRole", v.to_string()));
        }
        if let Some(v) = jukebox_role {
            params.push(("jukeboxRole", v.to_string()));
        }
        if let Some(v) = download_role {
            params.push(("downloadRole", v.to_string()));
        }
        if let Some(v) = upload_role {
            params.push(("uploadRole", v.to_string()));
        }
        if let Some(v) = playlist_role {
            params.push(("playlistRole", v.to_string()));
        }
        if let Some(v) = cover_art_role {
            params.push(("coverArtRole", v.to_string()));
        }
        if let Some(v) = comment_role {
            params.push(("commentRole", v.to_string()));
        }
        if let Some(v) = podcast_role {
            params.push(("podcastRole", v.to_string()));
        }
        if let Some(v) = share_role {
            params.push(("shareRole", v.to_string()));
        }
        if let Some(v) = video_conversion_role {
            params.push(("videoConversionRole", v.to_string()));
        }
        if let Some(v) = max_bit_rate {
            params.push(("maxBitRate", v.to_string()));
        }
        for folder_id in music_folder_ids {
            params.push(("musicFolderId", folder_id.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_response("updateUser", &param_refs).await?;
        Ok(())
    }

    /// Delete a user (admin only).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/deleteuser/>
    pub async fn delete_user(&self, username: &str) -> Result<(), Error> {
        self.get_response("deleteUser", &[("username", username)])
            .await?;
        Ok(())
    }

    /// Change a user's password.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/changepassword/>
    pub async fn change_password(&self, username: &str, password: &str) -> Result<(), Error> {
        self.get_response(
            "changePassword",
            &[("username", username), ("password", password)],
        )
        .await?;
        Ok(())
    }
}
