use sqlx::SqlitePool;

use std::sync::Arc;
use tokamak::{
    ws::{WebSocketReceiver, WebSocketSender},
    Request, Responder, Result,
};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = tokamak::new(sqlx::SqlitePool::connect("DATABASE_URL").await?);

    app.at("/dogs").get(delete_todo).ws(todo_ws);

    app.listen("127.0.0.1").await?;

    Ok(())
}

async fn delete_todo(state: Arc<SqlitePool>, req: Request) -> impl Responder {
    // Insert the task, then obtain the ID of this row
    let id = sqlx::query(r#" INSERT INTO todos ( description ) VALUES ( ?1 ) "#)
        .execute(&mut state.acquire().await.unwrap())
        .await
        .unwrap()
        .last_insert_rowid();

    "".to_string()
}

async fn todo_ws(
    state: Arc<SqlitePool>,
    mut tx: WebSocketSender,
    mut rx: WebSocketReceiver,
) -> Result<()> {
    tx.send(tokamak::Message::Text("Hello".to_string())).await?;

    while let Ok(Some(msg)) = rx.recv().await {
        // Do something with the message
    }

    Ok(())
}
