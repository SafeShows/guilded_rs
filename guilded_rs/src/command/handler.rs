use crate::{bot_http::BotHttp, event::models::ChatMessage};

use super::Command;

pub struct CommandHandler {
    prefix: String,
    commands: Vec<Box<dyn Command>>,
    bot: BotHttp,
}

impl CommandHandler {
    pub fn new(prefix: String, bot: BotHttp) -> Self {
        Self {
            prefix,
            bot,
            commands: Vec::new(),
        }
    }

    pub fn add_command(mut self, command: Box<dyn Command>) {
        self.commands.push(command);
    }

    pub fn get_commands(&mut self) -> &Vec<Box<dyn Command>> {
        &self.commands
    }

    pub fn parse_message_and_handle_command(&mut self, message: ChatMessage) {
        let prefix = self.prefix.as_str();
        self.commands.iter_mut().for_each(|cmd| {
            match message.content.clone() {
                Some(mut content) => {
                    match content.as_mut_str().trim().starts_with(prefix) {
                        true => {
                            match content
                                .strip_prefix(prefix)
                                .unwrap()
                                .starts_with(cmd.name().as_str())
                            {
                                true => match content.strip_prefix(cmd.name().as_str()) {
                                    Some(args) => {
                                        let mut args = args.split(" ").collect::<Vec<&str>>();
                                        cmd.handler(
                                            super::CommandContext {
                                                bot: self.bot.clone(),
                                                message: message.clone(),
                                            },
                                            args.iter_mut()
                                                .map(|arg| arg.to_string())
                                                .collect::<Vec<String>>(),
                                        )
                                    }
                                    None => {}
                                },
                                false => {}
                            }
                        }
                        false => {}
                    };
                }
                // we ignore messages that don't have content
                None => {}
            }
        });
    }
}
