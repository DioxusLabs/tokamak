use tokamak::{Request, Response, TokamakResult};

#[tokio::main]
async fn main() -> TokamakResult<()> {
    let mut app = tokamak::new(());

    app.listen("127.0.0.1").await;

    Ok(())
}
