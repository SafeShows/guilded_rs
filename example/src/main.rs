use std::{env, time::Duration};

use guilded_rs::{
    event::{models::ChatMessage, Event},
    models::ChatMessage as Message,
    task::Task,
    Bot, BotHttp, Command,
};

#[derive(Debug)]
struct PingCommand;
impl Command for PingCommand {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "returns pong".to_string()
    }

    fn handler(&self, ctx: guilded_rs::CommandContext, args: Vec<String>) {
        ctx.reply("pong".to_string());
    }
}

fn main() {
    dotenv::dotenv().ok();

    let ping_command: PingCommand = PingCommand {};

    let token = env::var("TOKEN").expect("TOKEN must be set.");
    let mut bot = Bot::new(token);
    bot.add_event_handler(|_bot, event| match event {
        Event::ChatMessageCreated(data) => {
            println!("{:?}", data);
        }
        _ => {}
    });
    bot.add_task(Task {
        interval: Duration::from_secs(1),
        handler: |_| {
            println!("Hi from task that runs every second");
            //this will run every 10 seconds
        },
    });
    bot.add_task(Task {
        interval: Duration::from_millis(500),
        handler: |_| {
            println!("Hi from task that runs every 500ms");
            //this will run every 500ms
        },
    });
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
