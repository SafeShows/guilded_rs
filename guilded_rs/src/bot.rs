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
    task::{Task, TaskPool},
};

use crate::event::Event;

pub struct Bot {
    task_pool: TaskPool,
    socket: Client<TlsStream<TcpStream>>,
    http: BotHttp,
    other: BotOtherFields,
}

struct BotOtherFields {
    event_handler: Option<fn(bot: &mut BotHttp, event: Event)>,
    http: BotHttp,
}

impl BotOtherFields {
    fn handle_events(&mut self, event: Event) {
        if let Some(handler) = self.event_handler {
            (handler)(&mut self.http, event.clone());
        };
    }
}
impl Bot {
    pub fn new(token: String) -> Self {
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
    pub fn add_event_handler(&mut self, handler: fn(bot: &mut BotHttp, event: Event)) -> &mut Self {
        self.other.event_handler = Some(handler);
        self
    }

    ///
    /// Adds a task to the task pool.
    ///
    pub fn add_task(&mut self, task: Task) -> &mut Self {
        self.task_pool.add_task(task);
        self
    }

    pub fn run(mut self) {
        self.task_pool.start_handler(self.http.clone());
        loop {
            match self.socket.recv_message() {
                Ok(message) => match message {
                    OwnedMessage::Ping(data) => {
                        self.socket
                            .send_message::<Message>(&Message::pong(data))
                            .unwrap();
                    }
                    OwnedMessage::Text(text) => match serde_json::from_str::<Event>(&text) {
                        Ok(event) => self.other.handle_events(event),
                        Err(_) => {}
                    },
                    _ => {}
                },
                Err(_) => todo!(),
            }
            thread::sleep(Duration::from_millis(10));
        }
    }
}
