// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Core HTTP client for the Subsonic / OpenSubsonic REST API.

use futures_util::TryStreamExt;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use url::Url;

use crate::auth::Auth;
use crate::data::ServerInfo;
use crate::error::{Error, SubsonicApiError};
use crate::params::Params;

/// A boxed stream of response body chunks, as returned by streaming media endpoints.
pub type ByteStream =
    std::pin::Pin<Box<dyn futures_util::Stream<Item = Result<bytes::Bytes, Error>> + Send>>;

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
    /// Authentication configuration (includes username when applicable).
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
    /// * `auth` — Authentication method (see [`Auth::token`], [`Auth::plain`],
    ///   [`Auth::api_key`]).
    ///
    /// # Errors
    /// Returns [`Error::Url`] if `base_url` cannot be parsed.
    pub fn new(base_url: &str, auth: Auth) -> Result<Self, Error> {
        let base_url = Url::parse(base_url)?;
        Ok(Self {
            base_url,
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
    /// This builds a fresh [`reqwest::Client`] and **replaces** any client previously set
    /// with [`Client::with_http_client`]; likewise a later `with_http_client` call replaces
    /// the client built here.
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

/// Maximum number of characters of a response body embedded in parse errors.
const MAX_SNIPPET_CHARS: usize = 256;

/// Query parameters whose values must never be logged or surfaced in errors.
///
/// `password` covers `createUser` / `updateUser` / `changePassword`.
const SECRET_QUERY_KEYS: [&str; 5] = ["p", "t", "s", "apiKey", "password"];

impl Client {
    /// Build a full request URL for the given API endpoint using [`Params`].
    ///
    /// The resulting URL has the form:
    /// ```text
    /// {base_url}/rest/{endpoint}?u=…&t=…&s=…&v=…&c=…&f=json&{extra params}
    /// ```
    ///
    /// For API key authentication the `u` parameter is omitted and `apiKey` is sent instead.
    pub(crate) fn endpoint_url(&self, endpoint: &str, params: &Params) -> Result<Url, Error> {
        self.build_url_iter(endpoint, params.iter())
    }

    fn build_url_iter<'a>(
        &self,
        endpoint: &str,
        params: impl Iterator<Item = (&'a str, &'a str)>,
    ) -> Result<Url, Error> {
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
            // Username, for auth methods where applicable.
            if let Some(username) = self.auth.username() {
                query.append_pair("u", username);
            }
            // Auth params (apiKey, token+salt, or password).
            for (k, v) in self.auth.params() {
                query.append_pair(k, &v);
            }
            // Protocol version & client id.
            query.append_pair("v", &self.api_version);
            query.append_pair("c", &self.client_name);
            // Always request JSON.
            query.append_pair("f", "json");
            // Endpoint-specific params.
            for (k, v) in params {
                query.append_pair(k, v);
            }
        }

        Ok(url)
    }

    /// GET `url` and return the parsed envelope (errors converted).
    async fn load_envelope(&self, url: Url) -> Result<SubsonicResponseInner, Error> {
        log::debug!("GET {}", redact_url(&url));
        let resp = self.http.get(url).send().await?.error_for_status()?;
        let text = resp.text().await?;
        parse_envelope(&text)
    }

    /// Send a GET and check status plus JSON-error content type for binary endpoints.
    async fn load_binary_response(&self, url: Url) -> Result<reqwest::Response, Error> {
        log::debug!("GET (binary) {}", redact_url(&url));
        let resp = self.http.get(url).send().await?.error_for_status()?;

        // Some servers return a JSON error even on binary endpoints.
        let content_type = resp
            .headers()
            .get(reqwest::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_lowercase();

        if content_type.contains("application/json") || content_type.contains("text/json") {
            let text = resp.text().await?;
            parse_envelope(&text)?;
            // Status is ok but content-type is JSON: something unexpected happened.
            return Err(Error::Parse(
                "Expected binary response but got JSON with status=ok".into(),
            ));
        }
        Ok(resp)
    }

    /// Perform a GET request and return the inner data map (envelope fields stripped).
    pub(crate) async fn get_map(
        &self,
        endpoint: &str,
        params: &Params,
    ) -> Result<Map<String, Value>, Error> {
        let url = self.endpoint_url(endpoint, params)?;
        Ok(self.load_envelope(url).await?.data)
    }

    /// Perform a GET request expecting no payload.
    pub(crate) async fn get_unit(&self, endpoint: &str, params: &Params) -> Result<(), Error> {
        self.get_map(endpoint, params).await.map(drop)
    }

    /// GET and deserialize the value at `key`; a missing key is a parse error.
    pub(crate) async fn get_field<T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &Params,
        key: &str,
    ) -> Result<T, Error> {
        let mut map = self.get_map(endpoint, params).await?;
        take_field(&mut map, key)
    }

    /// GET and deserialize the value at `key`; missing or `null` yields `T::default()`.
    pub(crate) async fn get_field_or_default<T: DeserializeOwned + Default>(
        &self,
        endpoint: &str,
        params: &Params,
        key: &str,
    ) -> Result<T, Error> {
        let mut map = self.get_map(endpoint, params).await?;
        take_field_or_default(&mut map, key)
    }

    /// POST a JSON body and deserialize the value at `key` from the response.
    pub(crate) async fn post_field<B: Serialize + ?Sized, T: DeserializeOwned>(
        &self,
        endpoint: &str,
        params: &Params,
        body: &B,
        key: &str,
    ) -> Result<T, Error> {
        let url = self.endpoint_url(endpoint, params)?;
        log::debug!("POST {}", redact_url(&url));
        let resp = self
            .http
            .post(url)
            .json(body)
            .send()
            .await?
            .error_for_status()?;
        let text = resp.text().await?;
        let mut map = parse_envelope(&text)?.data;
        take_field(&mut map, key)
    }

    /// Perform a GET request and return the raw response bytes.
    ///
    /// If the server returns a JSON error body instead of binary data it is parsed as an
    /// API error.
    pub(crate) async fn get_binary(
        &self,
        endpoint: &str,
        params: &Params,
    ) -> Result<bytes::Bytes, Error> {
        let url = self.endpoint_url(endpoint, params)?;
        Ok(self.load_binary_response(url).await?.bytes().await?)
    }

    /// Like [`Client::get_binary`] but returns the body as a chunk stream.
    pub(crate) async fn get_binary_stream(
        &self,
        endpoint: &str,
        params: &Params,
    ) -> Result<ByteStream, Error> {
        let url = self.endpoint_url(endpoint, params)?;
        let resp = self.load_binary_response(url).await?;
        Ok(Box::pin(resp.bytes_stream().map_err(Error::from)))
    }

    /// Query the server (via `ping`) and return the metadata from the response envelope.
    ///
    /// # Errors
    /// Returns an error if the request fails or the server reports a failure.
    pub async fn server_info(&self) -> Result<ServerInfo, Error> {
        let url = self.endpoint_url("ping", &Params::new())?;
        let inner = self.load_envelope(url).await?;
        Ok(ServerInfo {
            version: inner.version,
            server_type: inner.server_type,
            server_version: inner.server_version,
            open_subsonic: inner.open_subsonic.unwrap_or(false),
        })
    }
}

/// Remove `key` from `map` and deserialize it; a missing key is a parse error.
pub(crate) fn take_field<T: DeserializeOwned>(
    map: &mut Map<String, Value>,
    key: &str,
) -> Result<T, Error> {
    let value = map
        .remove(key)
        .ok_or_else(|| Error::Parse(format!("Missing '{key}' in response")))?;
    serde_json::from_value(value).map_err(|e| Error::Parse(format!("Invalid '{key}': {e}")))
}

/// Remove `key` from `map` and deserialize it; missing or `null` yields `T::default()`.
pub(crate) fn take_field_or_default<T: DeserializeOwned + Default>(
    map: &mut Map<String, Value>,
    key: &str,
) -> Result<T, Error> {
    match map.remove(key) {
        None | Some(Value::Null) => Ok(T::default()),
        Some(value) => {
            serde_json::from_value(value).map_err(|e| Error::Parse(format!("Invalid '{key}': {e}")))
        }
    }
}

/// Truncate `text` to at most [`MAX_SNIPPET_CHARS`] characters, appending `…` if cut.
fn truncate_snippet(text: &str) -> String {
    match text.char_indices().nth(MAX_SNIPPET_CHARS) {
        Some((idx, _)) => format!("{}…", &text[..idx]),
        None => text.to_owned(),
    }
}

/// Parse response text into the inner envelope; `status != "ok"` becomes [`Error::Api`].
fn parse_envelope(text: &str) -> Result<SubsonicResponseInner, Error> {
    let wrapper: SubsonicResponseWrapper = serde_json::from_str(text)
        .map_err(|e| Error::Parse(format!("{e}: {}", truncate_snippet(text))))?;
    let inner = wrapper.response;

    if inner.status != "ok" {
        let api_err = inner.error.map_or_else(
            || SubsonicApiError {
                code: 0,
                message: "Unknown API error (status != ok but no error object)".into(),
                help_url: None,
            },
            |e| SubsonicApiError {
                code: e.code,
                message: e.message.unwrap_or_default(),
                help_url: e.help_url,
            },
        );
        return Err(Error::Api(api_err));
    }
    Ok(inner)
}

/// Render a URL for logging with secret query values replaced by `<redacted>`.
fn redact_url(url: &Url) -> String {
    let mut out = url.clone();
    redact_url_in_place(&mut out);
    out.to_string()
}

/// Replace secret query values and any userinfo password in `url` with `<redacted>`.
pub(crate) fn redact_url_in_place(url: &mut Url) {
    if url.password().is_some() {
        // Only fails for cannot-be-a-base URLs, which never carry credentials.
        let _ = url.set_password(Some("redacted"));
    }
    if url.query().is_none() {
        return;
    }
    let pairs: Vec<(String, String)> = url
        .query_pairs()
        .map(|(k, v)| {
            let v = if SECRET_QUERY_KEYS.contains(&k.as_ref()) {
                "<redacted>".to_owned()
            } else {
                v.into_owned()
            };
            (k.into_owned(), v)
        })
        .collect();
    url.query_pairs_mut().clear().extend_pairs(pairs);
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
    version: Option<String>,
    /// Server implementation type (OpenSubsonic extension, e.g. `"navidrome"`).
    #[serde(rename = "type", default)]
    server_type: Option<String>,
    /// Server software version (OpenSubsonic extension).
    #[serde(rename = "serverVersion", default)]
    server_version: Option<String>,
    /// Whether the server supports OpenSubsonic extensions.
    #[serde(rename = "openSubsonic", default)]
    open_subsonic: Option<bool>,
    /// Present only when `status == "failed"`.
    error: Option<ApiErrorResponse>,
    /// All remaining fields (the actual endpoint-specific data).
    #[serde(flatten)]
    data: Map<String, Value>,
}

/// Subsonic API error object embedded in the response.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct ApiErrorResponse {
    code: i32,
    message: Option<String>,
    help_url: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auth::Auth;

    #[test]
    fn build_url_contains_required_params() {
        let client =
            Client::new("https://music.example.com", Auth::token("admin", "pass")).unwrap();
        let url = client.endpoint_url("ping", &Params::new()).unwrap();
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
            Auth::token("admin", "pass"),
        )
        .unwrap();
        let url = client.endpoint_url("ping", &Params::new()).unwrap();

        assert_eq!(url.path(), "/music/rest/ping");
    }

    #[test]
    fn build_url_preserves_base_path_with_trailing_slash() {
        let client = Client::new(
            "https://host.example.com/music/",
            Auth::token("admin", "pass"),
        )
        .unwrap();
        let url = client.endpoint_url("getArtists", &Params::new()).unwrap();

        assert_eq!(url.path(), "/music/rest/getArtists");
    }

    #[test]
    fn build_url_with_extra_params() {
        let client =
            Client::new("https://music.example.com", Auth::plain("admin", "pass")).unwrap();
        let url = client
            .endpoint_url("getAlbum", &Params::new().with("id", "42"))
            .unwrap();
        let query = url.query().unwrap().to_string();

        assert!(query.contains("id=42"));
        assert!(query.contains("p=enc%3A70617373") || query.contains("p=enc:70617373"));
    }

    #[test]
    fn build_url_api_key_auth() {
        let client =
            Client::new("https://music.example.com", Auth::api_key("my-api-key-123")).unwrap();
        let url = client.endpoint_url("ping", &Params::new()).unwrap();
        let query = url.query().unwrap().to_string();

        assert_eq!(url.path(), "/rest/ping");
        // API key auth must NOT include a username parameter.
        assert!(!query.contains("u="));
        // Must include the apiKey parameter.
        assert!(query.contains("apiKey=my-api-key-123"));
        // Standard params still present.
        assert!(query.contains("v=1.16.1"));
        assert!(query.contains("c=opensubsonic-rs"));
        assert!(query.contains("f=json"));
    }

    #[test]
    fn builder_methods() {
        let client = Client::new("https://example.com", Auth::token("u", "p"))
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

    #[test]
    fn take_field_missing_and_present() {
        let mut m = Map::new();
        m.insert("a".into(), serde_json::json!([1, 2]));
        let v: Vec<i32> = take_field(&mut m, "a").unwrap();
        assert_eq!(v, vec![1, 2]);
        assert!(m.is_empty());
        match take_field::<Vec<i32>>(&mut m, "a") {
            Err(Error::Parse(msg)) => assert_eq!(msg, "Missing 'a' in response"),
            other => panic!("unexpected: {other:?}"),
        }
    }

    #[test]
    fn take_field_or_default_handles_missing_and_null() {
        let mut m = Map::new();
        m.insert("n".into(), Value::Null);
        m.insert("v".into(), serde_json::json!([3]));
        assert!(
            take_field_or_default::<Vec<i32>>(&mut m, "n")
                .unwrap()
                .is_empty()
        );
        assert!(
            take_field_or_default::<Vec<i32>>(&mut m, "zz")
                .unwrap()
                .is_empty()
        );
        assert_eq!(
            take_field_or_default::<Vec<i32>>(&mut m, "v").unwrap(),
            vec![3]
        );
        m.insert("bad".into(), serde_json::json!("x"));
        assert!(take_field_or_default::<Vec<i32>>(&mut m, "bad").is_err());
    }

    #[test]
    fn envelope_error_includes_help_url() {
        let json = r#"{"subsonic-response":{"status":"failed","error":{"code":40,"message":"bad","helpUrl":"https://h/x"}}}"#;
        match parse_envelope(json) {
            Err(Error::Api(e)) => {
                assert_eq!(e.code, 40);
                assert_eq!(e.message, "bad");
                assert_eq!(e.help_url.as_deref(), Some("https://h/x"));
            }
            _ => panic!("expected api error"),
        }
        let json = r#"{"subsonic-response":{"status":"failed"}}"#;
        assert!(matches!(parse_envelope(json), Err(Error::Api(e)) if e.code == 0));
    }

    #[test]
    fn envelope_ok_strips_known_fields() {
        let json = r#"{"subsonic-response":{"status":"ok","version":"1.16.1","x":1}}"#;
        let inner = parse_envelope(json).unwrap();
        assert_eq!(inner.data.len(), 1);
        assert_eq!(inner.version.as_deref(), Some("1.16.1"));
    }

    #[test]
    fn parse_error_truncates_body() {
        let body = "é".repeat(1000);
        match parse_envelope(&body) {
            Err(Error::Parse(msg)) => {
                assert!(msg.ends_with('…'));
                assert!(msg.chars().count() < 400);
            }
            _ => panic!("expected parse error"),
        }
        assert_eq!(truncate_snippet("short"), "short");
        assert_eq!(truncate_snippet(&"a".repeat(256)).chars().count(), 256);
    }

    #[test]
    fn redact_url_hides_secrets() {
        let client = Client::new("https://h.example.com", Auth::token("admin", "pw")).unwrap();
        let url = client
            .endpoint_url("getAlbum", &Params::new().with("id", "42"))
            .unwrap();
        let r = redact_url(&url);
        assert!(r.contains("u=admin") && r.contains("id=42"));
        assert!(r.contains("t=%3Credacted%3E") || r.contains("t=<redacted>"));
        let plain = Client::new("https://h", Auth::plain("a", "secret")).unwrap();
        let r = redact_url(&plain.endpoint_url("ping", &Params::new()).unwrap());
        assert!(!r.contains("736563726574"));
        let key = Client::new("https://h", Auth::api_key("KEY123")).unwrap();
        let r = redact_url(&key.endpoint_url("ping", &Params::new()).unwrap());
        assert!(!r.contains("KEY123") && r.contains("apiKey="));
        let base = format!("https://{}:{}@h", "admin", "hunter2");
        let admin = Client::new(&base, Auth::api_key("k")).unwrap();
        let r = redact_url(
            &admin
                .endpoint_url("changePassword", &Params::new().with("password", "newpw"))
                .unwrap(),
        );
        assert!(!r.contains("hunter2") && !r.contains("newpw"));
    }
}
