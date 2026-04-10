//! # tgbotrs
//!
//! A fully-featured, auto-generated Telegram Bot API library for Rust.
//!
//! All **285 types** and **165 methods** from [Telegram Bot API](https://core.telegram.org/bots/api)
//! Strongly typed, fully async, automatically kept in sync with every official release.
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
//!             let Some(text) = msg.get_text().map(str::to_owned) else { return };
//!             let _ = msg.reply(&bot, text, None).await;
//!         })
//!     });
//!
//!     Poller::new(bot, handler).timeout(30).start().await.unwrap();
//! }
//! ```
//!
//! ## Dispatcher + Updater (recommended for non-trivial bots)
//!
//! ```rust,no_run
//! use tgbotrs::{Bot, CommandHandler, Dispatcher, DispatcherOpts, Updater};
//! use tgbotrs::framework::{HandlerResult, Context};
//!
//! async fn start(bot: tgbotrs::Bot, ctx: Context) -> HandlerResult {
//!     if let Some(msg) = ctx.effective_message() {
//!         msg.reply(&bot, "Hello! 👋", None).await?;
//!     }
//!     Ok(())
//! }
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new("YOUR_TOKEN").await.unwrap();
//!     let mut dp = Dispatcher::new(DispatcherOpts::default());
//!     dp.add_handler(CommandHandler::new("start", start));
//!     Updater::new(bot, dp).start_polling().await.unwrap();
//! }
//! ```
//!
//! ## Webhook Server
//!
//! Enable the `webhook` feature in `Cargo.toml`:
//!
//! ```toml
//! tgbotrs = { version = "0.2", features = ["webhook"] }
//! ```
//!
//! ```rust,ignore
//! use tgbotrs::{Bot, Dispatcher, DispatcherOpts, Updater};
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new("YOUR_TOKEN").await.unwrap();
//!     let dp  = Dispatcher::new(DispatcherOpts::default());
//!     Updater::new(bot, dp)
//!         .webhook_port(8443)
//!         .webhook_secret("my_secret")
//!         .start_webhook("https://yourdomain.com/bot")
//!         .await.unwrap();
//! }
//! ```
//!
//! ## Custom HTTP Client (for testing)
//!
//! ```rust,no_run
//! use tgbotrs::{Bot, client::{BotClient, FormPart}};
//! use tgbotrs::BotError;
//! use async_trait::async_trait;
//!
//! #[derive(Debug)]
//! struct MockClient;
//!
//! #[async_trait]
//! impl BotClient for MockClient {
//!     async fn post_json(&self, _url: &str, _body: serde_json::Value)
//!         -> Result<bytes::Bytes, BotError>
//!     {
//!         Ok(bytes::Bytes::from(r#"{"ok":true,"result":{"id":42,"is_bot":true,"first_name":"Test","username":"testbot"}}"#))
//!     }
//!     async fn post_form(&self, _url: &str, _parts: Vec<FormPart>)
//!         -> Result<bytes::Bytes, BotError>
//!     {
//!         Ok(bytes::Bytes::from(r#"{"ok":true,"result":true}"#))
//!     }
//! }
//!
//! # fn main() {
//! let bot = Bot::with_client("1234:TOKEN", "https://api.telegram.org", MockClient).unwrap();
//! # }
//! ```
//!
//! ## License
//!
//! MIT License - Copyright (c) 2024-present Ankit Chaubey

#![allow(clippy::all)]

mod bot;
mod chat_id;
pub mod client;
pub mod entities;
mod error;
mod helpers; // extension impls on Message, Chat, File, InaccessibleMessage
mod input_file;
mod polling;
mod reply_markup;
pub mod types;
mod updater;

pub mod gen_methods;
mod gen_types;

#[cfg(feature = "webhook")]
mod webhook;

#[cfg(feature = "bot-mapping")]
pub mod bot_mapping;

#[cfg(feature = "client-ureq")]
pub mod client_sync;

pub use bot::Bot;
pub use chat_id::ChatId;
pub use client::{BotClient, FormBody, FormPart, ReqwestClient};
pub use entities::{parse_entities, parse_entity, MessageEntityExt, ParsedEntity};
pub use error::BotError;
pub use input_file::{InputFile, InputFileOrString};
pub use polling::{Poller, UpdateHandler};
pub use reply_markup::ReplyMarkup;
pub use types::*;
pub use updater::Updater;

#[cfg(feature = "webhook")]
pub use webhook::WebhookServer;

#[cfg(feature = "bot-mapping")]
pub use bot_mapping::BotMapping;

#[cfg(feature = "client-ureq")]
pub use client_sync::SyncBot;

/// Typed enum for `sendMediaGroup` and related methods.
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

/// `Default` for `InlineKeyboardButton` - only `text` is required by the API.
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

pub mod framework;

// Top-level re-exports for convenience.
pub use framework::{
    CallbackQueryHandler, CommandHandler, Context, ContinueGroups, ConversationHandler,
    ConversationOpts, Dispatcher, DispatcherAction, DispatcherOpts, EndConversation, EndGroups,
    FilterExt, Handler, HandlerResult, InMemoryStorage, KeyStrategy, MessageHandler, NextState,
};

#[cfg(test)]
mod tests;
