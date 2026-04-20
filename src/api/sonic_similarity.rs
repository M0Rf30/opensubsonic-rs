//! Sonic Similarity API endpoints (OpenSubsonic extension).

use crate::Client;
use crate::data::SonicMatch;
use crate::error::Error;

impl Client {
    /// Get tracks sonically similar to the given song (OpenSubsonic, sonicSimilarity extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getsonicsimilartracks/>
    pub async fn get_sonic_similar_tracks(
        &self,
        id: &str,
        count: Option<i32>,
    ) -> Result<Vec<SonicMatch>, Error> {
        let mut params = vec![("id", id.to_string())];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self
            .get_response("getSonicSimilarTracks", &param_refs)
            .await?;
        let matches = data
            .get("sonicSimilarTracks")
            .and_then(|v| v.get("sonicMatch"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(matches)?)
    }

    /// Find a path of sonically similar tracks between two songs
    /// (OpenSubsonic, sonicSimilarity extension).
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/findsonicpath/>
    pub async fn find_sonic_path(
        &self,
        start_song_id: &str,
        end_song_id: &str,
        count: Option<i32>,
    ) -> Result<Vec<SonicMatch>, Error> {
        let mut params = vec![
            ("startSongId", start_song_id.to_string()),
            ("endSongId", end_song_id.to_string()),
        ];
        if let Some(c) = count {
            params.push(("count", c.to_string()));
        }
        let param_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        let data = self.get_response("findSonicPath", &param_refs).await?;
        let matches = data
            .get("sonicPath")
            .and_then(|v| v.get("sonicMatch"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(matches)?)
    }
}
