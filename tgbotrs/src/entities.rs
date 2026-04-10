//! Helpers for parsing [`MessageEntity`] offsets correctly.
//!
//! Telegram encodes entity `offset` and `length` as **UTF-16 code unit** counts,
//! *not* UTF-8 bytes or Unicode scalar values. Slicing the message text naively
//! with those numbers will corrupt any text containing characters outside the
//! Basic Multilingual Plane (emoji, most CJK, etc.).
//!
//! Use [`ParsedEntity`] and the helpers on [`Message`] to get correct UTF-8
//! `&str` slices for each entity.
//!
//! # Example
//!
//! ```rust,no_run
//! use tgbotrs::types::Message;
//! use tgbotrs::MessageEntityExt;
//!
//! fn print_bold(msg: &Message) {
//!     for ent in msg.parse_entities() {
//!         if ent.kind == "bold" {
//!             println!("bold: {}", ent.text);
//!         }
//!     }
//! }
//! ```

use crate::types::{Message, MessageEntity};

/// A [`MessageEntity`] with its text slice already decoded to a UTF-8 `String`.
///
/// Offsets inside [`MessageEntity`] are rewritten to UTF-8 byte positions so
/// they are usable with standard Rust string slicing.
#[derive(Debug, Clone, PartialEq)]
pub struct ParsedEntity {
    /// The entity type (e.g. `"bold"`, `"url"`, `"mention"`).
    pub kind: String,
    /// The decoded text that this entity covers.
    pub text: String,
    /// UTF-8 byte offset in the original text where this entity starts.
    pub offset: usize,
    /// UTF-8 byte length of the entity text.
    pub length: usize,
    /// For `"text_link"` entities, the URL.
    pub url: Option<String>,
    /// For `"text_mention"` entities, the user.
    pub user: Option<Box<crate::types::User>>,
    /// For `"pre"` entities, the programming language.
    pub language: Option<String>,
    /// For `"custom_emoji"` entities, the emoji ID.
    pub custom_emoji_id: Option<String>,
}

/// Parse a single [`MessageEntity`] against the given text.
///
/// Both `text` and `entity` must come from the same message field (either
/// both from `message.text` + `message.entities`, or both from
/// `message.caption` + `message.caption_entities`).
pub fn parse_entity(text: &str, entity: &MessageEntity) -> ParsedEntity {
    parse_entity_from_utf16(&utf16_encode(text), entity)
}

/// Parse all entities from a text + entity-list pair.
pub fn parse_entities(text: &str, entities: &[MessageEntity]) -> Vec<ParsedEntity> {
    let utf16 = utf16_encode(text);
    entities
        .iter()
        .map(|e| parse_entity_from_utf16(&utf16, e))
        .collect()
}

// Helpers
fn utf16_encode(s: &str) -> Vec<u16> {
    s.encode_utf16().collect()
}

fn parse_entity_from_utf16(utf16: &[u16], entity: &MessageEntity) -> ParsedEntity {
    let start16 = entity.offset as usize;
    let end16 = start16 + entity.length as usize;

    // Clamp to avoid panics on malformed data from the server.
    let start16 = start16.min(utf16.len());
    let end16 = end16.min(utf16.len());

    let entity_text = String::from_utf16_lossy(&utf16[start16..end16]);

    // Convert UTF-16 offsets to UTF-8 byte offsets in the original string.
    let prefix_utf8 = String::from_utf16_lossy(&utf16[..start16]);
    let utf8_start = prefix_utf8.len();
    let utf8_len = entity_text.len();

    // For URL entities, Telegram does not always fill in the url field -
    // the URL is the entity text itself.
    let url = entity
        .url
        .clone()
        .or_else(|| (entity.r#type == "url").then(|| entity_text.clone()));

    ParsedEntity {
        kind: entity.r#type.clone(),
        text: entity_text,
        offset: utf8_start,
        length: utf8_len,
        url,
        user: entity.user.clone(),
        language: entity.language.clone(),
        custom_emoji_id: entity.custom_emoji_id.clone(),
    }
}

// Extension methods on Message
/// Entity-parsing extension methods for [`Message`].
pub trait MessageEntityExt {
    /// Parse all text entities (from `message.text` + `message.entities`).
    fn parse_entities(&self) -> Vec<ParsedEntity>;

    /// Parse all caption entities (from `message.caption` + `message.caption_entities`).
    fn parse_caption_entities(&self) -> Vec<ParsedEntity>;

    /// Parse whichever entities are present (text entities first, then caption entities).
    fn parse_any_entities(&self) -> Vec<ParsedEntity>;
}

impl MessageEntityExt for Message {
    fn parse_entities(&self) -> Vec<ParsedEntity> {
        match (&self.text, &self.entities) {
            (Some(text), Some(entities)) => parse_entities(text, entities),
            _ => vec![],
        }
    }

    fn parse_caption_entities(&self) -> Vec<ParsedEntity> {
        match (&self.caption, &self.caption_entities) {
            (Some(caption), Some(entities)) => parse_entities(caption, entities),
            _ => vec![],
        }
    }

    fn parse_any_entities(&self) -> Vec<ParsedEntity> {
        let mut out = self.parse_entities();
        out.extend(self.parse_caption_entities());
        out
    }
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MessageEntity;

    fn make_entity(kind: &str, offset: i64, length: i64) -> MessageEntity {
        MessageEntity {
            r#type: kind.to_string(),
            offset,
            length,
            url: None,
            user: None,
            language: None,
            custom_emoji_id: None,
            unix_time: None,
            date_time_format: None,
        }
    }

    #[test]
    fn ascii_entity() {
        let text = "Hello world";
        let entity = make_entity("bold", 6, 5);
        let parsed = parse_entity(text, &entity);
        assert_eq!(parsed.text, "world");
        assert_eq!(parsed.offset, 6);
    }

    #[test]
    fn emoji_entity_utf16() {
        // "Hi 😀 there": 😀 is U+1F600, encoded as 2 UTF-16 code units.
        // Telegram would report offset=3, length=2 (UTF-16 units for 😀).
        // UTF-8 offset should be 3 bytes, length 4 bytes.
        let text = "Hi 😀 there";
        let entity = make_entity("bold", 3, 2); // select 😀
        let parsed = parse_entity(text, &entity);
        assert_eq!(parsed.text, "😀");
        // UTF-8 byte position of 😀 is 3
        assert_eq!(&text[parsed.offset..parsed.offset + parsed.length], "😀");
    }

    #[test]
    fn multi_emoji() {
        // "😀😀": 4 UTF-16 units total; select second emoji (offset=2, length=2)
        let text = "😀😀";
        let entity = make_entity("italic", 2, 2);
        let parsed = parse_entity(text, &entity);
        assert_eq!(parsed.text, "😀");
    }

    #[test]
    fn url_entity_fills_url_field() {
        let text = "https://example.com";
        let entity = make_entity("url", 0, 19);
        let parsed = parse_entity(text, &entity);
        assert_eq!(parsed.url, Some("https://example.com".to_string()));
    }

    #[test]
    fn empty_entities() {
        let entities: Vec<MessageEntity> = vec![];
        let result = parse_entities("some text", &entities);
        assert!(result.is_empty());
    }
}
