use crate::endpoint::Endpoint;
use crate::state::SharedState;
use crate::{Request, Responder};
use hyper::{Method, StatusCode};
use route_recognizer::Params;
use std::collections::HashMap;

type DynEndpoint = dyn Endpoint + Send + Sync + 'static;

type Recogniser = route_recognizer::Router<Box<DynEndpoint>>;

pub(crate) struct Router {
    methods: HashMap<Method, Recogniser>,
    all: Recogniser,
}

pub(crate) struct RouteTarget<'a> {
    pub(crate) ep: &'a DynEndpoint,
    pub(crate) params: Params,
}

impl Router {
    pub(crate) fn new() -> Self {
        Self {
            methods: HashMap::new(),
            all: Recogniser::new(),
        }
    }

    pub(crate) fn add(
        &mut self,
        method: Method,
        path: &str,
        ep: impl Endpoint + Sync + Send + 'static,
    ) {
        self.methods
            .entry(method)
            .or_insert_with(route_recognizer::Router::new)
            .add(path, Box::new(ep))
    }

    pub(crate) fn add_all(&mut self, path: &str, ep: impl Endpoint + Sync + Send + 'static) {
        self.all.add(path, Box::new(ep))
    }

    pub(crate) fn lookup(&self, method: &Method, path: &str) -> RouteTarget {
        if let Some(match_) = self
            .methods
            .get(method)
            .and_then(|recog| recog.recognize(path).ok())
        {
            RouteTarget {
                ep: &***match_.handler(),
                params: match_.params().clone(), // TODO - avoid this clone?
            }
        } else if let Ok(match_) = self.all.recognize(path) {
            RouteTarget {
                ep: &***match_.handler(),
                params: match_.params().clone(), // TODO - avoid this clone?
            }
        } else if self
            .methods
            .iter()
            .filter(|(k, _)| k != method)
            .any(|(_, recog)| recog.recognize(path).is_ok())
        {
            RouteTarget {
                ep: &method_not_allowed,
                params: Params::new(),
            }
        } else {
            RouteTarget {
                ep: &not_found,
                params: Params::new(),
            }
        }
    }
}

async fn method_not_allowed(_: Request) -> impl Responder {
    StatusCode::METHOD_NOT_ALLOWED
}

async fn not_found(_: Request) -> impl Responder {
    StatusCode::NOT_FOUND
}
