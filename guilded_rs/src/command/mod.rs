mod context;
pub use context::*;
mod handler;
pub use handler::*;

use crate::{bot_http::BotHttp, event::models::ChatMessage};

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
