use crate::innerlude::*;
use futures_util::Future;

pub trait EndPoint<'a, S, A>: 'static {
    fn call(&self, req: Request, state: &'a A) -> Box<dyn Future<Output = TokamakResult> + 'a> {
        todo!()
    }
}

pub struct Stateless;
impl<'a, F, Fut, S> EndPoint<'a, Stateless, S> for F
where
    F: Fn(Request) -> Fut + 'static,
    Fut: Future<Output = TokamakResult> + 'a,
{
    fn call(&self, req: Request, _: &'a S) -> Box<dyn Future<Output = TokamakResult> + 'a> {
        Box::new((*self)(req))
    }
}

pub struct Stateful;
impl<'a, F, Fut, S> EndPoint<'a, Stateful, S> for F
where
    F: Fn(Request, &'a S) -> Fut + 'static,
    Fut: Future<Output = TokamakResult> + 'a,
    S: 'static,
    Fut: 'a,
{
    fn call(&self, req: Request, state: &'a S) -> Box<dyn Future<Output = TokamakResult> + 'a> {
        Box::new((*self)(req, state))
    }
}

pub struct StatefulImmediate;
impl<'a, F, S> EndPoint<'a, StatefulImmediate, S> for F
where
    F: Fn(Request, &'a S) -> TokamakResult + 'static,
    S: 'static,
{
    fn call(&self, req: Request, state: &'a S) -> Box<dyn Future<Output = TokamakResult> + 'a> {
        let fut = futures_util::future::ready((*self)(req, state));
        Box::new(fut)
    }
}

pub struct StatelessImmediate;
impl<'a, F, S> EndPoint<'a, StatelessImmediate, S> for F
where
    F: Fn(Request) -> TokamakResult + 'static,
{
    fn call(&self, req: Request, _: &'a S) -> Box<dyn Future<Output = TokamakResult> + 'a> {
        let fut = futures_util::future::ready((*self)(req));
        Box::new(fut)
    }
}
