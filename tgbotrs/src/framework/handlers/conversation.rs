//! Stateful conversation handler.
//!
//! # Usage
//!
//! ```rust,no_run
//! use tgbotrs::framework::handlers::conversation::{
//!     ConversationHandler, ConversationOpts, InMemoryStorage, KeyStrategy,
//!     NextState, EndConversation,
//! };
//! use tgbotrs::{CommandHandler, MessageHandler};
//!
//! fn any_message(_: &tgbotrs::Message) -> bool { true }
//!
//! let handler = ConversationHandler::new(
//!     vec![Box::new(CommandHandler::new("start", |bot, ctx| Box::pin(async move { Ok(()) })))],
//!     std::collections::HashMap::from([
//!         ("ask_name".to_string(), vec![
//!             Box::new(MessageHandler::new("ask_name_handler", any_message, |bot, ctx| Box::pin(async move { Ok(()) }))) as Box<dyn tgbotrs::Handler>,
//!         ]),
//!     ]),
//!     ConversationOpts::default(),
//! );
//! ```

use std::{
    collections::HashMap,
    error::Error,
    fmt,
    sync::{Arc, RwLock},
};

use async_trait::async_trait;

use crate::{
    framework::{
        context::Context,
        handler::{Handler, HandlerResult},
    },
    Bot,
};

// State key
/// How to derive the storage key for a conversation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum KeyStrategy {
    /// One conversation per (bot_id, sender_id, chat_id) - the default.
    #[default]
    SenderAndChat,
    /// One conversation per (bot_id, sender_id) across all chats.
    Sender,
    /// One conversation per (bot_id, chat_id) shared among all senders.
    Chat,
}

/// Derive the storage key from a context using the given strategy.
pub fn conversation_key(ctx: &Context, strategy: KeyStrategy) -> Option<String> {
    let _update = &ctx.update;
    let bot_id: i64 = 0; // no live bot available at check time; use 0 as placeholder

    let chat_id = ctx.effective_chat().map(|c| c.id);
    let user_id = ctx.effective_user().map(|u| u.id);

    match strategy {
        KeyStrategy::SenderAndChat => Some(format!("{}/{}/{}", bot_id, user_id?, chat_id?)),
        KeyStrategy::Sender => Some(format!("{}/{}", bot_id, user_id?)),
        KeyStrategy::Chat => Some(format!("{}/{}", bot_id, chat_id?)),
    }
}

// Storage trait
/// Error returned when a conversation key is not in storage.
#[derive(Debug, Clone, Copy)]
pub struct KeyNotFound;
impl fmt::Display for KeyNotFound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("conversation key not found")
    }
}
impl Error for KeyNotFound {}

/// Pluggable storage backend for conversation state.
pub trait ConversationStorage: Send + Sync {
    fn get(&self, key: &str) -> Result<std::sync::Arc<str>, KeyNotFound>;
    fn set(&self, key: &str, state: &str);
    fn delete(&self, key: &str);
}

// In-memory storage
/// Thread-safe in-memory storage (default).
#[derive(Debug, Default)]
pub struct InMemoryStorage {
    // Arc<str> values make `.clone()` a pointer bump instead of a heap allocation.
    map: RwLock<HashMap<String, std::sync::Arc<str>>>,
}

impl InMemoryStorage {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }
}

impl ConversationStorage for InMemoryStorage {
    fn get(&self, key: &str) -> Result<std::sync::Arc<str>, KeyNotFound> {
        self.map
            .read()
            .unwrap()
            .get(key)
            .cloned()
            .ok_or(KeyNotFound)
    }
    fn set(&self, key: &str, state: &str) {
        self.map
            .write()
            .unwrap()
            .insert(key.to_string(), std::sync::Arc::from(state));
    }
    fn delete(&self, key: &str) {
        self.map.write().unwrap().remove(key);
    }
}

// State change sentinels
/// Return this from a handler to transition to the named state.
#[derive(Debug, Clone)]
pub struct NextState(pub String);
impl fmt::Display for NextState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "NextState({})", self.0)
    }
}
impl Error for NextState {}

/// Return this from a handler to end the conversation.
#[derive(Debug, Clone, Copy)]
pub struct EndConversation;
impl fmt::Display for EndConversation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EndConversation")
    }
}
impl Error for EndConversation {}

// ConversationHandler
/// Options for `ConversationHandler`.
#[derive(Default)]
pub struct ConversationOpts {
    /// Handlers that exit the conversation (e.g. `/cancel`). They end the
    /// conversation even if they return `Ok(())`.
    pub exits: Vec<Box<dyn Handler>>,
    /// Handlers tried when no state handler matches.
    pub fallbacks: Vec<Box<dyn Handler>>,
    /// If `true`, hitting an entry point from within a conversation restarts it.
    pub allow_re_entry: bool,
    /// Storage backend. Defaults to `InMemoryStorage(SenderAndChat)`.
    pub storage: Option<Arc<dyn ConversationStorage>>,
    /// Key strategy. Defaults to `SenderAndChat`.
    pub key_strategy: KeyStrategy,
    /// Unique name for this handler (for removal from dispatcher groups).
    pub name: Option<String>,
}

/// Stateful conversation handler (item 21).
pub struct ConversationHandler {
    name: String,
    entry_points: Vec<Box<dyn Handler>>,
    states: HashMap<String, Vec<Box<dyn Handler>>>,
    exits: Vec<Box<dyn Handler>>,
    fallbacks: Vec<Box<dyn Handler>>,
    allow_re_entry: bool,
    storage: Arc<dyn ConversationStorage>,
    key_strategy: KeyStrategy,
}

impl ConversationHandler {
    pub fn new(
        entry_points: Vec<Box<dyn Handler>>,
        states: HashMap<String, Vec<Box<dyn Handler>>>,
        opts: ConversationOpts,
    ) -> Self {
        let storage = opts
            .storage
            .unwrap_or_else(|| InMemoryStorage::new() as Arc<dyn ConversationStorage>);
        Self {
            name: opts
                .name
                .unwrap_or_else(|| format!("conversation_{:p}", &storage)),
            entry_points,
            states,
            exits: opts.exits,
            fallbacks: opts.fallbacks,
            allow_re_entry: opts.allow_re_entry,
            storage,
            key_strategy: opts.key_strategy,
        }
    }

    fn get_key(&self, ctx: &Context) -> Option<String> {
        conversation_key(ctx, self.key_strategy)
    }

    fn find_matching<'a>(
        handlers: &'a [Box<dyn Handler>],
        ctx: &Context,
    ) -> Option<&'a dyn Handler> {
        handlers
            .iter()
            .find(|h| h.check_update(ctx))
            .map(|h| h.as_ref())
    }

    fn next_handler<'a>(&'a self, ctx: &Context) -> Option<(HandlerKind, &'a dyn Handler)> {
        let key = self.get_key(ctx)?;
        let in_conversation = self.storage.get(&key).is_ok();

        // Check re-entry first if already in conversation.
        if in_conversation && self.allow_re_entry {
            if let Some(h) = Self::find_matching(&self.entry_points, ctx) {
                return Some((HandlerKind::Entry, h));
            }
        }

        if !in_conversation {
            // New conversation: try entry points.
            return Self::find_matching(&self.entry_points, ctx).map(|h| (HandlerKind::Entry, h));
        }

        let state = self.storage.get(&key).ok()?;

        // Exit handlers (auto-end the conversation).
        if let Some(h) = Self::find_matching(&self.exits, ctx) {
            return Some((HandlerKind::Exit, h));
        }

        // State-specific handlers.
        if let Some(handlers) = self.states.get(state.as_ref()) {
            if let Some(h) = Self::find_matching(handlers, ctx) {
                return Some((HandlerKind::State, h));
            }
        }

        // Fallbacks.
        Self::find_matching(&self.fallbacks, ctx).map(|h| (HandlerKind::Fallback, h))
    }
}

#[derive(Debug, Clone, Copy)]
enum HandlerKind {
    Entry,
    Exit,
    State,
    Fallback,
}

#[async_trait]
impl Handler for ConversationHandler {
    fn name(&self) -> &str {
        &self.name
    }

    fn check_update(&self, ctx: &Context) -> bool {
        self.next_handler(ctx).is_some()
    }

    async fn handle_update(&self, bot: Bot, ctx: Context) -> HandlerResult {
        let key = match self.get_key(&ctx) {
            Some(k) => k,
            None => return Ok(()),
        };

        let (kind, handler) = match self.next_handler(&ctx) {
            Some(h) => h,
            None => return Ok(()),
        };

        let result = handler.handle_update(bot, ctx).await;

        match kind {
            HandlerKind::Exit => {
                // Exit handlers always end the conversation.
                self.storage.delete(&key);
            }
            HandlerKind::Entry | HandlerKind::State | HandlerKind::Fallback => {
                match result {
                    Ok(()) => {} // no state change
                    Err(ref e) => {
                        if let Some(ns) = e.downcast_ref::<NextState>() {
                            if self.states.contains_key(&ns.0) {
                                self.storage.set(&key, &ns.0);
                            }
                        } else if e.is::<EndConversation>() {
                            self.storage.delete(&key);
                        } else {
                            return result;
                        }
                    }
                }
            }
        }

        Ok(())
    }
}
