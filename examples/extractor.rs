use std::marker::PhantomData;

use tokamak::{
    innerlude::{FromRequest, FromRequestReturn},
    Request, Response, Result,
};

fn main() {
    let mut app = tokamak::default();

    app.at("asd")
        .get(|_| Response::ok())
        .get(|req: Request| async move {
            //
            Ok(Response::ok())
        });
}

async fn my_stuff(req: Request, g: Admin, b: Admin) -> Result {
    Ok(Response::ok())
}

struct Admin;
impl FromRequest for Admin {
    fn from_request(req: &Request) -> FromRequestReturn<Self> {
        todo!()
    }
}

async fn combined_request(req: MyRequest) -> Result {
    Ok(Response::ok())
}

struct MyRequest<P = ()> {
    state: std::rc::Rc<P>,
}
