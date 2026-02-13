//! Transcoding API endpoints (OpenSubsonic extension).

use bytes::Bytes;
use url::Url;

use crate::data::TranscodeDecision;
use crate::error::Error;
use crate::Client;

impl Client {
    /// Get a transcode decision for a song (OpenSubsonic extension).
    ///
    /// This endpoint uses POST with a JSON body containing client info.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/gettranscodedecision/>
    pub async fn get_transcode_decision(
        &self,
        id: &str,
        max_bit_rate: Option<i32>,
        format: Option<&str>,
        client_info: Option<&crate::data::ClientInfo>,
    ) -> Result<TranscodeDecision, Error> {
        // This is a POST endpoint with query params for id/maxBitRate/format
        // and JSON body for clientInfo. For simplicity, we use GET params when no body.
        let mut params = vec![("id", id.to_string())];
        if let Some(br) = max_bit_rate {
            params.push(("maxBitRate", br.to_string()));
        }
        if let Some(f) = format {
            params.push(("format", f.to_string()));
        }

        if let Some(info) = client_info {
            // Build URL with params and do POST with JSON body.
            let param_refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let url = self.build_url("getTranscodeDecision", &param_refs)?;
            log::debug!("POST {url}");
            let resp = self
                .http
                .post(url)
                .json(info)
                .send()
                .await?
                .error_for_status()?;
            let text = resp.text().await?;
            let wrapper: serde_json::Value = serde_json::from_str(&text)
                .map_err(|e| Error::Parse(format!("{e}: {text}")))?;
            let inner = wrapper
                .get("subsonic-response")
                .ok_or_else(|| Error::Parse("Missing subsonic-response".into()))?;
            let status = inner.get("status").and_then(|s| s.as_str()).unwrap_or("");
            if status != "ok" {
                let code = inner
                    .get("error")
                    .and_then(|e| e.get("code"))
                    .and_then(|c| c.as_i64())
                    .unwrap_or(0) as i32;
                let msg = inner
                    .get("error")
                    .and_then(|e| e.get("message"))
                    .and_then(|m| m.as_str())
                    .unwrap_or("")
                    .to_string();
                return Err(Error::Api(crate::error::SubsonicApiError {
                    code,
                    message: msg,
                }));
            }
            let decision = inner
                .get("transcodeDecision")
                .ok_or_else(|| {
                    Error::Parse("Missing 'transcodeDecision' in response".into())
                })?;
            Ok(serde_json::from_value(decision.clone())?)
        } else {
            let param_refs: Vec<(&str, &str)> =
                params.iter().map(|(k, v)| (*k, v.as_str())).collect();
            let data = self
                .get_response("getTranscodeDecision", &param_refs)
                .await?;
            let decision = data
                .get("transcodeDecision")
                .ok_or_else(|| {
                    Error::Parse("Missing 'transcodeDecision' in response".into())
                })?;
            Ok(serde_json::from_value(decision.clone())?)
        }
    }

    /// Get a transcoded stream URL (OpenSubsonic extension).
    ///
    /// Returns the URL for streaming transcoded audio. Does not make an HTTP request.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/gettranscodestream/>
    pub fn get_transcode_stream_url(
        &self,
        id: &str,
        max_bit_rate: Option<i32>,
        format: Option<&str>,
    ) -> Result<Url, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(br) = max_bit_rate {
            params.push(("maxBitRate", br.to_string()));
        }
        if let Some(f) = format {
            params.push(("format", f.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.build_url("getTranscodeStream", &param_refs)
    }

    /// Get a transcoded stream as raw bytes (OpenSubsonic extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/gettranscodestream/>
    pub async fn get_transcode_stream(
        &self,
        id: &str,
        max_bit_rate: Option<i32>,
        format: Option<&str>,
    ) -> Result<Bytes, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(br) = max_bit_rate {
            params.push(("maxBitRate", br.to_string()));
        }
        if let Some(f) = format {
            params.push(("format", f.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.get_bytes("getTranscodeStream", &param_refs).await
    }
}
