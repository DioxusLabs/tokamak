use sqlx::{Column, Row, SqlitePool};
use tokamak::{Request, ResponseResult, ToResponse};

#[tokio::main]
async fn main() {
    let mut app = tokamak::new(sqlx::SqlitePool::connect("DATABASE_URL").await.unwrap());

    app.at("/dogs").get(fetch_dogs);

    app.listen("127.0.0.1").await.unwrap();
}

async fn fetch_dogs(req: Request, state: &SqlitePool) -> ResponseResult {
    let dogs = sqlx::query("SELECT * FROM dogs")
        .fetch_all(state)
        .await
        .unwrap()
        .into_iter()
        .map(|row| row.columns()[0].name().to_string())
        .collect::<Vec<_>>();

    serde_json::to_string(&dogs).unwrap().to_response()
}
