use std::sync::Arc;

use serde::Deserialize;

use crate::{
    client::{BotClient, FormPart, ReqwestClient},
    input_file::{InputFile, InputFileOrString},
    types::User,
    BotError,
};

fn infer_mime(filename: &str) -> String {
    let ext = filename.rsplit('.').next().unwrap_or("").to_lowercase();
    match ext.as_str() {
        "jpg" | "jpeg" => "image/jpeg",
        "png" => "image/png",
        "gif" => "image/gif",
        "webp" => "image/webp",
        "mp4" => "video/mp4",
        "mp3" => "audio/mpeg",
        "ogg" => "audio/ogg",
        "pdf" => "application/pdf",
        "webm" => "video/webm",
        _ => "application/octet-stream",
    }
    .to_string()
}

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
    pub token: String,
    /// Bot info populated via `getMe` on creation.
    pub me: User,
    /// API base URL (default: `https://api.telegram.org`).
    pub api_url: String,
    /// Pre-computed "{api_url}/bot{token}/" — avoids a format! on every API call.
    pub(crate) base: String,
    /// Pluggable HTTP back-end. Defaults to [`ReqwestClient`].
    pub(crate) client: Arc<dyn BotClient>,
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

fn parse_bot_id(token: &str) -> Result<i64, BotError> {
    token
        .split(':')
        .next()
        .and_then(|s| s.parse::<i64>().ok())
        .ok_or(BotError::InvalidToken)
}

fn stub_user(id: i64) -> User {
    User {
        id,
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
        can_manage_bots: None,
    }
}

impl Bot {
    // Constructors
    /// Create a new Bot and verify the token by calling `getMe`.
    /// Uses a **30-second** HTTP timeout.
    pub async fn new(token: impl Into<String>) -> Result<Self, BotError> {
        Self::with_timeout(token, DEFAULT_API_URL, std::time::Duration::from_secs(30)).await
    }

    /// Create a Bot pointing at a custom API server. Calls `getMe` on creation.
    pub async fn with_api_url(
        token: impl Into<String>,
        api_url: impl Into<String>,
    ) -> Result<Self, BotError> {
        Self::with_timeout(token, api_url, std::time::Duration::from_secs(30)).await
    }

    /// Create a Bot with a custom HTTP timeout and API URL. Calls `getMe`.
    pub async fn with_timeout(
        token: impl Into<String>,
        api_url: impl Into<String>,
        timeout: std::time::Duration,
    ) -> Result<Self, BotError> {
        let token = token.into();
        let api_url = api_url.into();

        if !token.contains(':') {
            return Err(BotError::InvalidToken);
        }

        let bot_id = parse_bot_id(&token)?;
        let client = ReqwestClient::with_timeout(timeout)?;

        let base = format!("{}/bot{}/", api_url, token);
        let mut bot = Bot {
            token,
            me: stub_user(bot_id),
            api_url,
            base,
            client: Arc::new(client),
        };

        bot.me = bot.call_api("getMe", serde_json::json!({})).await?;
        Ok(bot)
    }

    /// Create a Bot with a **custom HTTP client** implementing [`BotClient`].
    ///
    /// The main hook for testing or custom transports. The bot ID is parsed
    /// from the token so `bot.me.id` is always valid. Call `getMe` yourself
    /// if you need the rest of the `me` fields populated.
    pub fn with_client(
        token: impl Into<String>,
        api_url: impl Into<String>,
        client: impl BotClient + 'static,
    ) -> Result<Self, BotError> {
        let token = token.into();
        if !token.contains(':') {
            return Err(BotError::InvalidToken);
        }
        let bot_id = parse_bot_id(&token)?;
        let api_url = api_url.into();
        let base = format!("{}/bot{}/", api_url, token);
        Ok(Bot {
            token,
            me: stub_user(bot_id),
            api_url,
            base,
            client: Arc::new(client),
        })
    }

    /// Create a Bot **without** calling `getMe` (no network on startup).
    ///
    /// The bot ID is parsed from the token so `bot.me.id` is always valid.
    /// All other `me` fields are left as zero-values until you call `getMe`.
    pub fn new_unverified(token: impl Into<String>) -> Result<Self, BotError> {
        let token = token.into();
        let bot_id = parse_bot_id(&token)?;
        let client = ReqwestClient::with_timeout(std::time::Duration::from_secs(30))
            .expect("default client should build");
        let api_url = DEFAULT_API_URL.to_string();
        let base = format!("{}/bot{}/", api_url, token);
        Ok(Bot {
            token,
            me: stub_user(bot_id),
            api_url,
            base,
            client: Arc::new(client),
        })
    }

    // API plumbing
    /// Build the full endpoint URL for a Telegram method name.
    pub fn endpoint(&self, method: &str) -> String {
        // `self.base` is pre-computed as "{api_url}/bot{token}/" at construction time.
        format!("{}{}", self.base, method)
    }

    /// Make a JSON API call and deserialize the result.
    pub async fn call_api<T>(&self, method: &str, body: serde_json::Value) -> Result<T, BotError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.endpoint(method);
        let bytes = self.client.post_json(&url, body).await?;
        let tg: TelegramResponse<T> = serde_json::from_slice(&bytes)?;
        self.unwrap_response(tg)
    }

    /// Make an API call using multipart when a `Memory` file is present,
    /// JSON otherwise.
    pub async fn call_api_with_file<T>(
        &self,
        method: &str,
        body: serde_json::Map<String, serde_json::Value>,
        file_field: &str,
        file: InputFileOrString,
    ) -> Result<T, BotError>
    where
        T: for<'de> Deserialize<'de>,
    {
        match file {
            InputFileOrString::File(InputFile::Memory { filename, data }) => {
                let mime = infer_mime(&filename);

                let mut parts: Vec<FormPart> = body
                    .into_iter()
                    .filter(|(_, v)| !v.is_null())
                    .map(|(k, v)| {
                        let text = match &v {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        };
                        FormPart::text(k, text)
                    })
                    .collect();

                parts.push(FormPart::bytes(file_field, filename, mime, data));
                self.call_api_multipart(method, parts).await
            }
            other => {
                let mut req = body;
                req.insert(
                    file_field.into(),
                    serde_json::to_value(other).unwrap_or_default(),
                );
                self.call_api(method, serde_json::Value::Object(req)).await
            }
        }
    }

    /// Make a `multipart/form-data` API call directly from [`FormPart`]s.
    pub async fn call_api_multipart<T>(
        &self,
        method: &str,
        parts: Vec<FormPart>,
    ) -> Result<T, BotError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let url = self.endpoint(method);
        let bytes = self.client.post_form(&url, parts).await?;
        let tg: TelegramResponse<T> = serde_json::from_slice(&bytes)?;
        self.unwrap_response(tg)
    }

    fn unwrap_response<T>(&self, tg: TelegramResponse<T>) -> Result<T, BotError> {
        if tg.ok {
            tg.result
                .ok_or_else(|| BotError::Other("ok=true but result is null".into()))
        } else {
            Err(BotError::Api {
                code: tg.error_code.unwrap_or(0),
                description: tg.description.unwrap_or_else(|| "Unknown error".into()),
                retry_after: tg.parameters.as_ref().and_then(|p| p.retry_after),
                migrate_to_chat_id: tg.parameters.as_ref().and_then(|p| p.migrate_to_chat_id),
            })
        }
    }
}
