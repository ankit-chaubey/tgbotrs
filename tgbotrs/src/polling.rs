use crate::gen_methods::GetUpdatesParams;
use crate::types::Update;
use crate::{Bot, BotError};
use std::future::Future;
use std::pin::Pin;

/// A function type that handles incoming updates.
pub type UpdateHandler =
    Box<dyn Fn(Bot, Update) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Long-polling update dispatcher.
pub struct Poller {
    bot: Bot,
    handler: UpdateHandler,
    /// How many seconds to long-poll (0 = short poll).
    timeout: i64,
    /// Max updates per batch.
    limit: i64,
    /// Update types to receive (empty = all).
    allowed_updates: Vec<String>,
}

impl Poller {
    /// Create a new Poller with the given bot and update handler.
    pub fn new(bot: Bot, handler: UpdateHandler) -> Self {
        Poller {
            bot,
            handler,
            timeout: 30,
            limit: 100,
            allowed_updates: vec![],
        }
    }

    /// Set the long-polling timeout in seconds.
    pub fn timeout(mut self, t: i64) -> Self {
        self.timeout = t;
        self
    }

    /// Set the max number of updates per getUpdates call.
    pub fn limit(mut self, l: i64) -> Self {
        self.limit = l;
        self
    }

    /// Specify which update types to receive.
    pub fn allowed_updates(mut self, updates: Vec<String>) -> Self {
        self.allowed_updates = updates;
        self
    }

    /// Start polling for updates, calling the handler for each one.
    /// Runs until the process exits or an unrecoverable error occurs.
    pub async fn start(self) -> Result<(), BotError> {
        let mut offset: i64 = 0;

        log_info("tgbotrs polling started");

        loop {
            let mut params = GetUpdatesParams::new()
                .offset(offset)
                .timeout(self.timeout)
                .limit(self.limit);

            if !self.allowed_updates.is_empty() {
                params = params.allowed_updates(self.allowed_updates.clone());
            }

            let updates = match self.bot.get_updates(Some(params)).await {
                Ok(u) => u,
                Err(e) => {
                    eprintln!("[tgbotrs] getUpdates error: {}", e);
                    tokio::time::sleep(std::time::Duration::from_secs(3)).await;
                    continue;
                }
            };

            for update in updates {
                offset = update.update_id + 1;
                let bot_clone = self.bot.clone();
                let fut = (self.handler)(bot_clone, update);
                tokio::spawn(fut);
            }
        }
    }
}

fn log_info(msg: &str) {
    println!("[tgbotrs] {}", msg);
}
