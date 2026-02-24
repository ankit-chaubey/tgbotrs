<div align="center">

<img src="https://rustacean.net/assets/rustacean-orig-noshadow.svg" width="110" alt="Ferris the Crab"/>

<h1>tgbotrs</h1>

<p><strong>A fully-featured, auto-generated Telegram Bot API library for Rust 🦀</strong></p>

[![Crates.io](https://img.shields.io/crates/v/tgbotrs?style=for-the-badge&logo=rust&color=f74c00&labelColor=1a1a2e)](https://crates.io/crates/tgbotrs)
[![docs.rs](https://img.shields.io/docsrs/tgbotrs?style=for-the-badge&logo=docs.rs&color=4a90d9&labelColor=1a1a2e)](https://docs.rs/tgbotrs)
[![CI](https://img.shields.io/github/actions/workflow/status/ankit-chaubey/tgbotrs/ci.yml?branch=main&style=for-the-badge&logo=github-actions&label=CI&color=2ea44f&labelColor=1a1a2e)](https://github.com/ankit-chaubey/tgbotrs/actions/workflows/ci.yml)
[![API Sync](https://img.shields.io/github/actions/workflow/status/ankit-chaubey/tgbotrs/auto-regenerate.yml?style=for-the-badge&logo=telegram&label=API+SYNC&color=0088cc&labelColor=1a1a2e)](https://github.com/ankit-chaubey/tgbotrs/actions/workflows/auto-regenerate.yml)

[![Bot API](https://img.shields.io/badge/Telegram%20Bot%20API-9.4-0088cc?style=flat-square&logo=telegram&logoColor=white)](https://core.telegram.org/bots/api)
[![Rust](https://img.shields.io/badge/Rust-1.75%2B-f74c00?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Types](https://img.shields.io/badge/Types-285-7c3aed?style=flat-square)](https://docs.rs/tgbotrs)
[![Methods](https://img.shields.io/badge/Methods-165-16a34a?style=flat-square)](https://docs.rs/tgbotrs)
[![Coverage](https://img.shields.io/badge/API%20Coverage-100%25-22c55e?style=flat-square)](https://github.com/ankit-chaubey/tgbotrs/actions)
[![Downloads](https://img.shields.io/crates/d/tgbotrs?style=flat-square&color=f97316&label=Downloads)](https://crates.io/crates/tgbotrs)
[![License](https://img.shields.io/badge/License-MIT-eab308?style=flat-square)](LICENSE)

<br/>

> All **285 types** and **165 methods** of the Telegram Bot API —  
> strongly typed, fully async, automatically kept in sync with every official release.

<br/>

[📦 Install](#-installation) · [🚀 Quick Start](#-quick-start) · [📖 Examples](#-examples) · [🔧 API Reference](#-api-reference) · [🔄 Auto-Codegen](#-auto-codegen) · [📚 docs.rs](https://docs.rs/tgbotrs)

</div>

---

## ✨ Features

<table>
<tr>
<td width="50%">

**🤖 Complete API Coverage**
- All **285 types** — structs, enums, markers
- All **165 methods** — fully async
- All **21 union types** as Rust enums
- **100 optional params structs** with builder pattern

</td>
<td width="50%">

**🔄 Auto-Generated & Always Fresh**
- Spec sourced from [tgapis/x](https://github.com/tgapis/x/tree/data) — auto-updated every 6 hours
- Pipeline dispatches regeneration on every new API version
- PR auto-opened with full semantic diff on every change
- Zero manual work to stay up-to-date

</td>
</tr>
<tr>
<td>

**🦀 Idiomatic Rust**
- Fully `async/await` with **Tokio**
- `Into<ChatId>` — accepts `i64` or `"@username"`
- `Into<String>` on all text params
- `Option<T>` for all optional fields
- `Box<T>` to break recursive type cycles

</td>
<td>

**🛡️ Fully Type-Safe**
- `ChatId` — integer or username, no stringly typing
- `InputFile` — file_id / URL / raw bytes
- `ReplyMarkup` — unified enum for all 4 keyboard types
- `InputMedia` — typed enum for media groups
- Compile-time guarantees on every API call

</td>
</tr>
<tr>
<td>

**📡 Flexible HTTP Layer**
- Custom API server support (local Bot API)
- Multipart file uploads built-in
- Configurable timeout
- Flood-wait aware error handling
- `reqwest` backend

</td>
<td>

**📬 Built-in Polling**
- Long-polling dispatcher included
- Spawns a Tokio task per update
- Configurable timeout, limit, allowed\_updates
- Clean concurrent update processing

</td>
</tr>
</table>

---

## 📦 Installation

Add to your `Cargo.toml`:

```toml
[package]
name = "mybot"
version = "0.1.0"
edition = "2021"

[dependencies]
tgbotrs = ">=0.1.5"
tokio   = { version = "1", features = ["full"] }
```

> **Requirements:** Rust `1.75+` · Tokio async runtime

---

## 🚀 Quick Start

```rust
use tgbotrs::Bot;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let bot = Bot::new("YOUR_BOT_TOKEN").await?;

    println!("✅ Running as @{}", bot.me.username.as_deref().unwrap_or("unknown"));
    println!("   ID: {}", bot.me.id);

    // chat_id accepts i64, negative group IDs, or "@username"
    let msg = bot.send_message(123456789i64, "Hello from tgbotrs! 🦀", None).await?;
    println!("📨 Sent message #{}", msg.message_id);

    Ok(())
}
```

---

## 📖 Examples

### 🔁 Echo Bot — Long Polling

The simplest possible bot. Receives every message and echoes it back.

```rust
use tgbotrs::{Bot, Poller, UpdateHandler};

#[tokio::main]
async fn main() {
    let bot = Bot::new(std::env::var("BOT_TOKEN").unwrap())
        .await
        .expect("Invalid token");

    println!("🤖 @{} is running...", bot.me.username.as_deref().unwrap_or(""));

    let handler: UpdateHandler = Box::new(|bot, update| {
        Box::pin(async move {
            let Some(msg) = update.message else { return };
            let Some(text) = msg.text else { return };
            let _ = bot.send_message(msg.chat.id, text, None).await;
        })
    });

    Poller::new(bot, handler)
        .timeout(30)
        .limit(100)
        .start()
        .await
        .unwrap();
}
```

---

### 💬 Formatted Messages

Send HTML or MarkdownV2 formatted messages with optional settings.

```rust
use tgbotrs::gen_methods::SendMessageParams;

let params = SendMessageParams::new()
    .parse_mode("HTML".to_string())
    .disable_notification(true);

bot.send_message(
    "@mychannel",
    "<b>Bold</b> · <i>Italic</i> · <code>code</code> · <a href='https://example.com'>Link</a>",
    Some(params),
).await?;
```

---

### 🎹 Inline Keyboards

Buttons embedded inside messages. Perfect for interactive menus.

```rust
use tgbotrs::{ReplyMarkup, gen_methods::SendMessageParams};
use tgbotrs::types::{InlineKeyboardButton, InlineKeyboardMarkup};

let keyboard = InlineKeyboardMarkup {
    inline_keyboard: vec![
        vec![
            InlineKeyboardButton {
                text: "✅ Accept".into(),
                callback_data: Some("accept".into()),
                ..Default::default()
            },
            InlineKeyboardButton {
                text: "❌ Decline".into(),
                callback_data: Some("decline".into()),
                ..Default::default()
            },
        ],
        vec![
            InlineKeyboardButton {
                text: "🌐 Visit Website".into(),
                url: Some("https://ankitchaubey.in".into()),
                ..Default::default()
            },
        ],
    ],
};

let params = SendMessageParams::new()
    .parse_mode("HTML".to_string())
    .reply_markup(ReplyMarkup::InlineKeyboard(keyboard));

bot.send_message(chat_id, "<b>Make a choice:</b>", Some(params)).await?;
```

---

### ⚡ Callback Queries

Handle button taps from inline keyboards. Always acknowledge with `answer_callback_query`.

```rust
use tgbotrs::gen_methods::{AnswerCallbackQueryParams, EditMessageTextParams};
use tgbotrs::types::MaybeInaccessibleMessage;

let handler: UpdateHandler = Box::new(|bot, update| {
    Box::pin(async move {
        let Some(cq) = update.callback_query else { return };
        let data = cq.data.as_deref().unwrap_or("");

        // Always acknowledge — dismisses the loading spinner
        let _ = bot
            .answer_callback_query(
                cq.id.clone(),
                Some(
                    AnswerCallbackQueryParams::new()
                        .text(format!("You chose: {}", data))
                        .show_alert(false),
                ),
            )
            .await;

        // Edit the original message in-place
        if let Some(msg) = &cq.message {
            if let MaybeInaccessibleMessage::Message(m) = msg.as_ref() {
                let edit_params = EditMessageTextParams::new()
                    .chat_id(m.chat.id)
                    .message_id(m.message_id)
                    .parse_mode("HTML".to_string());

                let _ = bot
                    .edit_message_text(
                        format!("✅ You selected: <b>{}</b>", data),
                        Some(edit_params),
                    )
                    .await;
            }
        }
    })
});
```

---

### ⌨️ Reply Keyboards

Custom keyboard shown at the bottom of the screen. Great for persistent menu buttons.

```rust
use tgbotrs::{ReplyMarkup, gen_methods::SendMessageParams};
use tgbotrs::types::{KeyboardButton, ReplyKeyboardMarkup};

let keyboard = ReplyKeyboardMarkup {
    keyboard: vec![
        vec![
            KeyboardButton {
                text: "📍 Share Location".into(),
                request_location: Some(true),
                ..Default::default()
            },
            KeyboardButton {
                text: "📱 Share Contact".into(),
                request_contact: Some(true),
                ..Default::default()
            },
        ],
        vec![
            KeyboardButton { text: "🏠 Home".into(), ..Default::default() },
            KeyboardButton { text: "⚙️ Settings".into(), ..Default::default() },
        ],
    ],
    resize_keyboard: Some(true),
    one_time_keyboard: Some(true),
    ..Default::default()
};

let params = SendMessageParams::new()
    .reply_markup(ReplyMarkup::ReplyKeyboard(keyboard));

bot.send_message(chat_id, "Use the keyboard below 👇", Some(params)).await?;
```

---

### 📸 Send Photos & Files

Send files by file\_id, URL, or raw bytes from disk.

```rust
use tgbotrs::{InputFile, gen_methods::SendPhotoParams};

let params = SendPhotoParams::new()
    .caption("Look at this! 📷".to_string())
    .parse_mode("HTML".to_string());

// Fastest — already on Telegram's servers
bot.send_photo(chat_id, "AgACAgIAAxkBAAI...", Some(params.clone())).await?;

// Let Telegram download from a URL
bot.send_photo(chat_id, "https://example.com/photo.jpg", Some(params.clone())).await?;

// Upload raw bytes from disk
let data = tokio::fs::read("photo.jpg").await?;
bot.send_photo(chat_id, InputFile::memory("photo.jpg", data), Some(params)).await?;
```

---

### 🎬 Media Groups

Send multiple photos or videos as an album in a single message.

```rust
use tgbotrs::InputMedia;
use tgbotrs::types::{InputMediaPhoto, InputMediaVideo};

let media = vec![
    InputMedia::Photo(InputMediaPhoto {
        r#type: "photo".into(),
        media: "AgACAgIAAxkBAAI...".into(),
        caption: Some("First photo 📸".into()),
        ..Default::default()
    }),
    InputMedia::Video(InputMediaVideo {
        r#type: "video".into(),
        media: "BAACAgIAAxkBAAI...".into(),
        caption: Some("A video 🎬".into()),
        ..Default::default()
    }),
];

bot.send_media_group(chat_id, media, None).await?;
```

---

### 📊 Polls

Send polls — regular or quiz style.

```rust
use tgbotrs::gen_methods::SendPollParams;
use tgbotrs::types::InputPollOption;

let options = vec![
    InputPollOption { text: "🦀 Rust".into(),   ..Default::default() },
    InputPollOption { text: "🐹 Go".into(),     ..Default::default() },
    InputPollOption { text: "🐍 Python".into(), ..Default::default() },
];

let params = SendPollParams::new().is_anonymous(false);

bot.send_poll(chat_id, "Best language for bots?", options, Some(params)).await?;
```

---

### 🏪 Inline Queries

Handle `@yourbot query` inline mode from any chat.

```rust
use tgbotrs::types::{
    InlineQueryResult, InlineQueryResultArticle,
    InputMessageContent, InputTextMessageContent,
};

let results = vec![
    InlineQueryResult::InlineQueryResultArticle(InlineQueryResultArticle {
        r#type: "article".into(),
        id: "1".into(),
        title: "Hello World".into(),
        input_message_content: InputMessageContent::InputTextMessageContent(InputTextMessageContent {
            message_text: "Hello from inline mode! 👋".into(),
            ..Default::default()
        }),
        description: Some("Send a greeting".into()),
        reply_markup: None,
        url: None,
        thumbnail_url: None,
        thumbnail_width: None,
        thumbnail_height: None,
    }),
];

bot.answer_inline_query(query.id.clone(), results, None).await?;
```

---

### 🛒 Payments & Telegram Stars

Send invoices using Telegram Stars (`XTR`) or payment providers.

```rust
use tgbotrs::gen_methods::SendInvoiceParams;
use tgbotrs::types::LabeledPrice;

let prices = vec![
    LabeledPrice { label: "Premium Plan".into(), amount: 999 },
];

bot.send_invoice(
    chat_id,
    "Premium Access",
    "30 days of unlimited features",
    "payload_premium_30d",
    "XTR",   // Telegram Stars
    prices,
    None,
).await?;
```

---

### 🔔 Webhooks

Register a webhook URL so Telegram pushes updates to your server instead of you polling. Read detailed [webhook bot examples!](https://github.com/ankit-chaubey/tgbotrs/tree/main/examples/webhook)

```rust
use tgbotrs::gen_methods::SetWebhookParams;

// Register webhook
let params = SetWebhookParams::new()
    .max_connections(100i64)
    .allowed_updates(vec!["message".into(), "callback_query".into()])
    .secret_token("my_secret_token".to_string());

bot.set_webhook("https://mybot.example.com/webhook", Some(params)).await?;
```

**Full webhook server with [axum](https://github.com/tokio-rs/axum):**

```toml
# Cargo.toml
[dev-dependencies]
axum = "0.7"
```

```rust
use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use std::sync::Arc;
use tgbotrs::{gen_methods::SetWebhookParams, types::Update, Bot};

struct AppState { bot: Bot }

#[tokio::main]
async fn main() {
    let bot = Bot::new("YOUR_BOT_TOKEN").await.unwrap();

    bot.set_webhook(
        "https://yourdomain.com/webhook",
        Some(SetWebhookParams::new()),
    )
    .await
    .unwrap();

    let app = Router::new()
        .route("/webhook", post(handle_update))
        .with_state(Arc::new(AppState { bot }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_update(
    State(state): State<Arc<AppState>>,
    Json(update): Json<Update>,
) -> StatusCode {
    let bot = state.bot.clone();
    // Spawn immediately — return 200 fast or Telegram will retry
    tokio::spawn(async move {
        if let Some(msg) = update.message {
            let _ = bot
                .send_message(msg.chat.id, "Received via webhook! 🚀", None)
                .await;
        }
    });
    StatusCode::OK
}
```

> For local testing: `ngrok http 8080` → use the ngrok URL as your webhook

---

### 🌐 Local Bot API Server

Point the bot at a self-hosted [Telegram Bot API server](https://github.com/tdlib/telegram-bot-api) for higher file size limits and faster speeds.

```rust
let bot = Bot::with_api_url("YOUR_TOKEN", "http://localhost:8081").await?;
```

---

### 🛠️ Error Handling

Structured errors with helpers for flood-wait and common API errors.

```rust
use tgbotrs::BotError;

match bot.send_message(chat_id, "Hello!", None).await {
    Ok(msg) => println!("✅ Sent: #{}", msg.message_id),

    Err(BotError::Api { code: 403, .. }) => {
        eprintln!("🚫 Bot was blocked by user");
    }
    Err(BotError::Api { code: 400, description, .. }) => {
        eprintln!("⚠️  Bad request: {}", description);
    }
    Err(e) if e.is_api_error_code(429) => {
        if let Some(secs) = e.flood_wait_seconds() {
            println!("⏳ Flood wait: {} seconds", secs);
            tokio::time::sleep(std::time::Duration::from_secs(secs as u64)).await;
        }
    }
    Err(e) => eprintln!("❌ Unexpected error: {}", e),
}
```

---

## 🔧 API Reference

### `Bot` — Core Struct

```rust
pub struct Bot {
    pub token:   String,  // Bot token from @BotFather
    pub me:      User,    // Populated via getMe on creation
    pub api_url: String,  // Default: https://api.telegram.org
}
```

| Constructor | Description |
|---|---|
| `Bot::new(token)` | Create bot, calls `getMe`, verifies token |
| `Bot::with_api_url(token, url)` | Create with a custom/local API server |
| `Bot::new_unverified(token)` | Create without calling `getMe` |

---

### `ChatId` — Flexible Chat Identifier

Anywhere `ChatId` is expected, you can pass any of these:

```rust
bot.send_message(123456789i64,     "user by numeric id", None).await?;
bot.send_message(-100123456789i64, "group or channel",   None).await?;
bot.send_message("@channelname",   "by username",        None).await?;
bot.send_message(ChatId::Id(123),  "explicit wrapper",   None).await?;
```

---

### `InputFile` — File Sending

```rust
// Reference a file already on Telegram's servers (fastest)
InputFile::file_id("AgACAgIAAxkBAAI...")

// Let Telegram download from a URL
InputFile::url("https://example.com/image.png")

// Upload raw bytes directly
let data = tokio::fs::read("photo.jpg").await?;
InputFile::memory("photo.jpg", data)
```

---

### `ReplyMarkup` — All Keyboard Types

```rust
// Inline keyboard — buttons inside messages
ReplyMarkup::InlineKeyboard(InlineKeyboardMarkup { .. })

// Reply keyboard — custom keyboard at bottom of screen
ReplyMarkup::ReplyKeyboard(ReplyKeyboardMarkup { .. })

// Remove the reply keyboard
ReplyMarkup::ReplyKeyboardRemove(ReplyKeyboardRemove { remove_keyboard: true, .. })

// Force the user to reply to a message
ReplyMarkup::ForceReply(ForceReply { force_reply: true, .. })
```

---

### `Poller` — Long Polling Dispatcher

```rust
Poller::new(bot, handler)
    .timeout(30)           // Seconds to long-poll (0 = short poll)
    .limit(100)            // Max updates per request (1–100)
    .allowed_updates(vec![ // Filter which update types to receive
        "message".into(),
        "callback_query".into(),
        "inline_query".into(),
    ])
    .start()
    .await?;
```

---

### `BotError` — Error Variants

```rust
pub enum BotError {
    Http(reqwest::Error),       // Network / HTTP transport error
    Json(serde_json::Error),    // Serialization error
    Api {
        code: i64,                       // Telegram error code (400, 403, 429…)
        description: String,             // Human-readable message
        retry_after: Option<i64>,        // Flood-wait seconds (code 429)
        migrate_to_chat_id: Option<i64>, // Migration target (code 400)
    },
    InvalidToken,               // Token missing ':'
    Other(String),              // Catch-all
}

// Helper methods
error.is_api_error_code(429)   // → bool
error.flood_wait_seconds()     // → Option<i64>
```

---

### Builder Pattern for Optional Params

Every method with optional parameters has a `*Params` struct with a fluent builder API:

```rust
// Pattern: MethodNameParams::new().field(value).field(value)
let params = SendMessageParams::new()
    .parse_mode("HTML".to_string())
    .disable_notification(true)
    .protect_content(false)
    .message_thread_id(123i64)
    .reply_parameters(ReplyParameters { message_id: 42, ..Default::default() })
    .reply_markup(ReplyMarkup::ForceReply(ForceReply {
        force_reply: true,
        input_field_placeholder: None,
        selective: None,
    }));
```

---

## 📊 Coverage Statistics

| Category | Count | Status |
|:---|:---:|:---:|
| **Total Types** | **285** | ✅ 100% |
| ↳ Struct types | 257 | ✅ |
| ↳ Union / Enum types | 21 | ✅ |
| ↳ Marker types | 7 | ✅ |
| **Total Methods** | **165** | ✅ 100% |
| ↳ `set*` methods | 30 | ✅ |
| ↳ `get*` methods | 29 | ✅ |
| ↳ `send*` methods | 22 | ✅ |
| ↳ `edit*` methods | 12 | ✅ |
| ↳ `delete*` methods | 11 | ✅ |
| ↳ Other methods | 61 | ✅ |
| **Optional params structs** | 100 | ✅ |
| **Lines auto-generated** | ~11,258 | — |

---

## 🔄 Auto-Codegen

tgbotrs is the only Rust Telegram library that **automatically stays in sync** with the official API — no manual updates, no lag.

The spec is sourced from **[tgapis/x](https://github.com/tgapis/x/tree/data)** (`data` branch, `botapi.json`), which scrapes and parses the official Telegram Bot API page and auto-updates every **6 hours**. When a new API version is detected, the `tgapis/x` pipeline dispatches a trigger to this repo — regeneration kicks off immediately, no cron required.

### How It Works

```
tgapis/x scrapes Telegram Bot API
every 6 hours → pushes botapi.json
to the data branch
        │
        ▼
  ┌─────────────────────┐
  │  tgapis/x pipeline  │── No change? ──► Stop ✅
  │  detects new version│
  └──────────┬──────────┘
             │ Changed!
             │ repository_dispatch →
             │ event: x-data-updated
             ▼
  ┌─────────────────────┐
  │  Fetch botapi.json  │  ← raw.githubusercontent.com/tgapis/x/data/botapi.json
  │  (always latest)    │
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  Compare with       │── No change? ──► Stop ✅
  │  pinned api.json    │
  └──────────┬──────────┘
             │ Changed!
             ▼
  ┌─────────────────────┐
  │  diff_spec.py       │  ← Semantic diff (added/removed types & methods)
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  codegen.py         │  ← Pure Python, zero pip dependencies
  │                     │    Generates gen_types.rs + gen_methods.rs
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  validate.py        │  ← Verify 100% coverage
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  Open PR with       │  ← Rich report: summary table, per-field diff
  │  full report        │    New/removed items, checklist
  └──────────┬──────────┘
             │
             ▼
  ┌─────────────────────┐
  │  On PR merge:       │
  │  • Bump semver      │
  │  • Git tag          │
  │  • GitHub Release   │
  │  • crates.io        │
  └─────────────────────┘
```

### Regenerate Manually

```sh
# 1. Pull latest spec from tgapis/x data branch into repo root
curl -sSf https://raw.githubusercontent.com/tgapis/x/data/botapi.json -o api.json

# 2. Run codegen (no pip installs needed)
python3 codegen/codegen.py api.json tgbotrs/src/

# 3. Rebuild
cargo build
```

### GitHub Actions Workflows

| Workflow | Trigger | Purpose |
|:---|:---|:---|
| `auto-regenerate.yml` | 📡 Pipeline dispatch from `tgapis/x` + manual | Spec sync → diff → codegen → PR |
| `ci.yml` | Every push / PR | Build, test, lint on 3 OS × 2 Rust versions (spec fetched live from `tgapis/x`) |
| `release.yml` | PR merged → main | Semver bump → tag → crates.io publish |
| `notify.yml` | After regen | GitHub Issue with full change summary |

---

## 🤝 Contributing

Contributions are welcome!

**Report issues:**
- 🐛 Bug → [open a bug report](https://github.com/ankit-chaubey/tgbotrs/issues/new?template=bug_report.md)
- 💡 Feature → [open a feature request](https://github.com/ankit-chaubey/tgbotrs/issues/new?template=feature_request.md)
- 🔒 Security → email [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com) directly

**Development workflow:**

```sh
git clone https://github.com/ankit-chaubey/tgbotrs && cd tgbotrs

cargo build --workspace                    # Build everything
cargo test --workspace                     # Run tests
cargo clippy --workspace -- -D warnings    # Lint
cargo fmt --all                            # Format

# Pull the latest spec from tgapis/x into repo root and regenerate
curl -sSf https://raw.githubusercontent.com/tgapis/x/data/botapi.json -o api.json
python3 codegen/codegen.py api.json tgbotrs/src/

# Validate 100% coverage
python3 .github/scripts/validate_generated.py \
  api.json tgbotrs/src/gen_types.rs tgbotrs/src/gen_methods.rs
```

**PR guidelines:**
- One concern per PR
- Always run `cargo fmt` and `cargo clippy` before submitting
- Never edit `gen_types.rs` or `gen_methods.rs` directly — edit `codegen.py` instead
- Add examples for any new helpers

---

## 📜 Changelog

See [CHANGELOG.md](CHANGELOG.md) for the full release history.

---

### Developed by [Ankit Chaubey](https://github.com/ankit-chaubey)

**tgbotrs** started as a personal tool.  
I was constantly running into limitations, missing features, and unsupported things,  
so in 2024 I decided to build my own solution.

After using **tgbotrs** for a long time (2024-26) and refining it along the way,  
I felt it could be useful for others too — so I made it public.

If this helps you in any way, feel free to ⭐ star it or 🍴 fork it 😁

Developed and maintained by Ankit Chaubey [(@ankit-chaubey)](https://github.com/ankit-chaubey)

<p align="center">
  <sub>
    Rust engineer · Open-source builder · Telegram & systems enthusiast
  </sub>
</p>

<p align="center">
  <a href="https://github.com/ankit-chaubey">
    <img src="https://img.shields.io/badge/GitHub-ankit--chaubey-181717?style=flat&logo=github" />
  </a>
  <a href="https://t.me/ankify">
    <img src="https://img.shields.io/badge/Telegram-@ankify-0088cc?style=flat&logo=telegram&logoColor=white" />
  </a>
  <a href="mailto:ankitchaubey.dev@gmail.com">
    <img src="https://img.shields.io/badge/Email-Contact-ea4335?style=flat&logo=gmail&logoColor=white" />
  </a>
  <a href="https://ankitchaubey.in">
    <img src="https://img.shields.io/badge/Website-ankitchaubey.in-4a90d9?style=flat&logo=google-chrome&logoColor=white" />
  </a>
</p>

<hr />

<p align="center">
  <a href="https://docs.rs/tgbotrs">
    <img src="https://img.shields.io/badge/docs.rs-tgbotrs-4a90d9?style=flat-square&logo=docs.rs" />
  </a>
  <a href="https://crates.io/crates/tgbotrs">
    <img src="https://img.shields.io/badge/crates.io-tgbotrs-f74c00?style=flat-square&logo=rust" />
  </a>
  <a href="https://github.com/ankit-chaubey/tgbotrs/stargazers">
    <img src="https://img.shields.io/github/stars/ankit-chaubey/tgbotrs?style=social" />
  </a>
  <a href="https://github.com/ankit-chaubey/tgbotrs/network/members">
    <img src="https://img.shields.io/github/forks/ankit-chaubey/tgbotrs?style=social" />
  </a>
</p>

---

## 🙏 Thanks & Credits

Special thanks to **[Paul / PaulSonOfLars](https://github.com/PaulSonOfLars)** — the auto-generation approach at the heart of this library was directly inspired by his excellent Go library **[gotgbot](https://github.com/PaulSonOfLars/gotgbot)** and api-spec gen. Seeing how clean and maintainable a fully-generated, strongly-typed Telegram library can be was the spark for building tgbotrs.

| | |
|:---|:---|
| [**Telegram**](https://core.telegram.org/bots/api) | The Bot API this library implements |
| [**PaulSonOfLars / gotgbot**](https://github.com/PaulSonOfLars/gotgbot) | Inspiration for the codegen-first approach |
| [**tgapis/x**](https://github.com/tgapis/x/tree/data) | Machine-readable spec source — auto-updated every 6 hours |

---

## 📄 License

MIT License © 2026 [Ankit Chaubey](https://github.com/ankit-chaubey)

---

<div align="center">

*If tgbotrs saved you time, a ⭐ on GitHub means a lot!*
