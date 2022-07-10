
<div align="center">
  <h1>Tokamak</h1>
  <p>
    <strong>A simple webserver framework for realtime apps.</strong>
  </p>
</div>

Tokamak is a simple web-framework for Rust/Tokio focused on maximum developer productivity.

## Features

- Dioxus LiveView for server-rendered apps
- Progressively rendered static pages
- Thread-per-core architecture for simple async
- Prebuilt components for auth, admin, cookies, jwt, sessions, sitemaps, and more
- Prebuilt admin panel and logging

## Philosophy

Tokamak combines the batteries-included philosophy of django with the composability of express. Bring your own parts, or compose our prebuilt components to build your app.

Tokamak is configured with "productive defaults."

## A snippet

```rust
let mut app = tokamak::new();
app.serve("127.0.0.1:8000");
```
