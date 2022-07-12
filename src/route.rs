use std::pin::Pin;

use futures_lite::Future;

use crate::{fromrequest::FromRequest, innerlude::*};

pub struct Route<'a, T: Send + Sync + 'static = ()> {
    pub path: &'static str,
    pub app: &'a mut App<T>,
}

impl<'a, T: Send + Sync> Route<'a, T> {
    pub fn get<'b, Sig, E0, E1, E2, E3, E4>(
        &mut self,
        t: impl Endpoint<'b, Sig, T, E0, E1, E2, E3, E4>,
    ) -> &mut Self {
        self
    }

    pub fn post<'b, F>(&mut self, t: impl Endpoint<'b, F, T>) -> &mut Self {
        self
    }

    pub fn any<'b, F>(&mut self, t: impl Endpoint<'a, F, T>) -> &mut Self {
        todo!()
    }

    pub fn filter<'b, F>(&mut self, f: impl Filter<'b, F, T>) -> &mut Self {
        todo!()
    }

    pub fn extract<S>(&mut self, f: impl Fn(Request) -> crate::Result<S>) -> &mut Self {
        todo!()
    }
}

pub trait Filter<'a, Sig, State>: 'static {
    fn call(&self, req: Request, state: &'a State) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        todo!()
    }
}

pub struct Stateless;
impl<'a, F, Fut, S> Filter<'a, Stateless, S> for F
where
    F: Fn(Request) -> Fut + 'static,
    Fut: Future<Output = bool> + 'a,
{
    fn call(&self, req: Request, _: &'a S) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        Box::pin((*self)(req))
    }
}

pub struct Stateful;
impl<'a, F, Fut, S> Filter<'a, Stateful, S> for F
where
    F: Fn(Request, &'a S) -> Fut + 'static,
    Fut: Future<Output = bool> + 'a,
    S: 'static,
    Fut: 'a,
{
    fn call(&self, req: Request, state: &'a S) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        Box::pin((*self)(req, state))
    }
}

pub struct StatefulImmediate;
impl<'a, F, S> Filter<'a, StatefulImmediate, S> for F
where
    F: Fn(Request, &'a S) -> bool + 'static,
    S: 'static,
{
    fn call(&self, req: Request, state: &'a S) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        let fut = futures_util::future::ready((*self)(req, state));
        Box::pin(fut)
    }
}

pub struct StatelessImmediate;
impl<'a, F, S> Filter<'a, StatelessImmediate, S> for F
where
    F: Fn(Request) -> bool + 'static,
{
    fn call(&self, req: Request, _: &'a S) -> Pin<Box<dyn Future<Output = bool> + 'a>> {
        let fut = futures_util::future::ready((*self)(req));
        Box::pin(fut)
    }
}
