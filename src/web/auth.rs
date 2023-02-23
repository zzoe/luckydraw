use async_trait::async_trait;
use minitrace::Span;
use r2d2_sqlite::rusqlite;
use r2d2_sqlite::rusqlite::OptionalExtension;
use serde::{Deserialize, Serialize};
use tide::{Body, Middleware, Next, Request, Response, Result, StatusCode};

use crate::web::session::SessionExt;
use crate::web::WebRequest;

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
    user_account: String,
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
            ("user_account", args.user_account.clone()),
            ("input password", args.password.clone()),
        ]
    });

    let pool = req.state().pool.clone();
    let conn = pool.get()?;

    //获取数据库密码
    let mut span = Span::enter_with_parent("查询数据库", &span);
    let account = args.user_account.clone();
    let user_password: Option<(isize, String)> = async_global_executor::spawn_blocking(
        move || -> rusqlite::Result<Option<(isize, String)>> {
            conn.query_row(
                "select user_id,user_password from ld_user where user_account = ?",
                [&account],
                |row| row.try_into(),
            )
            .optional()
        },
    )
    .await?;

    //判断密码是否正确并更新session的授权状态
    let mut authenticated = false;
    let mut userid = 0;
    if let Some((id, pass)) = user_password {
        authenticated = args.password.eq(&pass);
        userid = id as usize;
        req.session_mut().insert("userid", userid)?;
        span.add_property(|| ("database user_password", pass));
    }
    req.session_mut().insert("authenticated", authenticated)?;

    //授权失败
    if !authenticated {
        return Ok(Response::from(StatusCode::Unauthorized));
    }

    let reply = Body::from_json(&Reply { userid })?;
    Ok(Response::builder(StatusCode::Ok).body(reply).build())
}
