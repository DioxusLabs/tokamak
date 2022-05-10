use tokamak::{App, Request, Response, Result};

#[derive(Clone)]
pub struct State {
    pub db: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = App::new(State {
        db: sqlx::SqlitePool::connect("DATABASE_URL").await?,
    });

    app.at("/dogs")
        .get(get_dogs)
        .get(get_dogs)
        .get(get_dogs)
        .get(get_dogs);

    app.at("/dogs")
        .get(get_dogs)
        .get(get_dogs)
        .get(get_dogs)
        .get(get_dogs);

    app.listen("127.0.0.1").await?;

    Ok(())
}

pub async fn create_dogs(req: Request<State>) -> Response {
    Response::ok()
}

pub async fn get_dogs(req: Request<State>) -> Response {
    Response::ok()
}

pub async fn update_dogs(req: Request<State>) -> Response {
    Response::ok()
}

pub async fn delete_dogs(req: Request<State>) -> Response {
    Response::ok()
}
