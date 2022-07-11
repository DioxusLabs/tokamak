use headers::ContentLength;

use crate::{innerlude::TokamakError, Request, ResponseResult};

pub fn content_length_max(req: &Request, max: u64) -> Result<ContentLength, TokamakError> {
    todo!()
}

pub fn content_length(max: u64) -> impl Fn(Request) -> Result<bool, TokamakError> {
    move |req| content_length_max(&req, max).map(|_| true)
}
