//! # tgbotrs 🦀
//!
//! **A fully-featured, auto-generated Telegram Bot API library for Rust.**
//!
//! [![Crates.io](https://img.shields.io/crates/v/tgbotrs)](https://crates.io/crates/tgbotrs)
//! [![docs.rs](https://img.shields.io/docsrs/tgbotrs)](https://docs.rs/tgbotrs)
//!
//! Created and developed by **[Ankit Chaubey](https://github.com/ankit-chaubey)**
//! - 📧 [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com)
//! - 💬 Telegram: [@ankify](https://t.me/ankify)
//!
//! All **285 types** and **165 methods** from [Telegram Bot API 9.4](https://core.telegram.org/bots/api)
//! are fully implemented and auto-generated from the
//! [api-spec](https://github.com/ankit-chaubey/api-spec) repository.
//!
//! ---
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use tgbotrs::Bot;
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new("YOUR_BOT_TOKEN").await.unwrap();
//!     println!("Running as @{}", bot.me.username.as_deref().unwrap_or("unknown"));
//!
//!     let msg = bot.send_message(123456789i64, "Hello from tgbotrs! 🦀", None).await.unwrap();
//!     println!("Sent: #{}", msg.message_id);
//! }
//! ```
//!
//! ## Echo Bot with Long Polling
//!
//! ```rust,no_run
//! use tgbotrs::{Bot, Poller, UpdateHandler};
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new("YOUR_TOKEN").await.unwrap();
//!
//!     let handler: UpdateHandler = Box::new(|bot, update| {
//!         Box::pin(async move {
//!             let Some(msg) = update.message else { return };
//!             let Some(text) = msg.text else { return };
//!             let _ = bot.send_message(msg.chat.id, text, None).await;
//!         })
//!     });
//!
//!     Poller::new(bot, handler).timeout(30).start().await.unwrap();
//! }
//! ```
//!
//! ## Regenerating from the Latest API Spec
//!
//! ```sh
//! curl -o api.json https://raw.githubusercontent.com/ankit-chaubey/api-spec/main/api.json
//! python3 codegen/codegen.py api.json tgbotrs/src/
//! cargo build
//! ```
//!
//! ## License
//!
//! MIT — Copyright (c) 2024-present Ankit Chaubey

#![allow(clippy::all)]

mod bot;
mod chat_id;
mod error;
mod input_file;
mod polling;
mod reply_markup;
pub mod types;

pub mod gen_methods;
mod gen_types;

pub use bot::Bot;
pub use chat_id::ChatId;
pub use error::BotError;
pub use input_file::{InputFile, InputFileOrString};
pub use polling::{Poller, UpdateHandler};
pub use reply_markup::ReplyMarkup;
pub use types::*;

/// The `InputMedia` enum — used for `sendMediaGroup` and related methods.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum InputMedia {
    Photo(types::InputMediaPhoto),
    Video(types::InputMediaVideo),
    Audio(types::InputMediaAudio),
    Document(types::InputMediaDocument),
    Animation(types::InputMediaAnimation),
}

impl From<types::InputMediaPhoto> for InputMedia {
    fn from(v: types::InputMediaPhoto) -> Self {
        InputMedia::Photo(v)
    }
}
impl From<types::InputMediaVideo> for InputMedia {
    fn from(v: types::InputMediaVideo) -> Self {
        InputMedia::Video(v)
    }
}
impl From<types::InputMediaAudio> for InputMedia {
    fn from(v: types::InputMediaAudio) -> Self {
        InputMedia::Audio(v)
    }
}
impl From<types::InputMediaDocument> for InputMedia {
    fn from(v: types::InputMediaDocument) -> Self {
        InputMedia::Document(v)
    }
}
impl From<types::InputMediaAnimation> for InputMedia {
    fn from(v: types::InputMediaAnimation) -> Self {
        InputMedia::Animation(v)
    }
}

/// Default impl for `InlineKeyboardButton` — only `text` is required.
impl Default for crate::gen_types::InlineKeyboardButton {
    fn default() -> Self {
        Self {
            text: String::new(),
            icon_custom_emoji_id: None,
            style: None,
            url: None,
            callback_data: None,
            web_app: None,
            login_url: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
            switch_inline_query_chosen_chat: None,
            copy_text: None,
            callback_game: None,
            pay: None,
        }
    }
}
