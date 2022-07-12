use crate::{
    innerlude::{EndPointReturn, TokamakError},
    Request, Response,
};

pub trait FromRequest {
    fn from_request(req: &Request) -> EndPointReturn;
}

struct Admin;
impl FromRequest for Admin {
    fn from_request(req: &Request) -> EndPointReturn {
        EndPointReturn::Immediate({
            if req.uri() == "/admin" {
                Ok("admin".into())
            } else {
                Err(TokamakError::Http(Response::new(
                    http::StatusCode::UNAUTHORIZED,
                )))
            }
        })
    }
}

struct AdminAsync;
impl FromRequest for AdminAsync {
    fn from_request(req: &Request) -> EndPointReturn {
        EndPointReturn::Future(Box::pin(async move {
            if req.uri() == "/admin" {
                Ok("admin".into())
            } else {
                Err(TokamakError::Http(Response::new(
                    http::StatusCode::UNAUTHORIZED,
                )))
            }
        }))
    }
}
