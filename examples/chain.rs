fn main() {
    app.at("/").get(|req| async move {
        //
        req.respond().body("asd").send()
    });

    app.at("/").get(|req| async move {
        //
        req.respond().send()
    });

    app.at("/json").get(|req| async move {
        let cat = req.body_json::<Cat>().await?;
        req.respond().body("ci").send()
    });

    app.at("/builder").get(|req| async move {
        //
        Ok("asd")
    });

    app.at("/builder").get(|req| async move {
        //
        Ok("asd")
    });
}
