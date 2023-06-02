use crate::event::models::ChatMessage;

#[derive(Debug)]
pub struct CommandContext {
    pub message: ChatMessage,
}

impl CommandContext {
    pub fn reply(self, message: String) {}
}
