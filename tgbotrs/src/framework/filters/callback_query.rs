//! Filters for `CallbackQuery` updates.

use crate::types::CallbackQuery;
use regex::Regex;

pub fn all() -> impl super::Filter<CallbackQuery> {
    |_: &CallbackQuery| true
}

pub fn data_eq(expected: impl Into<String>) -> impl super::Filter<CallbackQuery> {
    let e = expected.into();
    move |cq: &CallbackQuery| cq.data.as_deref() == Some(&e)
}

pub fn data_starts_with(prefix: impl Into<String>) -> impl super::Filter<CallbackQuery> {
    let p = prefix.into();
    move |cq: &CallbackQuery| {
        cq.data
            .as_deref()
            .map(|d| d.starts_with(&p as &str))
            .unwrap_or(false)
    }
}

/// Panics if the pattern is invalid.
pub fn data_regex(pattern: impl AsRef<str>) -> impl super::Filter<CallbackQuery> {
    let re = Regex::new(pattern.as_ref()).expect("invalid regex");
    move |cq: &CallbackQuery| cq.data.as_deref().map(|d| re.is_match(d)).unwrap_or(false)
}

pub fn data_regex_safe(pattern: impl AsRef<str>) -> Option<impl super::Filter<CallbackQuery>> {
    let re = Regex::new(pattern.as_ref()).ok()?;
    Some(move |cq: &CallbackQuery| cq.data.as_deref().map(|d| re.is_match(d)).unwrap_or(false))
}

pub fn game() -> impl super::Filter<CallbackQuery> {
    |cq: &CallbackQuery| cq.game_short_name.is_some()
}

pub fn from_user_id(id: i64) -> impl super::Filter<CallbackQuery> {
    move |cq: &CallbackQuery| cq.from.id == id
}
