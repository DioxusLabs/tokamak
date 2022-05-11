use crate::{Result, StatusCode};
use hyper::{Body, Response, body::Buf};
use serde::de::DeserializeOwned;

/// The response returned from the test client
/// This currently has an AsRef implementation to get the inner hyper response
/// but more helper methods will be added over time to reduce the need for touching
/// the raw hyper types.
pub struct TestResponse {
    inner: hyper::Response<Body>
}

impl From<hyper::Response<Body>> for TestResponse {
    fn from(resp: Response<Body>) -> Self {
        Self {
            inner: resp
        }
    }
}

impl TestResponse {
    /// Get the status code
    pub fn status(&self) -> StatusCode {
        self.inner.status()
    }

    /// Get the request body as UTF-8 data in a String
    pub async fn body_string(&mut self) -> Result<String> {
        let bytes = hyper::body::to_bytes(self.inner.body_mut()).await?;
        Ok(String::from_utf8(bytes.to_vec())?)
    }

    /// Get the request body as bytes in a Vec
    pub async fn body_bytes(&mut self) -> Result<Vec<u8>> {
        let bytes = hyper::body::to_bytes(self.inner.body_mut()).await?;
        Ok(bytes.to_vec())
    }

    /// Get the request body by decoding JSON. Any type that implements Deserialize can be used.
    pub async fn body_json<T: DeserializeOwned>(&mut self) -> Result<T> {
        let buffer = hyper::body::aggregate(self.inner.body_mut()).await?;
        let data = serde_json::from_reader(buffer.reader())?;
        Ok(data)
    }
}

impl AsRef<hyper::Response<Body>> for TestResponse {
    fn as_ref(&self) -> &hyper::Response<Body> {
        &self.inner
    }
}
