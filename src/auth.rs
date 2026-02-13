//! Authentication configuration for the Subsonic API.
//!
//! The Subsonic API supports two authentication methods:
//!
//! - **Token auth** (API ≥ 1.13.0, recommended): computes `t = md5(password + salt)` with a
//!   random per-request salt, sent as query parameters `t` and `s`.
//! - **Plain text** (legacy, pre-1.13.0): sends the password as `p=enc:<hex>`.

use md5::{Digest, Md5};
use rand::Rng;

/// Authentication configuration for Subsonic API requests.
#[derive(Debug, Clone)]
pub enum Auth {
    /// Token-based authentication (recommended, API ≥ 1.13.0).
    ///
    /// A random salt is generated for every call to [`Auth::params`], and the token is
    /// computed as `md5(password + salt)`.
    Token {
        /// The user's plaintext password (kept in memory to generate per-request tokens).
        password: String,
    },
    /// Plain text password authentication (legacy, pre-1.13.0).
    ///
    /// The password is sent hex-encoded as `p=enc:<hex>`.
    Plain {
        /// The user's plaintext password.
        password: String,
    },
}

impl Auth {
    /// Create token-based authentication from a password.
    ///
    /// Each call to [`Auth::params`] will generate a fresh random salt and compute the
    /// corresponding MD5 token.
    pub fn token(password: impl Into<String>) -> Self {
        Auth::Token {
            password: password.into(),
        }
    }

    /// Create plain text authentication from a password.
    ///
    /// The password will be sent hex-encoded (`p=enc:<hex>`) with each request.
    pub fn plain(password: impl Into<String>) -> Self {
        Auth::Plain {
            password: password.into(),
        }
    }

    /// Generate authentication query parameters for a single request.
    ///
    /// Returns a `Vec` of `(key, value)` pairs suitable for appending to a URL query string.
    ///
    /// - For [`Auth::Token`]: returns `[("t", token), ("s", salt)]`.
    /// - For [`Auth::Plain`]: returns `[("p", "enc:<hex>")]`.
    pub fn params(&self) -> Vec<(&'static str, String)> {
        match self {
            Auth::Token { password } => {
                let salt = generate_salt();
                let token = compute_token(password, &salt);
                vec![("t", token), ("s", salt)]
            }
            Auth::Plain { password } => {
                let hex_password = hex_encode(password.as_bytes());
                vec![("p", format!("enc:{hex_password}"))]
            }
        }
    }
}

/// Generate a random 12-character lowercase hex salt.
fn generate_salt() -> String {
    let mut rng = rand::thread_rng();
    let bytes: [u8; 6] = rng.gen(); // 6 bytes → 12 hex chars
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
        let auth = Auth::token("sesame");
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
        let auth = Auth::plain("sesame");
        let params = auth.params();
        assert_eq!(params.len(), 1);
        assert_eq!(params[0].0, "p");
        assert_eq!(params[0].1, "enc:736573616d65");
    }

    #[test]
    fn salt_is_random_per_call() {
        let auth = Auth::token("password");
        let p1 = auth.params();
        let p2 = auth.params();
        // Extremely unlikely to collide.
        assert_ne!(p1[1].1, p2[1].1);
    }
}
