//! Error types for the `opensubsonic` client.

use std::fmt;

/// Subsonic API error codes as defined in the protocol specification.
///
/// See <https://www.subsonic.org/pages/api.jsp> for the full list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubsonicErrorCode {
    /// A generic error (code 0).
    Generic = 0,
    /// Required parameter is missing (code 10).
    MissingParameter = 10,
    /// Incompatible Subsonic REST protocol version. Client must upgrade (code 20).
    ClientMustUpgrade = 20,
    /// Incompatible Subsonic REST protocol version. Server must upgrade (code 30).
    ServerMustUpgrade = 30,
    /// Wrong username or password (code 40).
    WrongCredentials = 40,
    /// Token authentication not supported for LDAP users (code 41).
    TokenAuthNotSupported = 41,
    /// User is not authorized for the given operation (code 50).
    NotAuthorized = 50,
    /// The trial period for the Subsonic server is over (code 60).
    TrialExpired = 60,
    /// The requested data was not found (code 70).
    NotFound = 70,
}

impl SubsonicErrorCode {
    /// Try to convert a numeric error code to a known [`SubsonicErrorCode`] variant.
    pub fn from_code(code: i32) -> Option<Self> {
        match code {
            0 => Some(Self::Generic),
            10 => Some(Self::MissingParameter),
            20 => Some(Self::ClientMustUpgrade),
            30 => Some(Self::ServerMustUpgrade),
            40 => Some(Self::WrongCredentials),
            41 => Some(Self::TokenAuthNotSupported),
            50 => Some(Self::NotAuthorized),
            60 => Some(Self::TrialExpired),
            70 => Some(Self::NotFound),
            _ => None,
        }
    }
}

impl fmt::Display for SubsonicErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Generic => write!(f, "Generic error"),
            Self::MissingParameter => write!(f, "Missing parameter"),
            Self::ClientMustUpgrade => write!(f, "Client must upgrade"),
            Self::ServerMustUpgrade => write!(f, "Server must upgrade"),
            Self::WrongCredentials => write!(f, "Wrong credentials"),
            Self::TokenAuthNotSupported => write!(f, "Token auth not supported"),
            Self::NotAuthorized => write!(f, "Not authorized"),
            Self::TrialExpired => write!(f, "Trial expired"),
            Self::NotFound => write!(f, "Not found"),
        }
    }
}

/// An error returned by the Subsonic API inside the response body.
#[derive(Debug, Clone)]
pub struct SubsonicApiError {
    /// The numeric error code from the API.
    pub code: i32,
    /// A human-readable error message.
    pub message: String,
}

impl SubsonicApiError {
    /// Try to interpret the numeric code as a known [`SubsonicErrorCode`].
    pub fn error_code(&self) -> Option<SubsonicErrorCode> {
        SubsonicErrorCode::from_code(self.code)
    }
}

impl fmt::Display for SubsonicApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(known) = self.error_code() {
            write!(
                f,
                "Subsonic API error {}: {} ({})",
                self.code, self.message, known
            )
        } else {
            write!(f, "Subsonic API error {}: {}", self.code, self.message)
        }
    }
}

impl std::error::Error for SubsonicApiError {}

/// All possible errors that can occur when using this client.
#[derive(Debug)]
pub enum Error {
    /// An HTTP request failed at the transport level.
    Http(reqwest::Error),
    /// The Subsonic API returned an error response (`status="failed"`).
    Api(SubsonicApiError),
    /// Failed to parse or deserialize the server's response.
    Parse(String),
    /// URL construction failed.
    Url(url::ParseError),
    /// Any other error.
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Http(e) => write!(f, "HTTP error: {e}"),
            Error::Api(e) => write!(f, "{e}"),
            Error::Parse(msg) => write!(f, "Parse error: {msg}"),
            Error::Url(e) => write!(f, "URL error: {e}"),
            Error::Other(msg) => write!(f, "{msg}"),
        }
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::Http(e) => Some(e),
            Error::Api(e) => Some(e),
            Error::Url(e) => Some(e),
            Error::Parse(_) | Error::Other(_) => None,
        }
    }
}

impl From<reqwest::Error> for Error {
    fn from(err: reqwest::Error) -> Self {
        Error::Http(err)
    }
}

impl From<url::ParseError> for Error {
    fn from(err: url::ParseError) -> Self {
        Error::Url(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::Parse(err.to_string())
    }
}
