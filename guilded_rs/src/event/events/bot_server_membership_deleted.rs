use serde::{Deserialize, Serialize};

use crate::event::payloads::Server;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotServerMembershipDeleted {
    deleted_by: String,
    server: Server,
}
