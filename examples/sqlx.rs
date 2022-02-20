use tokamak::prelude::{Request, Response, Result};

pub struct State {
    pub db: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = tokamak::with_state(State {
        db: sqlx::SqlitePool::connect("DATABASE_URL").await?,
    });

    app.at("/dogs")
        .get(get_dogs)
        .post(create_dogs)
        .patch(update_dogs)
        .delete(delete_dogs);

    app.listen("127.0.0.1").await
}

pub async fn create_dogs(req: Request) -> Response {
    Response::new(200)
}

pub async fn get_dogs(req: Request) -> Response {
    Response::new(200)
}

pub async fn update_dogs(req: Request) -> Response {
    Response::new(200)
}

pub async fn delete_dogs(req: Request) -> Response {
    Response::new(200)
}
