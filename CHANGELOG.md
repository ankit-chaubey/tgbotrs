# Changelog

All notable changes to tgbotrs are documented here.
Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

## [0.2.0] - 2026-04-10

### Added

- `Message` helpers: `get_text()`, `get_entities()`, `get_link()`, `reply(&bot, text, params)`.
- `Chat` helper: `send_message(&bot, text, params)`.
- `File::url(&bot)` - returns the HTTPS CDN URL for a file. Returns `None` when `file_path` is absent.
- `InaccessibleMessage::to_message()` - converts to a minimal `Message` with `message_id`, `date`, and `chat`.
- `BotClient` trait at `tgbotrs::client::BotClient`. `Bot` now stores `Arc<dyn BotClient>` instead of a raw `reqwest::Client`.
  - `Bot::with_client(token, api_url, client)` - create a `Bot` with a custom transport.
  - `ReqwestClient` - the default implementation, publicly exposed.
  - `FormPart` / `FormBody` - multipart abstraction for custom client implementors.
- `Updater` struct at `tgbotrs::Updater` - glues `Bot` and `Dispatcher` into a single runner.
  - Builder: `.poll_timeout(secs)`, `.poll_limit(n)`, `.allowed_updates(...)`, `.webhook_port(port)`, `.webhook_secret(secret)`.
  - `start_polling()` - long-polling mode.
  - `start_webhook(url)` - webhook mode (requires `webhook` feature).
- `Handler` trait - `name()`, `check_update()`, `handle_update()`, all `Send + Sync`.
- `Dispatcher` with handler groups - `add_handler`, `add_handler_to_group`, `remove_handler`, `remove_group`. Groups run in ascending order; first match per group fires.
  - `DispatcherOpts::on_error` and `on_panic` hooks.
  - `ContinueGroups` / `EndGroups` sentinel errors for flow control.
- Panic recovery in spawned tasks - handler panics are caught via `JoinHandle`, logged at `error!` level, and the polling loop or dispatcher continues unaffected. The `on_panic` hook receives the panic message string.
- Filter system - `Filter<T>` trait with composable operators: `.and()`, `.or()`, `.not()`.
  - Built-in filters: `filters::message` (text, command, photo, video, audio, document, sticker, caption, reply, forward), `filters::callback_query` (data match, regex), `filters::chat_member`, `filters::inline_query`.
- `CommandHandler` - parses `/command[@botname]` with configurable prefix, args available via `ctx.args()`.
- `MessageHandler` and `CallbackQueryHandler` - filter plus async fn wrapped as `Handler`.
- `ConversationHandler` - entry points, `states: HashMap<S, Vec<Handler>>`, fallbacks.
  - `ConversationStorage` trait with `InMemoryStorage` default.
  - `KeyStrategy`: `Sender`, `Chat`, `SenderAndChat`.
- `BotMapping` - routes webhook POSTs to the correct bot by URL path, for multi-bot setups.
- Sync client (`client-ureq` feature) - `ureq`-backed blocking `SyncBot`.
- WASM support - file upload paths gated behind `#[cfg(not(target_arch = "wasm32"))]`.
- `entities` module - `parse_entity`, `parse_entities`, `ParsedEntity`, `MessageEntityExt` trait. Entity offsets decoded from UTF-16 code units to UTF-8 byte positions.
- `Context` struct - wraps `Bot + Update`, exposes `effective_chat()`, `effective_user()`, `effective_message()`, `args()`.
- `#[non_exhaustive]` on `BotError`.
- `[package.metadata.docs.rs] all-features = true`.
- Spec drift CI (`spec-drift.yml`) - tracks `spec_commit`, diffs api.json, opens PR automatically on new Telegram Bot API releases.
- Examples: `colourbutton`, `functionalbot` (echo + advanced), `mock_client`, `webhook`.
- Tests: serde round-trips for major types, dispatcher group ordering, filter composition, helper methods.

### Fixed

- All `println!`/`eprintln!` removed from library code. Internal logging goes through `tracing`; the library is silent by default.
- Flood-wait retry uses the server-supplied `retry_after` from `BotError::Api` (429) instead of a hardcoded 3s.
- `Bot::new_unverified` now returns `Result<Self, BotError>`. Bot ID is parsed from the token string so `bot.me.id` is always non-zero.
- Configurable HTTP timeout via `Bot::with_timeout(token, api_url, duration)`. `Bot::new` defaults to 30s.

### Changed

- `Bot.client` changed from `reqwest::Client` to `Arc<dyn BotClient>`.
- `call_api_multipart` now takes `Vec<FormPart>` instead of `reqwest::multipart::Form`.
- Version bumped to 0.2.0.

## [0.1.7] - 2026-04-05

Telegram Bot API 9.6 - auto-generated from spec.

## [0.1.6] - 2026-03-01

Telegram Bot API 9.5 - auto-generated from spec.

## [0.1.5] - 2026-02-24

Telegram Bot API 9.4 - auto-generated from spec.

## [0.1.4] - 2026-02-18

Telegram Bot API 9.4 - auto-generated from spec.

## [0.1.3] - 2026-02-17

Telegram Bot API 9.4 - auto-generated from spec.

## [0.1.2] - 2026-02-17

Telegram Bot API 9.4 - auto-generated from spec.

## [0.1.1] - 2026-02-17

Telegram Bot API 9.4 - auto-generated from spec.

## [0.1.0] - 2024-01-01

Initial release.

- 285 types generated from Telegram Bot API 9.4 (257 structs, 21 union/enum types, 7 marker types).
- 165 async methods, 100 optional param structs with builder pattern.
- `Bot::new()`, `Bot::with_api_url()`, `Bot::new_unverified()`.
- `ChatId`, `InputFile`, `ReplyMarkup`, `InputMedia`.
- `Poller` for long-polling, `BotError` with flood-wait helpers, multipart upload.
- Codegen: `codegen/codegen.py` and `codegen/src/main.rs`.
- CI: auto-regenerate, build/test/lint, release, notify workflows.
