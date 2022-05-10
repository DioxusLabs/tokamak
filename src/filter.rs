use crate::endpoint::Endpoint;
/// Filters are reusable bits of logic that wrap endpoints.
///
/// (These are sometimes called "middleware" in other frameworks).
use crate::{Request, Response, Result, SharedState};
use async_trait::async_trait;
use std::future::Future;

mod log;
// pub mod session; // TODO - export the needed bits of this

pub use self::log::Log;

/// Represents either the next Filter in the chain, or the actual endpoint if the chain is
/// empty or completed. Use its `next` method to call the next filter/endpoint if the
/// request should continue to be processed.
pub struct Next<'a> {
    pub(crate) ep: &'a (dyn Endpoint + Send + Sync),
    pub(crate) rest: &'a [Box<dyn Filter + Send + Sync + 'static>],
}

impl Next<'_> {
    /// Call either the next filter in the chain, or the actual endpoint if there are no more
    /// filters. Filters are not required to call next (eg. to return a Forbidden status instead)
    pub async fn next(self, req: Request) -> Result<Response> {
        match self.rest.split_first() {
            Some((head, rest)) => {
                let next = Next { ep: self.ep, rest };
                head.apply(req, next).await
            }
            None => self.ep.call(req).await,
        }
    }
}

/// A Filter is a reusable bit of logic which wraps an endpoint to provide pre- and post-processing.
/// Filters can call the `Next` argument to continue processing, or may return early to stop the
/// chain. Filters can be used for logging, authentication, cookie handling and many other uses.
///
/// `Filter` uses the `#[async_trait]` attribute hence the signature presented in the docs here has
/// been modified. An example of implementing using the attribute:
/// ```rust
/// # use highnoon::{filter::{Filter, Next}, State, Result, Request, Response};
/// struct NoOpFilter;
///
/// #[async_trait]
/// impl<S: State> Filter for NoOpFilter
/// {
///     async fn apply(&self, req: Request, next: Next<'_>) -> Result<Response> {
///         next.next(req)
///     }
/// }
/// ```
#[async_trait]
pub trait Filter {
    async fn apply(&self, req: Request, next: Next<'_>) -> Result<Response>;
}

// implement for async functions
#[async_trait]
impl<F, Fut> Filter for F
where
    F: Send + Sync + 'static + for<'n> Fn(Request, Next<'n>) -> Fut,
    Fut: Send + 'static + Future<Output = Result<Response>>,
{
    async fn apply(&self, req: Request, next: Next<'_>) -> Result<Response> {
        self(req, next).await
    }
}
