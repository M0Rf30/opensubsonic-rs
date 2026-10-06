// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Shared helpers for integration tests.

#![allow(dead_code)] // Each test binary uses a different subset of the helpers.

use opensubsonic::{Auth, Client};
use serde_json::{Map, Value, json};
use wiremock::{Match, MockServer, Request, ResponseTemplate};

/// Build a successful `subsonic-response` envelope, merging `extra` fields into it.
pub fn ok_envelope(extra: Value) -> Value {
    let mut inner = Map::new();
    inner.insert("status".into(), json!("ok"));
    inner.insert("version".into(), json!("1.16.1"));
    inner.insert("type".into(), json!("navidrome"));
    inner.insert("serverVersion".into(), json!("0.54.0"));
    inner.insert("openSubsonic".into(), json!(true));
    if let Value::Object(map) = extra {
        inner.extend(map);
    }
    json!({ "subsonic-response": Value::Object(inner) })
}

/// Build a failed envelope with the given error code, message and optional help URL.
pub fn failed_envelope(code: i32, message: &str, help_url: Option<&str>) -> Value {
    let mut error = json!({ "code": code, "message": message });
    if let Some(url) = help_url {
        error["helpUrl"] = json!(url);
    }
    json!({
        "subsonic-response": {
            "status": "failed",
            "version": "1.16.1",
            "type": "navidrome",
            "serverVersion": "0.54.0",
            "openSubsonic": true,
            "error": error,
        }
    })
}

/// A 200 response with a JSON body.
pub fn json_response(body: &Value) -> ResponseTemplate {
    ResponseTemplate::new(200).set_body_json(body)
}

/// Client using token auth against the mock server.
pub fn token_client(server: &MockServer) -> Client {
    Client::new(&server.uri(), Auth::token("alice", "secret")).expect("valid mock URL")
}

/// Client using API-key auth against the mock server.
pub fn api_key_client(server: &MockServer) -> Client {
    Client::new(&server.uri(), Auth::api_key("my-key")).expect("valid mock URL")
}

/// Matches requests whose query holds exactly the given values for `key`, in order.
pub struct RepeatedQuery {
    key: &'static str,
    values: Vec<String>,
}

impl RepeatedQuery {
    /// Create a matcher for `key` repeated with `values`.
    pub fn new(key: &'static str, values: &[&str]) -> Self {
        Self {
            key,
            values: values.iter().map(|v| (*v).to_owned()).collect(),
        }
    }
}

impl Match for RepeatedQuery {
    fn matches(&self, request: &Request) -> bool {
        let found: Vec<String> = request
            .url
            .query_pairs()
            .filter(|(k, _)| k == self.key)
            .map(|(_, v)| v.into_owned())
            .collect();
        found == self.values
    }
}

/// Matches requests where `key` is absent from the query string.
pub struct NoQuery(pub &'static str);

impl Match for NoQuery {
    fn matches(&self, request: &Request) -> bool {
        !request.url.query_pairs().any(|(k, _)| k == self.0)
    }
}
