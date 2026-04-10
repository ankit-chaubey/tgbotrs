//! Synchronous `ureq`-backed HTTP client (item 23).
//!
//! Enable with the `client-ureq` Cargo feature:
//!
//! ```toml
//! tgbotrs = { version = "0.3", features = ["client-ureq"] }
//! ```
//!
//! # Example
//!
//! ```rust,no_run
//! use tgbotrs::client_sync::SyncBot;
//!
//! fn main() {
//!     let bot = SyncBot::new("YOUR_TOKEN");
//!     let me = bot.call_sync("getMe", serde_json::Value::Null).unwrap();
//!     println!("{me}");
//! }
//! ```

use serde_json::Value;
use ureq::Agent;

use crate::BotError;

/// A synchronous, blocking Telegram bot client backed by `ureq`.
///
/// This wraps the raw JSON API; all methods are blocking and return
/// `Result<serde_json::Value, BotError>`. Use the async [`Bot`](crate::Bot)
/// for production bots; `SyncBot` is best for scripts and CLI tools.
#[derive(Debug, Clone)]
pub struct SyncBot {
    api_url: String,
    agent: Agent,
}

impl SyncBot {
    /// Create a `SyncBot` using the default Telegram API URL.
    pub fn new(token: &str) -> Self {
        Self::with_url(token, "https://api.telegram.org")
    }

    /// Create a `SyncBot` with a custom API URL (e.g. a local Bot API server).
    pub fn with_url(token: &str, base_url: &str) -> Self {
        let agent = ureq::AgentBuilder::new()
            .timeout(std::time::Duration::from_secs(60))
            .build();
        Self {
            api_url: format!("{}/bot{}", base_url.trim_end_matches('/'), token),
            agent,
        }
    }

    /// Call any Telegram Bot API method synchronously.
    ///
    /// - `method`: e.g. `"sendMessage"`.
    /// - `params`: a JSON value; pass `Value::Null` for methods with no params.
    pub fn call_sync(&self, method: &str, params: Value) -> Result<Value, BotError> {
        let url = format!("{}/{}", self.api_url, method);

        let response: Value = if params.is_null() {
            self.agent
                .post(&url)
                .call()
                .map_err(|e| BotError::Other(e.to_string()))?
                .into_json()
                .map_err(|e| BotError::Other(e.to_string()))?
        } else {
            self.agent
                .post(&url)
                .send_json(params)
                .map_err(|e| BotError::Other(e.to_string()))?
                .into_json()
                .map_err(|e| BotError::Other(e.to_string()))?
        };

        if response["ok"].as_bool().unwrap_or(false) {
            Ok(response["result"].clone())
        } else {
            let code = response["error_code"].as_i64().unwrap_or(0);
            let description = response["description"]
                .as_str()
                .unwrap_or("unknown error")
                .to_string();
            let retry_after = response["parameters"]["retry_after"].as_i64();
            let migrate_to_chat_id = response["parameters"]["migrate_to_chat_id"].as_i64();
            Err(BotError::Api {
                code,
                description,
                retry_after,
                migrate_to_chat_id,
            })
        }
    }
}
