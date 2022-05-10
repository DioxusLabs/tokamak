use crate::endpoint::Endpoint;
use crate::state::SharedState;
use crate::{Request, Response, Result};
use async_trait::async_trait;
use futures_util::stream::{SplitSink, SplitStream};
use futures_util::{SinkExt, StreamExt, TryStreamExt};
use hyper::upgrade::Upgraded;
use hyper::StatusCode;
use std::future::Future;
use std::marker::PhantomData;
use std::sync::Arc;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use tracing::trace;

/// An endpoint for accepting a websocket connection.
/// Typically constructed by the `Route::ws` method.
#[derive(Debug)]
pub struct WsEndpoint<H, F>
where
    H: Send + Sync + 'static + Fn(WebSocketSender, WebSocketReceiver) -> F,
    F: Future<Output = Result<()>> + Send + 'static,
{
    handler: Arc<H>,
}

/// Create a websocket endpoint.
/// Typically called by the `Route::ws` method.
pub fn endpoint<H, F>(handler: H) -> WsEndpoint<H, F>
where
    H: Send + Sync + 'static + Fn(WebSocketSender, WebSocketReceiver) -> F,
    F: Future<Output = Result<()>> + Send + 'static,
{
    WsEndpoint {
        handler: Arc::new(handler),
    }
}

#[async_trait]
impl<H, F> Endpoint for WsEndpoint<H, F>
where
    H: Send + Sync + 'static + Fn(WebSocketSender, WebSocketReceiver) -> F,
    F: Future<Output = Result<()>> + Send + 'static,
{
    async fn call(&self, req: Request) -> Result<Response> {
        let handler = self.handler.clone();

        let res = upgrade_connection(req, handler).await;

        Ok(res)
    }
}

async fn upgrade_connection<H, F>(req: Request, handler: Arc<H>) -> Response
where
    H: Send + Sync + 'static + Fn(WebSocketSender, WebSocketReceiver) -> F,
    F: Future<Output = Result<()>> + Send + 'static,
{
    // TODO - check various headers

    if let Some(conn) = req.header::<headers::Connection>() {
        if !conn.contains(hyper::header::UPGRADE) {
            return Response::status(StatusCode::BAD_REQUEST);
        }
    } else {
        return Response::status(StatusCode::BAD_REQUEST);
    }

    if let Some(upgrade) = req.header::<headers::Upgrade>() {
        if upgrade != headers::Upgrade::websocket() {
            return Response::status(StatusCode::BAD_REQUEST);
        }
    } else {
        return Response::status(StatusCode::BAD_REQUEST);
    }

    let key = match req.header::<headers::SecWebsocketKey>() {
        Some(header) => header,
        None => return Response::status(StatusCode::BAD_REQUEST),
    };

    let res = Response::status(StatusCode::SWITCHING_PROTOCOLS)
        .header(headers::Upgrade::websocket())
        .header(headers::Connection::upgrade())
        .header(headers::SecWebsocketAccept::from(key));

    trace!("upgrading connection to websocket");

    tokio::spawn(async move {
        let upgraded = hyper::upgrade::on(req.into_inner())
            .await
            .expect("websocket upgrade failed - TODO report this error");

        let ws = WebSocketStream::from_raw_socket(
            upgraded,
            tokio_tungstenite::tungstenite::protocol::Role::Server,
            None,
        )
        .await;

        let (tx, rx) = ws.split();
        let _ = (handler)(
            WebSocketSender { inner: tx },
            WebSocketReceiver { inner: rx },
        )
        .await;
    });

    res
}

/// The sending half of the websocket connection
pub struct WebSocketSender {
    inner: SplitSink<WebSocketStream<Upgraded>, Message>,
}

impl WebSocketSender {
    /// Send a message over the websocket
    pub async fn send(&mut self, msg: Message) -> Result<()> {
        self.inner.send(msg).await?;
        Ok(())
    }
}

/// The receiving half of the websocket connection
pub struct WebSocketReceiver {
    inner: SplitStream<WebSocketStream<Upgraded>>,
}

impl WebSocketReceiver {
    /// Receive a message from the websocket
    pub async fn recv(&mut self) -> Result<Option<Message>> {
        let msg = self.inner.try_next().await?;
        Ok(msg)
    }
}
