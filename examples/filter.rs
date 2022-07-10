use std::rc::Rc;

use cookie::Cookie;
use headers::{CacheControl, ContentLength};
use http::{Method, StatusCode};
use tokamak::*;

#[derive(serde::Deserialize)]
struct Dog {
    name: String,
}

#[tokio::main]
async fn main() {
    let mut app = App::default();

    app.at("/")
        .filter(|req: Request| {
            req.content_length_max(10).unwrap();
            true
        })
        .get(|_| "get world!".to_response());

    app.at("/").any(|mut req: Request| async move {
        req.content_length_max(10)?;
        req.header_exact("billy", "bob")?;
        let dog = req.body_json::<Dog>().await?;
        // req.cookie_matches(Cookie::named("auth"), "auth-token")?;
        // req.content_length_max(10)?;
        // req.header_exact("billy", "bob")?;
        // req.cookie_matches(Cookie::named("auth"), "auth-token")?;

        match *req.method() {
            Method::GET => "hello world!".to_response(),
            Method::POST => "hello world!".to_response(),
            _ => Response::not_allowed(),
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
