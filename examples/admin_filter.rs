use tokamak::*;

#[tokio::main]
async fn main() {
    let mut app = tokamak::default();

    // Protect indivudal routes with a filter.
    app.at("/admin")
        .get(|req, state| {
            let admin = ensure_admin(req, state)?;
            Ok("welcome to the admin panel")
        })
        .get(|req, state| {
            let user = ensure_user(req, state)?;
            Ok("welcome to the user panel")
        })
        .get(|_| Response::redirect("/login"));

    // Protect the api to admins only
    app.at("/api")
        .extract(authorize::<Admin>())
        .get(|req, state, admin| {})
        .post(|req, state, admin| {});

    app.at("/login").get(|_| Ok(r#"some template here"#));
}
