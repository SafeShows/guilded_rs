use reqwest::{
    blocking::{Client, ClientBuilder},
    header::{HeaderMap, HeaderValue, AUTHORIZATION, CONTENT_TYPE},
};

use crate::event::models::ChatMessage as Message;
use crate::{event::Event, models::ChatMessage};

const API_BASE: &str = "https://www.guilded.gg/api/v1";

#[derive(Debug, Clone)]
pub struct BotHttp {
    http_client: Client,
    pub event: Option<Event>,
}

impl BotHttp {
    pub fn new(token: String, event: Option<Event>) -> Self {
        let mut client = ClientBuilder::new();
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            AUTHORIZATION,
            HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
        );
        default_headers.insert(
            CONTENT_TYPE,
            HeaderValue::from_str("application/json").unwrap(),
        );
        default_headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        client = client.default_headers(default_headers);
        Self {
            http_client: client.build().unwrap(),
            event,
        }
    }

    ///
    /// Sends a message in the channel and returns the message that just got created.
    /// **Returns None if the request failed and prints the error to the console**
    ///
    pub fn send_chat_message(&mut self, message: ChatMessage, channel_id: &str) -> Option<Message> {
        let message = serde_json::to_string::<ChatMessage>(&message).unwrap();
        println!("{}", message);
        match self
            .http_client
            .post(format!("{}/channels/{}/messages", API_BASE, channel_id))
            .body::<String>(message)
            .send()
        {
            Ok(res) => Some(serde_json::from_str::<Message>(res.text().unwrap().as_str()).unwrap()),
            Err(err) => {
                print!("{:?}", err);
                None
            }
        }
    }
}
