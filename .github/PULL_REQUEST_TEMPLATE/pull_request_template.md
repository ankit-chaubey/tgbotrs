## 📋 Description

<!-- Describe your changes clearly. What does this PR do and why? -->

## 🔗 Related Issue

Closes #<!-- issue number -->

## 🧪 Type of Change

- [ ] 🐛 Bug fix
- [ ] ✨ New feature
- [ ] 🔧 Codegen / codegen.py improvement
- [ ] 📖 Documentation update
- [ ] 🤖 Auto-generated API update (bot-api-update)
- [ ] Other: ___

## ✅ Checklist

- [ ] `cargo build --workspace` passes
- [ ] `cargo test --workspace` passes
- [ ] `cargo clippy --workspace --all-targets -- -D warnings` passes
- [ ] `cargo fmt --all -- --check` passes
- [ ] If `codegen.py` was changed: pulled latest spec and re-ran codegen
  ```sh
  curl -sSf https://raw.githubusercontent.com/tgapis/x/data/botapi.json -o api.json
  python3 codegen/codegen.py api.json tgbotrs/src/
  ```
- [ ] If `codegen.py` was changed: validation passes
  ```sh
  python3 .github/scripts/validate_generated.py \
    api.json tgbotrs/src/gen_types.rs tgbotrs/src/gen_methods.rs
  ```
- [ ] `gen_types.rs` and `gen_methods.rs` were **not** edited manually — only via `codegen.py`
- [ ] Documentation updated if needed

## 🔍 Test Plan

<!-- How did you test these changes? What cases did you cover? -->

## 📸 Screenshots / Output (if applicable)

<!-- Any relevant output, before/after comparisons, etc. -->
