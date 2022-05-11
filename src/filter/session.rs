use crate::filter::{Filter, Next};
use crate::{Request, Response, Result};


use async_trait::async_trait;
use cookie::Cookie;
use headers::{Header, SetCookie};
use std::borrow::Cow;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::sync::Mutex;
use tokio::sync::Mutex as AsyncMutex;
use tracing::debug;
use uuid::Uuid;

/// Trait for session storage
#[async_trait]
pub trait SessionStore {
    /// Get the data associated with session
    async fn get(&self, id: &str) -> Result<Option<String>>;
    /// Set the data for a session
    async fn set(&mut self, id: String, value: String) -> Result<()>;
    /// Clear data for a session
    async fn clear(&mut self, id: &str) -> Result<()>;
}

/// Memory backed implementation of session storage.
/// NOTE this is only meant for demos and examples. In a real server
/// you would store sessions externally (e.g. in redis or a database)
#[derive(Default)]
pub struct MemorySessionStore {
    data: HashMap<String, String>,
}

impl MemorySessionStore {
    /// Create a new memory session store
    pub fn new() -> Self {
        Self::default()
    }
}

#[async_trait]
impl SessionStore for MemorySessionStore {
    async fn get(&self, id: &str) -> Result<Option<String>> {
        debug!(id, "memory store get");
        Ok(self.data.get(id).cloned())
    }

    async fn set(&mut self, id: String, value: String) -> Result<()> {
        debug!(%id, %value, "memory store set");
        self.data.insert(id, value);
        Ok(())
    }

    async fn clear(&mut self, id: &str) -> Result<()> {
        debug!(id, "memory store clear");
        self.data.remove(id);
        Ok(())
    }
}

pub const DEFAULT_COOKIE_NAME: &str = "sid";

type DynCookieCallback = dyn Fn(&mut Cookie) + Send + Sync + 'static;

/// A filter for implementing basic session support
///
/// This filter requires that the Context implements HasSession
pub struct SessionFilter {
    cookie_name: Cow<'static, str>,
    expiry: time::Duration,
    cookie_callback: Option<Box<DynCookieCallback>>,
    store: AsyncMutex<Box<dyn SessionStore + Send + Sync + 'static>>,
}

impl SessionFilter {
    /// Create a new session filter using the provided store
    /// The default cookie name is [DEFAULT_COOKIE_NAME] and expiry is set to one hour
    pub fn new(store: impl SessionStore + Send + Sync + 'static) -> SessionFilter {
        SessionFilter {
            cookie_name: Cow::Borrowed(DEFAULT_COOKIE_NAME),
            expiry: time::Duration::hours(1),
            cookie_callback: None,
            store: AsyncMutex::new(Box::new(store)),
        }
    }

    /// Set the name of the cookie used to store the session ID
    pub fn with_cookie_name(mut self, name: impl Into<Cow<'static, str>>) -> Self {
        self.cookie_name = name.into();
        self
    }

    /// Set the expiry time set on the session ID cookie
    pub fn with_expiry(mut self, expiry: time::Duration) -> Self {
        self.expiry = expiry;
        self
    }

    /// Set a callback function to be used to customise the session ID cookie.
    /// The callback is called with the cookie before it is stored in the headers so you can change
    /// most settings (changing the name or value of the cookie may prevent sessions from working,
    /// so only change settings like same site, secure, etc...)
    pub fn with_callback<F>(mut self, callback: F) -> Self
    where
        F: Fn(&mut Cookie) + Send + Sync + 'static,
    {
        self.cookie_callback = Some(Box::new(callback));
        self
    }
}

#[derive(Default)]
struct SessionInner {
    modified: AtomicBool,
    data: Mutex<HashMap<String, String>>,
}

/// A session
#[derive(Default, Clone)]
pub struct Session {
    inner: Arc<SessionInner>,
}

impl SessionInner {
    fn get(&self, key: &str) -> Option<String> {
        debug!(key, "session get");
        let data = self.data.lock().unwrap();
        data.get(key).cloned()
    }

    fn set(&self, key: String, value: String) {
        debug!(%key, %value, "session set");
        self.data.lock().unwrap().insert(key, value);
        self.modified.store(true, Ordering::Relaxed);
    }

    fn is_modified(&self) -> bool {
        self.modified.load(Ordering::Relaxed)
    }

    fn load(&self, data: HashMap<String, String>) {
        *self.data.lock().unwrap() = data;

        // we just loaded fresh data into the session, so clear modified flag to
        // detect if any changes are made that need to be saved back to storage
        self.modified.store(false, Ordering::Relaxed);
    }
}

impl Session {
    /// Get a value from the session
    pub fn get(&self, key: &str) -> Option<String> {
        self.inner.get(key)
    }

    /// Store a value into the session
    pub fn set(&self, key: String, value: String) {
        self.inner.set(key, value)
    }

    /// Determine if the session has been modified
    pub fn is_modified(&self) -> bool {
        self.inner.is_modified()
    }
}

/// This trait must be implemented by the Context type in order to use the
/// SessionFilter
pub trait HasSession {
    /// Get a reference to the Session for this current request
    fn session(&mut self) -> &mut Session;
}

/// Implement HasSession on requests where the Context has sessions
impl HasSession for Request {
    fn session(&mut self) -> &mut Session {
        self.context_mut().session()
    }
}

#[async_trait]
impl Filter for SessionFilter {
    async fn apply(&self, mut req: Request, next: Next<'_>) -> Result<Response> {
        let session = Arc::clone(&req.session().inner);

        let maybe_sid = req
            .cookies()?
            .get(self.cookie_name.as_ref())
            .map(|c| c.value().to_owned());

        let sid = if let Some(sid) = maybe_sid {
            debug!(%sid, "request has session cookie");

            let store = self.store.lock().await;
            let raw_data = store.get(&sid).await?.unwrap_or_else(String::new);
            let data = serde_urlencoded::from_str(&raw_data)?;
            session.load(data);
            sid
        } else {
            debug!("request has no session cookie");
            Uuid::new_v4().to_string()
        };

        let mut resp = next.next(req).await?;

        if session.is_modified() {
            debug!("session was modified");

            let mut store = self.store.lock().await;
            let raw_data = {
                let data = session.data.lock().unwrap();
                serde_urlencoded::to_string(&*data)?
            };

            let mut cookie = Cookie::new(self.cookie_name.as_ref(), &sid);
            cookie.set_http_only(true);
            cookie.set_secure(true);
            cookie.set_same_site(cookie::SameSite::Strict);

            let expiry = time::OffsetDateTime::now_utc() + self.expiry;
            cookie.set_expires(expiry);

            if let Some(ref callback) = self.cookie_callback {
                callback(&mut cookie);
            }

            resp.set_raw_header(SetCookie::name(), cookie.to_string())?;

            store.set(sid, raw_data).await?;
        }

        Ok(resp)
    }
}
