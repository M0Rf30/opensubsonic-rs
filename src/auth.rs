//! Authentication configuration for the Subsonic API.
//!
//! The Subsonic API supports three authentication methods:
//!
//! - **API key** (OpenSubsonic extension): sends a pre-generated API key as the
//!   `apiKey` query parameter. No username is required.
//! - **Token auth** (API ≥ 1.13.0): computes `t = md5(password + salt)` with a
//!   random per-request salt, sent as query parameters `t` and `s`.
//! - **Plain text** (legacy, pre-1.13.0): sends the password as `p=enc:<hex>`.

use md5::{Digest, Md5};
use rand::Rng;

/// Authentication configuration for Subsonic API requests.
///
/// Each variant carries all the credentials needed to authenticate a request,
/// including the username when applicable.
#[derive(Debug, Clone)]
pub enum Auth {
    /// API key authentication (OpenSubsonic extension).
    ///
    /// Uses a pre-generated API key sent as the `apiKey` query parameter.
    /// No username is required — the key identifies the user.
    ApiKey {
        /// The API key string.
        api_key: String,
    },
    /// Token-based authentication (API ≥ 1.13.0).
    ///
    /// A random salt is generated for every call to [`Auth::params`], and the token is
    /// computed as `md5(password + salt)`.
    Token {
        /// The Subsonic user name.
        username: String,
        /// The user's plaintext password (kept in memory to generate per-request tokens).
        password: String,
    },
    /// Plain text password authentication (legacy, pre-1.13.0).
    ///
    /// The password is sent hex-encoded as `p=enc:<hex>`.
    Plain {
        /// The Subsonic user name.
        username: String,
        /// The user's plaintext password.
        password: String,
    },
}

impl Auth {
    /// Create API key authentication.
    ///
    /// The key is sent as the `apiKey` query parameter with each request.
    /// No username is needed.
    pub fn api_key(api_key: impl Into<String>) -> Self {
        Auth::ApiKey {
            api_key: api_key.into(),
        }
    }

    /// Create token-based authentication from a username and password.
    ///
    /// Each call to [`Auth::params`] will generate a fresh random salt and compute the
    /// corresponding MD5 token.
    pub fn token(username: impl Into<String>, password: impl Into<String>) -> Self {
        Auth::Token {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Create plain text authentication from a username and password.
    ///
    /// The password will be sent hex-encoded (`p=enc:<hex>`) with each request.
    pub fn plain(username: impl Into<String>, password: impl Into<String>) -> Self {
        Auth::Plain {
            username: username.into(),
            password: password.into(),
        }
    }

    /// Returns the username, if applicable for this authentication method.
    ///
    /// API key authentication does not carry a username.
    pub fn username(&self) -> Option<&str> {
        match self {
            Auth::ApiKey { .. } => None,
            Auth::Token { username, .. } | Auth::Plain { username, .. } => Some(username),
        }
    }

    /// Generate authentication query parameters for a single request.
    ///
    /// Returns a `Vec` of `(key, value)` pairs suitable for appending to a URL query string.
    ///
    /// - For [`Auth::ApiKey`]: returns `[("apiKey", key)]`.
    /// - For [`Auth::Token`]: returns `[("t", token), ("s", salt)]`.
    /// - For [`Auth::Plain`]: returns `[("p", "enc:<hex>")]`.
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            Auth::ApiKey { api_key } => {
                vec![("apiKey", api_key.clone())]
            }
            Auth::Token { password, .. } => {
                let salt = generate_salt();
                let token = compute_token(password, &salt);
                vec![("t", token), ("s", salt)]
            }
            Auth::Plain { password, .. } => {
                let hex_password = hex_encode(password.as_bytes());
                vec![("p", format!("enc:{hex_password}"))]
            }
        }
    }
}

/// Generate a random 12-character lowercase hex salt.
fn generate_salt() -> String {
    let mut rng = rand::rng();
    let bytes: [u8; 6] = rng.random(); // 6 bytes → 12 hex chars
    hex_encode(&bytes)
}

/// Compute `md5(password + salt)` and return the hex-encoded digest.
fn compute_token(password: &str, salt: &str) -> String {
    let mut hasher = Md5::new();
    hasher.update(password.as_bytes());
    hasher.update(salt.as_bytes());
    let result = hasher.finalize();
    hex_encode(&result)
}

/// Encode bytes as a lowercase hex string.
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn token_auth_produces_correct_params() {
        let auth = Auth::token("admin", "sesame");
        let params = auth.params();
        assert_eq!(params.len(), 2);
        assert_eq!(params[0].0, "t");
        assert_eq!(params[1].0, "s");
        // Salt should be 12 hex chars.
        assert_eq!(params[1].1.len(), 12);
        assert!(params[1].1.chars().all(|c| c.is_ascii_hexdigit()));
        // Token should be a 32-char hex MD5 digest.
        assert_eq!(params[0].1.len(), 32);
        assert!(params[0].1.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn token_auth_matches_known_md5() {
        // md5("sesame" + "abcdef012345") — verify manually.
        let salt = "abcdef012345";
        let token = compute_token("sesame", salt);
        // Compute expected: md5(b"sesameabcdef012345")
        let mut hasher = Md5::new();
        hasher.update(b"sesameabcdef012345");
        let expected = hex_encode(&hasher.finalize());
        assert_eq!(token, expected);
    }

    #[test]
    fn plain_auth_hex_encodes_password() {
        let auth = Auth::plain("admin", "sesame");
        let params = auth.params();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].0, "p");
        assert_eq!(params[0].1, "enc:736573616d65");
    }

    #[test]
    fn api_key_auth_produces_correct_params() {
        let auth = Auth::api_key("my-secret-key-123");
        let params = auth.params();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].0, "apiKey");
        assert_eq!(params[0].1, "my-secret-key-123");
    }

    #[test]
    fn username_accessor() {
        assert_eq!(Auth::token("alice", "pass").username(), Some("alice"));
        assert_eq!(Auth::plain("bob", "pass").username(), Some("bob"));
        assert_eq!(Auth::api_key("key").username(), None);
    }

    #[test]
    fn salt_is_random_per_call() {
        let auth = Auth::token("user", "password");
        let p1 = auth.params();
        let p2 = auth.params();
        // Extremely unlikely to collide.
        assert_ne!(p1[1].1, p2[1].1);
    }
}
