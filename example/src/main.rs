use std::{env, time::Duration};

use guilded_rs::{models::ChatMessage, task::Task, Bot};

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let token = env::var("TOKEN").expect("TOKEN must be set.");
    let mut bot = Bot::new(token).await;
    bot.add_event_handler(|bot, event| {
        println!("This is from add_event_handler!");
        println!("{:?}", event);
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
            //this will run every 10 seconds
        },
    });
    bot.start_task_pool_handler();
    bot.run();
}
