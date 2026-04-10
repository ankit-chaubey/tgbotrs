use std::{future::Future, sync::Arc};

use async_trait::async_trait;

use crate::{
    framework::{
        context::Context,
        filters::Filter,
        handler::{Handler, HandlerResult},
    },
    Bot,
};

type Fn = Arc<
    dyn std::ops::Fn(Bot, Context) -> std::pin::Pin<Box<dyn Future<Output = HandlerResult> + Send>>
        + Send
        + Sync,
>;

/// Fires when `update.message` matches a filter.
pub struct MessageHandler {
    name: String,
    filter: Box<dyn Filter<crate::types::Message>>,
    func: Fn,
}

impl MessageHandler {
    pub fn new<S, F, Fut>(
        name: S,
        filter: F,
        func: impl std::ops::Fn(Bot, Context) -> Fut + Send + Sync + 'static,
    ) -> Self
    where
        S: Into<String>,
        F: Filter<crate::types::Message> + 'static,
        Fut: Future<Output = HandlerResult> + Send + 'static,
    {
        Self {
            name: name.into(),
            filter: Box::new(filter),
            func: Arc::new(move |bot, ctx| Box::pin(func(bot, ctx))),
        }
    }
}

#[async_trait]
impl Handler for MessageHandler {
    fn name(&self) -> &str {
        &self.name
    }

    fn check_update(&self, ctx: &Context) -> bool {
        ctx.update
            .message
            .as_ref()
            .map(|m| self.filter.check(m))
            .unwrap_or(false)
    }

    async fn handle_update(&self, bot: Bot, ctx: Context) -> HandlerResult {
        (self.func)(bot, ctx).await
    }
}
