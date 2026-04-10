use tgbotrs::{Bot, Poller, UpdateHandler};

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_BOT_TOKEN")
        .await
        .expect("Failed to create bot");

    println!("Running as @{}", bot.me.username.as_deref().unwrap_or("unknown"));

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            let Some(msg) = update.message else { return };
            let Some(text) = &msg.text else { return };
            let chat_id = msg.chat.id;

            let reply = match text.as_str() {
                "/start" => "👋 Hello! I'm your bot. Send me anything!".to_string(),
                "/help"  => "Commands:\n/start - Welcome\n/help - This message".to_string(),
                other    => format!("You said: {}", other),
            };

            match bot.send_message(chat_id, reply, None).await {
                Ok(_)  => println!("Replied to {}", chat_id),
                Err(e) => eprintln!("Error: {}", e),
            }
        })
    });

    Poller::new(bot, handler)
        .timeout(30)
        .start()
        .await
        .expect("Polling failed");
}
