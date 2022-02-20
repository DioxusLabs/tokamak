use crate::error;
use futures::Future;

pub trait SharedState: 'static + Send + Sync {}
impl<T: 'static + Send + Sync> SharedState for T {}

// pub type Handler<S: SharedState, F: Future<Output = Result<Response, ()>>> = fn(Request<S>) -> F;

// // #[cfg(test)]
// mod tests {
//     use super::*;
//     fn is_handler<S: SharedState, F: Future<Output = Result<Response, ()>>>(f: Handler<S, F>) {
//         //
//     }

//     fn valid_handlers() {
//         async fn receives_thing() -> Result<Response, ()> {
//             Ok(Response::new())
//         }

//         is_handler(receives_thing);
//     }
// }
