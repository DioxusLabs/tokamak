use headers::ContentLength;

use crate::{innerlude::TokamakError, Request, ResponseResult};

pub fn content_length_max(req: &Request, max: u64) -> Result<ContentLength, TokamakError> {
    todo!()
}
