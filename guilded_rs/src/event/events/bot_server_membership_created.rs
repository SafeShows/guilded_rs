use serde::{Deserialize, Serialize};

use crate::event::payloads::Server;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BotServerMembershipCreated {
    created_by: String,
    server: Server,
}
