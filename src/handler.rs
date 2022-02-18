use async_trait::async_trait;

use crate::http::{Request, Response};
use crate::routing::FlowCtrl;
use crate::Depot;

/// Handler trait for handle http request.
#[async_trait]
pub trait Handler: Send + Sync + 'static {
    /// Handle http request.
    #[must_use = "handle future must be used"]
    async fn handle(
        &self,
        req: &mut Request,
        depot: &mut Depot,
        res: &mut Response,
        ctrl: &mut FlowCtrl,
    );
}
