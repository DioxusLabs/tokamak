use serde::Serialize;
use std::{
    any::{Any, TypeId},
    collections::HashMap,
    fmt::{Debug, Display},
    sync::Arc,
};
use std::{future::Future, rc::Rc};

use crate::{route::Route, traits::SharedState};

pub struct ServerCfg {
    num_threads: usize,
}

pub struct Server<F = ()> {
    num_threads: usize,
    state: Arc<F>,
}

impl<F: SharedState> Server<F> {
    pub fn new(state: F, num_threads: usize) -> Self {
        Self {
            num_threads,
            state: Arc::new(state),
        }
    }

    pub fn at<'a, 'b>(&'a mut self, path: &'b str) -> Route<F> {
        Route {
            path: path.to_string(),
            server: self,
        }
    }

    pub async fn listen(self, addr: &str) -> Result<()> {
        let mut tasks = vec![];

        for thread in 0..self.num_threads {
            tasks.push(tokio::task::spawn_blocking({
                let addr = addr.to_string();
                let state = self.state.clone();
                move || {
                    todo!()
                    // tokio::runtime::Builder::new_current_thread()
                    //     .build()
                    //     .unwrap()
                    //     .block_on(listener::listen(addr, state));
                }
            }));
        }

        futures::future::join_all(tasks).await;

        Ok(())
    }
}
