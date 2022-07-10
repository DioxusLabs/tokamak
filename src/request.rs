use std::{io::Read, net::SocketAddr};

use headers::{Cookie, Header, HeaderMapExt};
use hyper::Body;
use route_recognizer::Params;
use serde::de::DeserializeOwned;

use crate::{error::TokamakError, innerlude::TokamakResult, ResponseResult};

pub struct Request {
    params: Params,
    inner: hyper::Request<Body>,
    remote_addr: SocketAddr,
}

impl Request {
    pub fn cookie(&self, name: &str) -> TokamakResult<Cookie> {
        todo!()
    }

    /// Get a typed header from the request
    /// (See also `headers`)
    pub fn header<T: Header>(&self) -> TokamakResult<T> {
        let headers = self.inner.headers().typed_get::<T>();
        todo!()
    }

    pub fn check_header<T: Header>(
        &self,
        f: impl FnOnce(&T) -> bool,
    ) -> core::result::Result<T, TokamakError> {
        todo!()
        // self.inner.headers().typed_get()
    }

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
    pub fn query<T: DeserializeOwned>(&self) -> Result<T, TokamakError> {
        // if there is no query string we can default to empty string
        // serde_urlencode will work if T has all optional fields
        let q = self.inner.uri().query().unwrap_or("");
        let t = serde_urlencoded::from_str::<T>(q).map_err(|err| {
            TokamakError::bad_request(format!("invalid query parameter: {}", err))
        })?;
        Ok(t)
    }

    /// Get a route parameter (eg. `:key` or `*key` segments in the URI path)
    ///
    /// If the parameter is not present, logs an error and returns a `400 Bad Request` to the client
    pub fn param(&self, param: &str) -> TokamakResult<&str> {
        todo!()
        // self.params.find(param).ok_or_else(|| {
        //     error!("parameter {} not found", param);
        //     Error::http(StatusCode::BAD_REQUEST)
        // })
    }

    /// Get all route parameters
    pub fn params(&self) -> &Params {
        &self.params
    }

    // /// Get a reader to read the request body
    // ///
    // /// (This does buffer the whole body into memory, but not necessarily contiguous memory).
    // /// If you need to protect against malicious clients you should access the body via `body_mut`
    // pub async fn reader(&mut self) -> TokamakResult<impl Read + '_> {
    //     todo!()
    //     // let buffer = hyper::body::aggregate(self.inner.body_mut()).await?;
    //     // Ok(buffer.reader())
    // }

    /// Get the request body as raw bytes in a `Vec<u8>`
    pub async fn body_bytes(&mut self) -> TokamakResult<Vec<u8>> {
        todo!()
        // let bytes = hyper::body::to_bytes(self.inner.body_mut()).await?;
        // Ok(bytes.to_vec())
    }

    /// Get the request body as UTF-8 data in String
    pub async fn body_string(&mut self) -> TokamakResult<String> {
        todo!()
        // let bytes = hyper::body::to_bytes(self.inner.body_mut()).await?;
        // Ok(String::from_utf8(bytes.to_vec())?)
    }

    /// Get the request body as JSON and deserialize into `T`.
    ///
    /// If deserialization fails, log an error and return `400 Bad Request`.
    /// (If this logic is not appropriate, consider using `reader` and using `serde_json` directly)
    pub async fn body_json<T: DeserializeOwned>(&mut self) -> TokamakResult<T> {
        todo!()
        // let reader = self.reader().await?;
        // serde_json::from_reader(reader).map_err(|err| {
        //     let msg = format!("error parsing request body as json: {}", err);
        //     error!("{}", msg);
        //     Error::http((StatusCode::BAD_REQUEST, msg))
        // })
    }
}

/// Some default filters
impl Request {
    pub fn content_length_max(&self, max: u64) -> Result<u64, TokamakError> {
        todo!()
    }

    pub fn header_exact(&self, name: &str, value: &str) -> Result<(), TokamakError> {
        todo!()
    }
}
