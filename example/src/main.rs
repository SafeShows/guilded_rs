use std::{env, time::Duration};

use guilded_rs::{
    models::{chat_embed::ChatEmbed, ChatMessage as Message},
    Bot, Command, CommandContext, Task,
};

struct PingCommand;
impl Command for PingCommand {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "returns pong".to_string()
    }

    fn handler(&self, mut ctx: CommandContext, args: Vec<String>) {
        let mut message = Message::default();
        message.set_content("pong");
        ctx.reply(message);
    }
}

struct TestCommand;
impl Command for TestCommand {
    fn name(&self) -> String {
        "test".to_string()
    }

    fn description(&self) -> String {
        "Basing command for testing the library".to_string()
    }

    fn handler(&self, mut ctx: CommandContext, args: Vec<String>) {
        let mut message = Message::default();
        message.set_content(format!("got message with `{:?}`", args).as_str());
        let mut embed = ChatEmbed::default();
        embed.set_title("Test Command".to_string());
        embed.set_color(0x141935);
        embed.set_description(args.join("\n").to_string());
        message.add_embed(embed);
        ctx.reply(message);
    }
}

fn main() {
    dotenv::dotenv().ok();

    let token = env::var("TOKEN").expect("TOKEN must be set.");
    let mut bot = Bot::new(token, "/".to_string());

    let ping_command: PingCommand = PingCommand {};
    bot.add_command(Box::new(ping_command));
    let test_command: TestCommand = TestCommand {};
    bot.add_command(Box::new(test_command));

    bot.add_task(Task {
        interval: Duration::from_secs(10),
        handler: |bot| {
            let mut message = Message::default();
            message.set_content("Content of the message");
            let channel_id = "2b203b41-5409-436e-9ade-a3c1b640b594";
            match bot.send_chat_message(message, channel_id) {
                Some(msg) => {
                    // msg is the message that just got created
                }
                None => {
                    // Returns None if the request failed.
                    // in this case look at the console for the error message.
                }
            }
        },
    });
    bot.run();
}
