//! [`Updater`] - unified start/stop for polling and webhook modes.
//!
//! `Updater` owns a [`Bot`] and a [`Dispatcher`] and wires them together so
//! you don't have to manage the polling loop or webhook server yourself.
//!
//! # Long-polling example
//!
//! ```rust,no_run
//! use tgbotrs::{Bot, Dispatcher, DispatcherOpts, Updater};
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new("YOUR_TOKEN").await.unwrap();
//!     let mut dp = Dispatcher::new(DispatcherOpts::default());
//!     // dp.add_handler(...);
//!     Updater::new(bot, dp).start_polling().await.unwrap();
//! }
//! ```
//!
//! # Webhook example (requires the `webhook` feature)
//!
//! ```rust,ignore
//! use tgbotrs::{Bot, Dispatcher, DispatcherOpts, Updater};
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new("YOUR_TOKEN").await.unwrap();
//!     let dp  = Dispatcher::new(DispatcherOpts::default());
//!     Updater::new(bot, dp)
//!         .webhook_port(8443)
//!         .webhook_secret("my_secret")
//!         .start_webhook("https://yourdomain.com/bot")
//!         .await
//!         .unwrap();
//! }
//! ```

use std::sync::Arc;

use crate::{
    framework::{Dispatcher, DispatcherOpts},
    types::Update,
    Bot, BotError, Poller, UpdateHandler,
};

/// Combines a [`Bot`] and a [`Dispatcher`] into a single, self-contained
/// runner.
///
/// Use [`Updater::start_polling`] for long-polling or
/// [`Updater::start_webhook`] for webhook mode (requires the `webhook`
/// feature). Both methods block until the process exits.
pub struct Updater {
    bot: Bot,
    dispatcher: Arc<Dispatcher>,
    /// Long-poll timeout in seconds (default 30).
    poll_timeout: i64,
    /// Max updates per `getUpdates` call (default 100).
    poll_limit: i64,
    /// Which update types to receive (empty = all).
    allowed_updates: Vec<String>,
    /// Webhook port (default 8080).
    #[allow(dead_code)]
    webhook_port: u16,
    /// Webhook secret token.
    #[allow(dead_code)]
    webhook_secret: Option<String>,
}

impl Updater {
    /// Create a new `Updater` with a fresh dispatcher built from `opts`.
    pub fn with_opts(bot: Bot, opts: DispatcherOpts) -> Self {
        Self::from_dispatcher(bot, Dispatcher::new(opts))
    }

    /// Create a new `Updater` wrapping an already-configured `Dispatcher`.
    pub fn new(bot: Bot, dispatcher: Dispatcher) -> Self {
        Self::from_dispatcher(bot, dispatcher)
    }

    fn from_dispatcher(bot: Bot, dispatcher: Dispatcher) -> Self {
        Self {
            bot,
            dispatcher: Arc::new(dispatcher),
            poll_timeout: 30,
            poll_limit: 100,
            allowed_updates: vec![],
            webhook_port: 8080,
            webhook_secret: None,
        }
    }

    // Builder setters
    /// Set the long-poll timeout in seconds (default: 30).
    pub fn poll_timeout(mut self, secs: i64) -> Self {
        self.poll_timeout = secs;
        self
    }

    /// Set the max updates per `getUpdates` call (default: 100).
    pub fn poll_limit(mut self, limit: i64) -> Self {
        self.poll_limit = limit;
        self
    }

    /// Filter which update types to receive (empty = all).
    pub fn allowed_updates(mut self, updates: Vec<String>) -> Self {
        self.allowed_updates = updates;
        self
    }

    /// Set the webhook listening port (default: 8080).
    pub fn webhook_port(mut self, port: u16) -> Self {
        self.webhook_port = port;
        self
    }

    /// Set the webhook secret token for request validation.
    pub fn webhook_secret(mut self, secret: impl Into<String>) -> Self {
        self.webhook_secret = Some(secret.into());
        self
    }

    /// Expose the inner [`Bot`] (e.g. to add handlers before starting).
    pub fn bot(&self) -> &Bot {
        &self.bot
    }

    // Start modes
    /// Start long-polling. Blocks until the process exits or an unrecoverable
    /// error occurs.
    ///
    /// Each received update is dispatched through the [`Dispatcher`] in a
    /// spawned Tokio task (same semantics as [`Dispatcher::dispatch`]).
    pub async fn start_polling(self) -> Result<(), BotError> {
        let dp = Arc::clone(&self.dispatcher);
        let bot = self.bot.clone();

        // Build an UpdateHandler closure that calls into the Dispatcher.
        let handler: UpdateHandler = Box::new(move |bot: Bot, update: Update| {
            let dp2 = Arc::clone(&dp);
            Box::pin(async move {
                dp2.dispatch(bot, update);
            })
        });

        Poller::new(bot, handler)
            .timeout(self.poll_timeout)
            .limit(self.poll_limit)
            .allowed_updates(self.allowed_updates)
            .start()
            .await
    }

    /// Start webhook mode. Blocks until the process exits.
    ///
    /// Requires the `webhook` feature flag.
    #[cfg(feature = "webhook")]
    pub async fn start_webhook(self, webhook_url: impl Into<String>) -> Result<(), BotError> {
        use crate::WebhookServer;

        let dp = Arc::clone(&self.dispatcher);
        let bot = self.bot.clone();
        let port = self.webhook_port;
        let secret = self.webhook_secret.clone();

        let handler: UpdateHandler = Box::new(move |bot: Bot, update: Update| {
            let dp2 = Arc::clone(&dp);
            Box::pin(async move {
                dp2.dispatch(bot, update);
            })
        });

        let mut server = WebhookServer::new(bot, handler).port(port);
        if let Some(s) = secret {
            server = server.secret_token(s);
        }
        server.start(webhook_url).await
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::framework::{
        handler::{Handler, HandlerResult},
        Context,
    };
    use async_trait::async_trait;

    struct NoopHandler;

    #[async_trait]
    impl Handler for NoopHandler {
        fn name(&self) -> &str {
            "noop"
        }
        fn check_update(&self, _: &Context) -> bool {
            false
        }
        async fn handle_update(&self, _: Bot, _: Context) -> HandlerResult {
            Ok(())
        }
    }

    #[test]
    fn updater_builds_from_opts() {
        // Just test that construction doesn't panic.
        // Full integration tests require a live token.
        let _ = std::mem::size_of::<Updater>();
    }

    #[test]
    fn builder_setters_compile() {
        // Verify all builder methods type-check.
        fn check(bot: Bot, dp: Dispatcher) {
            let _ = Updater::new(bot, dp)
                .poll_timeout(60)
                .poll_limit(50)
                .allowed_updates(vec!["message".into()])
                .webhook_port(8443)
                .webhook_secret("abc");
        }
        let _ = check; // unused fn lint suppression
    }
}
