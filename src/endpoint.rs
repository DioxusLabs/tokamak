use std::pin::Pin;

use crate::innerlude::*;
use futures_util::Future;

pub struct TrueEndpoint<S> {
    inner: Box<
        dyn for<'a> Fn(Request, &'a S) -> Pin<Box<dyn Future<Output = ResponseResult> + 'a>>
            + Send
            + Sync,
    >,
}

impl<State> TrueEndpoint<State> {
    pub fn new<'a, Sig>(inner: impl EndPoint<'a, Sig, State>) -> Self {
        todo!()
        // let f = Box::pin(move |req: Request, s: &State| inner.call(req, s))
        //     as Box<
        //         dyn for<'a> Fn(
        //             Request,
        //             &'a State,
        //         )
        //             -> Pin<Box<dyn Future<Output = TokamakResult> + 'a>>,
        //     >;

        // Self { inner: f }
    }
}

pub trait EndPoint<'a, Sig, State>: 'static {
    fn call(
        &self,
        req: Request,
        state: &'a State,
    ) -> Pin<Box<dyn Future<Output = ResponseResult> + 'a>> {
        todo!()
    }
}

pub struct Stateless;
impl<'a, F, Fut, S> EndPoint<'a, Stateless, S> for F
where
    F: Fn(Request) -> Fut + 'static,
    Fut: Future<Output = ResponseResult> + 'a,
{
    fn call(&self, req: Request, _: &'a S) -> Pin<Box<dyn Future<Output = ResponseResult> + 'a>> {
        Box::pin((*self)(req))
    }
}

pub struct Stateful;
impl<'a, F, Fut, S> EndPoint<'a, Stateful, S> for F
where
    F: Fn(Request, &'a S) -> Fut + 'static,
    Fut: Future<Output = ResponseResult> + 'a,
    S: 'static,
    Fut: 'a,
{
    fn call(
        &self,
        req: Request,
        state: &'a S,
    ) -> Pin<Box<dyn Future<Output = ResponseResult> + 'a>> {
        Box::pin((*self)(req, state))
    }
}

pub struct StatefulImmediate;
impl<'a, F, S> EndPoint<'a, StatefulImmediate, S> for F
where
    F: Fn(Request, &'a S) -> ResponseResult + 'static,
    S: 'static,
{
    fn call(
        &self,
        req: Request,
        state: &'a S,
    ) -> Pin<Box<dyn Future<Output = ResponseResult> + 'a>> {
        let fut = futures_util::future::ready((*self)(req, state));
        Box::pin(fut)
    }
}

pub struct StatelessImmediate;
impl<'a, F, S> EndPoint<'a, StatelessImmediate, S> for F
where
    F: Fn(Request) -> ResponseResult + 'static,
{
    fn call(&self, req: Request, _: &'a S) -> Pin<Box<dyn Future<Output = ResponseResult> + 'a>> {
        let fut = futures_util::future::ready((*self)(req));
        Box::pin(fut)
    }
}
