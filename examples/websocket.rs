use tokamak::{Request, Response, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = tokamak::new(());

    app.listen("127.0.0.1").await;

    Ok(())
}
