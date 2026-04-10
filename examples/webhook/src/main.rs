use tgbotrs::{
    gen_methods::{
        AnswerCallbackQueryParams, EditMessageTextParams, SendMessageParams,
    },
    InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage,
    Bot, ReplyMarkup, UpdateHandler, WebhookServer,
};

fn inline_kb(rows: Vec<Vec<(&str, &str)>>) -> InlineKeyboardMarkup {
    InlineKeyboardMarkup {
        inline_keyboard: rows
            .into_iter()
            .map(|row| {
                row.into_iter()
                    .map(|(text, data)| InlineKeyboardButton {
                        text: text.to_string(),
                        callback_data: Some(data.to_string()),
                        ..Default::default()
                    })
                    .collect()
            })
            .collect(),
    }
}

fn make_handler() -> UpdateHandler {
    Box::new(|bot, update| {
        Box::pin(async move {
            if let Some(msg) = update.message {
                let chat_id = msg.chat.id;
                let text = msg.text.as_deref().unwrap_or("");
                let name = msg
                    .from
                    .as_ref()
                    .map(|u| u.first_name.as_str())
                    .unwrap_or("there");

                match text {
                    "/start" => {
                        let params = SendMessageParams::new()
                            .parse_mode("HTML")
                            .reply_markup(ReplyMarkup::InlineKeyboard(inline_kb(vec![
                                vec![("📖 Help", "help"), ("🎲 Random fact", "fact")],
                                vec![("🌐 GitHub", "github")],
                            ])));

                        let _ = bot
                            .send_message(
                                chat_id,
                                format!(
                                    "👋 Hello, <b>{name}</b>!\n\n\
                                    I'm a webhook bot built with \
                                    <a href=\"https://github.com/ankit-chaubey/tgbotrs\">tgbotrs</a> 🦀\n\n\
                                    Pick an option below or just send me a message."
                                ),
                                Some(params),
                            )
                            .await;
                    }

                    "/help" => {
                        let params = SendMessageParams::new().parse_mode("HTML");
                        let _ = bot
                            .send_message(
                                chat_id,
                                "<b>Available commands:</b>\n\
                                /start - welcome message\n\
                                /help  - this list\n\
                                /about - about this bot\n\n\
                                Or just type anything and I'll echo it back! 🦜",
                                Some(params),
                            )
                            .await;
                    }

                    "/about" => {
                        let params = SendMessageParams::new().parse_mode("HTML");
                        let _ = bot
                            .send_message(
                                chat_id,
                                "🤖 <b>mybot</b>\n\n\
                                Built with <code>tgbotrs</code> - \
                                a fully-generated Rust Telegram Bot API library \
                                covering all 285 types and 165 methods.\n\n\
                                🔗 <a href=\"https://github.com/ankit-chaubey/tgbotrs\">Source on GitHub</a>",
                                Some(params),
                            )
                            .await;
                    }

                    _ if !text.is_empty() => {
                        let params = SendMessageParams::new().parse_mode("HTML");
                        let _ = bot
                            .send_message(
                                chat_id,
                                format!("🦜 You said:\n<blockquote>{text}</blockquote>"),
                                Some(params),
                            )
                            .await;
                    }

                    _ => {}
                }
            }

            if let Some(cbq) = update.callback_query {
                let query_id = cbq.id.clone();
                let data = cbq.data.as_deref().unwrap_or("").to_string();

                // Acknowledge immediately so the button stops spinning
                let _ = bot
                    .answer_callback_query(
                        &query_id,
                        Some(AnswerCallbackQueryParams::new()),
                    )
                    .await;

                if let Some(maybe_msg) = cbq.message {
                    if let MaybeInaccessibleMessage::Message(msg) = *maybe_msg {
                        let chat_id = msg.chat.id;
                        let message_id = msg.message_id;

                        let (new_text, new_kb) = match data.as_str() {
                            "help" => (
                                "📖 <b>Help</b>\n\n\
                                /start  - welcome screen\n\
                                /help   - command list\n\
                                /about  - about this bot\n\n\
                                Just type anything to echo it back!"
                                    .to_string(),
                                inline_kb(vec![vec![("⬅️ Back", "back")]]),
                            ),
                            "fact" => (
                                "🎲 <b>Random Rust fact:</b>\n\n\
                                Rust has no garbage collector - memory is managed \
                                at compile time through the borrow checker. \
                                Zero runtime overhead and no GC pauses! 🔥"
                                    .to_string(),
                                inline_kb(vec![
                                    vec![("🎲 Another!", "fact"), ("⬅️ Back", "back")],
                                ]),
                            ),
                            "github" => (
                                "🌐 <b>Check out the source:</b>\n\n\
                                <a href=\"https://github.com/ankit-chaubey/tgbotrs\">github.com/ankit-chaubey/tgbotrs</a>"
                                    .to_string(),
                                inline_kb(vec![vec![("⬅️ Back", "back")]]),
                            ),
                            "back" => (
                                "👋 Welcome back! Pick an option or send me a message."
                                    .to_string(),
                                inline_kb(vec![
                                    vec![("📖 Help", "help"), ("🎲 Random fact", "fact")],
                                    vec![("🌐 GitHub", "github")],
                                ]),
                            ),
                            _ => return,
                        };

                        let edit_params = EditMessageTextParams::new()
                            .chat_id(chat_id)
                            .message_id(message_id)
                            .parse_mode("HTML")
                            .reply_markup(Box::new(new_kb));

                        let _ = bot.edit_message_text(new_text, Some(edit_params)).await;
                    }
                }
            }
        })
    })
}

#[tokio::main]
async fn main() {
    let _ = dotenvy::dotenv();

    let token       = std::env::var("TOKEN").expect("TOKEN not set");
    let webhook_url = std::env::var("WEBHOOK_URL").expect("WEBHOOK_URL not set");
    let port: u16   = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".into())
        .parse()
        .expect("PORT must be a number");
    let secret = std::env::var("SECRET").unwrap_or_default();

    let bot = Bot::new(token).await.expect("Failed to init bot - check TOKEN");
    println!("Logged in as @{}", bot.me.username.as_deref().unwrap_or("unknown"));

    let mut server = WebhookServer::new(bot, make_handler())
        .port(port)
        .path("/webhook")
        .drop_pending_updates();

    if !secret.is_empty() {
        server = server.secret_token(secret);
    }

    server.start(&webhook_url).await.expect("Webhook server crashed");
}
