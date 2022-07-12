
<div align="center">
  <h1>Tokamak</h1>
  <p>
    <strong>Rust's friendliest http framework</strong>
  </p>
</div>

Tokamak is a simple web-framework for Rust/Tokio focused on maximum developer productivity.

## Features

- Thread-per-core architecture for simple async
- Prebuilt components for auth, admin, cookies, jwt, sessions, sitemaps, and more
- Dioxus LiveView for server-rendered apps
- Progressively rendered static pages
- Prebuilt admin panel and logging
- Hyper and Tower integrations (like Axum)

Tokamak combines the batteries-included philosophy of django with the composability of express. Bring your own parts, or compose our prebuilt components to fit your app.

## A snippet

Tokamak is incredibly simple but incredibly flexible.

```rust
#[tokio::main]
async fn main() {
    let mut app = tokamak::new();

    app.at("/").get(|_| "hello world!");

    app.listen("0.0.0.0:8080").await;
}
```

## Filters

In Tokamak, you can easily filter endpoints based on criteria. Tokamak prefers functions over traits, and all filters are just functions. This saves you from thinking about lifetimes, async traits, pinning, boxing, or anything else that might seem too advanced.

```rust
 app.at("/")
    .filter(|req| req.header_exact("auth-bearer"))
    .get(|_| "hello world!");
```

## Extractors

Extractors provide a way of guarding an endpoint and returning a value.
```rust
 app.at("/")
    .extract(|req, state| state.authorize(req))
    .get(|req, state, user: Admin| "hello world!");
```

In the cases where you need an extraction just for a single handler, it can be easily done from within the handler itself.

```rust
fn admin_panel(req: Request, state: &State) -> Response {
  let admin = state.authorize::<Admin>(req)?;
}
```

Both of these strategies are "explicit" extractors. However, some extractors can be made implicit through the FromRequest trait.

```rust
fn admin_panel(req: Request, state: &State, admin: Admin) -> Response {
  todo!()
}

struct Admin {
  id: Uuid
}

impl FromRequest for Admin {
  fn parse(req: Request) -> TokamakResult<Self> {
    let auth = req.context::<AuthEngine>()?
    let token = req.cookie("app-auth")?;
    auth.is_authorized(token)
  }
}
```

They then can be easily composed in your app as a quick fallback strategy:

```rust
app.with(AuthEngine::new());

app.at("/api")
    .get(|req, state, admin: Admin| Ok("You are an admin!"))
    .get(|_| "You are not an admin");
```

## Middleware

Again, like filters, extractors, and endpoints, Tokamak's middleware is just another function. No traits!

```rust
app.at("/")
    .with(|req, state, res| res.insert_header("request-number", format!(state.count_up())))
    .get(|_| "hello world!");
```

Of course, we can refactor out our middleware into dedicated functions

```rust
app.at("/")
    .with(counting_middleware)
    .get(|_| "hello world!");
```

## Tower Layers

Because Tokamak uses hyper under the hood, you are also free to add Tower layers into your app:

```rust
app.at("/")
    .layer(identity_layer)
    .get(|_| "hello world!");
```

## Context

Middleware have the ability to add their own state into your app. This is provided as "context":

```rust
let mut app = tokamak::new();

app.with(AuthEngine::new());

app.at("/").get(|req| {
  let engine = req.context::<AuthEngine>()?;
})
```

This also means an ecosystem of layers, middleware,


## Paths, params, queries, and uri

Everything you might care about for an endpoint is accessible on the request directly.

```rust
fn load_app(req: Request) -> Response {
    let id = req.param("id")?;
    let name = req.param("name")?;
    let body = req.body_json::<Dog>()?;

    Ok(format!("Hello, {name}, your request ID is {id}").into())
}
```

## Magical Macros

If you like macros, then we've got them. Tokamak's macros essentially take care of the parsing steps that you'd write on your own. No magic, just cleaning up boilerplate:

```rust
#[get("/app/:id/:name")]
fn load_app(req: Request, id: i32, name: String, body_json: Dog) -> Response {
  Ok(format!("Hello, {name}, your request ID is {id}").into())
}
```

## Forms


## Websockets, SSE

Tokamak is built for websockets and server-side-events. WebSocket handlers are !Send, so you can freely use Cell/Rc, etc, etc.

```rust
app.at("app").ws(|req, state, (rx, tx)| {
  loop {
    let msg = rx.next().await?;
    tx.send(b"hello!")?;
  }
});
```

```rust
app.at("app").sse(|req, state, (rx, tx)| {
  loop {
    let msg = rx.next().await?;
    tx.send(b"hello!")?;
  }
});
```

## Immediate returns

Lots of endpoints don't need async. However, most Rust HTTP frameworks end up pin/boxing these anways. Tokamak knows the difference between a sync and an async handler and will immediately return any value if possible. No heap allocation required!

## !Send handlers and blocking handlers.

Internally, Tokamak uses a LocalPool to manage threads to spawn handle requests. This means all handlers are !Send.

However, you can tweak an endpoint or Router be completely blocking (allowing syncronous IO) or disable !Send for theoretical performance improvements.

```rust
// Blocking
app.blocking_at("app").get(|_| block_some_io());

// work stealing
app.work_stealing_at("app").get(|_| "asd");
```


## Remaining topics:

- Nested
- Sessions
- Cookies
