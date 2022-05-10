use crate::{Responder, Response, Result};
use hyper::StatusCode;
use std::error::Error as StdError;
use std::fmt::Formatter;

/// Error type expected to be returned by endpoints.
///
/// It can represent an HTTP level error which is useful for helper functions
/// that wish to cause an early return from a handler (using the question mark operator).
/// It can also represent any other kind of error (using the `anyhow::Error` type). These
/// errors are logged (if you enable the logging filter) and converted to a 500 Internal Server Error
/// with no other details.
///
/// HTTP level error should be created with the `http` methods (which accepts a `Responder` rather than
/// just `Response`) and Internal errors should be created with the `From`/`Into` implementation.
pub enum Error {
    /// An error that should get returned to the client
    Http(Response),
    /// Internal errors, reported as 500 Internal Server Error and logged locally
    Internal(anyhow::Error),
}

impl Error {
    /// Convert this error into a boxed std::error::Error
    pub(crate) fn into_std(self) -> Box<dyn StdError + Send + Sync + 'static> {
        match self {
            Error::Http(_) => panic!("http error??!"),
            Error::Internal(err) => err.into(),
        }
    }

    /// Create an Error from a `Responder` - the `Responder` will be converted to a response
    /// and returned to the HTTP Client exactly the same way as an `Result::Ok` would be.
    /// This is useful in conjunction with the `?` operator for early returns.
    pub fn http(resp: impl Responder) -> Self {
        match resp.into_response() {
            Ok(r) => Self::Http(r),
            Err(e) => e,
        }
    }

    /// Create a 400 Bad Request Error from a `Responder` - this method is similar to [Error::http]
    /// but it also sets the status code
    pub fn bad_request(resp: impl Responder) -> Self {
        Self::http((StatusCode::BAD_REQUEST, resp))
    }
}

impl Responder for Error {
    fn into_response(self) -> Result<Response> {
        match self {
            Error::Http(resp) => Ok(resp),
            Error::Internal(_err) => {
                //log::error!("internal server error: {}", err);
                Ok(Response::status(StatusCode::INTERNAL_SERVER_ERROR))
            }
        }
    }
}

impl<E> From<E> for Error
where
    //E: std::error::Error + Send + Sync + 'static,
    E: Into<anyhow::Error>,
{
    fn from(e: E) -> Self {
        //Error::Internal(anyhow::Error::new(e))
        Error::Internal(e.into())
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(err) => f
                .debug_struct("Error::Internal")
                .field("inner", err)
                .finish(),
            Error::Http(resp) => f.debug_struct("Error::Http").field("inner", resp).finish(),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Internal(err) => write!(f, "Internal Error: {:?}", err),
            Error::Http(resp) => write!(f, "{:?}", resp),
        }
    }
}
