use crate::event::models::ChatMessage as EventChatMessage;
use crate::{bot_http::BotHttp, models::ChatMessage};
#[derive(Clone, Debug)]
pub struct CommandContext {
    pub bot: BotHttp,
    pub message: EventChatMessage,
}

impl CommandContext {
    pub fn new(bot: BotHttp, message: EventChatMessage) -> Self {
        Self { bot, message }
    }
    pub fn reply(&mut self, message: ChatMessage) {
        self.bot
            .send_chat_message(message, &self.message.channel_id.clone().unwrap().as_str());
    }
}
