use reqwest::{
    header::{HeaderMap, HeaderValue},
    Client, ClientBuilder,
};

use crate::{event::models::ChatMessage, event::Event};

const API_BASE: &str = "https://www.guilded.gg/api/v1/";

#[derive(Debug, Clone)]
pub struct BotHttp {
    http_client: Client,
    event: Event,
}

impl From<&mut BotHttp> for BotHttp {
    fn from(value: &mut BotHttp) -> Self {
        Self {
            http_client: value.http_client.clone(),
            event: value.event.clone(),
        }
    }
}

impl BotHttp {
    pub fn new(token: String, event: Event) -> Self {
        let mut client = ClientBuilder::new();
        let mut default_headers = HeaderMap::new();
        default_headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("Bearer {}", token).as_str()).unwrap(),
        );
        default_headers.insert(
            "Content-type",
            HeaderValue::from_str("application/json").unwrap(),
        );
        default_headers.insert("Accept", HeaderValue::from_str("application/json").unwrap());
        client = client.default_headers(default_headers);
        Self {
            http_client: client.build().unwrap(),
            event,
        }
    }

    pub async fn send_chat_message(
        &mut self,
        message: ChatMessage,
        channel_id: Option<String>,
        is_reply: bool,
        // ) -> Result<ChatMessage, ()> {
    ) {
        match channel_id {
            Some(channel_id) => {
                match is_reply {
                    true => {}
                    false => {}
                }
                let res = self
                    .http_client
                    .post(format!("{}/channels/{}/messages", API_BASE, channel_id))
                    .body(serde_json::to_string::<ChatMessage>(&message).unwrap())
                    .send()
                    .await;
                print!("{:?}", res);
            }
            None => {
                self.http_client.post(format!("{}", API_BASE));
            }
        }
    }
}
