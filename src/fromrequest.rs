use crate::{
    innerlude::{Error, GenericReturn},
    Request, Response,
};

pub type FromRequestReturn<'a, T> = GenericReturn<'a, crate::Result<T>>;

pub trait FromRequest: Sized {
    fn from_request(req: &Request) -> FromRequestReturn<Self>;
}

struct Admin;
impl FromRequest for Admin {
    fn from_request(req: &Request) -> FromRequestReturn<Self> {
        GenericReturn::Immediate({
            if req.uri() == "/admin" {
                Ok(Admin {})
            } else {
                Err(Error::Http(Response::new(http::StatusCode::UNAUTHORIZED)))
            }
        })
    }
}

struct AdminAsync;
impl FromRequest for AdminAsync {
    fn from_request(req: &Request) -> FromRequestReturn<Self> {
        GenericReturn::Future(Box::pin(async move {
            if req.uri() == "/admin" {
                Ok(AdminAsync {})
            } else {
                Err(Error::Http(Response::new(http::StatusCode::UNAUTHORIZED)))
            }
        }))
    }
}
