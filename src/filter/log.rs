use crate::filter::{Filter, Next};
use crate::{Error, Request, Response, Result};
use async_trait::async_trait;

use tracing::{debug, error, info, warn};

/// A logging filter. Logs all requests at debug level, and logs responses at error, warn or info
/// level depending on the status code (5xx, 4xx, and everything else)
pub struct Log;

fn log_response(method: String, uri: String, resp: &Response) {
    let status = resp.as_ref().status();
    if status.is_server_error() {
        error!(%method, %uri, %status, "response");
    } else if status.is_client_error() {
        warn!(%method, %uri, %status, "response");
    } else {
        info!(%method, %uri, %status, "response");
    }
}

#[async_trait]
impl Filter for Log {
    async fn apply(&self, req: Request, next: Next<'_, '_>) -> Result<Response> {
        let method = req.method().to_string();
        let uri = req.uri().to_string();

        debug!(%method, %uri, "request");

        let result = next.next(req).await;

        match &result {
            Ok(resp) => log_response(method, uri, resp),
            Err(Error::Http(resp)) => log_response(method, uri, resp),
            Err(Error::Internal(err)) => {
                error!(%method,
                    %uri,
                    error=%err,
                    backtrace=?err,
                   "internal server error"
                );
            }
        }

        result
    }
}
