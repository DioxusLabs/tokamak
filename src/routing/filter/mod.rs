//! filter

mod method;
mod opts;
mod path;

use self::opts::*;
use crate::http::{Method, Request};
use crate::routing::PathState;

pub use method::*;
pub use path::*;

/// Fiter trait for filter request.
pub trait Filter: Send + Sync + 'static {
    /// Create a new filter use ```And``` filter.
    fn and<F>(self, other: F) -> And<Self, F>
    where
        Self: Sized,
        F: Filter + Sync + Send,
    {
        And {
            first: self,
            second: other,
        }
    }

    /// Create a new filter use ```Or``` filter.
    fn or<F>(self, other: F) -> Or<Self, F>
    where
        Self: Sized,
        F: Filter + Sync + Send,
    {
        Or {
            first: self,
            second: other,
        }
    }

    /// Create a new filter use ```AndThen``` filter.
    fn and_then<F>(self, fun: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: Fn(&mut Request, &mut PathState) -> bool + Send + Sync + 'static,
    {
        AndThen {
            filter: self,
            callback: fun,
        }
    }

    /// Create a new filter use ```OrElse``` filter.
    fn or_else<F>(self, fun: F) -> OrElse<Self, F>
    where
        Self: Sized,
        F: Fn(&mut Request, &mut PathState) -> bool + Send + Sync + 'static,
    {
        OrElse {
            filter: self,
            callback: fun,
        }
    }

    /// Filter ```Request``` and returns false or true.
    fn filter(&self, req: &mut Request, path: &mut PathState) -> bool;
}

/// ```FnFilter``` accpect a function as it's param, use this function to filter request.
#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
pub struct FnFilter<F>(pub F);

impl<F> Filter for FnFilter<F>
where
    F: Fn(&mut Request, &mut PathState) -> bool + Send + Sync + 'static,
{
    #[inline]
    fn filter(&self, req: &mut Request, path: &mut PathState) -> bool {
        self.0(req, path)
    }
}

/// Filter request use ```PathFilter```.
#[inline]
pub fn path(path: impl Into<String>) -> PathFilter {
    PathFilter::new(path)
}
/// Filter request, only allow get method.
#[inline]
pub fn get() -> MethodFilter {
    MethodFilter(Method::GET)
}
/// Filter request, only allow head method.
#[inline]
pub fn head() -> MethodFilter {
    MethodFilter(Method::HEAD)
}
/// Filter request, only allow options method.
#[inline]
pub fn options() -> MethodFilter {
    MethodFilter(Method::OPTIONS)
}
/// Filter request, only allow post method.
#[inline]
pub fn post() -> MethodFilter {
    MethodFilter(Method::POST)
}
/// Filter request, only allow path method.
#[inline]
pub fn patch() -> MethodFilter {
    MethodFilter(Method::PATCH)
}
/// Filter request, only allow put method.
#[inline]
pub fn put() -> MethodFilter {
    MethodFilter(Method::PUT)
}

/// Filter request, only allow delete method.
#[inline]
pub fn delete() -> MethodFilter {
    MethodFilter(Method::DELETE)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_methods() {
        assert!(get() == MethodFilter(Method::GET));
        assert!(head() == MethodFilter(Method::HEAD));
        assert!(options() == MethodFilter(Method::OPTIONS));
        assert!(post() == MethodFilter(Method::POST));
        assert!(patch() == MethodFilter(Method::PATCH));
        assert!(put() == MethodFilter(Method::PUT));
        assert!(delete() == MethodFilter(Method::DELETE));
    }

    #[test]
    fn test_opts() {
        fn has_one(_req: &mut Request, path: &mut PathState) -> bool {
            path.url_path.contains("one")
        }
        fn has_two(_req: &mut Request, path: &mut PathState) -> bool {
            path.url_path.contains("two")
        }

        let one_filter = FnFilter(has_one);
        let two_filter = FnFilter(has_two);

        let mut req = Request::default();
        let mut path_state = PathState::new("http://localhost/one");
        assert!(one_filter.filter(&mut req, &mut path_state));
        assert!(!two_filter.filter(&mut req, &mut path_state));
        assert!(one_filter
            .or_else(has_two)
            .filter(&mut req, &mut path_state));
        assert!(one_filter.or(two_filter).filter(&mut req, &mut path_state));
        assert!(!one_filter
            .and_then(has_two)
            .filter(&mut req, &mut path_state));
        assert!(!one_filter.and(two_filter).filter(&mut req, &mut path_state));

        let mut path_state = PathState::new("http://localhost/one/two");
        assert!(one_filter.filter(&mut req, &mut path_state));
        assert!(two_filter.filter(&mut req, &mut path_state));
        assert!(one_filter
            .or_else(has_two)
            .filter(&mut req, &mut path_state));
        assert!(one_filter.or(two_filter).filter(&mut req, &mut path_state));
        assert!(one_filter
            .and_then(has_two)
            .filter(&mut req, &mut path_state));
        assert!(one_filter.and(two_filter).filter(&mut req, &mut path_state));

        let mut path_state = PathState::new("http://localhost/two");
        assert!(!one_filter.filter(&mut req, &mut path_state));
        assert!(two_filter.filter(&mut req, &mut path_state));
        assert!(one_filter
            .or_else(has_two)
            .filter(&mut req, &mut path_state));
        assert!(one_filter.or(two_filter).filter(&mut req, &mut path_state));
        assert!(!one_filter
            .and_then(has_two)
            .filter(&mut req, &mut path_state));
        assert!(!one_filter.and(two_filter).filter(&mut req, &mut path_state));
    }
}
