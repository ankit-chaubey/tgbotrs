# Changelog

All notable changes to **tgbotrs** are documented here.

Format follows [Keep a Changelog](https://keepachangelog.com/en/1.0.0/).
Versioning follows [Semantic Versioning](https://semver.org/).

> 📌 **API updates are tracked automatically** by the [auto-regenerate workflow](.github/workflows/auto-regenerate.yml).
> A new crate version is published on every Telegram Bot API update.

---

## [0.1.4] — 2026-02-18

### Telegram Bot API: `Bot API 9.4`

Auto-generated from latest Telegram Bot API spec.

---

## [0.1.1] — 2026-02-18

### Telegram Bot API: `Bot API 9.4`

Auto-generated from latest Telegram Bot API spec.

---

## [0.1.3] — 2026-02-17

### Telegram Bot API: `Bot API 9.4`

Auto-generated from latest Telegram Bot API spec.

---

## [0.1.2] — 2026-02-17

### Telegram Bot API: `Bot API 9.4`

Auto-generated from latest Telegram Bot API spec.

---

## [0.1.1] — 2026-02-17

### Telegram Bot API: `Bot API 9.4`

Auto-generated from latest Telegram Bot API spec.

---

## [0.1.0] — 2024-01-01

### Added — Initial Release

**tgbotrs** — a fully auto-generated Telegram Bot API library for Rust, built and maintained by [Ankit Chaubey](https://github.com/ankit-chaubey).

#### Library
- ✅ **285 types** auto-generated from Telegram Bot API 9.4
  - 257 struct types (all Telegram objects)
  - 21 union/enum types (`ChatMember`, `MessageOrigin`, `InlineQueryResult`, etc.)
  - 7 marker types (empty structs)
- ✅ **165 methods** fully implemented and async
  - 30 `set*` methods
  - 29 `get*` methods
  - 22 `send*` methods
  - 12 `edit*` methods
  - 11 `delete*` methods
  - 61 other methods
- ✅ **100 optional params structs** with builder pattern
- ✅ `Bot::new()` — creates bot and verifies token via `getMe`
- ✅ `Bot::with_api_url()` — custom/local Bot API server support
- ✅ `Bot::new_unverified()` — skip getMe call
- ✅ `ChatId` — unified type accepting `i64` or `"@username"`
- ✅ `InputFile` — file_id, URL, or raw bytes upload
- ✅ `InputFileOrString` — for fields accepting both
- ✅ `ReplyMarkup` — unified enum for all 4 keyboard variants
- ✅ `InputMedia` — typed enum for `sendMediaGroup`
- ✅ `Poller` — async long-polling dispatcher with configurable options
- ✅ `BotError` — typed error variants with flood-wait helpers
- ✅ Multipart file upload support via `call_api_multipart`

#### Codegen
- ✅ `codegen/codegen.py` — pure Python code generator, zero dependencies
- ✅ `codegen/src/main.rs` — Rust code generator binary (alternative)
- ✅ Generates from [ankit-chaubey/api-spec](https://github.com/ankit-chaubey/api-spec)

#### GitHub Actions
- ✅ `auto-regenerate.yml` — daily spec check, codegen, auto-PR with diff report
- ✅ `ci.yml` — build/test/lint on Ubuntu, Windows, macOS × stable/beta Rust
- ✅ `release.yml` — auto version bump, GitHub Release, crates.io publish
- ✅ `notify.yml` — GitHub Issue notification on API updates

#### Automation Scripts
- ✅ `diff_spec.py` — semantic diff between two api.json versions
- ✅ `validate_generated.py` — 100% coverage validator
- ✅ `build_pr_body.py` — rich PR description generator
- ✅ `coverage_report.py` — Markdown coverage table
- ✅ `update_changelog.py` — auto CHANGELOG updater

---

*Future versions will be added automatically as Telegram releases new Bot API versions.*
