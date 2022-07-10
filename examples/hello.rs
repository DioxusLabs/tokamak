use tokamak::*;

#[tokio::main]
async fn main() {
    let mut app = App::default();

    app.at("/").get(|_| async { "hello world!".to_response() });

    app.listen("0.0.0.0:8080").await.unwrap();
}
