use crate::{types::User, BotError};
use reqwest::Client;
use serde::Deserialize;

const DEFAULT_API_URL: &str = "https://api.telegram.org";

/// The main Bot struct. Create one per bot token.
///
/// # Example
/// ```rust,no_run
/// # use tgbotrs::Bot;
/// # #[tokio::main]
/// # async fn main() {
/// let bot = Bot::new("YOUR_TOKEN").await.unwrap();
/// println!("Running as @{}", bot.me.username.as_deref().unwrap_or(""));
/// # }
/// ```
#[derive(Debug, Clone)]
pub struct Bot {
    /// The bot's token.
    pub token: String,
    /// Info about the bot, retrieved on creation via getMe.
    pub me: User,
    /// The API base URL (default: https://api.telegram.org).
    pub api_url: String,
    /// The underlying HTTP client.
    pub(crate) client: Client,
}

#[derive(Debug, Deserialize)]
struct TelegramResponse<T> {
    ok: bool,
    result: Option<T>,
    error_code: Option<i64>,
    description: Option<String>,
    parameters: Option<ResponseParameters>,
}

#[derive(Debug, Deserialize)]
struct ResponseParameters {
    migrate_to_chat_id: Option<i64>,
    retry_after: Option<i64>,
}

impl Bot {
    /// Create a new Bot and verify the token by calling getMe.
    pub async fn new(token: impl Into<String>) -> Result<Self, BotError> {
        Self::with_api_url(token, DEFAULT_API_URL).await
    }

    /// Create a Bot with a custom API server URL (e.g., for local Bot API server).
    pub async fn with_api_url(
        token: impl Into<String>,
        api_url: impl Into<String>,
    ) -> Result<Self, BotError> {
        let token = token.into();
        let api_url = api_url.into();

        // Validate token format
        if !token.contains(':') {
            return Err(BotError::InvalidToken);
        }

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(BotError::Http)?;

        let mut bot = Bot {
            token,
            me: User {
                id: 0,
                is_bot: true,
                first_name: String::new(),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
                can_join_groups: None,
                can_read_all_group_messages: None,
                supports_inline_queries: None,
                can_connect_to_business: None,
                has_main_web_app: None,
                has_topics_enabled: None,
                allows_users_to_create_topics: None,
            },
            api_url,
            client,
        };

        // Call getMe to verify and populate bot info
        let me: User = bot.call_api("getMe", &serde_json::json!({})).await?;
        bot.me = me;

        Ok(bot)
    }

    /// Create a Bot without verifying the token (skips getMe call).
    pub fn new_unverified(token: impl Into<String>) -> Self {
        Bot {
            token: token.into(),
            me: User {
                id: 0,
                is_bot: true,
                first_name: String::new(),
                last_name: None,
                username: None,
                language_code: None,
                is_premium: None,
                added_to_attachment_menu: None,
                can_join_groups: None,
                can_read_all_group_messages: None,
                supports_inline_queries: None,
                can_connect_to_business: None,
                has_main_web_app: None,
                has_topics_enabled: None,
                allows_users_to_create_topics: None,
            },
            api_url: DEFAULT_API_URL.to_string(),
            client: Client::new(),
        }
    }

    /// Get the full API endpoint URL for a method.
    pub fn endpoint(&self, method: &str) -> String {
        format!("{}/bot{}/{}", self.api_url, self.token, method)
    }

    /// Make a raw API call with a JSON body.
    pub async fn call_api<T>(&self, method: &str, body: &serde_json::Value) -> Result<T, BotError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.endpoint(method);

        let response = self
            .client
            .post(&url)
            .json(body)
            .send()
            .await
            .map_err(BotError::Http)?;

        let tg_response: TelegramResponse<T> = response.json().await.map_err(BotError::Http)?;

        if tg_response.ok {
            tg_response
                .result
                .ok_or_else(|| BotError::Other("ok=true but result is null".into()))
        } else {
            Err(BotError::Api {
                code: tg_response.error_code.unwrap_or(0),
                description: tg_response
                    .description
                    .unwrap_or_else(|| "Unknown error".into()),
                retry_after: tg_response.parameters.as_ref().and_then(|p| p.retry_after),
                migrate_to_chat_id: tg_response
                    .parameters
                    .as_ref()
                    .and_then(|p| p.migrate_to_chat_id),
            })
        }
    }

    /// Make a raw API call using multipart/form-data (for file uploads).
    pub async fn call_api_multipart<T>(
        &self,
        method: &str,
        form: reqwest::multipart::Form,
    ) -> Result<T, BotError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.endpoint(method);

        let response = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(BotError::Http)?;

        let tg_response: TelegramResponse<T> = response.json().await.map_err(BotError::Http)?;

        if tg_response.ok {
            tg_response
                .result
                .ok_or_else(|| BotError::Other("ok=true but result is null".into()))
        } else {
            Err(BotError::Api {
                code: tg_response.error_code.unwrap_or(0),
                description: tg_response
                    .description
                    .unwrap_or_else(|| "Unknown error".into()),
                retry_after: tg_response.parameters.as_ref().and_then(|p| p.retry_after),
                migrate_to_chat_id: tg_response
                    .parameters
                    .as_ref()
                    .and_then(|p| p.migrate_to_chat_id),
            })
        }
    }
}
