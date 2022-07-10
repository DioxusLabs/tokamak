use tokamak::*;

#[tokio::main]
async fn main() {
    let mut app = App::default();

    app.at("/").get(|_| Ok("hello world!".into()));

    app.listen("0.0.0.0:8080").await.unwrap();
}
