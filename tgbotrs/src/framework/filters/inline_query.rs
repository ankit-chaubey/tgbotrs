//! Filters for `InlineQuery` updates.

use crate::types::InlineQuery;
use regex::Regex;

pub fn all() -> impl super::Filter<InlineQuery> {
    |_: &InlineQuery| true
}
pub fn empty() -> impl super::Filter<InlineQuery> {
    |iq: &InlineQuery| iq.query.is_empty()
}

pub fn starts_with(prefix: impl Into<String>) -> impl super::Filter<InlineQuery> {
    let p = prefix.into();
    move |iq: &InlineQuery| iq.query.starts_with(&p as &str)
}

/// Panics if the pattern is invalid.
pub fn regex(pattern: impl AsRef<str>) -> impl super::Filter<InlineQuery> {
    let re = Regex::new(pattern.as_ref()).expect("invalid regex");
    move |iq: &InlineQuery| re.is_match(&iq.query)
}

pub fn from_user_id(id: i64) -> impl super::Filter<InlineQuery> {
    move |iq: &InlineQuery| iq.from.id == id
}
