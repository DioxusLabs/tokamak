
<div align="center">
  <h1>Tokamak</h1>
  <p>
    <strong>A simple webserver framework for realtime apps.</strong>
  </p>
</div>

Tokamak is a simple web-framework for Rust/Tokio focused on maximum developer productivity.

## Features

- Thread-per-core architecture for simple async
- Prebuilt components for auth, admin, cookies, jwt, sessions, sitemaps, and more
- Dioxus LiveView for server-rendered apps
- Progressively rendered static pages
- Prebuilt admin panel and logging

## Philosophy

Tokamak combines the batteries-included philosophy of django with the composability of express. Bring your own parts, or compose our prebuilt components to build your app.

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

