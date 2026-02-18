//! Built-in webhook server for tgbotrs.
//!
//! Enable with the `webhook` feature flag:
//!
//! ```toml
//! [dependencies]
//! tgbotrs = { version = "0.1", features = ["webhook"] }
//! tokio   = { version = "1",   features = ["full"] }
//! ```
//!
//! # How it works
//!
//! 1. Calls `setWebhook` on Telegram with your public HTTPS URL.
//! 2. Starts an `axum` HTTP server that receives `POST /your-path`.
//! 3. Validates the `X-Telegram-Bot-Api-Secret-Token` header (if you set one).
//! 4. Dispatches each [`Update`] to your [`UpdateHandler`] in a spawned task.
//! 5. Returns `200 OK` immediately — Telegram retries if we're slow.
//!
//! On drop / shutdown the webhook is **not** automatically removed; call
//! [`Bot::delete_webhook`] yourself if you want to switch back to polling.
//!
//! # Example
//!
//! ```rust,no_run
//! use tgbotrs::{Bot, UpdateHandler, WebhookServer};
//!
//! #[tokio::main]
//! async fn main() {
//!     let bot = Bot::new("YOUR_TOKEN").await.unwrap();
//!
//!     let handler: UpdateHandler = Box::new(|bot, update| {
//!         Box::pin(async move {
//!             let Some(msg) = update.message else { return };
//!             let _ = bot.send_message(msg.chat.id, "Got it! 🚀", None).await;
//!         })
//!     });
//!
//!     WebhookServer::new(bot, handler)
//!         .port(8080)
//!         .secret_token("my_secret")
//!         .start("https://yourdomain.com")
//!         .await
//!         .unwrap();
//! }
//! ```

use crate::{Bot, BotError};
use crate::gen_methods::SetWebhookParams;
use crate::polling::UpdateHandler;
use crate::types::Update;

use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
    Json, Router,
};
use std::net::SocketAddr;
use std::sync::Arc;

// ── Shared state passed into every axum handler ───────────────────────────────

struct AppState {
    bot: Bot,
    handler: Arc<UpdateHandler>,
    secret_token: Option<String>,
}

// ── WebhookServer ─────────────────────────────────────────────────────────────

/// A built-in HTTP server that receives Telegram webhook updates.
///
/// Same `UpdateHandler` interface as [`Poller`](crate::Poller) — swap one line
/// to switch between long-polling and webhooks.
///
/// # Quick start
///
/// ```rust,no_run
/// use tgbotrs::{Bot, UpdateHandler, WebhookServer};
///
/// #[tokio::main]
/// async fn main() {
///     let bot = Bot::new("YOUR_TOKEN").await.unwrap();
///     let handler: UpdateHandler = Box::new(|bot, upd| {
///         Box::pin(async move {
///             if let Some(msg) = upd.message {
///                 let _ = bot.send_message(msg.chat.id, "pong", None).await;
///             }
///         })
///     });
///     WebhookServer::new(bot, handler)
///         .port(8080)
///         .start("https://yourdomain.com")
///         .await
///         .unwrap();
/// }
/// ```
pub struct WebhookServer {
    bot: Bot,
    handler: UpdateHandler,
    /// Local port to bind (default: 8080)
    port: u16,
    /// URL path that Telegram will POST to (default: "/webhook")
    path: String,
    /// Optional secret token sent in `X-Telegram-Bot-Api-Secret-Token`
    secret_token: Option<String>,
    /// Update types to receive (empty = all)
    allowed_updates: Vec<String>,
    /// Max simultaneous HTTPS connections Telegram will open (1–100)
    max_connections: Option<i64>,
    /// Whether to drop pending updates on webhook registration
    drop_pending_updates: bool,
}

impl WebhookServer {
    /// Create a new `WebhookServer` with the given bot and handler.
    pub fn new(bot: Bot, handler: UpdateHandler) -> Self {
        Self {
            bot,
            handler,
            port: 8080,
            path: "/webhook".to_string(),
            secret_token: None,
            allowed_updates: vec![],
            max_connections: None,
            drop_pending_updates: false,
        }
    }

    /// Set the local port to listen on (default: `8080`).
    ///
    /// Telegram supports ports **80, 88, 443, 8443** for webhooks.
    /// Port 8080 works fine behind a reverse proxy (nginx/caddy).
    pub fn port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Set the URL path that receives webhook POSTs (default: `"/webhook"`).
    ///
    /// The full URL Telegram will call is `{webhook_url}{path}`.
    pub fn path(mut self, path: impl Into<String>) -> Self {
        self.path = path.into();
        self
    }

    /// Set a secret token for request validation (recommended in production).
    ///
    /// Telegram will send this in the `X-Telegram-Bot-Api-Secret-Token` header.
    /// Requests without the correct token are rejected with `403 Forbidden`.
    pub fn secret_token(mut self, token: impl Into<String>) -> Self {
        self.secret_token = Some(token.into());
        self
    }

    /// Filter which update types to receive (default: all types).
    ///
    /// ```rust,no_run
    /// # use tgbotrs::{Bot, UpdateHandler, WebhookServer};
    /// # async fn example(bot: Bot, handler: UpdateHandler) {
    /// WebhookServer::new(bot, handler)
    ///     .allowed_updates(vec![
    ///         "message".to_string(),
    ///         "callback_query".to_string(),
    ///     ])
    ///     .start("https://example.com")
    ///     .await
    ///     .unwrap();
    /// # }
    /// ```
    pub fn allowed_updates(mut self, updates: Vec<String>) -> Self {
        self.allowed_updates = updates;
        self
    }

    /// Set max simultaneous connections Telegram opens (1–100, default 40).
    pub fn max_connections(mut self, n: i64) -> Self {
        self.max_connections = Some(n);
        self
    }

    /// Drop all pending updates when registering the webhook.
    pub fn drop_pending_updates(mut self) -> Self {
        self.drop_pending_updates = true;
        self
    }

    /// Register the webhook with Telegram and start the HTTP server.
    ///
    /// `webhook_url` is your public HTTPS base URL, e.g. `"https://mybot.example.com"`.
    /// The full webhook URL becomes `{webhook_url}{self.path}`.
    ///
    /// This call blocks until the server shuts down.
    pub async fn start(self, webhook_url: &str) -> Result<(), BotError> {
        // ── 1. Build the full URL ─────────────────────────────────────────
        let full_url = format!(
            "{}{}",
            webhook_url.trim_end_matches('/'),
            self.path
        );

        // ── 2. Register with Telegram ─────────────────────────────────────
        let mut params = SetWebhookParams::new();

        if let Some(ref token) = self.secret_token {
            params = params.secret_token(token.clone());
        }
        if let Some(n) = self.max_connections {
            params = params.max_connections(n);
        }
        if !self.allowed_updates.is_empty() {
            params = params.allowed_updates(self.allowed_updates.clone());
        }
        if self.drop_pending_updates {
            params = params.drop_pending_updates(true);
        }

        self.bot
            .set_webhook(full_url.clone(), Some(params))
            .await?;

        println!(
            "[tgbotrs] ✅ Webhook registered: {}",
            full_url
        );

        // ── 3. Build axum app ─────────────────────────────────────────────
        let state = Arc::new(AppState {
            bot: self.bot,
            handler: Arc::new(self.handler),
            secret_token: self.secret_token,
        });

        let app = Router::new()
            .route(&self.path, post(handle_update))
            .with_state(state);

        // ── 4. Bind and serve ─────────────────────────────────────────────
        let addr = SocketAddr::from(([0, 0, 0, 0], self.port));
        println!("[tgbotrs] 🚀 Listening on http://0.0.0.0:{}", self.port);

        let listener = tokio::net::TcpListener::bind(addr)
            .await
            .map_err(|e| BotError::Other(format!("Failed to bind port {}: {}", self.port, e)))?;

        axum::serve(listener, app)
            .await
            .map_err(|e| BotError::Other(format!("Webhook server error: {}", e)))?;

        Ok(())
    }
}

// ── Axum request handler ──────────────────────────────────────────────────────

async fn handle_update(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(update): Json<Update>,
) -> StatusCode {
    // Validate secret token if configured
    if let Some(ref expected) = state.secret_token {
        let provided = headers
            .get("x-telegram-bot-api-secret-token")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if provided != expected {
            eprintln!("[tgbotrs] ⚠️  Invalid secret token — request rejected");
            return StatusCode::FORBIDDEN;
        }
    }

    // Spawn the handler so we return 200 immediately.
    // Telegram retries if we take too long or return non-2xx.
    let bot = state.bot.clone();
    let handler = Arc::clone(&state.handler);

    tokio::spawn(async move {
        (handler)(bot, update).await;
    });

    StatusCode::OK
}
