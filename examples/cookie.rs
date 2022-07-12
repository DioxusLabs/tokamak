use cookie::Cookie;
use tokamak::{Request, Response};

#[tokio::main]
async fn main() {
    let mut app = tokamak::default();

    app.at("/")
        .get(|req: Request| req.cookie("hello").map(|f| format!("hello cookies: {f:?}")));

    app.at("/set")
        .get(|_| Response::ok().with_cookie(Cookie::new("hello", "world")));

    app.at("/remove")
        .get(|_| Response::ok().with_remove_cookie(Cookie::named("hello")));

    app.listen("127.0.0.1:8080").await.unwrap();
}
