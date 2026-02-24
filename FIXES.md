# tgbotrs Bug Fixes

## Library Bugs Fixed

### 1. `InputFile::memory()` file uploads were broken (all file methods)
**Affected:** `send_photo`, `send_audio`, `send_video`, `send_document`, `send_animation`,
`send_sticker`, `send_video_note`, `send_voice`, `set_chat_photo`, `upload_sticker_file`

**Root cause:** All file-sending methods always called `call_api()` (JSON), even when given
an `InputFile::Memory` (raw bytes). Telegram requires multipart/form-data for binary uploads.

**Fix:** Added `Bot::call_api_with_file()` helper that auto-detects `InputFile::Memory` and
switches to multipart, while file_id/URL strings continue to use JSON as before.

### 2. `send_media_group` signature was wrong
**Before:** `media: impl Into<InputMedia>` (single item)
**After:**  `media: Vec<InputMedia>` (array)

**Root cause:** Telegram's `sendMediaGroup` API requires a JSON array. The generated method
accepted a single item and serialized it as `{}` instead of `[{}]`, causing a 400 error.

---

## README Bugs Fixed

### 3. `InlineQueryResult::Article` — wrong variant name
**Before:** `InlineQueryResult::Article(InlineQueryResultArticle { ... })`
**After:**  `InlineQueryResult::InlineQueryResultArticle(InlineQueryResultArticle { ... })`

### 4. Missing `#[derive(Default)]` on multiple types
The README used `..Default::default()` struct update syntax for types that didn't implement
`Default`, causing compile errors.

**Fixed by adding `Default` to:**
- `KeyboardButton`
- `ReplyKeyboardMarkup`
- `InputMediaPhoto`
- `InputMediaAnimation`
- `InputMediaAudio`
- `InputMediaDocument`
- `InputMediaVideo`
- `InputPollOption`
- `ForceReply`
- `InlineQueryResultArticle`
- `InputTextMessageContent`
- `ReplyParameters`
