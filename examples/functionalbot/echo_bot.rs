//! Minimal echo bot using the tgbotrs helper methods.
//!
//! Demonstrates: `msg.get_text()`, `msg.reply()`, `Updater`.

use tgbotrs::{Bot, CommandHandler, Dispatcher, DispatcherOpts, HandlerResult, Updater, Context};

async fn echo(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        if let Some(text) = msg.get_text() {
            msg.reply(&bot, text, None).await?;
        }
    }
    Ok(())
}

async fn start(bot: Bot, ctx: Context) -> HandlerResult {
    if let Some(msg) = ctx.effective_message() {
        let link = msg.get_link();
        let info = if link.is_empty() {
            "No public link (private/group chat)".into()
        } else {
            format!("Message link: {link}")
        };
        msg.reply(&bot, format!("Hello! I'll echo your messages.\n{info}"), None).await?;
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("BOT_TOKEN").expect("BOT_TOKEN not set");
    let bot   = Bot::new(token).await.expect("failed to connect");

    let mut dp = Dispatcher::new(DispatcherOpts::default());
    dp.add_handler(CommandHandler::new("start", start));
    dp.add_handler(CommandHandler::new("echo",  echo));

    println!("Polling as @{}", bot.me.username.as_deref().unwrap_or("unknown"));
    Updater::new(bot, dp)
        .poll_timeout(30)
        .start_polling()
        .await
        .expect("polling failed");
}
