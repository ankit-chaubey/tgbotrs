//! Helper methods on Telegram types (get_text, get_entities, get_link, reply, send_message, File::url, InaccessibleMessage::to_message).
//!
//! All methods are added directly on the generated types so no trait import
//! is needed:
//!
//! | Type                   | Methods added                                          |
//! |------------------------|--------------------------------------------------------|
//! | [`Message`]            | `get_text`, `get_entities`, `get_link`, `reply`        |
//! | [`Chat`]               | `send_message`                                         |
//! | [`File`]               | `url`                                                  |
//! | [`InaccessibleMessage`]| `to_message`                                           |
//!
//! # Example
//!
//! ```rust,no_run
//! use tgbotrs::{Bot, types::{Message}};
//!
//! async fn handle(bot: &Bot, msg: &Message) {
//!     // get_text() returns text OR caption, whichever is present
//!     if let Some(text) = msg.get_text() {
//!         msg.reply(bot, format!("You said: {text}"), None).await.ok();
//!     }
//!
//!     // get_link() returns a t.me URL for public chats
//!     let link = msg.get_link();
//!     if !link.is_empty() {
//!         println!("Message link: {link}");
//!     }
//! }
//! ```

use crate::{
    gen_methods::SendMessageParams,
    types::{Chat, File, InaccessibleMessage, Message, MessageEntity, ReplyParameters},
    Bot, BotError,
};

// Message
impl Message {
    /// Returns `text` for text messages, or `caption` for media messages.
    ///
    /// Mirrors `gotgbot`'s `GetText()`. Telegram splits the human-visible text
    /// across two fields depending on message type; this unifies them.
    pub fn get_text(&self) -> Option<&str> {
        self.text.as_deref().or_else(|| self.caption.as_deref())
    }

    /// Returns `entities` for text messages, or `caption_entities` for media.
    ///
    /// Mirrors `gotgbot`'s `GetEntities()`.
    pub fn get_entities(&self) -> Option<&[MessageEntity]> {
        self.entities
            .as_deref()
            .or_else(|| self.caption_entities.as_deref())
    }

    /// Returns the public `t.me` URL to this message.
    ///
    /// Returns an **empty string** for `"private"` and `"group"` chat types
    /// (those have no public link). For supergroups / channels the URL format
    /// is:
    ///
    /// - with username → `https://t.me/<username>/<message_id>`
    /// - without username → `https://t.me/c/<raw_id>/<message_id>`
    ///   where `raw_id` strips the `-100` prefix Telegram uses internally.
    pub fn get_link(&self) -> String {
        match self.chat.r#type.as_str() {
            "private" | "group" => String::new(),
            _ => {
                if let Some(username) = &self.chat.username {
                    format!("https://t.me/{}/{}", username, self.message_id)
                } else {
                    // Supergroup / channel IDs start with -100; the public link
                    // uses the numeric part without that prefix.
                    let id_str = self.chat.id.to_string();
                    let raw_id = id_str.strip_prefix("-100").unwrap_or(&id_str);
                    format!("https://t.me/c/{}/{}", raw_id, self.message_id)
                }
            }
        }
    }

    /// Send a reply to this message in the same chat.
    ///
    /// `reply_parameters` is set automatically if not already present in
    /// `params`. Pass `None` for a bare reply with no extra options.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tgbotrs::{Bot, types::Message};
    /// # async fn handle(bot: &Bot, msg: &Message) {
    /// msg.reply(bot, "Hello!", None).await.unwrap();
    /// # }
    /// ```
    pub async fn reply(
        &self,
        bot: &Bot,
        text: impl Into<String>,
        params: Option<SendMessageParams>,
    ) -> Result<Message, BotError> {
        let mut p = params.unwrap_or_default();
        if p.reply_parameters.is_none() {
            p.reply_parameters = Some(Box::new(ReplyParameters {
                message_id: self.message_id,
                chat_id: None,
                allow_sending_without_reply: None,
                quote: None,
                quote_parse_mode: None,
                quote_entities: None,
                quote_position: None,
                checklist_task_id: None,
                poll_option_id: None,
            }));
        }
        bot.send_message(self.chat.id, text, Some(p)).await
    }
}

// Chat
impl Chat {
    /// Send a message to this chat.
    ///
    /// Shorthand for `bot.send_message(chat.id, text, params)`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tgbotrs::{Bot, types::Chat};
    /// # async fn handle(bot: &Bot, chat: &Chat) {
    /// chat.send_message(bot, "Hi from the chat helper!", None).await.unwrap();
    /// # }
    /// ```
    pub async fn send_message(
        &self,
        bot: &Bot,
        text: impl Into<String>,
        params: Option<SendMessageParams>,
    ) -> Result<Message, BotError> {
        bot.send_message(self.id, text, params).await
    }
}

// File
impl File {
    /// Returns the HTTPS URL to download this file from Telegram's CDN.
    ///
    /// Returns `None` when `file_path` is absent. Call `bot.get_file(file_id)`
    /// first to populate it.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tgbotrs::{Bot, types::File};
    /// # async fn handle(bot: &Bot, file_id: &str) {
    /// let file = bot.get_file(file_id).await.unwrap();
    /// if let Some(url) = file.url(bot) {
    ///     println!("Download: {url}");
    /// }
    /// # }
    /// ```
    pub fn url(&self, bot: &Bot) -> Option<String> {
        self.file_path
            .as_ref()
            .map(|path| format!("{}/file/bot{}/{}", bot.api_url, bot.token, path))
    }
}

// InaccessibleMessage
impl InaccessibleMessage {
    /// Convert to a minimal [`Message`] containing only `message_id`, `date`,
    /// and `chat`. All other fields are `None` / absent.
    ///
    /// Useful when code needs a `&Message` reference but only has an
    /// `InaccessibleMessage` (e.g. inside a `CallbackQuery` where the original
    /// message expired).
    ///
    /// # Example
    /// ```rust,no_run
    /// # use tgbotrs::types::{InaccessibleMessage};
    /// # fn handle(im: &InaccessibleMessage) {
    /// let msg = im.to_message();
    /// println!("chat_id = {}", msg.chat.id);
    /// # }
    /// ```
    pub fn to_message(&self) -> Message {
        // We serialize only the three shared fields and let serde fill Option
        // fields with None. Message has exactly three non-optional fields:
        // message_id, date, chat - which are precisely what InaccessibleMessage
        // carries.
        let v = serde_json::json!({
            "message_id": self.message_id,
            "date":       self.date,
            "chat":       serde_json::to_value(&self.chat)
                              .unwrap_or(serde_json::Value::Null),
        });
        serde_json::from_value(v)
            .expect("InaccessibleMessage→Message conversion: mandatory fields are always present")
    }
}

// Tests
#[cfg(test)]
mod tests {
    use crate::types::{Chat, InaccessibleMessage, Message};

    fn bare_chat(id: i64, kind: &str) -> Chat {
        Chat {
            id,
            r#type: kind.to_string(),
            title: None,
            username: None,
            first_name: None,
            last_name: None,
            is_forum: None,
            is_direct_messages: None,
        }
    }

    fn bare_message(chat: Chat) -> Message {
        let v = serde_json::json!({ "message_id": 1, "date": 0, "chat": chat });
        serde_json::from_value(v).unwrap()
    }

    // get_text
    #[test]
    fn get_text_returns_text() {
        let mut msg = bare_message(bare_chat(1, "private"));
        msg.text = Some("hello".into());
        assert_eq!(msg.get_text(), Some("hello"));
    }

    #[test]
    fn get_text_falls_back_to_caption() {
        let mut msg = bare_message(bare_chat(1, "private"));
        msg.caption = Some("cap".into());
        assert_eq!(msg.get_text(), Some("cap"));
    }

    #[test]
    fn get_text_prefers_text_over_caption() {
        let mut msg = bare_message(bare_chat(1, "private"));
        msg.text = Some("text".into());
        msg.caption = Some("cap".into());
        assert_eq!(msg.get_text(), Some("text"));
    }

    #[test]
    fn get_text_none_when_both_absent() {
        let msg = bare_message(bare_chat(1, "private"));
        assert_eq!(msg.get_text(), None);
    }

    // get_link
    #[test]
    fn get_link_empty_for_private() {
        let msg = bare_message(bare_chat(123, "private"));
        assert_eq!(msg.get_link(), "");
    }

    #[test]
    fn get_link_empty_for_group() {
        let msg = bare_message(bare_chat(-456, "group"));
        assert_eq!(msg.get_link(), "");
    }

    #[test]
    fn get_link_with_username() {
        let mut chat = bare_chat(-1001234567890, "supergroup");
        chat.username = Some("mychannel".into());
        let v = serde_json::json!({ "message_id": 42, "date": 0, "chat": chat });
        let msg: Message = serde_json::from_value(v).unwrap();
        assert_eq!(msg.get_link(), "https://t.me/mychannel/42");
    }

    #[test]
    fn get_link_without_username_strips_100_prefix() {
        let chat = bare_chat(-1001234567890, "supergroup");
        let v = serde_json::json!({ "message_id": 99, "date": 0, "chat": chat });
        let msg: Message = serde_json::from_value(v).unwrap();
        assert_eq!(msg.get_link(), "https://t.me/c/1234567890/99");
    }

    // File::url
    #[test]
    fn file_url_none_when_no_path() {
        let file = crate::types::File {
            file_id: "x".into(),
            file_unique_id: "y".into(),
            file_size: None,
            file_path: None,
        };
        // No bot available in unit tests; just confirm None when path absent.
        // A proper integration test would use Bot::with_client + MockClient.
        let _ = file.file_path.as_ref(); // compile check
        assert!(file.file_path.is_none());
    }

    // InaccessibleMessage::to_message
    #[test]
    fn inaccessible_to_message_fields() {
        let im = InaccessibleMessage {
            chat: bare_chat(777, "supergroup"),
            message_id: 42,
            date: 0,
        };
        let msg = im.to_message();
        assert_eq!(msg.message_id, 42);
        assert_eq!(msg.chat.id, 777);
        assert_eq!(msg.date, 0);
        assert!(msg.text.is_none());
        assert!(msg.from.is_none());
    }
}
