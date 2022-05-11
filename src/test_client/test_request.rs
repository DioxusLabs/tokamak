use std::sync::Arc;
use headers::{Header, HeaderMapExt};
use crate::Result;
use hyper::{Body, HeaderMap, http};
use hyper::header::{HeaderName, HeaderValue};
use serde::Serialize;
use crate::{App, State};
//use crate::test_client::into_body::IntoBody;
use crate::test_client::test_response::TestResponse;

enum PartialReq {
    Builder(http::request::Builder),
    Request(hyper::Request<Body>),
}

/// A fake request used for testing an App. Obtain one by calling the relevant methods on
/// the [TestClient] (eg. [TestClient::get], [TestClient::post]...)
/// After optionally adding headers and a body you can send the request to receive the response
/// from the App.
pub struct TestRequest<S: State> {
    app: Arc<App<S>>,
    req: PartialReq,
}

impl<S: State> TestRequest<S> {
    pub(crate) fn new(app: Arc<App<S>>, builder: http::request::Builder) -> Self {
        Self {
            app,
            req: PartialReq::Builder(builder)
        }
    }

    fn headers_mut(&mut self) -> &mut HeaderMap {
        match &mut self.req {
            PartialReq::Builder(b) => {
                b.headers_mut().expect("error getting headers")
            }
            PartialReq::Request(req) => {
                req.headers_mut()
            }
        }
    }

    /// Set a header (from the `headers` crate)
    pub fn header<H: Header>(mut self, h: H) -> Self {
        self.headers_mut().typed_insert(h);
        self
    }

    /// Set a raw header (from the `http` crate)
    pub fn raw_header<N, K>(mut self, name: N, key: K) -> Result<Self>
        where
            N: TryInto<HeaderName>,
            K: TryInto<HeaderValue>,
            <N as TryInto<HeaderName>>::Error: Into<anyhow::Error>,
            <K as TryInto<HeaderValue>>::Error: Into<anyhow::Error>,
    {
        self.headers_mut().insert(name.try_into()?, key.try_into()?);
        Ok(self)
    }

    /// Add a body to this request.
    pub fn body(mut self, body: impl Into<Body>) -> Result<Self> {
        self.req = match self.req {
            PartialReq::Builder(b) => PartialReq::Request(b.body(body.into())?),
            PartialReq::Request(_req) => {
                panic!("body already set!")
            }
        };
        Ok(self)
    }

    /// Add a JSON encoded body to this request, and set the `Content-Type` header
    /// to `application/json`
    pub fn json(self, data: impl Serialize) -> Result<Self> {
        let body = serde_json::to_string(&data)?;
        self.body(body)
    }

    /// Send the request to the App and receive the response.
    pub async fn send(self) -> Result<TestResponse> {
        let req = match self.req {
            PartialReq::Builder(b) => b.body(Body::empty())?,
            PartialReq::Request(r) => r,
        };

        let addr = "127.0.0.1:8080".parse().expect("socket addr is invalid?");
        let resp = App::serve_one_req(self.app, req, addr).await?;
        Ok(TestResponse::from(resp))
    }
}
