use crate::conn::{Request, Response};
use crate::server::Server;
use crate::traits::SharedState;
use futures::Future;

pub struct Route<'a, S> {
    pub(crate) server: &'a mut Server<S>,
    pub(crate) path: String,
}

pub type Handler<S, F> = fn(Request<S>) -> F;

impl<'a, S: SharedState> Route<'a, S> {
    pub fn get<I, F>(&mut self, handler: Handler<I, F>) -> &mut Self
    where
        I: Into<Response>,
        F: Future<Output = Result<I>>,
    {
        self
    }

    pub fn post<I, F>(&mut self, handler: Handler<I, F>) -> &mut Self
    where
        I: Into<Response>,
        F: Future<Output = Result<I>>,
    {
        self
    }

    pub fn put<I, F>(&mut self, handler: Handler<I, F>) -> &mut Self
    where
        I: Into<Response>,
        F: Future<Output = Result<I>>,
    {
        self
    }

    pub fn patch<I, F>(&mut self, handler: Handler<I, F>) -> &mut Self
    where
        I: Into<Response>,
        F: Future<Output = Result<I>>,
    {
        self
    }

    pub fn delete<I, F>(&mut self, handler: Handler<I, F>) -> &mut Self
    where
        I: Into<Response>,
        F: Future<Output = Result<I>>,
    {
        self
    }
}
