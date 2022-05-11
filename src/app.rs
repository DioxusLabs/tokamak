use crate::endpoint::Endpoint;
use crate::filter::{Filter, Next};
use crate::router::{RouteTarget, Router};
use crate::state::State;
use crate::static_files::StaticFiles;
use crate::test_client::TestClient;
use crate::ws::{WebSocketReceiver, WebSocketSender};
use crate::{Request, Responder, Response, Result};
use async_trait::async_trait;
use hyper::server::conn::{AddrIncoming, AddrStream};
use hyper::server::Builder;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method};
use std::convert::Infallible;
use std::future::Future;
use std::net::SocketAddr;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::net::ToSocketAddrs;
use tracing::info;

/// The main entry point to highnoon. An `App` can be launched as a server
/// or mounted into another `App`.
/// Each `App` has a chain of [`Filters`](Filter)
/// which are applied to each request.
pub struct App<S: State> {
    state: S,
    routes: Router<S>,
    filters: Vec<Box<dyn Filter<S> + Send + Sync + 'static>>,
}

/// Returned by [App::at] and attaches method handlers to a route.
pub struct Route<'a, 'p, S: State> {
    path: &'p str,
    app: &'a mut App<S>,
}

impl<'a, 'p, S: State> Route<'a, 'p, S> {
    /// Attach an endpoint for a specific HTTP method
    pub fn method(self, method: Method, ep: impl Endpoint<S> + Send + Sync + 'static) -> Self {
        self.app.routes.add(method, self.path, ep);
        self
    }

    /// Attach an endpoint for all HTTP methods. These will be checked only if no
    /// specific endpoint exists for the method.
    pub fn all(self, ep: impl Endpoint<S> + Send + Sync + 'static) -> Self {
        self.app.routes.add_all(self.path, ep);
        self
    }

    /// Attach an endpoint for GET requests
    pub fn get(self, ep: impl Endpoint<S> + Send + Sync + 'static) -> Self {
        self.method(Method::GET, ep)
    }

    /// Attach an endpoint for POST requests
    pub fn post(self, ep: impl Endpoint<S> + Send + Sync + 'static) -> Self {
        self.method(Method::POST, ep)
    }

    /// Attach an endpoint for PUT requests
    pub fn put(self, ep: impl Endpoint<S> + Send + Sync + 'static) -> Self {
        self.method(Method::PUT, ep)
    }

    /// Attach an endpoint for DELETE requests
    pub fn delete(self, ep: impl Endpoint<S> + Send + Sync + 'static) -> Self {
        self.method(Method::DELETE, ep)
    }

    /// Serve static files located in the path `root`. The path should end with a wildcard segment
    /// (ie. `/*`). The wildcard portion of the URL will be appended to `root` to form the full
    /// path. The file extension is used to guess a mime type. Files outside of `root` will return
    /// a FORBIDDEN error code; `..` and `.` path segments are allowed as long as they do not navigate
    /// outside of `root`.
    pub fn static_files(self, root: impl Into<PathBuf>) -> Self {
        let prefix = self.path.to_owned(); // TODO - borrow issue here
        self.method(Method::GET, StaticFiles::new(root, prefix))
    }

    /// Mount an app to handle all requests from this path.
    /// The path may contain parameters and these will be merged into
    /// the parameters from individual paths in the inner `App`.
    /// The App may have a different state type, but its `Context` must implement `From` to perform
    /// the conversion from the parent state's `Context` - *the inner `App`'s `new_context` won't
    /// be called*.
    pub fn mount(&mut self, app: App<S>) {
        let path = self.path.to_owned() + "/*-highnoon-path-rest-";
        let mounted = MountedApp { app: Arc::new(app) };
        self.app.at(&path).all(mounted);
    }

    /// Attach a websocket handler to this route
    pub fn ws<H, F>(self, handler: H)
    where
        H: Send + Sync + 'static + Fn(Arc<App<S>>, WebSocketSender, WebSocketReceiver) -> F,
        F: Future<Output = Result<()>> + Send + 'static,
    {
        self.method(Method::GET, crate::ws::endpoint(handler));
    }
}

impl<S: State> App<S> {
    /// Create a new `App` with the given state.
    /// State must be `Send + Sync + 'static` because it gets shared by all route handlers.
    /// If you need inner mutability use a `Mutex` or similar.
    pub fn new(state: S) -> Self {
        Self {
            state,
            routes: Router::new(),
            filters: vec![],
        }
    }

    /// Create a test client by consuming this App. The test client can be used to send fake
    /// requests to the App and receive responses back. This can be used in unit and
    /// integration tests.
    pub fn test(self) -> TestClient<S> {
        TestClient::new(self)
    }

    /// Get a reference to this App's state
    #[inline]
    pub fn state(&self) -> &S {
        &self.state
    }

    /// Append a filter to the chain. Filters are applied to all endpoints in this app, and are
    /// applied in the order they are registered.
    pub fn with<F>(&mut self, filter: F)
    where
        F: Filter<S> + Send + Sync + 'static,
    {
        self.filters.push(Box::new(filter));
    }

    /// Create a route at the given path. Returns a [Route] object on which you can
    /// attach handlers for each HTTP method
    pub fn at<'a, 'p>(&'a mut self, path: &'p str) -> Route<'a, 'p, S> {
        Route { path, app: self }
    }

    /// Start a server listening on the given address (See [ToSocketAddrs] from tokio)
    /// This method only returns if there is an error. (Graceful shutdown is TODO)
    pub async fn listen(self, host: impl ToSocketAddrs) -> anyhow::Result<()> {
        let mut addrs = tokio::net::lookup_host(host).await?;
        let addr = addrs
            .next()
            .ok_or_else(|| anyhow::Error::msg("host lookup returned no hosts"))?;

        let builder = hyper::Server::try_bind(&addr)?;
        self.internal_serve(builder).await
    }

    /// Start a server listening on the provided [std::net::TcpListener]
    /// This method only returns if there is an error. (Graceful shutdown is TODO)
    pub async fn listen_on(self, tcp: std::net::TcpListener) -> anyhow::Result<()> {
        let builder = hyper::Server::from_tcp(tcp)?;
        self.internal_serve(builder).await
    }

    async fn internal_serve(self, builder: Builder<AddrIncoming>) -> anyhow::Result<()> {
        let app = Arc::new(self);

        let make_svc = make_service_fn(|addr_stream: &AddrStream| {
            let app = app.clone();
            let addr = addr_stream.remote_addr();

            async move {
                Ok::<_, Infallible>(service_fn(move |req: hyper::Request<Body>| {
                    let app = app.clone();
                    async move {
                        App::serve_one_req(app, req, addr)
                            .await
                            .map_err(|err| err.into_std())
                    }
                }))
            }
        });

        let server = builder.serve(make_svc);
        info!("server listening on {}", server.local_addr());
        server.await?;
        Ok(())
    }

    pub(crate) async fn serve_one_req(
        app: Arc<App<S>>,
        req: hyper::Request<Body>,
        addr: SocketAddr,
    ) -> Result<hyper::Response<Body>> {
        let RouteTarget { ep, params } = app.routes.lookup(req.method(), req.uri().path());

        let req = Request::new(app.clone(), req, params, addr);

        let next = Next {
            ep,
            rest: &*app.filters,
        };

        next.next(req)
            .await
            .or_else(|err| err.into_response())
            .map(|resp| resp.into_inner())
    }
}

struct MountedApp<S: State> {
    app: Arc<App<S>>,
}

#[async_trait]
impl<S: State> Endpoint<S> for MountedApp<S> {
    async fn call(&self, req: Request<S>) -> Result<Response> {
        // deconstruct the request from the outer state
        let (inner, params, remote_addr) = req.into_parts();
        // get the part of the path still to be routed
        let path_rest = params
            .find("-highnoon-path-rest-")
            .expect("-highnoon-path-rest- is missing!");
        // lookup the target for the request in the nested app
        let RouteTarget {
            ep,
            params: params2,
        } = self.app.routes.lookup(inner.method(), path_rest);

        // construct a new request for the inner state type
        let mut req2 = Request::new(self.app.clone(), inner, params, remote_addr);

        // merge the inner params
        req2.merge_params(params2);

        // start the filter chain for the nested app
        let next = Next {
            ep,
            rest: &*self.app.filters,
        };

        next.next(req2).await
    }
}
