use std::rc::Rc;

use cookie::Cookie;
use headers::{CacheControl, ContentLength};
use http::{Method, StatusCode};
use tokamak::*;

pub type RequestMut<'a> = &'a mut Request;

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

    // Try to handle as admin
    app.at("/posts").get(|req: Request| async move {
        req.header_exact("admin", "bob")?;
        Ok("get world!")
    });

    // Fallback to regular user
    app.at("/posts").get(|req: Request| async move {
        req.header_exact("admin", "bob")?;
        Ok("get world!")
    });

    app.at("/").any(|mut req: Request| async move {
        req.content_length_max(10)?;
        req.header_exact("billy", "bob")?;
        req.body_json::<Dog>().await?;

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

fn my_app() {
    let mut app = tokamak::default();

    // app.filter(ddos_protection());

    app.at("/admin")
        .filter(content_length_filter(10))
        .with(|req, state, res| {})
        .get(|req, state| Ok("asd"))
        .get(|req, state| Ok("asd"))
        .get(|_| Response::redirect("/login"));

    app.at("/login").get(|_| Ok(r#"some template"#));
}

// extractors are common to all endpoints
// They force all end points to have the same signature

fn admin_panel(req: RequestMut) -> Response {
    let admin = req.authorize()?;
    Ok(todo!())
}

struct Admin {}
struct User {}

mod wacky_shit {

    // // at max, 10 extractions
    // trait EndPoint2<A, B, C, D, E, F, G, H, I> {}

    // impl EndPoint2<A, B, C, D, E, F, G, H, I> for Fn() {}
}

// methods are spread out in your codebase
