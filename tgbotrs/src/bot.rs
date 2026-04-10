use crate::{
    input_file::{InputFile, InputFileOrString},
    types::User,
    BotError,
};
use reqwest::Client;
use serde::Deserialize;

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

fn empty_user() -> User {
    User {
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
        can_manage_bots: None,
    }
}

impl Bot {
    /// Create a new Bot and verify the token by calling `getMe`.
    pub async fn new(token: impl Into<String>) -> Result<Self, BotError> {
        Self::with_api_url(token, DEFAULT_API_URL).await
    }

    /// Create a Bot pointing at a custom API server (e.g. local Bot API).
    pub async fn with_api_url(
        token: impl Into<String>,
        api_url: impl Into<String>,
    ) -> Result<Self, BotError> {
        let token = token.into();
        let api_url = api_url.into();

        if !token.contains(':') {
            return Err(BotError::InvalidToken);
        }

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .map_err(BotError::Http)?;

        let mut bot = Bot {
            token,
            me: empty_user(),
            api_url,
            client,
        };

        bot.me = bot.call_api("getMe", &serde_json::json!({})).await?;

        Ok(bot)
    }

    /// Create a Bot without calling `getMe` (skips token verification).
    pub fn new_unverified(token: impl Into<String>) -> Self {
        Bot {
            token: token.into(),
            me: empty_user(),
            api_url: DEFAULT_API_URL.to_string(),
            client: Client::new(),
        }
    }

    /// Build the full endpoint URL for a method name.
    pub fn endpoint(&self, method: &str) -> String {
        format!("{}/bot{}/{}", self.api_url, self.token, method)
    }

    /// Make a JSON API call and deserialize the result.
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

        let tg: TelegramResponse<T> = response.json().await.map_err(BotError::Http)?;

        self.unwrap_response(tg)
    }

    /// Make an API call using multipart when a `Memory` file is present, JSON otherwise.
    ///
    /// `body` holds all params except the file field.
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
                let mut form = reqwest::multipart::Form::new();
                for (k, v) in &body {
                    if !v.is_null() {
                        let s = match v {
                            serde_json::Value::String(s) => s.clone(),
                            other => other.to_string(),
                        };
                        form = form.text(k.clone(), s);
                    }
                }
                let mime = infer_mime(&filename);
                let part = reqwest::multipart::Part::bytes(data.to_vec())
                    .file_name(filename)
                    .mime_str(&mime)
                    .map_err(|e| BotError::Other(e.to_string()))?;
                form = form.part(file_field.to_string(), part);
                self.call_api_multipart(method, form).await
            }
            other => {
                // file_id or URL - send as JSON
                let mut req = body;
                req.insert(
                    file_field.into(),
                    serde_json::to_value(other).unwrap_or_default(),
                );
                self.call_api(method, &serde_json::Value::Object(req)).await
            }
        }
    }

    /// Make a multipart/form-data API call (for file uploads).
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

        let tg: TelegramResponse<T> = response.json().await.map_err(BotError::Http)?;

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
