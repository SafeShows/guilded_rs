use std::time::Duration;

use crate::bot_http::BotHttp;

pub struct Task {
    pub interval: Duration,
    pub handler: Option<fn(bot: &mut BotHttp)>,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            interval: Duration::from_secs(1),
            handler: None,
        }
    }
}

impl Task {
    pub fn set_interval(&mut self, interval: Duration) -> &mut Self {
        self.interval = interval;
        self
    }

    pub fn set_handler(&mut self, handler: fn(bot: &mut BotHttp)) -> &mut Self {
        self.handler = Some(handler);
        self
    }
}
