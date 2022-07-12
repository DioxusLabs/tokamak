use tokamak::{Request, Response};

fn main() {
    take_extractor(req1);
    take_extractor(req2);
    take_extractor(req3);
    take_extractor(req4);
}

fn req1(r: Request) -> Response {
    todo!()
}

fn req2(r: Request, t: Admin) -> Response {
    todo!()
}

fn req3(r: Request, t: User) -> Response {
    todo!()
}

fn req4(r: Request, t: User, b: Admin) -> Response {
    todo!()
}

fn req5(r: Request, t: User, b: Admin, len: MaxContentLength<100>) -> Response {
    todo!()
}

struct Admin {}
impl FromRequest for Admin {}

struct User {}
impl FromRequest for User {}

struct MaxContentLength<const N: usize>(usize);
impl<const N: usize> FromRequest for MaxContentLength<N> {}

fn take_extractor<A, B, C, D, E, F, G>(f: impl Endpoint<A, B, C, D, E, F, G>) {}

trait Endpoint<A = (), B = (), C = (), D = (), E = (), F = (), G = ()> {}

impl<F> Endpoint for F where F: Fn(Request) -> Response {}

trait FromRequest {}

impl<F, A: FromRequest> Endpoint<A> for F where F: Fn(Request, A) -> Response {}

impl<F, A: FromRequest, B: FromRequest> Endpoint<A, B> for F where F: Fn(Request, A, B) -> Response {}

impl<F, A: FromRequest, B: FromRequest, C: FromRequest> Endpoint<A, B, C> for F where
    F: Fn(Request, A, B, C) -> Response
{
}
