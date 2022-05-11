use std::sync::Arc;

use sqlx::{Row, SqlitePool};
use tokamak::{Request, Responder, Response, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = tokamak::new(sqlx::SqlitePool::connect("DATABASE_URL").await?);

    app.at("/dogs")
        .get(|req: Request<SqlitePool>| async move {
            let dogs = sqlx::query("SELECT * FROM todos")
                .fetch_all(req.state())
                .await
                .unwrap();

            ""
        })
        .put(|req: Request<SqlitePool>| async move {
            // Insert the task, then obtain the ID of this row
            let id = sqlx::query(r#" INSERT INTO todos ( description ) VALUES ( ?1 ) "#)
                .execute(&mut req.state().acquire().await.unwrap())
                .await
                .unwrap()
                .last_insert_rowid();

            ""
        })
        .delete(delete_todo)
        .ws(|state, rx, tx| async move {
            //

            Ok(())
        });

    app.listen("127.0.0.1").await?;

    Ok(())
}

async fn delete_todo(req: Request<SqlitePool>) -> impl Responder {
    // Insert the task, then obtain the ID of this row
    let id = sqlx::query(r#" INSERT INTO todos ( description ) VALUES ( ?1 ) "#)
        .execute(&mut req.state().acquire().await.unwrap())
        .await
        .unwrap()
        .last_insert_rowid();

    "".to_string()
}
