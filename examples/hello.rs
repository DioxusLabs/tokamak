use tokamak::*;

#[tokio::main]
async fn main() {
    let mut app = tokamak::default();

    app.at("/").get(|req: Request| "hello world!");

    let mut app = tokamak::new(10i32);
    app.at("/")
        .get(|req: Request, state: &i32| Ok("hello world!"));

    app.at("/").get(|_| Ok("hello world!"));

    app.at("/").get(|_| async { "hello world!" });

    app.at("/").get(|_| async { Ok("hello world!") });

    app.at("/").get(|_| (StatusCode::FORBIDDEN, "Not allowed"));

    app.listen("0.0.0.0:8080").await.unwrap();
}
