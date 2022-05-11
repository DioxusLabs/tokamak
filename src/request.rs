use crate::state::State;
use crate::{App, Error, Result};
use cookie::{Cookie, CookieJar};
use headers::{Header, HeaderMapExt};
use hyper::header::HeaderValue;
use hyper::{body::Buf, Body, HeaderMap, StatusCode};
use route_recognizer::Params;
use serde::de::DeserializeOwned;
use std::io::Read;
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::error;

/// An incoming request
pub struct Request<S: State> {
    pub(crate) app: Arc<App<S>>,
    params: Params,
    inner: hyper::Request<Body>,
    remote_addr: SocketAddr,
}

impl<S: State> Request<S> {
    pub(crate) fn new(
        app: Arc<App<S>>,
        inner: hyper::Request<Body>,
        params: Params,
        remote_addr: SocketAddr,
    ) -> Self {
        Self {
            app,
            inner,
            params,
            remote_addr,
        }
    }

    pub(crate) fn into_parts(self) -> (hyper::Request<Body>, Params, SocketAddr) {
        (self.inner, self.params, self.remote_addr)
    }

    pub(crate) fn merge_params(&mut self, params: Params) {
        for (k, v) in params.iter() {
            self.params.insert(k.to_owned(), v.to_owned());
        }
    }

    /// Get a reference to the App's state
    #[inline]
    pub fn state(&self) -> &S {
        self.app.state()
    }

    /// Get the HTTP method being used by this request
    pub fn method(&self) -> &hyper::Method {
        self.inner.method()
    }

    /// Get the URI that was used for this request
    pub fn uri(&self) -> &hyper::Uri {
        self.inner.uri()
    }

    /// Parse the URI query string into an instance of `T` that derives `Deserialize`.
    ///
    /// (To get the raw query string access it via `req.uri().query()`).
    /// If there is no query string, deserialize an empty string.
    pub fn query<T: DeserializeOwned>(&self) -> Result<T> {
        // if there is no query string we can default to empty string
        // serde_urlencode will work if T has all optional fields
        let q = self.inner.uri().query().unwrap_or("");
        let t = serde_urlencoded::from_str::<T>(q)
            .map_err(|err| Error::bad_request(format!("invalid query parameter: {}", err)))?;
        Ok(t)
    }

    /// Get a typed header from the request
    /// (See also `headers`)
    pub fn header<T: Header>(&self) -> Option<T> {
        self.inner.headers().typed_get()
    }

    /// Get all headers as a `HeaderMap`
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.inner.headers()
    }

    /// Get the request's cookies
    pub fn cookies(&self) -> Result<CookieJar> {
        let mut cookies = CookieJar::new();

        for val in self.inner.headers().get_all(headers::Cookie::name()) {
            let c = Cookie::parse(val.to_str()?)?;
            cookies.add(c.into_owned());
        }

        Ok(cookies)
    }

    /// Get a route parameter (eg. `:key` or `*key` segments in the URI path)
    ///
    /// If the parameter is not present, logs an error and returns a `400 Bad Request` to the client
    pub fn param(&self, param: &str) -> Result<&str> {
        self.params.find(param).ok_or_else(|| {
            error!("parameter {} not found", param);
            Error::http(StatusCode::BAD_REQUEST)
        })
    }

    /// Get all route parameters
    pub fn params(&self) -> &Params {
        &self.params
    }

    /// Get the request body as a `hyper::Body`
    pub async fn body_mut(&mut self) -> Result<&mut Body> {
        Ok(self.inner.body_mut())
    }

    pub(crate) fn into_inner(self) -> hyper::Request<Body> {
        self.inner
    }

    /// Get a reader to read the request body
    ///
    /// (This does buffer the whole body into memory, but not necessarily contiguous memory).
    /// If you need to protect against malicious clients you should access the body via `body_mut`
    pub async fn reader(&mut self) -> Result<impl Read + '_> {
        let buffer = hyper::body::aggregate(self.inner.body_mut()).await?;
        Ok(buffer.reader())
    }

    /// Get the request body as raw bytes in a `Vec<u8>`
    pub async fn body_bytes(&mut self) -> Result<Vec<u8>> {
        let bytes = hyper::body::to_bytes(self.inner.body_mut()).await?;
        Ok(bytes.to_vec())
    }

    /// Get the request body as UTF-8 data in String
    pub async fn body_string(&mut self) -> Result<String> {
        let bytes = hyper::body::to_bytes(self.inner.body_mut()).await?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    /// Get the request body as JSON and deserialize into `T`.
    ///
    /// If deserialization fails, log an error and return `400 Bad Request`.
    /// (If this logic is not appropriate, consider using `reader` and using `serde_json` directly)
    pub async fn body_json<T: DeserializeOwned>(&mut self) -> Result<T> {
        let reader = self.reader().await?;
        serde_json::from_reader(reader).map_err(|err| {
            let msg = format!("error parsing request body as json: {}", err);
            error!("{}", msg);
            Error::http((StatusCode::BAD_REQUEST, msg))
        })
    }

    /// Get the address of the remote peer.
    ///
    /// This method uses the network level address only and hence may be incorrect if you are
    /// behind a proxy. (This does *not* check for any `Forwarded` headers etc...)
    pub fn remote_addr(&self) -> &SocketAddr {
        &self.remote_addr
    }
}
