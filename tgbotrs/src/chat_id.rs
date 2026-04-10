use serde::{Deserialize, Serialize};

/// A chat identifier - either a numeric ID or a `@username` string.
///
/// All methods that accept `ChatId` also accept `i64`, `&str`, and `String` via `Into<ChatId>`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ChatId {
    Id(i64),
    Username(String),
}

impl From<i64> for ChatId {
    fn from(id: i64) -> Self {
        ChatId::Id(id)
    }
}
impl From<i32> for ChatId {
    fn from(id: i32) -> Self {
        ChatId::Id(id as i64)
    }
}
impl From<u64> for ChatId {
    fn from(id: u64) -> Self {
        ChatId::Id(id as i64)
    }
}
impl From<String> for ChatId {
    fn from(s: String) -> Self {
        ChatId::Username(s)
    }
}
impl From<&str> for ChatId {
    fn from(s: &str) -> Self {
        ChatId::Username(s.to_string())
    }
}

impl std::fmt::Display for ChatId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChatId::Id(id) => write!(f, "{}", id),
            ChatId::Username(s) => write!(f, "{}", s),
        }
    }
}
