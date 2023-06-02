# guilded_rs

[![crates.io](https://img.shields.io/crates/v/guilded_rs.svg)](https://crates.io/crates/guilded_rs)
[![Documentation](https://docs.rs/guilded_rs/badge.svg)](https://docs.rs/guilded_rs)

Easy to use rust library for creating guilded bots using rust.

### Features:

- [Task Pool](#Task-Pool)
- [Event Handler](#event-handler)

### Planned Features:

- [Commands](#commands-planned)

### The `example` folder

This folder is used for showing live working example on how the lib should be used. But also use <br/>
for testing the library as I'm bad at writing unit test and I kinda hate 'em.

## Task Pool

Using Task Pool you have the ability to create tasks that run in the background.

### Examples

#### This will run every second

```rs
bot.add_task(Task {
    interval: Duration::from_secs(1),
    handler: |_| {
        println!("Hi from task that runs every second");
    },
});

```

#### This task will run every 10 seconds and send a message in a channel.

```rs
bot.add_task(Task {
    interval: Duration::from_secs(10),
    handler: |bot| {
        let mut message = ChatMessage::default();
        message.set_content("Content of the message");
        let channel_id = "2b203b41-5409-436e-9ade-a3c1b640b594";
        let is_reply = false;
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
```

<br/>

## Event Handler

Right now the only event the library parses is the `ChatMessageCreated` event from the WS server

```rs
// The bot variable is the http client of the bot.
// While the event is the enum of the event
bot.add_event_handler(|_bot, event| match event {
    Event::ChatMessageCreated(data) => {
        println!("{:?}", data);
    }
    _ => {}
});
```

<br/>

## Commands (Planned)

Rough example of the way commands will get declared.

```rs
struct PingCommand;
impl Command for PingCommand {
    fn name(&self) -> String {
        "ping".to_string()
    }

    fn description(&self) -> String {
        "returns pong".to_string()
    }

    fn handler(&self, ctx: CommandContext, _args: Vec<String>) {
        let mut message = Message::default();
        message.set_content("pong");
        ctx.reply(message);
    }
}
```
