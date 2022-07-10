use crate::response::Response;
use crate::Result;
use hyper::{Body, StatusCode};
use serde::Serialize;

/// This trait is implemented for all the common types you can return from an endpoint
///
/// It's also implemented for `Response` and `hyper::Response` for compatibility.
/// There is an implementation for `Result<R> where R: Responder` which allows fallible
/// functions to be used as endpoints
///
/// ```
/// use highnoon::{Request, Responder, Json, StatusCode};
///
/// fn example_1(_: Request<()>) -> impl Responder {
///     // return status code
///     StatusCode::NOT_FOUND
/// }
///
/// fn example_2(_: Request<()>) -> impl Responder {
///     // return strings (&str or String)
///     "Hello World"
/// }
///
/// fn example_3(_: Request<()>) -> impl Responder {
///     // return status code with data
///     (StatusCode::NOT_FOUND, "Not found!")
/// }
///
/// fn example_4(_: Request<()>) -> impl Responder {
///     // return JSON data - for any type implementing `serde::Serialize`
///     Json(vec![1, 2, 3])
/// }
///
/// fn example_5(_: Request<()>) -> highnoon::Result<impl Responder> {
///     // fallible functions too
///     // (also works the return type as `impl Responder` as long as Rust can infer
///     // the function returns `highnoon::Result`)
///     Ok((StatusCode::CONFLICT, "Already Exists"))
/// }
/// ```

pub trait Responder {
    fn into_response(self) -> Result<Response>;
}

impl Responder for StatusCode {
    fn into_response(self) -> Result<Response> {
        Ok(Response::status(self))
    }
}

impl Responder for String {
    fn into_response(self) -> Result<Response> {
        Ok(Response::ok().body(self))
    }
}

impl Responder for &str {
    fn into_response(self) -> Result<Response> {
        Ok(Response::ok().body(self.to_owned()))
    }
}

impl Responder for &[u8] {
    fn into_response(self) -> Result<Response> {
        Ok(Response::ok().body(self.to_vec()))
    }
}

impl Responder for Vec<u8> {
    fn into_response(self) -> Result<Response> {
        Ok(Response::ok().body(self))
    }
}

impl<R: Responder> Responder for (StatusCode, R) {
    fn into_response(self) -> Result<Response> {
        let mut resp = self.1.into_response()?;
        resp.set_status(self.0);
        Ok(resp)
    }
}

/// Returns `StatusCode::NotFound` for `None`, and the inner value for `Some`
impl<R: Responder> Responder for Option<R> {
    fn into_response(self) -> Result<Response> {
        match self {
            None => StatusCode::NOT_FOUND.into_response(),
            Some(r) => r.into_response(),
        }
    }
}

/// A Wrapper to return a JSON payload. This can be wrapped over any `serde::Serialize` type.
/// ```
/// use highnoon::{Request, Responder, Json};
/// fn returns_json(_: Request<()>) -> impl Responder {
///     Json(vec!["an", "array"])
/// }
/// ```
pub struct Json<T: Serialize>(pub T);

impl<T: Serialize> Responder for Json<T> {
    fn into_response(self) -> Result<Response> {
        Response::ok().json(self.0)
    }
}

/// A Wrapper to return Form data. This can be wrapped over any `serde::Serialize` type.
pub struct Form<T: Serialize>(pub T);

impl<T: Serialize> Responder for Form<T> {
    fn into_response(self) -> Result<Response> {
        Response::ok().form(self.0)
    }
}

/// Identity implementation
impl Responder for Response {
    fn into_response(self) -> Result<Response> {
        Ok(self)
    }
}

/// Compatibility with the inner hyper::Response
impl Responder for hyper::Response<Body> {
    fn into_response(self) -> Result<Response> {
        Ok(self.into())
    }
}

impl<R: Responder> Responder for Result<R> {
    fn into_response(self) -> Result<Response> {
        self.and_then(|r| r.into_response())
    }
}
