use crate::event::payloads::ChatMessage as ChatMessagePayload;
use crate::{bot_http::BotHttp, models::ChatMessage};
#[derive(Clone, Debug)]
pub struct CommandContext {
    pub bot: BotHttp,
    pub message: ChatMessagePayload,
}

impl CommandContext {
    pub fn new(bot: BotHttp, message: ChatMessagePayload) -> Self {
        Self { bot, message }
    }
    pub fn reply(&mut self, message: ChatMessage) {
        self.bot
            .send_chat_message(message, &self.message.channel_id.clone().unwrap().as_str());
    }
}
