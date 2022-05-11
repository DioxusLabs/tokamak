use std::net::SocketAddr;

use sqlx::SqlitePool;
use tokamak::{Request, Response, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let pool = sqlx::SqlitePool::connect("DATABASE_URL").await?;

    let mut app = tokamak::new();

    app.at("/dogs")
        .get(|req| async {
            pool.acquire().await.unwrap();

            Response::ok()
        })
        .post(|req| async {
            pool.acquire().await.unwrap();

            Response::ok()
        });

    app.at("/dogs/ws/").ws(|rx, tx| async {
        //
        Ok(())
    });

    Ok(())
}

async fn get_dogs(req: Request, pool: &SqlitePool) -> Result<Response> {
    Ok(Response::ok())
}
