use serde::{Deserialize, Serialize};

use crate::event::models::ChatMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageCreated {
    pub server_id: String,
    pub message: ChatMessage,
}
