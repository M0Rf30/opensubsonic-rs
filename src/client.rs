//! Core HTTP client for the Subsonic / OpenSubsonic REST API.

use serde::Deserialize;
use url::Url;

use crate::auth::Auth;
use crate::error::{Error, SubsonicApiError};

/// Default Subsonic REST API protocol version.
const DEFAULT_API_VERSION: &str = "1.16.1";
/// Default client identifier sent with every request.
const DEFAULT_CLIENT_NAME: &str = "opensubsonic-rs";

/// An async client for the Subsonic / OpenSubsonic REST API.
///
/// Construct via [`Client::new`] and optionally customise with the builder methods
/// ([`Client::with_client_name`], [`Client::with_api_version`], [`Client::with_http_client`]).
///
/// API endpoint methods are provided by the [`crate::api`] module and are available as methods
/// on this struct via extension traits.
#[derive(Debug, Clone)]
pub struct Client {
    /// Server base URL (e.g. `https://music.example.com`).
    base_url: Url,
    /// Subsonic user name.
    username: String,
    /// Authentication configuration.
    auth: Auth,
    /// Client application identifier sent as the `c` parameter.
    client_name: String,
    /// Subsonic REST protocol version sent as the `v` parameter.
    api_version: String,
    /// Underlying HTTP client (reused across requests for connection pooling).
    pub(crate) http: reqwest::Client,
}

// ── Constructor & builders ──────────────────────────────────────────────────

impl Client {
    /// Create a new Subsonic API client.
    ///
    /// # Arguments
    /// * `base_url` — The server base URL, e.g. `"https://music.example.com"`.
    /// * `username` — The Subsonic user name.
    /// * `auth` — Authentication method (see [`Auth::token`] / [`Auth::plain`]).
    ///
    /// # Errors
    /// Returns [`Error::Url`] if `base_url` cannot be parsed.
    pub fn new(base_url: &str, username: &str, auth: Auth) -> Result<Self, Error> {
        let base_url = Url::parse(base_url)?;
        Ok(Self {
            base_url,
            username: username.to_owned(),
            auth,
            client_name: DEFAULT_CLIENT_NAME.to_owned(),
            api_version: DEFAULT_API_VERSION.to_owned(),
            http: reqwest::Client::new(),
        })
    }

    /// Override the client application name sent as the `c` parameter.
    #[must_use]
    pub fn with_client_name(mut self, name: &str) -> Self {
        self.client_name = name.to_owned();
        self
    }

    /// Override the Subsonic REST protocol version sent as the `v` parameter.
    #[must_use]
    pub fn with_api_version(mut self, version: &str) -> Self {
        self.api_version = version.to_owned();
        self
    }

    /// Inject a custom [`reqwest::Client`] (e.g. with custom timeouts or TLS settings).
    #[must_use]
    pub fn with_http_client(mut self, client: reqwest::Client) -> Self {
        self.http = client;
        self
    }

    /// Accept invalid TLS certificates (self-signed, expired, wrong hostname).
    ///
    /// **WARNING**: This disables TLS certificate verification and should only
    /// be used in trusted network environments (e.g. Tailscale, local LAN).
    ///
    /// # Errors
    /// Returns [`Error::Http`] if the HTTP client cannot be built.
    pub fn with_danger_accept_invalid_certs(mut self) -> Result<Self, Error> {
        self.http = reqwest::Client::builder()
            .danger_accept_invalid_certs(true)
            .build()?;
        Ok(self)
    }
}

// ── Internal transport helpers ──────────────────────────────────────────────

impl Client {
    /// Build a full request URL for the given API endpoint.
    ///
    /// The resulting URL has the form:
    /// ```text
    /// {base_url}/rest/{endpoint}?u=…&t=…&s=…&v=…&c=…&f=json&{extra params}
    /// ```
    pub(crate) fn build_url(&self, endpoint: &str, params: &[(&str, &str)]) -> Result<Url, Error> {
        // Append `/rest/{endpoint}` to the existing base URL path.
        // We cannot use `Url::join()` because it replaces the last path
        // segment instead of appending — e.g. joining `rest/ping` on
        // `https://host/music` would incorrectly produce `https://host/rest/ping`
        // instead of the desired `https://host/music/rest/ping`.
        let mut url = self.base_url.clone();
        {
            let mut path = url.path().to_owned();
            if !path.ends_with('/') {
                path.push('/');
            }
            path.push_str("rest/");
            path.push_str(endpoint);
            url.set_path(&path);
        }

        {
            let mut query = url.query_pairs_mut();
            // Username.
            query.append_pair("u", &self.username);
            // Auth params (token+salt or password).
            for (k, v) in self.auth.params() {
                query.append_pair(k, &v);
            }
            // Protocol version & client id.
            query.append_pair("v", &self.api_version);
            query.append_pair("c", &self.client_name);
            // Always request JSON.
            query.append_pair("f", "json");
            // Endpoint-specific params.
            for &(k, v) in params {
                query.append_pair(k, v);
            }
        }

        Ok(url)
    }

    /// Perform a GET request to `endpoint`, parse the JSON wrapper, check for errors,
    /// and return the inner data map.
    ///
    /// The returned [`serde_json::Map`] contains all fields from `subsonic-response`
    /// *except* the standard envelope fields (`status`, `version`, `type`, `serverVersion`,
    /// `openSubsonic`, `error`).
    pub(crate) async fn get_response(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<serde_json::Map<String, serde_json::Value>, Error> {
        let url = self.build_url(endpoint, params)?;
        log::debug!("GET {url}");

        let resp = self.http.get(url).send().await?.error_for_status()?;
        let text = resp.text().await?;

        let wrapper: SubsonicResponseWrapper =
            serde_json::from_str(&text).map_err(|e| Error::Parse(format!("{e}: {text}")))?;
        let inner = wrapper.response;

        if inner.status != "ok" {
            let api_err = inner.error.map_or_else(
                || SubsonicApiError {
                    code: 0,
                    message: "Unknown API error (status != ok but no error object)".into(),
                },
                |e| SubsonicApiError {
                    code: e.code,
                    message: e.message.unwrap_or_default(),
                },
            );
            return Err(Error::Api(api_err));
        }

        Ok(inner.data)
    }

    /// Perform a GET request and return the raw response bytes.
    ///
    /// Useful for binary endpoints such as `stream`, `getCoverArt`, `getAvatar`, and `download`.
    ///
    /// If the server returns a JSON error body instead of binary data the method will
    /// detect the `application/json` content type and parse it as an API error.
    pub(crate) async fn get_bytes(
        &self,
        endpoint: &str,
        params: &[(&str, &str)],
    ) -> Result<bytes::Bytes, Error> {
        let url = self.build_url(endpoint, params)?;
        log::debug!("GET (bytes) {url}");

        let resp = self.http.get(url).send().await?.error_for_status()?;

        // Some servers return a JSON error even on binary endpoints.
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        if content_type.contains("application/json") || content_type.contains("text/json") {
            // Likely an error response — try to parse it.
            let text = resp.text().await?;
            let wrapper: SubsonicResponseWrapper =
                serde_json::from_str(&text).map_err(|e| Error::Parse(format!("{e}: {text}")))?;
            let inner = wrapper.response;
            if inner.status != "ok" {
                let api_err = inner.error.map_or_else(
                    || SubsonicApiError {
                        code: 0,
                        message: "Unknown API error on binary endpoint".into(),
                    },
                    |e| SubsonicApiError {
                        code: e.code,
                        message: e.message.unwrap_or_default(),
                    },
                );
                return Err(Error::Api(api_err));
            }
            // If status is ok but content-type is JSON, something unexpected happened.
            return Err(Error::Parse(
                "Expected binary response but got JSON with status=ok".into(),
            ));
        }

        Ok(resp.bytes().await?)
    }
}

// ── Response deserialization helpers ────────────────────────────────────────

/// Top-level JSON wrapper returned by all Subsonic REST API endpoints.
#[derive(Deserialize)]
struct SubsonicResponseWrapper {
    #[serde(rename = "subsonic-response")]
    response: SubsonicResponseInner,
}

/// The contents of the `"subsonic-response"` object.
#[derive(Deserialize)]
struct SubsonicResponseInner {
    /// `"ok"` or `"failed"`.
    status: String,
    /// Protocol version echoed by the server.
    #[serde(default)]
    #[allow(dead_code)]
    version: Option<String>,
    /// Server implementation type (OpenSubsonic extension, e.g. `"navidrome"`).
    #[serde(rename = "type", default)]
    #[allow(dead_code)]
    server_type: Option<String>,
    /// Server software version (OpenSubsonic extension).
    #[serde(rename = "serverVersion", default)]
    #[allow(dead_code)]
    server_version: Option<String>,
    /// Whether the server supports OpenSubsonic extensions.
    #[serde(rename = "openSubsonic", default)]
    #[allow(dead_code)]
    open_subsonic: Option<bool>,
    /// Present only when `status == "failed"`.
    error: Option<ApiErrorResponse>,
    /// All remaining fields (the actual endpoint-specific data).
    #[serde(flatten)]
    data: serde_json::Map<String, serde_json::Value>,
}

/// Subsonic API error object embedded in the response.
#[derive(Deserialize)]
struct ApiErrorResponse {
    code: i32,
    message: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Auth;

    #[test]
    fn build_url_contains_required_params() {
        let client =
            Client::new("https://music.example.com", "admin", Auth::token("pass")).unwrap();
        let url = client.build_url("ping", &[]).unwrap();
        let query: String = url.query().unwrap().to_string();

        assert_eq!(url.path(), "/rest/ping");
        assert!(query.contains("u=admin"));
        assert!(query.contains("v=1.16.1"));
        assert!(query.contains("c=opensubsonic-rs"));
        assert!(query.contains("f=json"));
        // Token auth params.
        assert!(query.contains("t="));
        assert!(query.contains("s="));
    }

    #[test]
    fn build_url_preserves_base_path() {
        // When the base URL has a sub-path (e.g. /music), it must be preserved.
        let client = Client::new(
            "https://host.example.com/music",
            "admin",
            Auth::token("pass"),
        )
        .unwrap();
        let url = client.build_url("ping", &[]).unwrap();

        assert_eq!(url.path(), "/music/rest/ping");
    }

    #[test]
    fn build_url_preserves_base_path_with_trailing_slash() {
        let client = Client::new(
            "https://host.example.com/music/",
            "admin",
            Auth::token("pass"),
        )
        .unwrap();
        let url = client.build_url("getArtists", &[]).unwrap();

        assert_eq!(url.path(), "/music/rest/getArtists");
    }

    #[test]
    fn build_url_with_extra_params() {
        let client =
            Client::new("https://music.example.com", "admin", Auth::plain("pass")).unwrap();
        let url = client.build_url("getAlbum", &[("id", "42")]).unwrap();
        let query = url.query().unwrap().to_string();

        assert!(query.contains("id=42"));
        assert!(query.contains("p=enc%3A70617373") || query.contains("p=enc:70617373"));
    }

    #[test]
    fn builder_methods() {
        let client = Client::new("https://example.com", "u", Auth::token("p"))
            .unwrap()
            .with_client_name("my-app")
            .with_api_version("1.15.0");

        assert_eq!(client.client_name, "my-app");
        assert_eq!(client.api_version, "1.15.0");
    }

    #[test]
    fn parse_ok_response() {
        let json = r#"{
            "subsonic-response": {
                "status": "ok",
                "version": "1.16.1",
                "type": "navidrome",
                "serverVersion": "0.49.3",
                "openSubsonic": true
            }
        }"#;
        let wrapper: SubsonicResponseWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.response.status, "ok");
        assert_eq!(wrapper.response.version.as_deref(), Some("1.16.1"));
        assert_eq!(wrapper.response.server_type.as_deref(), Some("navidrome"));
        assert!(wrapper.response.error.is_none());
    }

    #[test]
    fn parse_error_response() {
        let json = r#"{
            "subsonic-response": {
                "status": "failed",
                "version": "1.16.1",
                "error": {
                    "code": 40,
                    "message": "Wrong username or password"
                }
            }
        }"#;
        let wrapper: SubsonicResponseWrapper = serde_json::from_str(json).unwrap();
        assert_eq!(wrapper.response.status, "failed");
        let err = wrapper.response.error.unwrap();
        assert_eq!(err.code, 40);
        assert_eq!(err.message.as_deref(), Some("Wrong username or password"));
    }
}
