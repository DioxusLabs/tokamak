use crate::state::State;
/// Exposes the `Endpoint` trait if you want to implement it for custom types.
///
/// This is not usually necessary since it's implemented for function types already.
use crate::{Request, Responder, Response, Result};
use async_trait::async_trait;
use std::future::Future;

/// Implement `Endpoint` for a type to be used as a method handler.
///
/// It is already implemented for functions of `Request` to `Result<Response>`
/// which is the simplest (and most convenient) kind of handler.
/// You can implement it manually for endpoints that may require some kind of local state.
///
/// `Endpoint` uses the `#[async_trait]` attribute hence the signature presented in the docs here
/// has been modified. An example of implementing using the attribute:
/// ```rust
/// # use highnoon::{Endpoint, State, Result, Request, Response};
/// struct NoOpEndpoint;
///
/// #[async_trait::async_trait]
/// impl<S: State> Endpoint<S> for NoOpEndpoint
/// {
///     async fn call(&self, req: Request<S>) -> Result<Response> {
///         Ok(Response::ok())
///     }
/// }
/// ```
#[async_trait]
pub trait Endpoint<S: State> {
    async fn call(&self, req: Request<S>) -> Result<Response>;
}

#[async_trait]
impl<S, F, Fut, R> Endpoint<S> for F
where
    F: Send + Sync + 'static + Fn(Request<S>) -> Fut,
    Fut: Future<Output = R> + Send + 'static,
    R: Responder + 'static,
    S: State,
{
    async fn call(&self, req: Request<S>) -> Result<Response> {
        (self)(req).await.into_response()
    }
}
