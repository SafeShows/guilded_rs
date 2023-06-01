use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChatMessageType {
    Default,
    System,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessage {
    pub id: Option<String>,
    pub r#type: Option<ChatMessageType>,
    pub server_id: Option<String>,
    pub group_id: Option<String>,
    pub channel_id: Option<String>,
    pub content: Option<String>,
    pub created_at: Option<Timestamp>,
    pub created_by: Option<String>,
    pub created_by_webhook_id: Option<String>,
    pub updated_at: Option<Timestamp>,
}
