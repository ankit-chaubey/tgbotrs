//! Dispatcher - routes updates to handler groups in ascending order.
//!
//! Within a group the first matching handler runs; dispatcher then moves to the next group.
//! Handlers can alter flow by returning `Err(ContinueGroups)` or `Err(EndGroups)`.

use std::{
    collections::BTreeMap,
    error::Error,
    sync::{Arc, RwLock},
};

use tokio::sync::Semaphore;
use tracing::{debug, error, warn};

use crate::{
    framework::{
        context::Context,
        handler::{ContinueGroups, EndGroups, Handler},
    },
    types::Update,
    Bot,
};

/// What the dispatcher does after an error hook returns.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum DispatcherAction {
    #[default]
    Noop, // move to next group
    ContinueGroups, // keep iterating current group
    EndGroups,      // stop everything
}

pub type ErrorHook = Arc<
    dyn std::ops::Fn(&Bot, &Context, &(dyn Error + Send + Sync)) -> DispatcherAction + Send + Sync,
>;

pub type PanicHook = Arc<dyn std::ops::Fn(&Bot, &Context, String) + Send + Sync>;

/// Options for `Dispatcher::new`.
#[derive(Clone, Default)]
pub struct DispatcherOpts {
    pub max_routines: Option<usize>,
    pub error_handler: Option<ErrorHook>,
    pub panic_handler: Option<PanicHook>,
}

impl DispatcherOpts {
    pub fn max_routines(mut self, n: usize) -> Self {
        self.max_routines = Some(n);
        self
    }

    pub fn on_error<F>(mut self, f: F) -> Self
    where
        F: std::ops::Fn(&Bot, &Context, &(dyn Error + Send + Sync)) -> DispatcherAction
            + Send
            + Sync
            + 'static,
    {
        self.error_handler = Some(Arc::new(f));
        self
    }

    pub fn on_panic<F>(mut self, f: F) -> Self
    where
        F: std::ops::Fn(&Bot, &Context, String) + Send + Sync + 'static,
    {
        self.panic_handler = Some(Arc::new(f));
        self
    }
}

type HandlerMap = BTreeMap<i32, Vec<Arc<dyn Handler>>>;

pub struct Dispatcher {
    handlers: Arc<RwLock<HandlerMap>>,
    error_handler: Option<ErrorHook>,
    panic_handler: Option<PanicHook>,
    semaphore: Option<Arc<Semaphore>>,
}

impl Dispatcher {
    pub fn new(opts: DispatcherOpts) -> Self {
        Self {
            handlers: Arc::new(RwLock::new(BTreeMap::new())),
            error_handler: opts.error_handler,
            panic_handler: opts.panic_handler,
            semaphore: opts.max_routines.map(|n| Arc::new(Semaphore::new(n))),
        }
    }

    pub fn add_handler<H: Handler + 'static>(&mut self, handler: H) {
        self.add_handler_to_group(handler, 0);
    }

    pub fn add_handler_to_group<H: Handler + 'static>(&mut self, handler: H, group: i32) {
        self.handlers
            .write()
            .unwrap()
            .entry(group)
            .or_default()
            .push(Arc::new(handler));
    }

    pub fn remove_handler(&mut self, name: &str, group: i32) -> bool {
        let mut map = self.handlers.write().unwrap();
        if let Some(vec) = map.get_mut(&group) {
            if let Some(pos) = vec.iter().position(|h| h.name() == name) {
                vec.remove(pos);
                return true;
            }
        }
        false
    }

    pub fn remove_handler_any_group(&mut self, name: &str) -> Option<i32> {
        let mut map = self.handlers.write().unwrap();
        for (&group, vec) in map.iter_mut() {
            if let Some(pos) = vec.iter().position(|h| h.name() == name) {
                vec.remove(pos);
                return Some(group);
            }
        }
        None
    }

    pub fn remove_group(&mut self, group: i32) -> bool {
        self.handlers.write().unwrap().remove(&group).is_some()
    }

    /// Dispatch an update, spawning a Tokio task. Returns immediately.
    pub fn dispatch(&self, bot: Bot, update: Update) {
        let handlers_arc = Arc::clone(&self.handlers);
        let error_hook = self.error_handler.clone();
        let panic_hook = self.panic_handler.clone();
        let semaphore = self.semaphore.clone();

        tokio::spawn(async move {
            let _permit = if let Some(sem) = &semaphore {
                Some(sem.clone().acquire_owned().await.ok())
            } else {
                None
            };

            let ctx = Context::new(update);
            // Snapshot the entire handler map once — single read lock, no per-group re-lock.
            let snapshot: HandlerMap = handlers_arc.read().unwrap().clone();

            'groups: for (group, handlers) in snapshot {
                for handler in handlers {
                    if !handler.check_update(&ctx) {
                        continue;
                    }

                    debug!(handler = handler.name(), group, "matched");

                    let h = Arc::clone(&handler);
                    let bot2 = bot.clone();
                    let ctx2 = ctx.clone();
                    let join = tokio::spawn(async move { h.handle_update(bot2, ctx2).await });

                    match join.await {
                        Err(e) if e.is_panic() => {
                            let msg = e
                                .into_panic()
                                .downcast::<String>()
                                .map(|s| *s)
                                .or_else(|p| p.downcast::<&str>().map(|s| s.to_string()))
                                .unwrap_or_else(|_| "<non-string panic>".into());
                            if let Some(hook) = &panic_hook {
                                hook(&bot, &ctx, msg);
                            } else {
                                error!(handler = handler.name(), group, panic = %msg, "panicked");
                            }
                            break;
                        }
                        Err(e) => {
                            error!(handler = handler.name(), group, error = %e, "task aborted");
                            break;
                        }
                        Ok(Err(e)) => {
                            if e.is::<ContinueGroups>() {
                                debug!(handler = handler.name(), "ContinueGroups");
                                continue;
                            }
                            if e.is::<EndGroups>() {
                                debug!(handler = handler.name(), "EndGroups");
                                break 'groups;
                            }
                            warn!(handler = handler.name(), group, error = %e);
                            let action = error_hook
                                .as_ref()
                                .map(|h| h(&bot, &ctx, e.as_ref()))
                                .unwrap_or_default();
                            match action {
                                DispatcherAction::Noop => break,
                                DispatcherAction::ContinueGroups => continue,
                                DispatcherAction::EndGroups => break 'groups,
                            }
                        }
                        Ok(Ok(())) => {
                            debug!(handler = handler.name(), group, "ok");
                            break;
                        }
                    }
                }
            }
        });
    }

    /// Run an update in the calling task (no panic recovery; useful for tests).
    pub async fn process_update(&self, bot: &Bot, update: Update) {
        let ctx = Context::new(update);
        let snapshot: HandlerMap = self.handlers.read().unwrap().clone();

        'groups: for (_, handlers) in snapshot {
            for handler in handlers {
                if !handler.check_update(&ctx) {
                    continue;
                }
                match handler.handle_update(bot.clone(), ctx.clone()).await {
                    Err(e) if e.is::<ContinueGroups>() => continue,
                    Err(e) if e.is::<EndGroups>() => break 'groups,
                    Err(e) => {
                        let action = self
                            .error_handler
                            .as_ref()
                            .map(|h| h(bot, &ctx, e.as_ref()))
                            .unwrap_or_default();
                        match action {
                            DispatcherAction::Noop => break,
                            DispatcherAction::ContinueGroups => continue,
                            DispatcherAction::EndGroups => break 'groups,
                        }
                    }
                    Ok(()) => break,
                }
            }
        }
    }
}
