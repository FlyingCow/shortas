use anyhow::{Ok, Result};
use std::{fmt, future::Future, pin::Pin};
use std::fmt::{Debug, Error, Formatter};

pub trait Middleware<FR>: Debug + Send + Sync + 'static {
    /// Handle of the middleware logic.
    fn handle(
        &self,
        request: &FR,
        next: MiddlewareNext<FR>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}
/// Continuation of a [`Middleware`] chain.
pub struct MiddlewareNext<'a, FR> {
    pub(crate) chain: &'a mut (dyn Iterator<Item = &'a dyn Middleware<FR>>),
    // Since request_fn consumes the Payload<'a>, we must have an FnOnce.
    //
    // It's possible to get rid of this Box if we make MiddlewareNext generic
    // over some type variable, i.e. MiddlewareNext<'a, R> where R: FnOnce...
    // however that would "leak" to Middleware::handle introducing a complicated
    // type signature that is totally irrelevant for someone implementing a middleware.
    //
    // So in the name of having a sane external API, we accept this Box.
    pub(crate) request_fn:
        Box<dyn FnOnce(&FR) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + 'a>,
}

impl<'a, FR> MiddlewareNext<'a, FR> 
where FR: 'static {
    /// Continue the middleware chain by providing (a possibly amended) [`Request`].
    pub fn handle(self, request: &FR) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        if let Some(step) = self.chain.next() {
            step.handle(request, self)
        } else {
            (self.request_fn)(request)
        }
    }
}

impl<F, FR> Middleware<FR> for F
where
    F:Debug,
    F: Fn(&FR, MiddlewareNext<FR>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
        + Send
        + Sync
        + 'static,
{
    fn handle(
        &self,
        request: &FR,
        next: MiddlewareNext<FR>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        (self)(request, next)
    }
}

pub trait FlowRouterModule {
    
}