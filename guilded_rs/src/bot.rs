use std::{
    marker::PhantomData,
    net::TcpStream,
    thread::{self, spawn},
    time::Duration,
};

use websocket::{
    header::{Authorization, Headers},
    message::Message,
    native_tls::TlsStream,
    sync::Client,
    ClientBuilder, OwnedMessage,
};

use crate::{bot_http::BotHttp, task::Task};

use crate::event::Event;

pub struct Bot {
    tasks: Option<Vec<Task>>,
    event_handler: Option<fn(bot: &mut BotHttp, event: Event)>,
    socket: Client<TlsStream<TcpStream>>,
    token: String,
}

impl Bot {
    pub async fn new(token: String) -> Self {
        let mut headers = Headers::new();
        headers.set(Authorization(format!("Bearer {}", token)));
        let socket = ClientBuilder::new("wss://www.guilded.gg/websocket/v1")
            .unwrap()
            .custom_headers(&headers)
            .connect_secure(None)
            .unwrap();
        print!("{}[2J", 27 as char);
        Self {
            socket,
            event_handler: None,
            tasks: None,
            token,
        }
    }

    ///
    /// *You can only have one event handler at this time*
    ///
    pub fn add_event_handler(&mut self, handler: fn(bot: &mut BotHttp, event: Event)) -> &mut Self {
        self.event_handler = Some(handler);
        self
    }

    fn handle_events(&mut self, event: Event) {
        match self.event_handler {
            Some(handler) => (handler)(
                &mut BotHttp::new(self.token.clone(), event.clone()),
                event.clone(),
            ),
            None => todo!(),
        }
    }

    ///
    /// Adds a task to the task pool.
    ///
    pub fn add_task(&mut self, handler: fn(bot: &mut BotHttp), interval: Duration) -> &mut Self {
        match self.tasks.as_mut() {
            Some(tasks) => tasks.push(Task {
                interval,
                handler: Some(handler),
            }),
            None => {
                self.tasks = Some(vec![Task {
                    interval,
                    handler: Some(handler),
                }]);
            }
        }
        self
    }

    pub fn start_task_pool_handler(&mut self) {}

    pub fn run(mut self) {
        loop {
            match self.socket.recv_message() {
                Ok(message) => match message {
                    OwnedMessage::Ping(data) => {
                        self.socket
                            .send_message::<Message>(&Message::pong(data))
                            .unwrap();
                    }
                    OwnedMessage::Text(text) => match serde_json::from_str::<Event>(&text) {
                        Ok(event) => self.handle_events(event),
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
