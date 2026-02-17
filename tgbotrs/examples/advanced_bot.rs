//! Advanced bot example — inline keyboards, parse mode, file sending.
//!
//! Run: BOT_TOKEN=your_token cargo run --example advanced_bot

use tgbotrs::gen_methods::{SendMessageParams, SendPhotoParams};
use tgbotrs::types::{InlineKeyboardButton, InlineKeyboardMarkup};
use tgbotrs::{Bot, Poller, ReplyMarkup, UpdateHandler};

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Set BOT_TOKEN");
    let bot = Bot::new(token).await.unwrap();
    println!("Running as @{}", bot.me.username.as_deref().unwrap_or("?"));

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            if let Some(msg) = update.message {
                let chat_id = msg.chat.id;

                if let Some(text) = &msg.text {
                    match text.as_str() {
                        "/start" => {
                            // Send a formatted message with an inline keyboard
                            let keyboard = InlineKeyboardMarkup {
                                inline_keyboard: vec![vec![
                                    InlineKeyboardButton {
                                        text: "🦀 Rust".into(),
                                        callback_data: Some("rust".into()),
                                        ..Default::default()
                                    },
                                    InlineKeyboardButton {
                                        text: "📘 Docs".into(),
                                        url: Some("https://core.telegram.org/bots/api".into()),
                                        ..Default::default()
                                    },
                                ]],
                            };

                            let params = SendMessageParams::new()
                                .parse_mode("HTML".to_string())
                                .reply_markup(ReplyMarkup::InlineKeyboard(keyboard));

                            let _ = bot
                                .send_message(
                                    chat_id,
                                    "<b>Welcome to tgbotrs!</b>\n\nChoose an option below:",
                                    Some(params),
                                )
                                .await;
                        }
                        "/photo" => {
                            // Send a photo by URL
                            let params = SendPhotoParams::new().caption("A Ferris! 🦀".to_string());

                            let _ = bot
                                .send_photo(
                                    chat_id,
                                    "https://www.rust-lang.org/logos/rust-logo-512x512.png",
                                    Some(params),
                                )
                                .await;
                        }
                        _ => {
                            // Echo back
                            let _ = bot.send_message(chat_id, text.clone(), None).await;
                        }
                    }
                }

                // Handle callback queries
            } else if let Some(cq) = update.callback_query {
                if let Some(data) = &cq.data {
                    let reply = format!("You clicked: {}", data);
                    let _ = bot
                        .answer_callback_query(
                            cq.id.clone(),
                            Some(
                                tgbotrs::gen_methods::AnswerCallbackQueryParams::new().text(reply),
                            ),
                        )
                        .await;
                }
            }
        })
    });

    Poller::new(bot, handler).timeout(30).start().await.unwrap();
}
