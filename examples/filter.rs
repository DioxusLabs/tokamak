use std::rc::Rc;

use headers::{CacheControl, ContentLength};
use http::{Method, StatusCode};
use tokamak::*;

#[tokio::main]
async fn main() {
    let mut app = App::default();

    app.at("/").any(|req: Request| {
        let len = req.content_length_max(10)?;
        req.header::<CacheControl>()?;
        req.header_exact("billy", "bob")?;

        match *req.method() {
            Method::GET => "hello world!".to_response(),
            Method::POST => "hello world!".to_response(),
            _ => Response::new(StatusCode::METHOD_NOT_ALLOWED).ok(),
        }
    });

    app.at("/").any(|mut req: Request| async move {
        //
        let val = Rc::new(10);

        req.body_string().await?;

        dbg!(val);

        "hello world!".to_response()
    });

    app.listen("0.0.0.0:8080").await.unwrap();
}

fn content_length_filter(size: u64) -> impl Fn(Request) -> bool {
    move |req| {
        req.header::<ContentLength>()
            .map(|f| f.0 > size)
            .unwrap_or(false)
    }
}
