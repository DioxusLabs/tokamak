use std::{collections::HashMap, ops::Deref, sync::RwLock};

use futures_util::Future;
use sqlx::SqlitePool;
use tokamak::Request;
// use tokamak::{Request, Response};
// use tokamak::{
//     ws::{WebSocketReceiver, WebSocketSender},
//     Method, Request, Responder, Response, Result,
// };

/*



async fn handler(cx: Request<State, Context,>) {

}


*/

#[tokio::main]
async fn main() -> tokamak::ResponseResult<()> {
    let mut app = tokamak::new(sqlx::SqlitePool::connect("DATABASE_URL").await?);

    // app.at("/dogs").get(get_todo);

    // app.listen("127.0.0.1").await?;

    Ok(())
}

// async fn get_todo(req: Request<()>) -> Response {
//     todo!()
// }

type TokamakResult<T> = std::result::Result<T, tokamak::Error>;
type Result = std::result::Result<MyResponse, tokamak::Error>;

struct MyRequest;

impl MyRequest {
    fn param(&self, _: &str) -> TokamakResult<String> {
        todo!()
    }

    fn params<O, P: ParameterList<O>>(&self, list: P) -> TokamakResult<O> {
        todo!()
    }
}

trait ParameterList<O> {
    fn parse(&self, req: &Request) -> TokamakResult<O> {
        todo!()
    }
}
impl<A> ParameterList<(A,)> for (&str,) {}
impl<A, B> ParameterList<(A, B)> for (&str, &str) {}
impl<A, B, C> ParameterList<(A, B, C)> for (&str, &str, &str) {}
impl<A, B, C, D> ParameterList<(A, B, C, D)> for (&str, &str, &str, &str) {}

struct MyResponse;
impl MyResponse {
    fn not_found() -> Result {
        todo!()
    }

    fn ok_with_body(body: String) -> Result {
        todo!()
    }

    fn some_body<T>(_: Option<T>) -> Result {
        todo!()
    }

    fn ok() -> Result {
        todo!()
    }
}

trait EndPoint<'a, S, A>: 'static {}

struct NoState;
impl<F, Fut, S> EndPoint<'_, NoState, S> for F
where
    F: Fn(MyRequest) -> Fut + 'static,
    Fut: Future<Output = Result>,
{
}

struct Stateful;
impl<'a, F, Fut, S> EndPoint<'a, Stateful, S> for F
where
    F: Fn(MyRequest, &'a S) -> Fut + 'static,
    Fut: Future<Output = Result> + 'a,
    S: 'static,
    Fut: 'a,
{
}

struct MyState;

impl From<String> for MyResponse {
    fn from(_: String) -> Self {
        todo!()
    }
}

#[derive(Default)]
struct App<T: Send + Sync = ()> {
    state: T,
}

impl<T: Send + Sync> App<T> {
    fn new(state: T) -> Self {
        Self { state }
    }

    fn at(&mut self, path: &'static str) -> Route<T> {
        Route { app: self, path }
    }

    fn get<'a, F>(&mut self, t: impl EndPoint<'a, F, T>) {
        todo!()
    }

    fn filter(&mut self, f: impl Fn(MyRequest) -> bool) {
        todo!()
    }
}

struct Route<'a, T: Send + Sync = ()> {
    path: &'static str,
    app: &'a mut App<T>,
}

impl<'a, T: Send + Sync> Route<'a, T> {
    fn get<'b, F>(&mut self, t: impl EndPoint<'b, F, T>) -> &mut Self {
        self
    }

    fn post<'b, F>(&mut self, t: impl EndPoint<'b, F, T>) -> &mut Self {
        self
    }

    fn filter(&mut self, f: impl Fn(MyRequest) -> bool) {
        todo!()
    }
}

fn it_works() {
    let mut app = App::new(MyState);

    app.get(simple);
    app.get(stateful);
}

async fn simple(req: MyRequest) -> Result {
    todo!()
}

async fn stateful(req: MyRequest, state: &MyState) -> Result {
    todo!()
}
/*


artbiter (requests -> workers)
---
worker 1 - task....
worker 2 - task
worker 3 - task task taask
worker 4 - task
worker 5 - task task task task task  -> pending/working
worker 6 - request->task
worker 7
worker 8

latency
p95   - ==
p98   - ==============
p99   - ===========================================
p99.9 - ============================================================================
*/

// fn main2() {
//     let mut app = App::new(KvStore::default());

//     app.at("/:key").get(get_key).post(set_key);
//     app.at("/keys").get(get_keys);

//     app.listen("0.0.0.0:8080").unwrap();
// }

// type KvStore = RwLock<HashMap<String, String>>;

// async fn get_key(req: MyRequest, store: &KvStore, key: String) -> Result {
//     MyResponse::some_body(req.state::<KvStore>()?)
// }

// async fn set_key(req: MyRequest, state: &KvStore) -> Result {
//     let (key, value): (String, String) = req.params(("key", "value"))?;
//     state.write().unwrap().insert(key, value);

//     MyResponse::ok()
// }

// async fn get_keys(req: MyRequest, state: &KvStore) -> Result {
//     Ok(
//         serde_json::to_string(&state.read().unwrap().deref().keys().collect::<Vec<_>>())
//             .unwrap()
//             .into(),
//     )
// }
