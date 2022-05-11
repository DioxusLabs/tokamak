pub use headers;
pub use hyper::{Method, StatusCode};
pub use mime::Mime;
pub use tokio_tungstenite::tungstenite::Message;

mod app;
mod endpoint;
mod error;
pub mod filter;
mod request;
mod responder;
mod response;
mod router;
mod state;
mod static_files;
mod test_client;
pub mod ws;

pub use app::{App, Route};
pub use endpoint::Endpoint;
pub use error::Error;
pub use request::Request;
pub use responder::{Form, Json, Responder};
pub use response::Response;
pub use state::State;

pub type Result<T> = std::result::Result<T, Error>;

pub fn new<S: State>(s: S) -> App<S> {
    todo!()
}
