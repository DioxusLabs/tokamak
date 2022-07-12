use tokamak::*;

#[tokio::main]
async fn main() {
    let mut app = tokamak::default();

    app.at("/").get(|_| "hello world!");

    app.at("/").get(|_| (StatusCode::FORBIDDEN, "Not allowed"));

    app.listen("0.0.0.0:8080").await.unwrap();
}
