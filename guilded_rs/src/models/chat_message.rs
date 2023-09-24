use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

use super::chat_embed::ChatEmbed;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub is_private: Option<bool>,
    pub is_silent: Option<bool>,
    pub reply_message_ids: Option<Vec<String>>,
    pub content: Option<String>,
    pub embeds: Option<Vec<ChatEmbed>>,
}
impl Default for ChatMessage {
    ///
    /// Sets everything to None
    ///
    fn default() -> Self {
        Self {
            is_private: None,
            is_silent: None,
            reply_message_ids: None,
            content: None,
            embeds: None,
        }
    }
}

impl ChatMessage {
    ///
    /// This will set the message content
    /// This is optional if message has one or more
    ///
    pub fn set_content(&mut self, content: &str) -> &mut Self {
        self.content = Some(content.to_string());
        self
    }
    ///
    /// This will add an embed to the message
    /// This is optional if message has content
    ///
    pub fn add_embed(&mut self, embed: ChatEmbed) -> &mut Self {
        match self.embeds.as_mut() {
            Some(embeds) => {
                embeds.push(embed);
            }
            None => {
                self.embeds = Some(vec![embed]);
            }
        }
        self
    }

    ///
    /// This will mark the message as a reply
    /// Right now the max limit is 5 replies per message
    ///
    pub fn add_reply_message(&mut self, message_id: String) -> &mut Self {
        match self.reply_message_ids.as_mut() {
            Some(reply_message_ids) => {
                reply_message_ids.push(message_id);
            }
            None => {
                self.reply_message_ids = Some(vec![message_id]);
            }
        }
        self
    }

    pub fn set_silent(&mut self, is_silent: bool) -> &mut Self {
        self.is_silent = Some(is_silent);
        self
    }
    pub fn set_is_private(&mut self, is_private: bool) -> &mut Self {
        self.is_private = Some(is_private);
        self
    }
}
