use std::{
    net::TcpStream,
    thread::{self},
    time::Duration,
};

use websocket::{
    header::{Authorization, Headers},
    message::Message,
    native_tls::TlsStream,
    sync::Client,
    ClientBuilder, OwnedMessage,
};

use crate::{
    bot_http::BotHttp,
    command::{Command, CommandHandler},
    task::{Task, TaskPool},
};

use crate::event::Event;

///
/// Main struct of the library
///
pub struct Bot {
    task_pool: TaskPool,
    socket: Client<TlsStream<TcpStream>>,
    http: BotHttp,
    other: BotOtherFields,
}

struct BotOtherFields {
    command_handler: CommandHandler,
    event_handler: Option<fn(bot: &mut BotHttp, event: Event)>,
    http: BotHttp,
}

impl BotOtherFields {
    fn set_event_handler(&mut self, handler: fn(bot: &mut BotHttp, event: Event)) {
        self.event_handler = Some(handler);
    }

    fn handle_events(&mut self, event: Event) {
        match event.clone() {
            Event::ChatMessageCreated(data) => {
                self.command_handler
                    .parse_message_and_handle_command(data.clone().message);
                if let Some(handler) = self.event_handler {
                    (handler)(&mut self.http, event);
                };
            }
            _ => {
                if let Some(handler) = self.event_handler {
                    (handler)(&mut self.http, event);
                };
            }
        };
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.command_handler.add_command(command);
    }
}
impl Bot {
    pub fn new(token: String, prefix: String) -> Self {
        let http = BotHttp::new(token.clone(), None);
        let mut headers = Headers::new();
        headers.set(Authorization(format!("Bearer {}", token.clone())));
        let socket = ClientBuilder::new("wss://www.guilded.gg/websocket/v1")
            .unwrap()
            .custom_headers(&headers)
            .connect_secure(None)
            .unwrap();
        print!("{}[2J", 27 as char);
        Self {
            other: BotOtherFields {
                command_handler: CommandHandler::new(prefix, http.clone()),
                event_handler: None,
                http: http.clone(),
            },
            socket,
            task_pool: TaskPool::new(),
            http,
        }
    }

    ///
    /// *You can only have one event handler at this time*
    ///
    pub fn add_event_handler(&mut self, handler: fn(bot: &mut BotHttp, event: Event)) {
        self.other.set_event_handler(handler)
    }

    ///
    /// Adds a task to the task pool.
    ///
    pub fn add_task(&mut self, task: Task) {
        self.task_pool.add_task(task);
    }

    pub fn add_command(&mut self, command: Box<dyn Command>) {
        self.other.add_command(command);
    }

    pub fn run(mut self) {
        self.task_pool.start_handler(self.http.clone());
        loop {
            match self.socket.recv_message() {
                Ok(message) => {
                    match message {
                        OwnedMessage::Ping(data) => {
                            self.socket
                                .send_message::<Message>(&Message::pong(data))
                                .unwrap();
                        }
                        OwnedMessage::Text(text) => match serde_json::from_str::<Event>(&text) {
                            Ok(event) => {
                                self.other.handle_events(event);
                            }
                            Err(err) => {
                                println!("{err:?}");
                            }
                        },
                        _ => {
                            println!("{message:?}");
                        }
                    };
                }
                Err(err) => {
                    println!("{err:?}");
                }
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}
