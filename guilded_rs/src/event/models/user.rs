use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UserType {
    System,
    Bot,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    id: String,
    r#type: Option<UserType>,
    name: String,
    avatar: Option<String>,
    banner: Option<String>,
    created_at: Timestamp,
    status: UserStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserStatus {
    content: Option<String>,
    emote_id: i64,
}
