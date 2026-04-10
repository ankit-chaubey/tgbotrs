use thiserror::Error;

/// The main error type for tgbotrs.
#[derive(Debug, Error)]
#[non_exhaustive]
pub enum BotError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Telegram returned `ok: false`.
    #[error("Telegram API error [{code}]: {description}")]
    Api {
        code: i64,
        description: String,
        /// Seconds to wait before retrying (flood-wait, code 429).
        retry_after: Option<i64>,
        /// Chat to migrate to (code 400 migration errors).
        migrate_to_chat_id: Option<i64>,
    },

    #[error("Invalid bot token")]
    InvalidToken,

    #[error("{0}")]
    Other(String),
}

impl BotError {
    /// Returns `true` if this is a Telegram API error with the given code.
    pub fn is_api_error_code(&self, expected_code: i64) -> bool {
        matches!(self, BotError::Api { code, .. } if *code == expected_code)
    }

    /// Returns the flood-wait duration in seconds if this is a 429 error.
    pub fn flood_wait_seconds(&self) -> Option<i64> {
        if let BotError::Api { retry_after, .. } = self {
            return *retry_after;
        }
        None
    }
}
