//! Example: testing handlers with a custom `BotClient` (no live token needed).
//!
//! Demonstrates the replaceable HTTP client via the BotClient trait.

use async_trait::async_trait;
use tgbotrs::{
    client::{BotClient, FormPart},
    framework::{Context, Dispatcher, DispatcherOpts, HandlerResult},
    Bot, BotError, CommandHandler,
};

/// Fake client that returns canned JSON for every request.
#[derive(Debug)]
struct MockClient {
    get_me_json: &'static str,
    reply_json: &'static str,
}

#[async_trait]
impl BotClient for MockClient {
    async fn post_json(
        &self,
        url: &str,
        _body: serde_json::Value,
    ) -> Result<bytes::Bytes, BotError> {
        let json = if url.ends_with("/getMe") {
            self.get_me_json
        } else {
            self.reply_json
        };
        Ok(bytes::Bytes::from(json))
    }

    async fn post_form(&self, _url: &str, _parts: Vec<FormPart>) -> Result<bytes::Bytes, BotError> {
        Ok(bytes::Bytes::from(self.reply_json))
    }
}

async fn ping_handler(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        msg.reply(&bot, "pong!", None).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let client = MockClient {
        get_me_json: r#"{"ok":true,"result":{"id":1,"is_bot":true,"first_name":"TestBot","username":"testbot"}}"#,
        reply_json: r#"{"ok":true,"result":{"message_id":2,"date":0,"chat":{"id":1,"type":"private"},"text":"pong!"}}"#,
    };

    let bot = Bot::with_client("1:TOKEN", "https://api.telegram.org", client)
        .expect("token parse failed");

    println!("Bot ID (from token): {}", bot.me.id);

    // Test a File url helper (item 15)
    let file = tgbotrs::types::File {
        file_id: "abc".into(),
        file_unique_id: "def".into(),
        file_size: Some(1024),
        file_path: Some("photos/file_1.jpg".into()),
    };
    let url = file.url(&bot).expect("file_path is set");
    println!("File URL: {url}");
    // → https://api.telegram.org/file/bot1:TOKEN/photos/file_1.jpg

    // Test InaccessibleMessage::to_message (item 16)
    let im = tgbotrs::types::InaccessibleMessage {
        chat: tgbotrs::types::Chat {
            id: 999,
            r#type: "supergroup".into(),
            title: Some("Test Group".into()),
            username: None,
            first_name: None,
            last_name: None,
            is_forum: None,
            is_direct_messages: None,
        },
        message_id: 77,
        date: 0,
    };
    let msg = im.to_message();
    println!(
        "Converted message: chat_id={}, msg_id={}",
        msg.chat.id, msg.message_id
    );

    // Dispatcher test
    let mut dp = Dispatcher::new(DispatcherOpts::default());
    dp.add_handler(CommandHandler::new("ping", ping_handler));
    let update: tgbotrs::types::Update = serde_json::from_str(
        r#"{
        "update_id": 1,
        "message": {
            "message_id": 1,
            "date": 0,
            "chat": {"id": 1, "type": "private"},
            "from": {"id": 2, "is_bot": false, "first_name": "User"},
            "text": "/ping"
        }
    }"#,
    )
    .unwrap();

    dp.process_update(&bot, update).await;
    println!("Dispatcher handled /ping successfully.");
}
