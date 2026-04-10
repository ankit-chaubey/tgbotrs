use std::{future::Future, sync::Arc};

use async_trait::async_trait;

use crate::{
    framework::{
        context::Context,
        handler::{Handler, HandlerResult},
    },
    Bot,
};

type Fn = Arc<
    dyn std::ops::Fn(Bot, Context) -> std::pin::Pin<Box<dyn Future<Output = HandlerResult> + Send>>
        + Send
        + Sync,
>;

/// Fires on `/command[@botname] arg1 arg2`. Args land in `ctx.args()`.
pub struct CommandHandler {
    name: String,
    command: String,
    prefix: char,
    func: Fn,
}

impl CommandHandler {
    /// `command` is the word after the prefix, e.g. `"start"` for `/start`.
    pub fn new<S, Fut>(
        command: S,
        func: impl std::ops::Fn(Bot, Context) -> Fut + Send + Sync + 'static,
    ) -> Self
    where
        S: Into<String>,
        Fut: Future<Output = HandlerResult> + Send + 'static,
    {
        let cmd = command.into();
        Self {
            name: format!("command:{cmd}"),
            command: cmd,
            prefix: '/',
            func: Arc::new(move |bot, ctx| Box::pin(func(bot, ctx))),
        }
    }

    /// Override the prefix character (default `/`).
    pub fn prefix(mut self, p: char) -> Self {
        self.prefix = p;
        self
    }

    /// Override the handler name (default `"command:<cmd>"`).
    pub fn named(mut self, name: impl Into<String>) -> Self {
        self.name = name.into();
        self
    }

    /// Returns parsed args on match, `None` on mismatch.
    /// Pass `bot_username` to enforce `@mention` targeting.
    fn parse(&self, text: &str, bot_username: Option<&str>) -> Option<Vec<String>> {
        let text = text.trim();
        if !text.starts_with(self.prefix) {
            return None;
        }
        let rest = &text[self.prefix.len_utf8()..];

        let mut parts = rest.splitn(2, char::is_whitespace);
        let cmd_word = parts.next().unwrap_or("");
        let arg_str = parts.next().unwrap_or("").trim();

        // Strip optional @botname suffix.
        let cmd_word = if let Some(at) = cmd_word.find('@') {
            let suffix = &cmd_word[at + 1..];
            if let Some(uname) = bot_username {
                if !suffix.eq_ignore_ascii_case(uname) {
                    return None;
                }
            }
            &cmd_word[..at]
        } else {
            cmd_word
        };

        if !cmd_word.eq_ignore_ascii_case(&self.command) {
            return None;
        }

        let args = if arg_str.is_empty() {
            vec![]
        } else {
            arg_str.split_whitespace().map(String::from).collect()
        };
        Some(args)
    }
}

#[async_trait]
impl Handler for CommandHandler {
    fn name(&self) -> &str {
        &self.name
    }

    fn check_update(&self, ctx: &Context) -> bool {
        let msg = match ctx.update.message.as_ref() {
            Some(m) => m,
            None => return false,
        };
        let text = match msg.text.as_deref().or(msg.caption.as_deref()) {
            Some(t) => t,
            None => return false,
        };

        // Strip @suffix from the command token before checking (username validated in handle_update).
        let stripped = if let Some(at) = text.find('@') {
            let sp = text.find(char::is_whitespace).unwrap_or(text.len());
            if at < sp {
                format!("{}{}", &text[..at], &text[sp..])
            } else {
                text.to_string()
            }
        } else {
            text.to_string()
        };

        self.parse(&stripped, None).is_some()
    }

    async fn handle_update(&self, bot: Bot, mut ctx: Context) -> HandlerResult {
        let msg = match ctx.update.message.as_ref() {
            Some(m) => m,
            None => return Ok(()),
        };
        let text = match msg.text.as_deref().or(msg.caption.as_deref()) {
            Some(t) => t,
            None => return Ok(()),
        };

        if let Some(args) = self.parse(text, bot.me.username.as_deref()) {
            ctx.args = args;
            (self.func)(bot, ctx).await
        } else {
            Ok(())
        }
    }
}
