use std::env;

use guilded_rs::{models::ChatMessage, Bot};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN must be set.");
    let mut bot = Bot::new(token).await;
    bot.add_event_handler(|bot, event| {
        println!("This is from add_event_handler!");
        println!("{:?}", event);
    });
    bot.start_task_pool_handler();
    bot.run();
}
