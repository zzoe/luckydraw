use async_trait::async_trait;
use minitrace::Span;
use serde::{Deserialize, Serialize};
use tide::{Body, Middleware, Next, Request, Response, Result, StatusCode};

use crate::web::log_ext::Serial;

use super::SessionExt;
use super::WebRequest;

#[derive(Debug)]
pub(crate) struct Authentication;

impl Authentication {
    pub(crate) fn new() -> Self {
        Authentication {}
    }
}

#[async_trait]
impl<State: Clone + Send + Sync + 'static> Middleware<State> for Authentication {
    async fn handle(&self, req: Request<State>, next: Next<'_, State>) -> Result {
        if !req.url().path().starts_with("/api/") {
            return Ok(next.run(req).await);
        }

        match req.session().get::<bool>("authenticated") {
            Some(auth) if auth => Ok(next.run(req).await),
            _ => Ok(Response::new(StatusCode::Unauthorized)),
        }
    }
}

#[derive(Deserialize)]
struct Args {
    username: String,
    password: String,
}

#[derive(Serialize)]
struct Reply {
    userid: usize,
}

pub(crate) async fn login(mut req: WebRequest) -> Result {
    let mut span = Span::enter_with_local_parent("login");
    let args = req.body_json::<Args>().await?;
    span.add_properties(|| {
        vec![
            ("username", args.username.clone()),
            ("password", args.password.clone()),
        ]
    });

    let authenticated = args.username.eq("admin") && args.password.eq("admin");
    req.session_mut().insert("authenticated", authenticated)?;

    if !authenticated {
        return Ok(Response::from(StatusCode::Unauthorized));
    }

    let seq = *req.ext::<Serial>().unwrap_or(&Serial::default());
    let reply = Body::from_json(&Reply {
        userid: *seq as usize,
    })?;

    Ok(Response::builder(StatusCode::Ok).body(reply).build())
}
