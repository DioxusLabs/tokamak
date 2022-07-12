use crate::{innerlude::TokamakResult, ResponseResult};
use cookie::Cookie;
use http::StatusCode;

pub struct Response {}

impl Response {
    pub fn new(code: StatusCode) -> Self {
        todo!()
    }

    pub fn redirect(to: &str) -> TokamakResult<Self> {
        todo!()
    }

    pub fn some_body<T>(body: Option<T>) -> TokamakResult<Self> {
        todo!()
    }

    pub fn insert_cookie(&mut self, cookie: Cookie<'_>) {
        todo!()
    }
    pub fn remove_cookie(&mut self, cookie: Cookie<'_>) {
        todo!()
    }
    pub fn with_cookie(mut self, cookie: Cookie<'_>) -> Self {
        todo!()
    }

    pub fn with_remove_cookie(mut self, cookie: Cookie<'_>) -> Self {
        todo!()
    }

    pub fn build(self) -> ResponseResult {
        todo!()
    }

    pub fn ok() -> ResponseResult {
        todo!()
    }

    pub fn not_allowed() -> ResponseResult {
        todo!()
    }
}

impl From<Response> for crate::ResponseResult {
    fn from(response: Response) -> Self {
        todo!()
    }
}

impl From<serde_json::Value> for Response {
    fn from(json_value: serde_json::Value) -> Self {
        todo!()
        // Body::from_json(&json_value)
        //     .map(|body| body.into())
        //     .unwrap_or_else(|_| Response::new(StatusCode::InternalServerError))
    }
}

// impl From<Error> for Response {
//     fn from(err: Error) -> Self {
//         // Self {
//         //     res: http::Response::new(err.status()),
//         //     error: Some(err),
//         //     #[cfg(feature = "cookies")]
//         //     cookie_events: vec![],
//         // }
//     }
// }

// impl From<http::Response> for Response {
//     fn from(res: http::Response) -> Self {
//         todo!()
//         // Self {
//         //     res,
//         //     error: None,
//         //     #[cfg(feature = "cookies")]
//         //     cookie_events: vec![],
//         // }
//     }
// }

// impl From<StatusCode> for Response {
//     fn from(status: StatusCode) -> Self {
//         todo!()
//         // let res: http::Response = status.into();
//         // res.into()
//     }
// }

impl From<String> for Response {
    fn from(s: String) -> Self {
        todo!()
        // Body::from_string(s).into()
    }
}

impl From<(http::StatusCode, String)> for Response {
    fn from(a: (http::StatusCode, String)) -> Self {
        todo!()
        // Body::from_string(s).into()
    }
}

impl From<(http::StatusCode, &str)> for Response {
    fn from(a: (http::StatusCode, &str)) -> Self {
        todo!()
        // Body::from_string(s).into()
    }
}

impl<'a> From<&'a str> for Response {
    fn from(s: &'a str) -> Self {
        todo!()
        // Body::from_string(String::from(s)).into()
    }
}

pub trait ToResponse {
    fn to_response(self) -> crate::ResponseResult;
}

impl ToResponse for String {
    fn to_response(self) -> crate::ResponseResult {
        todo!()
    }
}

impl ToResponse for &str {
    fn to_response(self) -> crate::ResponseResult {
        todo!()
    }
}
