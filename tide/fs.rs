use crate::{Body, Endpoint, Request, Response, Result, StatusCode};

// use tokio::fs::File
// use async_std::path::PathBuf as AsyncPathBuf;

use std::path::{Path, PathBuf};
use std::{ffi::OsStr, io};

pub(crate) struct ServeDir {
    prefix: String,
    dir: PathBuf,
}

impl ServeDir {
    /// Create a new instance of `ServeDir`.
    pub(crate) fn new(prefix: String, dir: PathBuf) -> Self {
        Self { prefix, dir }
    }
}

#[async_trait::async_trait]
impl Endpoint for ServeDir {
    async fn call(&self, req: Request) -> Result {
        let path = req.url().path();
        let path = path
            .strip_prefix(&self.prefix.trim_end_matches('*'))
            .unwrap();
        let path = path.trim_start_matches('/');
        let mut file_path = self.dir.clone();
        for p in Path::new(path) {
            if p == OsStr::new(".") {
                continue;
            } else if p == OsStr::new("..") {
                file_path.pop();
            } else {
                file_path.push(&p);
            }
        }

        log::info!("Requested file: {:?}", file_path);

        todo!()
        // let file_path = AsyncPathBuf::from(file_path);
        // if !file_path.starts_with(&self.dir) {
        //     log::warn!("Unauthorized attempt to read: {:?}", file_path);
        //     Ok(Response::new(StatusCode::Forbidden))
        // } else {
        //     match Body::from_file(&file_path).await {
        //         Ok(body) => Ok(Response::builder(StatusCode::Ok).body(body).build()),
        //         Err(e) if e.kind() == io::ErrorKind::NotFound => {
        //             log::warn!("File not found: {:?}", &file_path);
        //             Ok(Response::new(StatusCode::NotFound))
        //         }
        //         Err(e) => Err(e.into()),
        //     }
        // }
    }
}

// use crate::{Body, Endpoint, Request, Response, Result, StatusCode};
// use std::io;
// use std::path::Path;

// use async_std::path::PathBuf as AsyncPathBuf;
use async_trait::async_trait;

pub(crate) struct ServeFile {
    // path: AsyncPathBuf,
}

impl ServeFile {
    /// Create a new instance of `ServeFile`.
    pub(crate) fn init(path: impl AsRef<Path>) -> io::Result<Self> {
        let file = path.as_ref().to_owned().canonicalize()?;
        Ok(Self {
            // path: AsyncPathBuf::from(file),
        })
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Endpoint for ServeFile {
    async fn call(&self, _: Request) -> Result {
        match Body::from_file(&self.path).await {
            Ok(body) => Ok(Response::builder(StatusCode::Ok).body(body).build()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => {
                log::warn!("File not found: {:?}", &self.path);
                Ok(Response::new(StatusCode::NotFound))
            }
            Err(e) => Err(e.into()),
        }
    }
}
