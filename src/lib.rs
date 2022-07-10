mod app;
mod endpoint;
mod error;
mod filter;
pub mod filters;
mod request;
mod response;
mod route;
mod router;

pub mod innerlude {
    pub use super::AppState;
    pub use crate::app::*;
    pub use crate::endpoint::*;
    pub use crate::error::*;
    pub use crate::filter::*;
    pub use crate::request::*;
    pub use crate::response::*;
    pub use crate::route::*;
    pub use crate::router::*;
}

pub use innerlude::{App, Request, Response, ResponseResult, ToResponse};

pub trait AppState: Send + Sync + 'static {}
impl<T> AppState for T where T: Send + Sync + 'static {}

pub fn new<S: AppState>(s: S) -> App<S> {
    App::new(s)
}

pub fn default() -> App<()> {
    App::new(())
}
