use std::{collections::HashMap, ops::Deref, sync::RwLock};

use tokamak::*;

type KvStore = RwLock<HashMap<String, String>>;

#[tokio::main]
async fn main() {
    let mut app = tokamak::new(KvStore::default());

    app.at("/:key").get(get_key).post(set_key);

    app.at("/keys").get(get_keys);

    app.listen("0.0.0.0:8080").await.unwrap();
}

fn get_key(req: Request, state: &KvStore) -> ResponseResult {
    Response::some_body(state.write().unwrap().get(req.param("key")?).cloned())
}

fn set_key(req: Request, state: &KvStore) -> ResponseResult {
    state.write().unwrap().insert(
        req.param("key")?.to_string(),
        req.param("value")?.to_string(),
    );
    Response::ok()
}

fn get_keys(_req: Request, state: &KvStore) -> ResponseResult {
    Ok(
        serde_json::to_string(&state.read().unwrap().deref().keys().collect::<Vec<_>>())
            .unwrap()
            .into(),
    )
}
