pub mod addr;
pub mod error;
pub mod listener;

pub mod depot;
pub mod route;
pub mod server;
pub mod traits;
pub mod writer;
pub mod conn {
    pub mod form;
    pub mod http_error;
    pub mod mime_util;
    pub mod range;
    pub mod read_error;
    pub mod request;
    pub mod response;

    pub use cookie;
    pub use headers;
    pub use http::method::Method;
    pub use http::{header, method, uri, version, HeaderMap, HeaderValue, StatusCode};
    pub use http_error::HttpError;
    pub use hyper::body::HttpBody;
    pub use mime::Mime;
    pub use range::HttpRange;
    pub use read_error::ReadError;
    pub use request::Request;
    pub use response::Response;
}

pub use conn::{Request, Response};
pub use depot::Depot;

// pub mod response;
// pub mod response_builder;
pub mod prelude {
    // pub use crate::response::Response;
    // pub use crate::response_builder::ResponseBuilder;

    // pub use crate::response_builder::ResponseBuilder;
}

use server::{Server, ServerCfg};

/// Create a new Tokamak Server instance
///
///
///
pub fn new<F: 'static + Send + Sync>(f: F) -> server::Server {
    server::Server::new((), 8)
}

pub fn with_state<F: traits::SharedState>(f: F) -> Server<F> {
    server::Server::new(f, 8)
}

pub fn with_state_cfg<F: traits::SharedState>(f: F, cfg: impl FnOnce() -> ServerCfg) -> Server<F> {
    server::Server::new(f, 8)
}
