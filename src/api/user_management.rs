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

/// Append the shared role-flag parameters in their wire order.
fn role_params(params: Params, flags: [(&'static str, Option<bool>); 13]) -> Params {
    flags.into_iter().fold(params, |p, (k, v)| p.with_opt(k, v))
}

/// Optional parameters for [`Client::create_user`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct CreateUserOptions {
    /// Value for `ldapAuthenticated` (authenticate via LDAP).
    pub ldap_authenticated: Option<bool>,
    /// Value for `adminRole` (administrator).
    pub admin_role: Option<bool>,
    /// Value for `settingsRole` (may change personal settings/password).
    pub settings_role: Option<bool>,
    /// Value for `streamRole` (may play files).
    pub stream_role: Option<bool>,
    /// Value for `jukeboxRole` (may control the jukebox).
    pub jukebox_role: Option<bool>,
    /// Value for `downloadRole` (may download files).
    pub download_role: Option<bool>,
    /// Value for `uploadRole` (may upload files).
    pub upload_role: Option<bool>,
    /// Value for `playlistRole` (may create and delete playlists).
    pub playlist_role: Option<bool>,
    /// Value for `coverArtRole` (may change cover art and tags).
    pub cover_art_role: Option<bool>,
    /// Value for `commentRole` (may create comments and ratings).
    pub comment_role: Option<bool>,
    /// Value for `podcastRole` (may administer podcasts).
    pub podcast_role: Option<bool>,
    /// Value for `shareRole` (may share files).
    pub share_role: Option<bool>,
    /// Value for `videoConversionRole` (may start video conversions).
    pub video_conversion_role: Option<bool>,
    /// Music folder IDs the user may access (`musicFolderId`, repeated).
    pub music_folder_ids: Vec<i64>,
}

impl CreateUserOptions {
    /// Create empty options (all server defaults).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `ldapAuthenticated` (authenticate via LDAP).
    #[must_use]
    pub fn ldap_authenticated(mut self, v: bool) -> Self {
        self.ldap_authenticated = Some(v);
        self
    }

    /// Set `adminRole` (administrator).
    #[must_use]
    pub fn admin_role(mut self, v: bool) -> Self {
        self.admin_role = Some(v);
        self
    }

    /// Set `settingsRole` (may change personal settings/password).
    #[must_use]
    pub fn settings_role(mut self, v: bool) -> Self {
        self.settings_role = Some(v);
        self
    }

    /// Set `streamRole` (may play files).
    #[must_use]
    pub fn stream_role(mut self, v: bool) -> Self {
        self.stream_role = Some(v);
        self
    }

    /// Set `jukeboxRole` (may control the jukebox).
    #[must_use]
    pub fn jukebox_role(mut self, v: bool) -> Self {
        self.jukebox_role = Some(v);
        self
    }

    /// Set `downloadRole` (may download files).
    #[must_use]
    pub fn download_role(mut self, v: bool) -> Self {
        self.download_role = Some(v);
        self
    }

    /// Set `uploadRole` (may upload files).
    #[must_use]
    pub fn upload_role(mut self, v: bool) -> Self {
        self.upload_role = Some(v);
        self
    }

    /// Set `playlistRole` (may create and delete playlists).
    #[must_use]
    pub fn playlist_role(mut self, v: bool) -> Self {
        self.playlist_role = Some(v);
        self
    }

    /// Set `coverArtRole` (may change cover art and tags).
    #[must_use]
    pub fn cover_art_role(mut self, v: bool) -> Self {
        self.cover_art_role = Some(v);
        self
    }

    /// Set `commentRole` (may create comments and ratings).
    #[must_use]
    pub fn comment_role(mut self, v: bool) -> Self {
        self.comment_role = Some(v);
        self
    }

    /// Set `podcastRole` (may administer podcasts).
    #[must_use]
    pub fn podcast_role(mut self, v: bool) -> Self {
        self.podcast_role = Some(v);
        self
    }

    /// Set `shareRole` (may share files).
    #[must_use]
    pub fn share_role(mut self, v: bool) -> Self {
        self.share_role = Some(v);
        self
    }

    /// Set `videoConversionRole` (may start video conversions).
    #[must_use]
    pub fn video_conversion_role(mut self, v: bool) -> Self {
        self.video_conversion_role = Some(v);
        self
    }

    /// Set the `musicFolderId` values (repeated key) the user may access.
    #[must_use]
    pub fn music_folder_ids(mut self, v: impl IntoIterator<Item = i64>) -> Self {
        self.music_folder_ids = v.into_iter().collect();
        self
    }
}

/// Optional parameters for [`Client::update_user`].
#[derive(Debug, Clone, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct UpdateUserOptions {
    /// New password (`password`).
    pub password: Option<String>,
    /// New email address (`email`).
    pub email: Option<String>,
    /// Value for `ldapAuthenticated` (authenticate via LDAP).
    pub ldap_authenticated: Option<bool>,
    /// Value for `adminRole` (administrator).
    pub admin_role: Option<bool>,
    /// Value for `settingsRole` (may change personal settings/password).
    pub settings_role: Option<bool>,
    /// Value for `streamRole` (may play files).
    pub stream_role: Option<bool>,
    /// Value for `jukeboxRole` (may control the jukebox).
    pub jukebox_role: Option<bool>,
    /// Value for `downloadRole` (may download files).
    pub download_role: Option<bool>,
    /// Value for `uploadRole` (may upload files).
    pub upload_role: Option<bool>,
    /// Value for `playlistRole` (may create and delete playlists).
    pub playlist_role: Option<bool>,
    /// Value for `coverArtRole` (may change cover art and tags).
    pub cover_art_role: Option<bool>,
    /// Value for `commentRole` (may create comments and ratings).
    pub comment_role: Option<bool>,
    /// Value for `podcastRole` (may administer podcasts).
    pub podcast_role: Option<bool>,
    /// Value for `shareRole` (may share files).
    pub share_role: Option<bool>,
    /// Value for `videoConversionRole` (may start video conversions).
    pub video_conversion_role: Option<bool>,
    /// Maximum bit rate in kbps (`maxBitRate`).
    pub max_bit_rate: Option<i32>,
    /// Music folder IDs the user may access (`musicFolderId`, repeated).
    pub music_folder_ids: Vec<i64>,
}

impl UpdateUserOptions {
    /// Create empty options (all server defaults).
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Set `password`.
    #[must_use]
    pub fn password(mut self, v: impl Into<String>) -> Self {
        self.password = Some(v.into());
        self
    }

    /// Set `email`.
    #[must_use]
    pub fn email(mut self, v: impl Into<String>) -> Self {
        self.email = Some(v.into());
        self
    }

    /// Set `ldapAuthenticated` (authenticate via LDAP).
    #[must_use]
    pub fn ldap_authenticated(mut self, v: bool) -> Self {
        self.ldap_authenticated = Some(v);
        self
    }

    /// Set `adminRole` (administrator).
    #[must_use]
    pub fn admin_role(mut self, v: bool) -> Self {
        self.admin_role = Some(v);
        self
    }

    /// Set `settingsRole` (may change personal settings/password).
    #[must_use]
    pub fn settings_role(mut self, v: bool) -> Self {
        self.settings_role = Some(v);
        self
    }

    /// Set `streamRole` (may play files).
    #[must_use]
    pub fn stream_role(mut self, v: bool) -> Self {
        self.stream_role = Some(v);
        self
    }

    /// Set `jukeboxRole` (may control the jukebox).
    #[must_use]
    pub fn jukebox_role(mut self, v: bool) -> Self {
        self.jukebox_role = Some(v);
        self
    }

    /// Set `downloadRole` (may download files).
    #[must_use]
    pub fn download_role(mut self, v: bool) -> Self {
        self.download_role = Some(v);
        self
    }

    /// Set `uploadRole` (may upload files).
    #[must_use]
    pub fn upload_role(mut self, v: bool) -> Self {
        self.upload_role = Some(v);
        self
    }

    /// Set `playlistRole` (may create and delete playlists).
    #[must_use]
    pub fn playlist_role(mut self, v: bool) -> Self {
        self.playlist_role = Some(v);
        self
    }

    /// Set `coverArtRole` (may change cover art and tags).
    #[must_use]
    pub fn cover_art_role(mut self, v: bool) -> Self {
        self.cover_art_role = Some(v);
        self
    }

    /// Set `commentRole` (may create comments and ratings).
    #[must_use]
    pub fn comment_role(mut self, v: bool) -> Self {
        self.comment_role = Some(v);
        self
    }

    /// Set `podcastRole` (may administer podcasts).
    #[must_use]
    pub fn podcast_role(mut self, v: bool) -> Self {
        self.podcast_role = Some(v);
        self
    }

    /// Set `shareRole` (may share files).
    #[must_use]
    pub fn share_role(mut self, v: bool) -> Self {
        self.share_role = Some(v);
        self
    }

    /// Set `videoConversionRole` (may start video conversions).
    #[must_use]
    pub fn video_conversion_role(mut self, v: bool) -> Self {
        self.video_conversion_role = Some(v);
        self
    }

    /// Set `maxBitRate` (kbps).
    #[must_use]
    pub fn max_bit_rate(mut self, v: i32) -> Self {
        self.max_bit_rate = Some(v);
        self
    }

    /// Set the `musicFolderId` values (repeated key) the user may access.
    #[must_use]
    pub fn music_folder_ids(mut self, v: impl IntoIterator<Item = i64>) -> Self {
        self.music_folder_ids = v.into_iter().collect();
        self
    }
}

fn create_user_params(
    username: &str,
    password: &str,
    email: &str,
    options: &CreateUserOptions,
) -> Params {
    let params = Params::new()
        .with("username", username)
        .with("password", password)
        .with("email", email);
    role_params(
        params,
        [
            ("ldapAuthenticated", options.ldap_authenticated),
            ("adminRole", options.admin_role),
            ("settingsRole", options.settings_role),
            ("streamRole", options.stream_role),
            ("jukeboxRole", options.jukebox_role),
            ("downloadRole", options.download_role),
            ("uploadRole", options.upload_role),
            ("playlistRole", options.playlist_role),
            ("coverArtRole", options.cover_art_role),
            ("commentRole", options.comment_role),
            ("podcastRole", options.podcast_role),
            ("shareRole", options.share_role),
            ("videoConversionRole", options.video_conversion_role),
        ],
    )
    .with_all("musicFolderId", options.music_folder_ids.iter().copied())
}

fn update_user_params(username: &str, options: &UpdateUserOptions) -> Params {
    let params = Params::new()
        .with("username", username)
        .with_opt("password", options.password.as_deref())
        .with_opt("email", options.email.as_deref());
    role_params(
        params,
        [
            ("ldapAuthenticated", options.ldap_authenticated),
            ("adminRole", options.admin_role),
            ("settingsRole", options.settings_role),
            ("streamRole", options.stream_role),
            ("jukeboxRole", options.jukebox_role),
            ("downloadRole", options.download_role),
            ("uploadRole", options.upload_role),
            ("playlistRole", options.playlist_role),
            ("coverArtRole", options.cover_art_role),
            ("commentRole", options.comment_role),
            ("podcastRole", options.podcast_role),
            ("shareRole", options.share_role),
            ("videoConversionRole", options.video_conversion_role),
        ],
    )
    .with_opt("maxBitRate", options.max_bit_rate)
    .with_all("musicFolderId", options.music_folder_ids.iter().copied())
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
    /// Optional role flags and music folders are set via [`CreateUserOptions`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/createuser/>
    pub async fn create_user(
        &self,
        username: &str,
        password: &str,
        email: &str,
        options: &CreateUserOptions,
    ) -> Result<(), Error> {
        let params = create_user_params(username, password, email, options);
        self.get_unit("createUser", &params).await
    }

    /// Update an existing user (admin only).
    ///
    /// Every field to change is set via [`UpdateUserOptions`].
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/updateuser/>
    pub async fn update_user(
        &self,
        username: &str,
        options: &UpdateUserOptions,
    ) -> Result<(), Error> {
        let params = update_user_params(username, options);
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Auth;

    fn tail(endpoint: &str, params: &Params) -> Vec<String> {
        let c = Client::new("https://h", Auth::api_key("k")).unwrap();
        c.endpoint_url(endpoint, params)
            .unwrap()
            .query_pairs()
            .filter(|(k, _)| !matches!(k.as_ref(), "v" | "c" | "f" | "apiKey"))
            .map(|(k, v)| format!("{k}={v}"))
            .collect()
    }

    #[test]
    fn create_user_query_full() {
        let o = CreateUserOptions::new()
            .ldap_authenticated(true)
            .admin_role(false)
            .settings_role(true)
            .stream_role(true)
            .jukebox_role(false)
            .download_role(true)
            .upload_role(false)
            .playlist_role(true)
            .cover_art_role(false)
            .comment_role(true)
            .podcast_role(false)
            .share_role(true)
            .video_conversion_role(false)
            .music_folder_ids([1, 2]);
        let p = create_user_params("bob", "pw", "b@x", &o);
        assert_eq!(
            tail("createUser", &p),
            [
                "username=bob",
                "password=pw",
                "email=b@x",
                "ldapAuthenticated=true",
                "adminRole=false",
                "settingsRole=true",
                "streamRole=true",
                "jukeboxRole=false",
                "downloadRole=true",
                "uploadRole=false",
                "playlistRole=true",
                "coverArtRole=false",
                "commentRole=true",
                "podcastRole=false",
                "shareRole=true",
                "videoConversionRole=false",
                "musicFolderId=1",
                "musicFolderId=2",
            ]
        );
    }

    #[test]
    fn create_user_defaults_only_required() {
        let p = create_user_params("bob", "pw", "b@x", &CreateUserOptions::new());
        assert_eq!(
            tail("createUser", &p),
            ["username=bob", "password=pw", "email=b@x"]
        );
    }

    #[test]
    fn update_user_query_full() {
        let o = UpdateUserOptions::new()
            .password("np")
            .email("e@x")
            .admin_role(true)
            .share_role(false)
            .max_bit_rate(192)
            .music_folder_ids([7]);
        let p = update_user_params("bob", &o);
        assert_eq!(
            tail("updateUser", &p),
            [
                "username=bob",
                "password=np",
                "email=e@x",
                "adminRole=true",
                "shareRole=false",
                "maxBitRate=192",
                "musicFolderId=7",
            ]
        );
    }
}
