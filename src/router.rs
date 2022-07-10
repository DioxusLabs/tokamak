use hyper::{Method, StatusCode};
use route_recognizer::Params;
use std::collections::HashMap;
use std::sync::Arc;

use routefinder::{Captures, Router as MethodRouter};

use crate::{endpoint::TrueEndpoint, Request, Response};

/// The routing table used by `Server`
///
/// Internally, we have a separate state machine per http method; indexing
/// by the method first allows the table itself to be more efficient.
pub struct Router<State: Send + Sync + 'static> {
    method_map: HashMap<Method, MethodRouter<TrueEndpoint<State>>>,
    all_method_router: MethodRouter<TrueEndpoint<State>>,
}

/// The result of routing a URL
pub(crate) struct Selection<'a, State> {
    pub(crate) endpoint: &'a TrueEndpoint<State>,
    pub(crate) params: Captures<'static, 'static>,
}

impl<State: Send + Sync + 'static> Router<State> {
    pub(crate) fn new() -> Self {
        Router {
            method_map: HashMap::default(),
            all_method_router: MethodRouter::new(),
        }
    }

    pub(crate) fn add(&mut self, path: &str, method: Method, ep: TrueEndpoint<State>) {
        self.method_map
            .entry(method)
            .or_insert_with(MethodRouter::new)
            .add(path, ep)
            .unwrap()
    }

    pub(crate) fn add_all(&mut self, path: &str, ep: TrueEndpoint<State>) {
        self.all_method_router.add(path, ep).unwrap()
    }

    pub(crate) fn route(&self, path: &str, method: Method) -> Selection<'_, State> {
        if let Some(m) = self
            .method_map
            .get(&method)
            .and_then(|r| r.best_match(path))
        {
            Selection {
                endpoint: m.handler(),
                params: m.captures().into_owned(),
            }
        } else if let Some(m) = self.all_method_router.best_match(path) {
            Selection {
                endpoint: m.handler(),
                params: m.captures().into_owned(),
            }
        } else if method == Method::HEAD {
            // If it is a HTTP HEAD request then check if there is a callback in the endpoints map
            // if not then fallback to the behavior of HTTP GET else proceed as usual

            self.route(path, Method::GET)
        } else if self
            .method_map
            .iter()
            .filter(|(k, _)| **k != method)
            .any(|(_, r)| r.best_match(path).is_some())
        {
            todo!()
            // If this `path` can be handled by a callback registered with a different HTTP method
            // should return 405 Method Not Allowed
            // Selection {
            //     endpoint: &method_not_allowed,
            //     params: Captures::default(),
            // }
        } else {
            todo!("asd")
            // Selection {
            //     endpoint: &not_found_endpoint,
            //     params: Captures::default(),
            // }
        }
    }
}

async fn not_found_endpoint<State: Clone + Send + Sync + 'static>(
    _req: Request,
    state: &State,
) -> crate::ResponseResult {
    Ok(Response::new(StatusCode::NOT_FOUND))
}

async fn method_not_allowed<State: Clone + Send + Sync + 'static>(
    _req: Request,
    state: State,
) -> crate::ResponseResult {
    Ok(Response::new(StatusCode::METHOD_NOT_ALLOWED))
}
