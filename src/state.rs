/// State must be implemented for any type being used as the App's state
///
/// State is shared by all requests, and must be safe to be shared between
/// threads (Send + Sync + 'static)
///
/// The state also creates the Context objects used to store request local
/// data.
/// Before processing a request a new context is created
pub trait SharedState: Send + Sync + 'static + Clone {}
impl<T: Send + Sync + 'static + Clone> SharedState for T {}
