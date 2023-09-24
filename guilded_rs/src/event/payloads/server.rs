use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    ///
    /// The ID of the server
    ///
    pub id: String,

    ///
    /// The ID of the user who created this server
    ///
    pub owner_id: String,

    ///
    /// The type of server designated from the server's settings page
    ///
    pub r#type: Option<ServerType>,

    ///
    /// The name given to the server
    ///
    pub name: String,

    ///
    /// The URL that the server can be accessible from. <br/>
    /// For example, a value of "Guilded-Official" means<br/>
    /// the server can be accessible from https://www.guilded.gg/Guilded-Official
    ///
    pub url: Option<String>,

    ///
    /// The description associated with the server
    ///
    pub about: Option<String>,

    ///
    /// The avatar image associated with the server
    ///
    pub avatar: Option<String>,

    ///
    /// The banner image associated with the server
    ///
    pub banner: Option<String>,

    ///
    /// The timezone associated with the server
    ///
    pub timezone: Option<String>,

    ///
    /// The verified status of the server
    ///
    pub is_verified: Option<bool>,

    ///
    /// The channel ID of the default channel of the server.<br/>
    /// This channel is defined as the first chat or voice channel in the left sidebar of a server in our UI.<br/>
    /// This channel is useful for sending welcome messages,<br/>though note that a bot may not have permissions to interact with this channel depending on how the server is configured.
    ///
    pub default_channel_id: Option<String>,

    ///
    /// The ISO 8601 timestamp that the server was created at
    ///
    pub created_at: Timestamp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ServerType {
    Team,
    Organization,
    Community,
    Clan,
    Guild,
    Friends,
    Streaming,
    Other,
}
