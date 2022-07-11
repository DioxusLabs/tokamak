use tokamak::{Request, Response};

#[tokio::main]
async fn main() {
    let mut app = tokamak::default();

    app.at("/").get(|_| Ok(""));
    app.at("/user").post(|_| Response::ok());
    app.at("/user/:id")
        .get(|req: Request| Ok(req.param("id")?.to_string()));

    app.listen("0.0.0.0:3000").await.unwrap()
}
