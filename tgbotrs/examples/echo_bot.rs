//! Simple echo bot example for tgbotrs.
//!
//! Run: BOT_TOKEN=your_token cargo run --example echo_bot

use tgbotrs::{Bot, Poller, UpdateHandler};

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("Set BOT_TOKEN environment variable");

    println!("Starting bot...");
    let bot = Bot::new(token).await.expect("Failed to create bot");
    println!(
        "Running as @{}",
        bot.me.username.as_deref().unwrap_or("unknown")
    );

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            // Handle text messages
            if let Some(msg) = update.message {
                if let Some(text) = &msg.text {
                    let chat_id = msg.chat.id;
                    println!("[{}] {}", chat_id, text);

                    match bot.send_message(chat_id, text.clone(), None).await {
                        Ok(sent) => println!("Replied: message_id={}", sent.message_id),
                        Err(e) => eprintln!("Error sending message: {}", e),
                    }
                }
            }
        })
    });

    println!("Polling for updates...");
    Poller::new(bot, handler)
        .timeout(30)
        .start()
        .await
        .expect("Polling failed");
}
