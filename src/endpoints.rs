use crate::innerlude::*;
use crate::Result;
use futures_util::Future;
use std::pin::Pin;

pub trait Endpoint<'a, Sig, State, Ex0 = (), Ex1 = (), Ex2 = (), Ex3 = (), Ex4 = ()>:
    'static
{
    fn call(&self, req: Request, state: &'a State) -> EndPointReturn<'a>;
}

// get(|req| T)
pub struct StatelessImmediateInto;
impl<'a, F, S, O> Endpoint<'a, StatelessImmediateInto, S> for F
where
    F: Fn(Request) -> O + 'static,
    O: Into<Response>,
{
    fn call(&self, req: Request, _: &'a S) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req| Ok(T))
pub struct StatelessImmediate;
impl<'a, F, S, O> Endpoint<'a, StatelessImmediate, S> for F
where
    F: Fn(Request) -> Result<O> + 'static,
    O: Into<Response>,
{
    fn call(&self, req: Request, _: &'a S) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req| async move { Ok(T)})
pub struct StatelessFuture;
impl<'a, Fun, Fut, State, Output> Endpoint<'a, StatelessFuture, State> for Fun
where
    Fun: Fn(Request) -> Fut + 'static,
    Fut: Future<Output = Result<Output>> + 'a,
    Output: Into<Response>,
{
    fn call(&self, req: Request, _: &'a State) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req| async move { T })
pub struct StatelessFutureInto;
impl<'a, Fun, Fut, State, Output> Endpoint<'a, StatelessFutureInto, State> for Fun
where
    Fun: Fn(Request) -> Fut + 'static,
    Fut: Future<Output = Output> + 'a,
    Output: Into<Response>,
{
    fn call(&self, req: Request, _: &'a State) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req, state| Ok(T))
pub struct StatefulImmediate;
impl<'a, F, S, O> Endpoint<'a, StatefulImmediate, S> for F
where
    F: Fn(Request, &'a S) -> Result<O> + 'static,
    O: Into<Response>,
    S: 'static,
{
    fn call(&self, req: Request, state: &'a S) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req, state| async move { Ok(T)})
pub struct Stateful0;
impl<'a, F, Fut, S, O> Endpoint<'a, Stateful0, S> for F
where
    F: Fn(Request, &'a S) -> Fut + 'static,
    Fut: Future<Output = Result<O>> + 'a,
    S: 'static,
    Fut: 'a,
    O: Into<Response>,
{
    fn call(&self, req: Request, state: &'a S) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req, state, ex0| async move { Ok(T)})
pub struct Stateful1;
impl<'a, F, Fut, S, O, Ex0> Endpoint<'a, Stateful1, S, Ex0> for F
where
    F: Fn(Request, &'a S, Ex0) -> Fut + 'static,
    Fut: Future<Output = Result<O>> + 'a,
    O: Into<Response>,
    S: 'static,
    Ex0: FromRequest,
    Fut: 'a,
{
    fn call(&self, req: Request, state: &'a S) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req, state, ex0, ex1| async move { Ok(T)})
pub struct Stateful2;
impl<'a, F, Fut, S, O, Ex0, Ex1> Endpoint<'a, Stateful1, S, Ex0, Ex1> for F
where
    F: Fn(Request, &'a S, Ex0, Ex1) -> Fut + 'static,
    Fut: Future<Output = Result<O>> + 'a,
    S: 'static,
    Fut: 'a,
    O: Into<Response>,
    Ex0: FromRequest,
    Ex1: FromRequest,
{
    fn call(&self, req: Request, state: &'a S) -> EndPointReturn<'a> {
        todo!()
    }
}

// get(|req, state, ex0, ex1, ex2| async move { Ok(T)})
pub struct Stateful3;
impl<'a, F, Fut, S, O, Ex0, Ex1, Ex2> Endpoint<'a, Stateful1, S, Ex0, Ex1, Ex2> for F
where
    F: Fn(Request, &'a S, Ex0, Ex1, Ex2) -> Fut + 'static,
    Fut: Future<Output = Result<O>> + 'a,
    S: 'static,
    Fut: 'a,
    O: Into<Response>,
    Ex0: FromRequest,
    Ex1: FromRequest,
    Ex2: FromRequest,
{
    fn call(&self, req: Request, state: &'a S) -> EndPointReturn<'a> {
        todo!()
    }
}
