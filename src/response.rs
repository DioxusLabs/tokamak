/// A wrapper over `hyper::Response` with better ergonomics
///
/// ```
/// use highnoon::{Request, Responder, Response};
/// fn example(_: Request<()>) -> impl Responder {
///     Response::ok().json(vec![1, 2, 3])
/// }
/// ```
use crate::Result;
use headers::{Header, HeaderMapExt};
use hyper::header::{HeaderName, HeaderValue};
use hyper::{Body, StatusCode};
use serde::Serialize;
use std::convert::TryInto;
use std::path::Path;
use tokio::io::AsyncRead;
use tokio_util::io::ReaderStream;
use tracing::debug;

/// A response to be returned to the client.
/// You do not always need to use this struct directly as endpoints can
/// return anything implementing `Responder`. However this is the most flexible
/// way to construct a reply, and it implements `Responder` (the "identity" implementation).
#[derive(Debug)]
pub struct Response {
    inner: hyper::Response<Body>,
}

impl Response {
    /// Create an empty response with status code OK (200)
    pub fn ok() -> Self {
        Self {
            inner: hyper::Response::builder()
                .status(StatusCode::OK)
                .body(Body::empty())
                .expect("ok status with empty body should never fail"),
        }
    }

    /// Create an empty response with the given status code
    pub fn status(s: StatusCode) -> Self {
        Self {
            inner: hyper::Response::builder()
                .status(s)
                .body(Body::empty())
                .expect("status with empty body should never fail"),
        }
    }

    /// Set the status code of a response
    pub fn set_status(&mut self, s: StatusCode) {
        *self.inner.status_mut() = s;
    }

    /// Get the status code of a response
    pub fn get_status(&self) -> StatusCode {
        self.inner.status()
    }

    /// Set the body of the response
    pub fn body(mut self, body: impl Into<Body>) -> Self {
        *self.inner.body_mut() = body.into();
        self
    }

    /// Set the body to an AsyncRead object
    pub fn reader(mut self, r: impl AsyncRead + Send + 'static) -> Self {
        let body = Body::wrap_stream(ReaderStream::new(r));
        *self.inner.body_mut() = body;
        self
    }

    /// Set the body to the content of a file given by a Path
    /// Also sets a content type by guessing the mime type from the path name
    pub async fn path(self, path: impl AsRef<Path>) -> Result<Self> {
        let target = path.as_ref();

        let reader = tokio::fs::File::open(&target).await?;

        let mime = mime_guess::from_path(&target).first_or_text_plain();
        debug!("guessed mime: {}", mime);

        Ok(self.header(headers::ContentType::from(mime)).reader(reader))
    }

    /// Set the body of the response to a JSON payload
    pub fn json(mut self, body: impl Serialize) -> Result<Self> {
        let data = serde_json::to_vec(&body)?;
        self.set_header(headers::ContentType::json());
        *self.inner.body_mut() = Body::from(data);
        Ok(self)
    }

    /// Set the body of the response to form data
    pub fn form(mut self, body: impl Serialize) -> Result<Self> {
        let form = serde_urlencoded::to_string(body)?;
        self.set_header(headers::ContentType::form_url_encoded());
        *self.inner.body_mut() = Body::from(form);
        Ok(self)
    }

    /// Set a header (from the `headers` crate)
    pub fn header<H: Header>(mut self, h: H) -> Self {
        self.set_header(h);
        self
    }

    /// Set a header (without consuming self - useful outside of method chains)
    pub fn set_header<H: Header>(&mut self, h: H) {
        self.inner.headers_mut().typed_insert(h);
    }

    /// Set a raw header (from the `http` crate)
    pub fn raw_header<N, K>(mut self, name: N, key: K) -> Result<Self>
    where
        N: TryInto<HeaderName>,
        K: TryInto<HeaderValue>,
        <N as TryInto<HeaderName>>::Error: Into<anyhow::Error>,
        <K as TryInto<HeaderValue>>::Error: Into<anyhow::Error>,
    {
        self.set_raw_header(name, key)?;
        Ok(self)
    }

    /// Set a raw header (without consuming self)
    pub fn set_raw_header<N, K>(&mut self, name: N, key: K) -> Result<()>
    where
        N: TryInto<HeaderName>,
        K: TryInto<HeaderValue>,
        <N as TryInto<HeaderName>>::Error: Into<anyhow::Error>,
        <K as TryInto<HeaderValue>>::Error: Into<anyhow::Error>,
    {
        self.inner
            .headers_mut()
            .insert(name.try_into()?, key.try_into()?);
        Ok(())
    }

    /// Consume this response and return the inner `hyper::Response`
    pub fn into_inner(self) -> hyper::Response<hyper::Body> {
        self.inner
    }
}

/// Create a `Response` from a `hyper::Response<hyper::Body>`
impl From<hyper::Response<Body>> for Response {
    fn from(hyper_response: hyper::Response<Body>) -> Self {
        Self {
            inner: hyper_response,
        }
    }
}

/// Get a reference to the inner `hyper::Response`
impl AsRef<hyper::Response<Body>> for Response {
    fn as_ref(&self) -> &hyper::Response<Body> {
        &self.inner
    }
}
