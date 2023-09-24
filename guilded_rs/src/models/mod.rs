//!
//! This contains all of the models that are required for sending data to guilded
//!
//! For example the ChatMessage struct/model is used for creating the chat message
//!

pub mod chat_embed;

mod chat_message;
pub use chat_message::*;
