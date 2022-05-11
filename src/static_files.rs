use crate::endpoint::Endpoint;
use crate::state::State;
use crate::{Request, Response, Result};
use async_trait::async_trait;
use hyper::StatusCode;
use std::marker::PhantomData;
use std::path::{Component, PathBuf};
use tracing::{debug, warn};

pub(crate) struct StaticFiles<S>
where
    S: Send + Sync + 'static,
{
    root: PathBuf,
    prefix: PathBuf,
    _phantom: PhantomData<S>,
}

impl<S> StaticFiles<S>
where
    S: Send + Sync + 'static,
{
    pub(crate) fn new(root: impl Into<PathBuf>, prefix: impl Into<PathBuf>) -> Self {
        let mut prefix = prefix.into();
        // remove the final wildcard path segment
        prefix.pop();

        Self {
            root: root.into(),
            prefix,
            _phantom: PhantomData,
        }
    }
}

#[async_trait]
impl<S: State> Endpoint<S> for StaticFiles<S> {
    async fn call(&self, req: Request<S>) -> Result<Response> {
        let path = PathBuf::from(req.uri().path());

        let mut target = self.root.clone();

        for part in path.strip_prefix(&self.prefix)?.components() {
            match part {
                Component::Normal(component) => {
                    target.push(component);
                }
                Component::Prefix(_) => {
                    // Windows path prefixes - all are forbidden
                    return Ok(Response::status(StatusCode::FORBIDDEN));
                }
                Component::RootDir => {
                    // ignored for URLs
                }
                Component::CurDir => {
                    // skip
                }
                Component::ParentDir => {
                    target.pop();
                }
            }
        }

        debug!("path {:?} resolved to file {:?}", path, target);

        if !target.starts_with(&self.root) {
            warn!("path tried to navigate out of the static files root dir");
            return Ok(Response::status(StatusCode::FORBIDDEN));
        }

        if !target.is_file() {
            // small race condition - if the file is deleted between
            // here and where we open it then we're going to return a 500
            // instead of 404
            warn!("path isn't a file");
            return Ok(Response::status(StatusCode::NOT_FOUND));
        }

        Response::ok().path(target).await
    }
}
