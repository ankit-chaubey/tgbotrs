use std::{error::Error, fmt};

use async_trait::async_trait;

use crate::{framework::context::Context, Bot};

/// Return from a handler to keep iterating the current group.
#[derive(Debug, Clone, Copy)]
pub struct ContinueGroups;
impl fmt::Display for ContinueGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ContinueGroups")
    }
}
impl Error for ContinueGroups {}

/// Return from a handler to stop all group iteration.
#[derive(Debug, Clone, Copy)]
pub struct EndGroups;
impl fmt::Display for EndGroups {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EndGroups")
    }
}
impl Error for EndGroups {}

/// Return type for all handler functions.
pub type HandlerResult = Result<(), Box<dyn Error + Send + Sync>>;

/// Core trait every handler implements.
#[async_trait]
pub trait Handler: Send + Sync {
    /// Unique name used for removal from groups.
    fn name(&self) -> &str;
    /// Return `true` if this handler should process the update. Keep it cheap.
    fn check_update(&self, ctx: &Context) -> bool;
    /// Run the handler. Return `Err(ContinueGroups)` or `Err(EndGroups)` to change dispatch flow.
    async fn handle_update(&self, bot: Bot, ctx: Context) -> HandlerResult;
}
