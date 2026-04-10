use crate::gen_methods::GetUpdatesParams;
use crate::types::Update;
use crate::{Bot, BotError};
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use tracing::{error, info, warn};

/// A function that handles an incoming update.
pub type UpdateHandler =
    Box<dyn Fn(Bot, Update) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync>;

/// Long-polling update dispatcher.
pub struct Poller {
    bot: Bot,
    handler: UpdateHandler,
    /// Seconds to long-poll per request (0 = short poll).
    timeout: i64,
    /// Max updates per `getUpdates` call.
    limit: i64,
    /// Update types to receive (empty = all).
    allowed_updates: Vec<String>,
}

impl Poller {
    /// Create a new Poller with the given bot and handler.
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

    /// Set the max number of updates per `getUpdates` call.
    pub fn limit(mut self, l: i64) -> Self {
        self.limit = l;
        self
    }

    /// Filter which update types to receive.
    pub fn allowed_updates(mut self, updates: Vec<String>) -> Self {
        self.allowed_updates = updates;
        self
    }

    /// Start polling. Runs until the process exits or an unrecoverable error occurs.
    pub async fn start(self) -> Result<(), BotError> {
        let mut offset: i64 = 0;
        // Clone once — this Vec is immutable for the lifetime of the poller.
        let allowed_updates = if self.allowed_updates.is_empty() {
            None
        } else {
            Some(self.allowed_updates.clone())
        };

        info!("polling started");

        loop {
            let mut params = GetUpdatesParams::new()
                .offset(offset)
                .timeout(self.timeout)
                .limit(self.limit);

            if let Some(ref au) = allowed_updates {
                params = params.allowed_updates(au.clone());
            }

            let updates = match self.bot.get_updates(Some(params)).await {
                Ok(u) => u,
                Err(e) => {
                    // On flood-wait (429) honour the server-supplied retry_after.
                    // On any other error back off 3 s to avoid hammering the API.
                    let sleep_secs = match &e {
                        BotError::Api {
                            retry_after: Some(secs),
                            ..
                        } => {
                            warn!(retry_after = secs, "flood-wait on getUpdates");
                            *secs as u64
                        }
                        _ => {
                            error!(error = %e, "getUpdates error, retrying in 3 s");
                            3
                        }
                    };
                    tokio::time::sleep(Duration::from_secs(sleep_secs)).await;
                    continue;
                }
            };

            for update in updates {
                offset = update.update_id + 1;
                let bot_clone = self.bot.clone();
                let fut = (self.handler)(bot_clone, update);

                // Single spawn per update. Tokio catches panics at the task
                // boundary — a panic aborts only this task, not the poller.
                tokio::spawn(fut);
            }
        }
    }
}
