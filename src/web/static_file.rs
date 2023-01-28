use std::ffi::OsStr;
use std::io;
use std::path::{Path, PathBuf};

use minitrace::Span;
use tide::{Body, Response, StatusCode};

use crate::web::WebRequest;

pub(crate) async fn get(req: WebRequest) -> tide::Result {
    let mut span = Span::enter_with_local_parent("static");
    let path = req.url().path();
    let path = path.trim_start_matches('/');
    let dir = PathBuf::from("dist");
    let mut file_path = dir.clone();
    for p in Path::new(path) {
        if p == OsStr::new(".") {
            continue;
        } else if p == OsStr::new("..") {
            file_path.pop();
        } else {
            file_path.push(&p);
        }
    }

    span.add_property(|| ("Requested file", file_path.to_string_lossy().to_string()));

    // let file_path = AsyncPathBuf::from(file_path);
    if !file_path.starts_with(&dir) {
        Ok(Response::new(StatusCode::Forbidden))
    } else if file_path.eq(&dir) {
        serve_index().await
    } else {
        match Body::from_file(&file_path).await {
            Ok(body) => Ok(Response::builder(StatusCode::Ok).body(body).build()),
            Err(e) if e.kind() == io::ErrorKind::NotFound => serve_index().await,
            Err(e) => Err(e.into()),
        }
    }
}

async fn serve_index() -> tide::Result {
    if let Ok(body) = Body::from_file("dist/index.html").await {
        return Ok(Response::builder(StatusCode::Ok).body(body).build());
    }
    Ok(Response::new(StatusCode::NotFound))
}
