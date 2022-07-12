use std::{any::Any, pin::Pin};

use crate::innerlude::*;
use futures_util::Future;

pub struct TrueEndpoint<S: Any> {
    inner: Box<
        dyn for<'a> Fn(Request<S>) -> Pin<Box<dyn Future<Output = ResponseResult>>> + Send + Sync,
    >,
}

impl<State: 'static> TrueEndpoint<State> {
    pub fn new<'a, Sig>(inner: impl Endpoint<'a, Sig, State>) -> Self {
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

pub type EndPointReturn<'a> = GenericReturn<'a, ResponseResult>;

pub enum GenericReturn<'a, T> {
    // Why box and pin a future if we don't need to?
    // Calculates the result immediately without calling into Rust's async machinery
    Immediate(T),

    // Calls into Rust's async machinery to calculate the result
    Future(Pin<Box<dyn Future<Output = T> + 'a>>),
}
