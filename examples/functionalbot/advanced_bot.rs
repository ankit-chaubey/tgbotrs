use tgbotrs::gen_methods::{
    AnswerCallbackQueryParams, EditMessageTextParams, SendMessageParams,
};
use tgbotrs::types::{InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage};
use tgbotrs::{Bot, Poller, ReplyMarkup, UpdateHandler};

fn make_button(text: &str, data: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.into(),
        callback_data: Some(data.into()),
        ..Default::default()
    }
}

fn link_button(text: &str, url: &str) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.into(),
        url: Some(url.into()),
        ..Default::default()
    }
}

fn main_menu_keyboard() -> ReplyMarkup {
    ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![
                make_button("🌦 Weather", "weather"),
                make_button("📰 News", "news"),
            ],
            vec![
                make_button("🎲 Roll Dice", "dice"),
                make_button("💡 Random Fact", "fact"),
            ],
            vec![link_button(
                "🔗 Telegram Docs",
                "https://core.telegram.org/bots/api",
            )],
        ],
    })
}

fn back_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![vec![make_button("⬅️ Back to Menu", "menu")]],
    }
}

fn main_menu_inline_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![
                make_button("🌦 Weather", "weather"),
                make_button("📰 News", "news"),
            ],
            vec![
                make_button("🎲 Roll Dice", "dice"),
                make_button("💡 Random Fact", "fact"),
            ],
            vec![link_button(
                "🔗 Telegram Docs",
                "https://core.telegram.org/bots/api",
            )],
        ],
    }
}

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_BOT_TOKEN")
        .await
        .expect("Failed to create bot");

    println!("Running as @{}", bot.me.username.as_deref().unwrap_or("unknown"));

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            if let Some(msg) = update.message {
                let chat_id = msg.chat.id;

                if let Some(text) = &msg.text {
                    match text.as_str() {
                        "/start" | "/menu" => {
                            let params = SendMessageParams::new()
                                .parse_mode("HTML".to_string())
                                .reply_markup(main_menu_keyboard());

                            let _ = bot
                                .send_message(
                                    chat_id,
                                    "👋 <b>Welcome!</b>\n\nI'm your advanced bot. Pick an option:",
                                    Some(params),
                                )
                                .await;
                        }
                        "/help" => {
                            let params = SendMessageParams::new()
                                .parse_mode("HTML".to_string())
                                .reply_markup(ReplyMarkup::InlineKeyboard(back_keyboard()));

                            let _ = bot
                                .send_message(
                                    chat_id,
                                    "<b>📖 Help</b>\n\n\
                                    /start - Show main menu\n\
                                    /help  - This message\n\
                                    /menu  - Show menu again\n\n\
                                    Or just tap buttons below 👇",
                                    Some(params),
                                )
                                .await;
                        }
                        other => {
                            let params = SendMessageParams::new()
                                .parse_mode("HTML".to_string())
                                .reply_markup(ReplyMarkup::InlineKeyboard(back_keyboard()));

                            let reply = format!(
                                "You said: <code>{}</code>\n\nUse /menu to navigate.",
                                other
                            );
                            let _ = bot.send_message(chat_id, reply, Some(params)).await;
                        }
                    }
                }
            } else if let Some(cq) = update.callback_query {
                let cq_id = cq.id.clone();

                let (chat_id, message_id) = match &cq.message {
                    Some(m) => match m.as_ref() {
                        MaybeInaccessibleMessage::Message(msg) => (msg.chat.id, msg.message_id),
                        MaybeInaccessibleMessage::InaccessibleMessage(_) => return,
                    },
                    None => return,
                };

                let data = cq.data.as_deref().unwrap_or("");

                let _ = bot
                    .answer_callback_query(cq_id, Some(AnswerCallbackQueryParams::new()))
                    .await;

                let (new_text, keyboard) = match data {
                    "menu" => (
                        "👋 <b>Main Menu</b>\n\nPick an option:".to_string(),
                        main_menu_inline_keyboard(),
                    ),
                    "weather" => (
                        "🌦 <b>Weather</b>\n\nThis is a demo - plug in a real API here!\n\n\
                        Example: <code>20°C, Sunny ☀️</code>"
                            .to_string(),
                        back_keyboard(),
                    ),
                    "news" => (
                        "📰 <b>Latest News</b>\n\nThis is a demo - plug in a news API here!\n\n\
                        Example: <i>\"Rust 2.0 announced!\"</i>"
                            .to_string(),
                        back_keyboard(),
                    ),
                    "dice" => {
                        let roll = (rand_u8() % 6) + 1;
                        let face = ["⚀", "⚁", "⚂", "⚃", "⚄", "⚅"][(roll - 1) as usize];
                        (
                            format!("🎲 <b>Dice Roll</b>\n\nYou rolled: {} <b>{}</b>", face, roll),
                            back_keyboard(),
                        )
                    }
                    "fact" => {
                        let facts = [
                            "🦀 Rust was voted the most loved language 8 years in a row!",
                            "🤖 Telegram bots can send up to 30 messages/sec.",
                            "🌍 There are about 8 billion people on Earth.",
                            "🧠 Your brain uses ~20% of your body's energy.",
                            "🚀 Rust compiles to native code with zero runtime.",
                        ];
                        let fact = facts[rand_u8() as usize % facts.len()];
                        (
                            format!("💡 <b>Random Fact</b>\n\n{}", fact),
                            back_keyboard(),
                        )
                    }
                    _ => return,
                };

                let edit_params = EditMessageTextParams::new()
                    .chat_id(chat_id)
                    .message_id(message_id)
                    .parse_mode("HTML".to_string())
                    .reply_markup(Box::new(keyboard));

                let _ = bot.edit_message_text(new_text, Some(edit_params)).await;
            }
        })
    });

    Poller::new(bot, handler)
        .timeout(30)
        .start()
        .await
        .expect("Polling failed");
}

/// Seedless pseudo-random u8 via system time nanoseconds.
fn rand_u8() -> u8 {
    use std::time::{SystemTime, UNIX_EPOCH};
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .subsec_nanos();
    (nanos ^ (nanos >> 8) ^ (nanos >> 16)) as u8
}
