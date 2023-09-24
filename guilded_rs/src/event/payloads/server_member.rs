use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

use super::user::User;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerMember {
    pub user: User,
    pub role_ids: Vec<i64>,
    pub nickname: Option<String>,

    ///
    /// The ISO 8601 timestamp that the member joined the server
    ///
    pub joined_at: Timestamp,
    pub is_owner: Option<bool>,
}
