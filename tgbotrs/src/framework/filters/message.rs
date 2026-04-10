//! Filters for `Message` updates.

use crate::types::Message;
use regex::Regex;

fn text_or_caption(m: &Message) -> Option<&str> {
    m.text.as_deref().or(m.caption.as_deref())
}

pub fn all() -> impl super::Filter<Message> {
    |_: &Message| true
}
pub fn text() -> impl super::Filter<Message> {
    |m: &Message| m.text.is_some()
}
pub fn caption() -> impl super::Filter<Message> {
    |m: &Message| m.caption.is_some()
}
pub fn photo() -> impl super::Filter<Message> {
    |m: &Message| m.photo.as_ref().map(|v| !v.is_empty()).unwrap_or(false)
}
pub fn video() -> impl super::Filter<Message> {
    |m: &Message| m.video.is_some()
}
pub fn audio() -> impl super::Filter<Message> {
    |m: &Message| m.audio.is_some()
}
pub fn document() -> impl super::Filter<Message> {
    |m: &Message| m.document.is_some()
}
pub fn sticker() -> impl super::Filter<Message> {
    |m: &Message| m.sticker.is_some()
}
pub fn voice() -> impl super::Filter<Message> {
    |m: &Message| m.voice.is_some()
}
pub fn video_note() -> impl super::Filter<Message> {
    |m: &Message| m.video_note.is_some()
}
pub fn location() -> impl super::Filter<Message> {
    |m: &Message| m.location.is_some()
}
pub fn contact() -> impl super::Filter<Message> {
    |m: &Message| m.contact.is_some()
}
pub fn poll() -> impl super::Filter<Message> {
    |m: &Message| m.poll.is_some()
}
pub fn venue() -> impl super::Filter<Message> {
    |m: &Message| m.venue.is_some()
}
pub fn animation() -> impl super::Filter<Message> {
    |m: &Message| m.animation.is_some()
}
pub fn reply() -> impl super::Filter<Message> {
    |m: &Message| m.reply_to_message.is_some()
}
pub fn forward() -> impl super::Filter<Message> {
    |m: &Message| m.forward_origin.is_some()
}

pub fn command() -> impl super::Filter<Message> {
    |m: &Message| {
        text_or_caption(m)
            .map(|t| t.starts_with('/'))
            .unwrap_or(false)
    }
}

pub fn private() -> impl super::Filter<Message> {
    |m: &Message| m.chat.r#type == "private"
}
pub fn group() -> impl super::Filter<Message> {
    |m: &Message| m.chat.r#type == "group"
}
pub fn supergroup() -> impl super::Filter<Message> {
    |m: &Message| m.chat.r#type == "supergroup"
}
pub fn channel() -> impl super::Filter<Message> {
    |m: &Message| m.chat.r#type == "channel"
}
pub fn from_bot() -> impl super::Filter<Message> {
    |m: &Message| m.from.as_ref().map(|u| u.is_bot).unwrap_or(false)
}
pub fn from_human() -> impl super::Filter<Message> {
    |m: &Message| m.from.as_ref().map(|u| !u.is_bot).unwrap_or(false)
}

pub fn new_chat_members() -> impl super::Filter<Message> {
    |m: &Message| {
        m.new_chat_members
            .as_ref()
            .map(|v| !v.is_empty())
            .unwrap_or(false)
    }
}
pub fn left_chat_member() -> impl super::Filter<Message> {
    |m: &Message| m.left_chat_member.is_some()
}

pub fn from_user_id(id: i64) -> impl super::Filter<Message> {
    move |m: &Message| m.from.as_ref().map(|u| u.id == id).unwrap_or(false)
}

pub fn from_username(username: impl Into<String>) -> impl super::Filter<Message> {
    let u = username.into();
    move |m: &Message| {
        m.from
            .as_ref()
            .and_then(|f| f.username.as_deref())
            .map(|n| n.eq_ignore_ascii_case(&u))
            .unwrap_or(false)
    }
}

pub fn chat_id(id: i64) -> impl super::Filter<Message> {
    move |m: &Message| m.chat.id == id
}

pub fn chat_username(username: impl Into<String>) -> impl super::Filter<Message> {
    let u = username.into();
    move |m: &Message| {
        m.chat
            .username
            .as_deref()
            .map(|n| n.eq_ignore_ascii_case(&u))
            .unwrap_or(false)
    }
}

/// Panics if the pattern is invalid. Use `regex_safe` if you need graceful handling.
pub fn regex(pattern: impl AsRef<str>) -> impl super::Filter<Message> {
    let re = Regex::new(pattern.as_ref()).expect("invalid regex");
    move |m: &Message| text_or_caption(m).map(|t| re.is_match(t)).unwrap_or(false)
}

pub fn regex_safe(pattern: impl AsRef<str>) -> Option<impl super::Filter<Message>> {
    let re = Regex::new(pattern.as_ref()).ok()?;
    Some(move |m: &Message| text_or_caption(m).map(|t| re.is_match(t)).unwrap_or(false))
}

pub fn contains(needle: impl Into<String>) -> impl super::Filter<Message> {
    let n = needle.into();
    move |m: &Message| {
        text_or_caption(m)
            .map(|t| t.contains(&n as &str))
            .unwrap_or(false)
    }
}

pub fn starts_with(prefix: impl Into<String>) -> impl super::Filter<Message> {
    let p = prefix.into();
    move |m: &Message| {
        text_or_caption(m)
            .map(|t| t.starts_with(&p as &str))
            .unwrap_or(false)
    }
}
