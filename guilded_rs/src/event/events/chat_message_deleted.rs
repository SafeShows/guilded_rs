use serde::{Deserialize, Serialize};

use crate::event::payloads::DeletedMessage;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatMessageDeleted {
    server_id: String,
    message: DeletedMessage,
}
