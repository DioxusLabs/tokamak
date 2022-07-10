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

use crate::innerlude::*;

pub struct App<T: Send + Sync = ()> {
    pub state: Arc<T>,
}
impl Default for App<()> {
    fn default() -> Self {
        Self {
            state: Arc::new(()),
        }
    }
}

impl<T: Send + Sync + 'static> App<T> {
    pub fn new(state: T) -> Self {
        Self {
            state: Arc::new(state),
        }
    }

    pub fn at(&mut self, path: &'static str) -> Route<T> {
        Route { app: self, path }
    }

    pub fn get<'a, F>(&mut self, t: impl EndPoint<'a, F, T>) {
        todo!()
    }

    pub fn filter(&mut self, f: impl Fn(Request) -> bool) {
        todo!()
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
        // info!("server listening on {}", server.local_addr());
        server.await?;
        Ok(())
    }

    pub(crate) async fn serve_one_req(
        app: Arc<App<T>>,
        req: hyper::Request<Body>,
        addr: SocketAddr,
    ) -> Result<hyper::Response<Body>, TokamakError> {
        todo!()
        // let RouteTarget { ep, params } = app.routes.lookup(req.method(), req.uri().path());

        // let req = Request::new(req, params, addr);

        // let next = Next {
        //     ep,
        //     rest: &*app.filters,
        // };

        // next.next(app.state.clone(), req)
        //     .await
        //     .or_else(|err| err.into_response())
        //     .map(|resp| resp.into_inner())
    }
}
