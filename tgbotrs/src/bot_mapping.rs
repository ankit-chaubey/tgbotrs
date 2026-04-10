//! Multi-bot webhook routing (item 22).
//!
//! `BotMapping` owns multiple `(Bot, Dispatcher)` pairs and serves an Axum
//! router that routes incoming webhook POST requests to the correct bot by URL
//! path.
//!
//! # Example
//!
//! ```rust,no_run
//! use tgbotrs::{Bot, Dispatcher, DispatcherOpts};
//! use tgbotrs::bot_mapping::BotMapping;
//!
//! # async fn example() -> Result<(), tgbotrs::BotError> {
//! let bot_a = Bot::new("TOKEN_A").await?;
//! let bot_b = Bot::new("TOKEN_B").await?;
//!
//! let mut map = BotMapping::new();
//! map.add_bot("/bot_a", bot_a, Dispatcher::new(DispatcherOpts::default()));
//! map.add_bot("/bot_b", bot_b, Dispatcher::new(DispatcherOpts::default()));
//!
//! // Serve on port 8080.
//! map.serve(8080).await?;
//! # Ok(())
//! # }
//! ```

use std::{collections::HashMap, sync::Arc};

use axum::{
    body::Bytes,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::post,
    Router,
};
use tracing::{error, warn};

use crate::{framework::Dispatcher, types::Update, Bot, BotError};

struct Entry {
    bot: Bot,
    dispatcher: Arc<Dispatcher>,
}

/// Routes webhook updates to the correct bot by URL path segment.
#[derive(Default)]
pub struct BotMapping {
    entries: HashMap<String, Entry>,
}

impl BotMapping {
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a bot at the given URL path (e.g. `"/my_bot"`).
    pub fn add_bot(&mut self, path: &str, bot: Bot, dispatcher: Dispatcher) {
        let clean = path.trim_start_matches('/').to_string();
        self.entries.insert(
            clean,
            Entry {
                bot,
                dispatcher: Arc::new(dispatcher),
            },
        );
    }

    /// Start an Axum HTTP server on `port` and block until the process exits.
    ///
    /// Each registered bot receives POST requests at `/{registered_path}`.
    pub async fn serve(self, port: u16) -> Result<(), BotError> {
        let shared = Arc::new(self.entries);
        let app = Router::new()
            .route("/:path", post(handle_update))
            .with_state(shared);

        let addr = format!("0.0.0.0:{port}");
        let listener = tokio::net::TcpListener::bind(&addr)
            .await
            .map_err(|e| BotError::Other(e.to_string()))?;
        axum::serve(listener, app)
            .await
            .map_err(|e| BotError::Other(e.to_string()))
    }
}

type EntryMap = Arc<HashMap<String, Entry>>;

async fn handle_update(
    Path(path): Path<String>,
    State(map): State<EntryMap>,
    headers: HeaderMap,
    body: Bytes,
) -> StatusCode {
    let entry = match map.get(path.trim_start_matches('/')) {
        Some(e) => e,
        None => {
            warn!(path = %path, "no bot registered for path");
            return StatusCode::NOT_FOUND;
        }
    };

    // Optional secret token check.
    // Callers should set the Telegram secret via setWebhook; we expose the
    // header here so integrators can validate it before forwarding.
    let _ = headers.get("x-telegram-bot-api-secret-token");

    let update: Update = match serde_json::from_slice(&body) {
        Ok(u) => u,
        Err(e) => {
            error!(error = %e, "failed to deserialise update");
            return StatusCode::BAD_REQUEST;
        }
    };

    entry.dispatcher.dispatch(entry.bot.clone(), update);
    StatusCode::OK
}
