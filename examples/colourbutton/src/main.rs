//! color_buttons_bot — Bot API 9.4 button styles + inline query
//!
//! Button styles (new in 9.4):
//!   "primary" → 🔵 blue   "success" → 🟢 green   "danger" → 🔴 red
//!
//! Commands:  /start  /reply  /all
//! Inline:    @yourbot <query>
//!
//! Run:
//!   BOT_TOKEN=your_token cargo run --example color_buttons_bot

use tgbotrs::{
    gen_methods::{AnswerCallbackQueryParams, AnswerInlineQueryParams, SendMessageParams},
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResult,
        InlineQueryResultArticle, InputMessageContent, InputTextMessageContent,
        KeyboardButton, ReplyKeyboardMarkup,
    },
    Bot, Poller, ReplyMarkup, UpdateHandler,
};

// ── Button helpers ────────────────────────────────────────────────────────────

fn ibtn(text: &str, data: &str, style: Option<&str>) -> InlineKeyboardButton {
    InlineKeyboardButton {
        text: text.to_string(),
        callback_data: Some(data.to_string()),
        style: style.map(|s| s.to_string()),
        icon_custom_emoji_id: None,
        url: None,
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

fn rbtn(text: &str, style: Option<&str>) -> KeyboardButton {
    KeyboardButton {
        text: text.to_string(),
        style: style.map(|s| s.to_string()),
        icon_custom_emoji_id: None,
        request_users: None,
        request_chat: None,
        request_contact: None,
        request_location: None,
        request_poll: None,
        web_app: None,
    }
}

// ── Keyboards ─────────────────────────────────────────────────────────────────

fn colored_inline_keyboard() -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: vec![
            vec![
                ibtn("🔵 Primary", "primary", Some("primary")),
                ibtn("🟢 Success", "success", Some("success")),
                ibtn("🔴 Danger",  "danger",  Some("danger")),
            ],
            vec![
                ibtn("⬜ Default", "default", None),
            ],
            vec![
                ibtn("✅ Confirm", "confirm", Some("success")),
                ibtn("❌ Cancel",  "cancel",  Some("danger")),
            ],
        ],
    }
}

fn colored_reply_keyboard() -> ReplyKeyboardMarkup {
    ReplyKeyboardMarkup {
        keyboard: vec![
            vec![
                rbtn("🔵 Primary", Some("primary")),
                rbtn("🟢 Success", Some("success")),
                rbtn("🔴 Danger",  Some("danger")),
            ],
            vec![
                rbtn("⬜ Default A", None),
                rbtn("⬜ Default B", None),
            ],
        ],
        is_persistent: None,
        resize_keyboard: Some(true),
        one_time_keyboard: Some(false),
        input_field_placeholder: None,
        selective: None,
    }
}

// ── Inline query results ──────────────────────────────────────────────────────

fn inline_results(query: &str) -> Vec<InlineQueryResult> {
    let items: &[(&str, &str, &str, &str)] = &[
        ("1", "🔵 Primary", "primary", "Blue — use for main actions"),
        ("2", "🟢 Success", "success", "Green — use for confirmations"),
        ("3", "🔴 Danger",  "danger",  "Red — use for destructive actions"),
        ("4", "⬜ Default", "default", "No style — app default colour"),
    ];

    items
        .iter()
        .filter(|(_, title, style, _)| {
            query.is_empty()
                || title.to_lowercase().contains(&query.to_lowercase())
                || style.to_lowercase().contains(&query.to_lowercase())
        })
        .map(|(id, title, style, desc)| {
            let btn_style = if *style == "default" { None } else { Some(*style) };
            let keyboard = InlineKeyboardMarkup {
                inline_keyboard: vec![vec![ibtn(
                    title,
                    &format!("inline_{}", style),
                    btn_style,
                )]],
            };

            InlineQueryResult::InlineQueryResultArticle(InlineQueryResultArticle {
                r#type: "article".to_string(),
                id: id.to_string(),
                title: title.to_string(),
                input_message_content: InputMessageContent::InputTextMessageContent(
                    InputTextMessageContent {
                        message_text: format!("<b>{}</b>\n\n{}", title, desc),
                        parse_mode: Some("HTML".to_string()),
                        entities: None,
                        link_preview_options: None,
                    },
                ),
                reply_markup: Some(Box::new(keyboard)),
                url: None,
                description: Some(desc.to_string()),
                thumbnail_url: None,
                thumbnail_width: None,
                thumbnail_height: None,
            })
        })
        .collect()
}

// ── Main ──────────────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Set BOT_TOKEN env var");

    let bot = Bot::new(token).await.expect("Failed to create bot");
    println!(
        "🤖 Running as @{}",
        bot.me.username.as_deref().unwrap_or("unknown")
    );
    println!("Commands: /start  /reply  /all");

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            // ── Commands & messages ───────────────────────────────────────
            if let Some(msg) = update.message {
                let chat_id = msg.chat.id;
                let text = msg.text.as_deref().unwrap_or("");

                match text {
                    "/start" => {
                        let params = SendMessageParams::new()
                            .parse_mode("HTML".to_string())
                            .reply_markup(ReplyMarkup::InlineKeyboard(
                                colored_inline_keyboard(),
                            ));
                        let _ = bot
                            .send_message(
                                chat_id,
                                "<b>🎨 Button Styles</b> — Bot API 9.4\n\n\
                                <code>primary</code> → 🔵 blue\n\
                                <code>success</code> → 🟢 green\n\
                                <code>danger</code>  → 🔴 red\n\
                                (none)          → ⬜ default",
                                Some(params),
                            )
                            .await;
                    }

                    "/reply" => {
                        let params = SendMessageParams::new()
                            .reply_markup(ReplyMarkup::ReplyKeyboard(
                                colored_reply_keyboard(),
                            ));
                        let _ = bot
                            .send_message(
                                chat_id,
                                "⌨️ Colored reply keyboard — tap any button:",
                                Some(params),
                            )
                            .await;
                    }

                    "/all" => {
                        let keyboard = InlineKeyboardMarkup {
                            inline_keyboard: vec![
                                vec![
                                    ibtn("🔵 primary", "all_primary", Some("primary")),
                                    ibtn("🟢 success", "all_success", Some("success")),
                                    ibtn("🔴 danger",  "all_danger",  Some("danger")),
                                    ibtn("⬜ default", "all_default", None),
                                ],
                                vec![
                                    ibtn("✅ Confirm", "all_confirm", Some("success")),
                                    ibtn("⚠️ Warning", "all_warn",    Some("primary")),
                                    ibtn("🗑 Delete",  "all_delete",  Some("danger")),
                                ],
                            ],
                        };
                        let params = SendMessageParams::new()
                            .parse_mode("HTML".to_string())
                            .reply_markup(ReplyMarkup::InlineKeyboard(keyboard));
                        let _ = bot
                            .send_message(
                                chat_id,
                                "<b>All styles at a glance</b>\n\nAlso try inline: type <code>@YourBot</code> in any chat",
                                Some(params),
                            )
                            .await;
                    }

                    other if !other.is_empty() && !other.starts_with('/') => {
                        let _ = bot
                            .send_message(
                                chat_id,
                                format!("You pressed: <b>{}</b>", other),
                                Some(SendMessageParams::new().parse_mode("HTML".to_string())),
                            )
                            .await;
                    }

                    _ => {}
                }
            }

            // ── Callback queries ──────────────────────────────────────────
            if let Some(cq) = update.callback_query {
                let data = cq.data.as_deref().unwrap_or("");
                let label = if data.contains("primary") {
                    "🔵 primary — blue"
                } else if data.contains("success") || data.contains("confirm") {
                    "🟢 success — green"
                } else if data.contains("danger") || data.contains("cancel") || data.contains("delete") {
                    "🔴 danger — red"
                } else {
                    "⬜ default — no style"
                };

                let _ = bot
                    .answer_callback_query(
                        cq.id.clone(),
                        Some(
                            AnswerCallbackQueryParams::new()
                                .text(label.to_string())
                                .show_alert(true),
                        ),
                    )
                    .await;
            }

            // ── Inline queries ────────────────────────────────────────────
            if let Some(iq) = update.inline_query {
                let results = inline_results(&iq.query);
                let _ = bot
                    .answer_inline_query(
                        iq.id.clone(),
                        results,
                        Some(AnswerInlineQueryParams::new().cache_time(0i64)),
                    )
                    .await;
            }
        })
    });

    Poller::new(bot, handler)
        .timeout(30)
        .allowed_updates(vec![
            "message".to_string(),
            "callback_query".to_string(),
            "inline_query".to_string(),
        ])
        .start()
        .await
        .expect("Polling failed");
}
