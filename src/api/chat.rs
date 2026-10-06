// SPDX-FileCopyrightText: 2026 Gianluca Boiano
// SPDX-License-Identifier: MIT OR Apache-2.0

//! Chat API endpoints.

use crate::Client;
use crate::data::ChatMessage;
use crate::error::Error;
use crate::params::Params;
use serde::Deserialize;

/// Wrapper for the nested `chatMessages.chatMessage` response shape.
#[derive(Deserialize, Default)]
struct MessagesWrapper {
    #[serde(default, rename = "chatMessage")]
    chat_message: Vec<ChatMessage>,
}

impl Client {
    /// Get chat messages.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/getchatmessages/>
    pub async fn get_chat_messages(&self, since: Option<i64>) -> Result<Vec<ChatMessage>, Error> {
        let params = Params::new().with_opt("since", since);
        let wrapper: Option<MessagesWrapper> = self
            .get_field_or_default("getChatMessages", &params, "chatMessages")
            .await?;
        Ok(wrapper.map(|w| w.chat_message).unwrap_or_default())
    }

    /// Add a chat message.
    ///
    /// See <https://opensubsonic.netlify.app/docs/endpoints/addchatmessage/>
    pub async fn add_chat_message(&self, message: &str) -> Result<(), Error> {
        self.get_unit("addChatMessage", &Params::new().with("message", message))
            .await
    }
}
