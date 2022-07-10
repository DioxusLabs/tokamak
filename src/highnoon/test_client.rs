use crate::test_client::test_request::TestRequest;
use crate::{App, Method, State};
use hyper::{http, Uri};
use std::sync::Arc;

mod test_request;
mod test_response;

/// A client that can send fake requests to an App and receive the responses back for unit
/// and integration testing. Obtain one by calling [App::test]
pub struct TestClient<S: State> {
    app: Arc<App<S>>,
}

impl<S: State> TestClient<S> {
    pub(crate) fn new(app: App<S>) -> Self {
        Self { app: Arc::new(app) }
    }

    /// Prepare a GET request. Returns a TestRequest which is used to add headers and the body
    /// before being sent.
    pub fn get<U>(&self, uri: U) -> TestRequest<S>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<hyper::http::Error>,
    {
        self.method(Method::GET, uri)
    }

    /// Prepare a POST request. Returns a TestRequest which is used to add headers and the body
    /// before being sent.
    pub fn post<U>(&self, uri: U) -> TestRequest<S>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<hyper::http::Error>,
    {
        self.method(Method::POST, uri)
    }

    /// Prepare a PUT request. Returns a TestRequest which is used to add headers and the body
    /// before being sent.
    pub fn put<U>(&self, uri: U) -> TestRequest<S>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<hyper::http::Error>,
    {
        self.method(Method::PUT, uri)
    }

    /// Prepare a DELETE request. Returns a TestRequest which is used to add headers and the body
    /// before being sent.
    pub fn delete<U>(&self, uri: U) -> TestRequest<S>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<hyper::http::Error>,
    {
        self.method(Method::DELETE, uri)
    }

    /// Prepare request with the given HTTP method. Returns a TestRequest which is used to add headers
    /// and the body before being sent.
    pub fn method<U>(&self, method: Method, uri: U) -> TestRequest<S>
    where
        Uri: TryFrom<U>,
        <Uri as TryFrom<U>>::Error: Into<hyper::http::Error>,
    {
        TestRequest::new(
            self.app.clone(),
            http::request::Builder::new().method(method).uri(uri),
        )
    }
}
