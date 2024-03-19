use anyhow::Result;
use std::fmt::Debug;
use std::{future::Future, pin::Pin};

pub trait Middleware<FR, CTX>: Debug + Send + Sync {
    /// Handle of the middleware logic.
    fn handle(
        &self,
        router: &FR,
        context: &CTX,
        next: MiddlewareNext<FR, CTX>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>;
}
/// Continuation of a [`Middleware`] chain.
pub struct MiddlewareNext<'a, FR, CTX> {
    pub(crate) chain: &'a mut (dyn Iterator<Item = &'a dyn Middleware<FR, CTX>>),

    // Since request_fn consumes the Payload<'a>, we must have an FnOnce.
    //
    // It's possible to get rid of this Box if we make MiddlewareNext generic
    // over some type variable, i.e. MiddlewareNext<'a, R> where R: FnOnce...
    // however that would "leak" to Middleware::handle introducing a complicated
    // type signature that is totally irrelevant for someone implementing a middleware.
    //
    // So in the name of having a sane external API, we accept this Box.
    pub(crate) request_fn:
        Box<dyn FnOnce(&FR, &CTX) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> + 'a>,
}

impl<'a, FR, CTX> MiddlewareNext<'a, FR, CTX>
where
    FR: 'static,
{
    /// Continue the middleware chain by providing (a possibly amended) [`Request`].
    pub fn handle(self, router: &FR, context: &CTX) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        if let Some(step) = self.chain.next() {
            step.handle(router, context, self)
        } else {
            (self.request_fn)(router, context)
        }
    }
}

impl<F, FR, CTX> Middleware<FR, CTX> for F
where
    F: Debug,
    F: Fn(&FR, &CTX, MiddlewareNext<FR, CTX>) -> Pin<Box<dyn Future<Output = Result<()>> + Send>>
        + Send
        + Sync
        + 'static,
{
    fn handle(
        &self,
        router: &FR,
        context: &CTX,
        next: MiddlewareNext<FR, CTX>,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + Send>> {
        (self)(router, context, next)
    }
}

pub trait FlowRouterModule {}
