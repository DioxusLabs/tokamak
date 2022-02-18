use vir::{Request, Response, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = vir::with_state(());

    app.at("/app").get(WebsocketHandler(liveview));

    app.listen("127.0.0.1").await
}

pub async fn liveview(req: WebsocketRequest<State>) -> Result<Response> {
    let (rx, tx) = req.upgrade().await;

    while let Some(msg) = rx.next().await {
        tx.send("Some msg").await?;
    }
}
