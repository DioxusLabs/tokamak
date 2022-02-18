use std::sync::Arc;

#[cfg(feature = "cookies")]
mod cookies;
mod endpoint;
mod fs;
mod middleware;
mod redirect;
mod request;
mod response;
mod response_builder;
mod route;
mod router;
mod server;

pub mod convert;
pub mod listener;
pub mod log;
pub mod prelude;
pub mod security;
pub mod sse;
pub mod utils;

#[cfg(feature = "sessions")]
pub mod sessions;

pub use endpoint::Endpoint;
pub use middleware::{Middleware, Next};
pub use redirect::Redirect;
pub use request::Request;
pub use response::Response;
pub use response_builder::ResponseBuilder;
pub use route::Route;
pub use server::Server;

pub use http_types::{self as http, Body, Error, Status, StatusCode};

pub struct Server {
    router: Arc<Router>,

    /// Holds the middleware stack.
    ///
    /// Note(Fishrock123): We do actually want this structure.
    /// The outer Arc allows us to clone in .respond() without cloning the array.
    /// The Vec allows us to add middleware at runtime.
    /// The inner Arc-s allow MiddlewareEndpoint-s to be cloned internally.
    /// We don't use a Mutex around the Vec here because adding a middleware during execution should be an error.
    #[allow(clippy::rc_buffer)]
    middleware: Arc<Vec<Arc<dyn Middleware>>>,
}

impl Server {
    /// Create a new Tide server with shared application scoped state.
    ///
    /// Application scoped state is useful for storing items
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::Request;
    ///
    /// /// The shared application state.
    /// #[derive(Clone)]
    /// struct State {
    ///     name: String,
    /// }
    ///
    /// // Define a new instance of the state.
    /// let state = State {
    ///     name: "Nori".to_string()
    /// };
    ///
    /// // Initialize the application with state.
    /// let mut app = tide::with_state(state);
    /// app.at("/").get(|req: Request| async move {
    ///     Ok(format!("Hello, {}!", &req.state().name))
    /// });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub fn with_state() -> Self {
        Self {
            router: Arc::new(Router::new()),
            middleware: Arc::new(vec![
                #[cfg(feature = "cookies")]
                Arc::new(cookies::CookiesMiddleware::new()),
                #[cfg(feature = "logger")]
                Arc::new(log::LogMiddleware::new()),
            ]),
            // state,
        }
    }

    /// Add a new route at the given `path`, relative to root.
    ///
    /// Routing means mapping an HTTP request to an endpoint. Here Tide applies
    /// a "table of contents" approach, which makes it easy to see the overall
    /// app structure. Endpoints are selected solely by the path and HTTP method
    /// of a request: the path determines the resource and the HTTP verb the
    /// respective endpoint of the selected resource. Example:
    ///
    /// ```rust,no_run
    /// # let mut app = tide::Server::new();
    /// app.at("/").get(|_| async { Ok("Hello, world!") });
    /// ```
    ///
    /// A path is comprised of zero or many segments, i.e. non-empty strings
    /// separated by '/'. There are two kinds of segments: concrete and
    /// wildcard. A concrete segment is used to exactly match the respective
    /// part of the path of the incoming request. A wildcard segment on the
    /// other hand extracts and parses the respective part of the path of the
    /// incoming request to pass it along to the endpoint as an argument. A
    /// wildcard segment is written as `:name`, which creates an endpoint
    /// parameter called `name`. It is not possible to define wildcard segments
    /// with different names for otherwise identical paths.
    ///
    /// Alternatively a wildcard definitions can start with a `*`, for example
    /// `*path`, which means that the wildcard will match to the end of given
    /// path, no matter how many segments are left, even nothing.
    ///
    /// The name of the parameter can be omitted to define a path that matches
    /// the required structure, but where the parameters are not required.
    /// `:` will match a segment, and `*` will match an entire path.
    ///
    /// Here are some examples omitting the HTTP verb based endpoint selection:
    ///
    /// ```rust,no_run
    /// # let mut app = tide::Server::new();
    /// app.at("/");
    /// app.at("/hello");
    /// app.at("add_two/:num");
    /// app.at("files/:user/*");
    /// app.at("static/*path");
    /// app.at("static/:context/:");
    /// ```
    ///
    /// There is no fallback route matching, i.e. either a resource is a full
    /// match or not, which means that the order of adding resources has no
    /// effect.
    pub fn at<'a>(&'a mut self, path: &str) -> Route<'a, State> {
        let router = Arc::get_mut(&mut self.router)
            .expect("Registering routes is not possible after the Server has started");
        Route::new(router, path.to_owned())
    }

    /// Add middleware to an application.
    ///
    /// Middleware provides customization of the request/response cycle, such as compression,
    /// logging, or header modification. Middleware is invoked when processing a request, and can
    /// either continue processing (possibly modifying the response) or immediately return a
    /// response. See the [`Middleware`] trait for details.
    ///
    /// Middleware can only be added at the "top level" of an application, and is processed in the
    /// order in which it is applied.
    pub fn with<M>(&mut self, middleware: M) -> &mut Self
    where
        M: Middleware,
    {
        log::trace!("Adding middleware {}", middleware.name());
        let m = Arc::get_mut(&mut self.middleware)
            .expect("Registering middleware is not possible after the Server has started");
        m.push(Arc::new(middleware));
        self
    }

    /// Asynchronously serve the app with the supplied listener.
    ///
    /// This is a shorthand for calling `Server::bind`, logging the `ListenInfo`
    /// instances from `Listener::info`, and then calling `Listener::accept`.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// let mut app = tide::new();
    /// app.at("/").get(|_| async { Ok("Hello, world!") });
    /// app.listen("127.0.0.1:8080").await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn listen<L: ToListener>(self, listener: L) -> io::Result<()> {
        let mut listener = listener.to_listener()?;
        listener.bind(self).await?;
        for info in listener.info().iter() {
            log::info!("Server listening on {}", info);
        }
        listener.accept().await?;
        Ok(())
    }

    /// Asynchronously bind the listener.
    ///
    /// Bind the listener. This starts the listening process by opening the
    /// necessary network ports, but not yet accepting incoming connections.
    /// `Listener::listen` should be called after this to start accepting
    /// connections.
    ///
    /// When calling `Listener::info` multiple `ListenInfo` instances may be
    /// returned. This is useful when using for example `ConcurrentListener`
    /// which enables a single server to listen on muliple ports.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use async_std::task::block_on;
    /// # fn main() -> Result<(), std::io::Error> { block_on(async {
    /// #
    /// use tide::prelude::*;
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|_| async { Ok("Hello, world!") });
    /// let mut listener = app.bind("127.0.0.1:8080").await?;
    /// for info in listener.info().iter() {
    ///     println!("Server listening on {}", info);
    /// }
    /// listener.accept().await?;
    /// #
    /// # Ok(()) }) }
    /// ```
    pub async fn bind<L: ToListener>(self, listener: L) -> io::Result<<L as ToListener>::Listener> {
        let mut listener = listener.to_listener()?;
        listener.bind(self).await?;
        Ok(listener)
    }

    /// Respond to a `Request` with a `Response`.
    ///
    /// This method is useful for testing endpoints directly,
    /// or for creating servers over custom transports.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # #[async_std::main]
    /// # async fn main() -> http_types::Result<()> {
    /// #
    /// use tide::http::{Url, Method, Request, Response};
    ///
    /// let mut app = tide::new();
    /// app.at("/").get(|_| async { Ok("hello world") });
    ///
    /// let req = Request::new(Method::Get, Url::parse("https://example.com")?);
    /// let res: Response = app.respond(req).await?;
    ///
    /// assert_eq!(res.status(), 200);
    /// #
    /// # Ok(()) }
    /// ```
    pub async fn respond<Req, Res>(&self, req: Req) -> http_types::Result<Res>
    where
        Req: Into<http_types::Request>,
        Res: From<http_types::Response>,
    {
        let req = req.into();
        let Self {
            router,
            state,
            middleware,
        } = self.clone();

        let method = req.method().to_owned();
        let Selection { endpoint, params } = router.route(req.url().path(), method);
        let route_params = vec![params];
        let req = Request::new(state, req, route_params);

        let next = Next {
            endpoint,
            next_middleware: &middleware,
        };

        let res = next.run(req).await;
        let res: http_types::Response = res.into();
        Ok(res.into())
    }

    /// Gets a reference to the server's state. This is useful for testing and nesting:
    ///
    /// # Example
    ///
    /// ```rust
    /// # #[derive(Clone)] struct SomeAppState;
    /// let mut app = tide::with_state(SomeAppState);
    /// let mut admin = tide::with_state(app.state().clone());
    /// admin.at("/").get(|_| async { Ok("nested app with cloned state") });
    /// app.at("/").nest(admin);
    /// ```
    pub fn state(&self) -> &State {
        &self.state
    }
}

struct Router {}

/// A specialized Result type for Tide.
pub type Result<T = Response> = std::result::Result<T, Error>;
