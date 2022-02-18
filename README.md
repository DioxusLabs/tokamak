<div align="center">
  <h1>Tokamak</h1>
  <p>
    <strong>A simple WebFramework for realtime apps.</strong>
  </p>
</div>

<div align="center">
  <!-- Crates version -->
  <a href="https://crates.io/crates/dioxus">
    <img src="https://img.shields.io/crates/v/dioxus.svg?style=flat-square"
    alt="Crates.io version" />
  </a>
  <!-- Downloads -->
  <a href="https://crates.io/crates/dioxus">
    <img src="https://img.shields.io/crates/d/dioxus.svg?style=flat-square"
      alt="Download" />
  </a>
  <!-- docs -->
  <a href="https://docs.rs/dioxus">
    <img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
      alt="docs.rs docs" />
  </a>
  <!-- CI -->
  <a href="https://github.com/jkelleyrtp/dioxus/actions">
    <img src="https://github.com/dioxuslabs/dioxus/actions/workflows/main.yml/badge.svg"
      alt="CI status" />
  </a>

  <!--Awesome -->
  <a href="https://github.com/dioxuslabs/awesome-dioxus">
    <img src="https://cdn.rawgit.com/sindresorhus/awesome/d7305f38d29fed78fa85652e3a63e154dd8e8829/media/badge.svg" alt="Awesome Page" />
  </a>
  <!-- Discord -->
  <a href="https://discord.gg/XgGxMSkvUM">
    <img src="https://img.shields.io/discord/899851952891002890.svg?logo=discord&style=flat-square" alt="Discord Link" />
  </a>
</div>


Tokamak is a simple web-framework for Rust/Tokio, heavily inspired by Tide. It is "thread-per-core", allowing the use of !Send types across await boundaries in handlers, making it the ideal choice for use with Dioxus LiveView apps.

You can roughly consider tokamak to be a Tokio-based version of Tide that supports !Send types in WebSocket handlers.

> Tokamak is associated with the Dioxus project and all support is done through Dioxus channels.

# Examples

Handling JSON:

```rust
use tokamak::prelude::*;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u16,
}


#[tokio::main]
async fn main() -> tokamak::Result<()> {
    let mut app = tokamak::new();
    app.at("/orders/shoes").post(order_shoes);
    app.listen("127.0.0.1:8080").await?
}

async fn order_shoes(mut req: tokamak::Request) -> tokamak::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}
```

Websockets:

```rust
use tokamak::{Request, Response, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = vir::with_state(());
    app.at("/app").get(WebsocketHandler(liveview));
    app.listen("127.0.0.1").await
}

pub async fn liveview(req: WebsocketRequest<State>) -> Result<Response> {
    let (rx, tx) = req.upgrade().await;

    while let Some(msg) = rx.next().await {
        tx.send("Some msg").await?;
    }
}
```

CRUD

```rust

use vir::{Request, Response, Result};

pub struct State {
    pub db: sqlx::SqlitePool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = vir::with_state(State {
        db: sqlx::SqlitePool::connect("DATABASE_URL").await?,
    });
    app.at("/dogs").get(|req| async {
        Response::new(200)
    });
    app.listen("127.0.0.1").await
}
```

## What is thread-per-core?


## Features

- Support for running in WASM for use in things like Cloudflare workers and running on the edge
- Integration with Dioxus LiveView
- Support for Tokio ecosystem (tower, tracing, etc)
