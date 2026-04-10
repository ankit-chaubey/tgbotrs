//! Filters for `ChatMemberUpdated` updates.

use crate::types::{ChatMember, ChatMemberUpdated};

fn status(cm: &ChatMember) -> &str {
    match cm {
        ChatMember::ChatMemberOwner(m) => &m.status,
        ChatMember::ChatMemberAdministrator(m) => &m.status,
        ChatMember::ChatMemberMember(m) => &m.status,
        ChatMember::ChatMemberRestricted(m) => &m.status,
        ChatMember::ChatMemberLeft(m) => &m.status,
        ChatMember::ChatMemberBanned(m) => &m.status,
    }
}

pub fn all() -> impl super::Filter<ChatMemberUpdated> {
    |_: &ChatMemberUpdated| true
}

pub fn joined() -> impl super::Filter<ChatMemberUpdated> {
    |c: &ChatMemberUpdated| {
        matches!(
            status(&c.new_chat_member),
            "member" | "administrator" | "creator"
        )
    }
}

pub fn left() -> impl super::Filter<ChatMemberUpdated> {
    |c: &ChatMemberUpdated| matches!(status(&c.new_chat_member), "left" | "kicked")
}

pub fn promoted() -> impl super::Filter<ChatMemberUpdated> {
    |c: &ChatMemberUpdated| status(&c.new_chat_member) == "administrator"
}

pub fn chat_id(id: i64) -> impl super::Filter<ChatMemberUpdated> {
    move |c: &ChatMemberUpdated| c.chat.id == id
}
