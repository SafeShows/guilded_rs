//!
//! To create a command you need to use the [Command](guilded::command::Command) trait.
//!
//! Example of a ping command
//!
//! ```
//! use guilded_rs::Command
//!
//! struct PingCommand;
//! impl Command for PingCommand {
//!     fn name(&self) -> String {
//!         "ping".to_string()
//!     }

//!     fn description(&self) -> String {
//!         "returns pong".to_string()
//!     }

//!     fn handler(&self, mut ctx: CommandContext, args: Vec<String>) {
//!         let mut message = Message::default();
//!         message.set_content("pong");
//!         ctx.reply(message);
//!     }
//! }
//! ```
//!

mod context;
pub use context::*;
mod handler;
pub use handler::*;

use crate::{bot_http::BotHttp, event::payloads::ChatMessage};

pub trait Command {
    ///
    /// This function should return the name of the command<br/>
    /// For example if you want to have a command `ping`<br/>
    /// then it should return "ping"<br/>
    ///
    fn name(&self) -> String;

    ///
    /// The description of the command<br/>
    /// that will show up in the
    /// `help` command
    ///
    fn description(&self) -> String;

    ///
    /// Command Handler
    ///
    fn handler(&self, ctx: CommandContext, args: Vec<String>);
}
