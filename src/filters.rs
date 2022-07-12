use headers::ContentLength;

use crate::{innerlude::Error, Request, ResponseResult};

pub fn content_length_max(req: Request, max: u64) -> Result<ContentLength, Error> {
    todo!()
}

pub fn content_length(max: u64) -> impl Fn(Request) -> Result<bool, Error> {
    move |req| content_length_max(req, max).map(|_| true)
}
