use crate::innerlude::*;

pub struct Route<'a, T: Send + Sync = ()> {
    pub path: &'static str,
    pub app: &'a mut App<T>,
}

impl<'a, T: Send + Sync> Route<'a, T> {
    pub fn get<'b, F>(&mut self, t: impl EndPoint<'b, F, T>) -> &mut Self {
        self
    }

    pub fn post<'b, F>(&mut self, t: impl EndPoint<'b, F, T>) -> &mut Self {
        self
    }

    pub fn filter(&mut self, f: impl Fn(Request) -> bool) {
        todo!()
    }
}
