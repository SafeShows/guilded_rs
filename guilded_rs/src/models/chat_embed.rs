//!
//! This module contains all of the
//!

use iso8601_timestamp::Timestamp;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChatEmbed {
    title: Option<String>,
    description: Option<String>,
    url: Option<String>,
    color: i64,
    timestamp: Option<Timestamp>,
    footer: Option<Footer>,
    thumbnail: Option<Thumbnail>,
    image: Option<Image>,
    author: Option<Author>,
    fields: Option<Vec<Field>>,
}

impl Default for ChatEmbed {
    ///
    /// Defaults everything to `None`
    ///
    fn default() -> Self {
        Self {
            title: None,
            description: None,
            url: None,
            color: 0xfff,
            timestamp: None,
            footer: None,
            thumbnail: None,
            image: None,
            author: None,
            fields: None,
        }
    }
}

impl ChatEmbed {
    ///
    /// Main header of the embed
    /// (max length 256)
    ///
    pub fn set_title(&mut self, title: String) -> &mut Self {
        self.title = Some(title);
        self
    }

    ///
    /// Subtext of the embed (max length 2048)
    ///
    pub fn set_description(&mut self, description: String) -> &mut Self {
        self.description = Some(description);
        self
    }

    ///
    /// URL to linkify the title field with (max length 1024)
    ///
    pub fn set_url(&mut self, url: String) -> &mut Self {
        self.url = Some(url);
        self
    }
    ///
    /// The color that the left border should be<br/>
    /// You can use any color picker that gives you a hex color and<br/>
    /// then remember to replace the `#` with `0x` followed by the color.<br/>
    /// For example if you want to use this color `#ff0000` you would<br/>
    /// need to remove the `#` and add `0x` in it's place like this `0xff0000`
    ///
    /// Code Example
    /// ```
    /// embed.set_color(0xfff) // this will set the color of the embed white
    /// embed.set_color(0x1bc271) // this will set the color of the embed green
    /// embed.set_color(0x3c537c) // this will set the color of the embed dark blue
    /// ```
    ///
    pub fn set_color(&mut self, color: i64) -> &mut Self {
        self.color = color;
        self
    }

    ///
    /// A timestamp to put in the footer
    ///
    pub fn set_timestamp(&mut self, timestamp: Timestamp) -> &mut Self {
        self.timestamp = Some(timestamp);
        self
    }

    ///
    /// An image to the right of the embed's content
    ///
    pub fn set_thumbnail(&mut self, thumbnail: Thumbnail) -> &mut Self {
        self.thumbnail = Some(thumbnail);
        self
    }

    ///
    /// The main picture to associate with the embed
    ///
    pub fn set_image(&mut self, image: Image) -> &mut Self {
        self.image = Some(image);
        self
    }

    ///
    /// The main picture to associate with the embed
    ///
    pub fn set_author(&mut self, author: Author) -> &mut Self {
        self.author = Some(author);
        self
    }
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thumbnail {
    ///
    /// URL of the image (max length 1024)
    ///
    pub url: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Footer {
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
    pub text: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Image {
    ///
    /// URL of the image (max length 1024)
    ///
    pub url: String,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Author {
    pub name: String,
    pub url: String,
    #[serde(rename = "icon_url")]
    pub icon_url: Option<String>,
}

#[skip_serializing_none]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Field {
    ///
    /// Header of the table-like cell (max length 256)
    ///
    pub name: String,

    ///
    /// Subtext of the table-like cell (max length 1024)
    ///
    pub value: String,

    ///
    /// If the field should wrap or not.<br/>
    ///
    pub inline: bool,
}
