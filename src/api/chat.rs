//! Chat API endpoints.

use crate::data::ChatMessage;
use crate::error::Error;
use crate::Client;

impl Client {
    /// Get chat messages.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getchatmessages/>
    pub async fn get_chat_messages(
        &self,
        since: Option<i64>,
    ) -> Result<Vec<ChatMessage>, Error> {
        let mut params = Vec::new();
        let since_str;
        if let Some(s) = since {
            since_str = s.to_string();
            params.push(("since", since_str.as_str()));
        }
        let data = self.get_response("getChatMessages", &params).await?;
        let messages = data
            .get("chatMessages")
            .and_then(|v| v.get("chatMessage"))
            .cloned()
            .unwrap_or_else(|| serde_json::Value::Array(vec![]));
        Ok(serde_json::from_value(messages)?)
    }

    /// Add a chat message.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/addchatmessage/>
    pub async fn add_chat_message(&self, message: &str) -> Result<(), Error> {
        self.get_response("addChatMessage", &[("message", message)])
            .await?;
        Ok(())
    }
}
