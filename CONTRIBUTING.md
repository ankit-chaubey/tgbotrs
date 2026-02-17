# Contributing to tgbotrs

Thank you for your interest in contributing! 🎉

**tgbotrs** is an auto-generated Telegram Bot API library for Rust. Understanding this architecture will help you contribute effectively.

---

## 🏗️ Architecture Overview

```
api.json (Telegram spec)
        │
        ▼
codegen/codegen.py
        │
        ├──► tgbotrs/src/gen_types.rs   (auto-generated)
        └──► tgbotrs/src/gen_methods.rs (auto-generated)

The handwritten files are:
- tgbotrs/src/bot.rs
- tgbotrs/src/error.rs
- tgbotrs/src/chat_id.rs
- tgbotrs/src/input_file.rs
- tgbotrs/src/reply_markup.rs
- tgbotrs/src/polling.rs
- tgbotrs/src/lib.rs
```

**Key rule:** Never edit `gen_types.rs` or `gen_methods.rs` by hand. Edit `codegen.py` instead and re-run the generator.

---

## 🚀 Getting Started

### Prerequisites

- Rust 1.75+
- Python 3.8+ (for codegen)
- Git

### Setup

```sh
# Fork and clone
git clone https://github.com/ankit-chaubey/tgbotrs
cd tgbotrs

# Verify everything builds
cargo build --workspace

# Run tests
cargo test --workspace

# Run lints
cargo clippy --workspace --all-targets
cargo fmt --all -- --check
```

---

## 🔧 Development Workflow

### Making Changes to the Codegen

If you want to change how types or methods are generated:

```sh
# 1. Edit the generator
$EDITOR codegen/codegen.py

# 2. Re-run it
python3 codegen/codegen.py api.json tgbotrs/src/

# 3. Validate coverage
python3 .github/scripts/validate_generated.py \
  api.json \
  tgbotrs/src/gen_types.rs \
  tgbotrs/src/gen_methods.rs

# 4. Build and test
cargo build --workspace
cargo test --workspace
```

### Making Changes to the Runtime

For changes to `bot.rs`, `error.rs`, `polling.rs`, etc.:

```sh
# Edit the file
$EDITOR tgbotrs/src/bot.rs

# Build and test
cargo build --workspace
cargo test --workspace

# Lint
cargo clippy --workspace --all-targets -- -D warnings
```

### Testing with a Real Bot

```sh
export BOT_TOKEN=your_test_bot_token_here

# Run the echo bot example
cargo run --example echo_bot

# Run the advanced bot example
cargo run --example advanced_bot
```

---

## 📋 What to Contribute

### High-Value Contributions

- 🧪 **Tests** — Unit tests and integration tests are very welcome
- 📖 **Examples** — More example bots in `tgbotrs/examples/`
- 🔧 **Codegen improvements** — Better Rust type generation, smarter handling of edge cases
- 🛠️ **Runtime improvements** — Better error messages, helper methods, quality-of-life APIs
- 📚 **Documentation** — More doc comments on public types and methods

### What NOT to Change

- `gen_types.rs` and `gen_methods.rs` — these are auto-generated. Edit `codegen.py` instead.
- `api.json` — this is auto-updated by the workflow. Don't manually edit it.

---

## 🐛 Reporting Bugs

Use the [bug report template](https://github.com/ankit-chaubey/tgbotrs/issues/new?template=bug_report.md).

Include:
- Minimal code to reproduce the issue
- Full error output
- Your Rust version (`rustc --version`)
- Your tgbotrs version

---

## 💡 Suggesting Features

Use the [feature request template](https://github.com/ankit-chaubey/tgbotrs/issues/new?template=feature_request.md).

---

## 📝 Pull Request Process

1. **Fork** the repo and create a branch: `git checkout -b my-feature`
2. **Make your changes**
3. **Run the checks:**
   ```sh
   cargo build --workspace
   cargo test --workspace
   cargo clippy --workspace --all-targets -- -D warnings
   cargo fmt --all -- --check
   ```
4. **Commit** with a meaningful message
5. **Open a PR** against `main`

### Commit Message Format

```
type(scope): short description

[optional longer description]
[optional: Closes #issue]
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

Examples:
```
feat(polling): add graceful shutdown support
fix(bot): handle empty response body from Telegram
docs(readme): add webhook example
chore(codegen): use ankit-chaubey/api-spec as source
```

---

## 🔒 Security Issues

Please **do not** open a public issue for security vulnerabilities.  
Email directly: [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com)

---

## 📬 Contact

- **Email:** [ankitchaubey.dev@gmail.com](mailto:ankitchaubey.dev@gmail.com)
- **Telegram:** [@ankify](https://t.me/ankify)
- **GitHub:** [@ankit-chaubey](https://github.com/ankit-chaubey)

---

Thank you for contributing! 🦀
