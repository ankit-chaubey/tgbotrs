use crate::types::{ForceReply, InlineKeyboardMarkup, ReplyKeyboardMarkup, ReplyKeyboardRemove};
use serde::{Deserialize, Serialize};

/// The reply_markup field can be one of four types.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ReplyMarkup {
    InlineKeyboard(InlineKeyboardMarkup),
    ReplyKeyboard(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(v: InlineKeyboardMarkup) -> Self {
        ReplyMarkup::InlineKeyboard(v)
    }
}
impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(v: ReplyKeyboardMarkup) -> Self {
        ReplyMarkup::ReplyKeyboard(v)
    }
}
impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(v: ReplyKeyboardRemove) -> Self {
        ReplyMarkup::ReplyKeyboardRemove(v)
    }
}
impl From<ForceReply> for ReplyMarkup {
    fn from(v: ForceReply) -> Self {
        ReplyMarkup::ForceReply(v)
    }
}
