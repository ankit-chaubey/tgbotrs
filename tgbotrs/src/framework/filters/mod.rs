//! Composable predicates for matching updates.

use std::sync::Arc;

/// Predicate over `T`. Implemented automatically for `Fn(&T) -> bool` closures.
pub trait Filter<T>: Send + Sync + 'static {
    fn check(&self, value: &T) -> bool;
}

impl<T, F> Filter<T> for F
where
    F: Fn(&T) -> bool + Send + Sync + 'static,
{
    fn check(&self, value: &T) -> bool {
        self(value)
    }
}

impl<T: 'static> Filter<T> for Arc<dyn Filter<T>> {
    fn check(&self, value: &T) -> bool {
        (**self).check(value)
    }
}

pub struct And<A, B, T>(A, B, std::marker::PhantomData<T>);
impl<A: Filter<T>, B: Filter<T>, T: Send + Sync + 'static> Filter<T> for And<A, B, T> {
    fn check(&self, v: &T) -> bool {
        self.0.check(v) && self.1.check(v)
    }
}

pub struct Or<A, B, T>(A, B, std::marker::PhantomData<T>);
impl<A: Filter<T>, B: Filter<T>, T: Send + Sync + 'static> Filter<T> for Or<A, B, T> {
    fn check(&self, v: &T) -> bool {
        self.0.check(v) || self.1.check(v)
    }
}

pub struct Not<F, T>(F, std::marker::PhantomData<T>);
impl<F: Filter<T>, T: Send + Sync + 'static> Filter<T> for Not<F, T> {
    fn check(&self, v: &T) -> bool {
        !self.0.check(v)
    }
}

/// Combinator methods available on every `Filter<T>`.
pub trait FilterExt<T>: Filter<T> + Sized {
    fn and<B: Filter<T>>(self, other: B) -> And<Self, B, T> {
        And(self, other, std::marker::PhantomData)
    }
    fn or<B: Filter<T>>(self, other: B) -> Or<Self, B, T> {
        Or(self, other, std::marker::PhantomData)
    }
    fn not(self) -> Not<Self, T> {
        Not(self, std::marker::PhantomData)
    }
}

impl<T: 'static, F: Filter<T> + Sized> FilterExt<T> for F {}

pub mod callback_query;
pub mod chat_member;
pub mod inline_query;
pub mod message;
