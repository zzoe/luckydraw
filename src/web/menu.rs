use anyhow::Result;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;
use tide::prelude::Deserialize;
use tide::{Body, Response, StatusCode};
use tracing::{debug, info, info_span, Span};

use crate::web::session::SessionExt;
use crate::web::WebRequest;

#[derive(Default, Deserialize)]
#[serde(default)]
struct MenuReq {
    sys: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct Menu {
    pub menu_id: usize,
    pub parent_id: usize,
    pub menu_type: usize,
    pub menu_name: String,
    pub page_id: usize,
}

impl Menu {
    fn new(
        menu_id: usize,
        parent_id: usize,
        menu_type: usize,
        menu_name: String,
        page_id: usize,
    ) -> Self {
        Menu {
            menu_id,
            parent_id,
            menu_type,
            menu_name,
            page_id,
        }
    }
}

pub(crate) async fn get(req: WebRequest) -> tide::Result {
    let userid: usize = match req.session().get("userid") {
        Some(id) => id,
        None => return Ok(Response::from(StatusCode::Unauthorized)),
    };
    let menu_req: MenuReq = req.query()?;

    info!("sys: {}, userid: {}", menu_req.sys, userid);

    let conn = req.state().pool.get()?;
    let span = info_span!(parent: Span::current(), "查询用户菜单").or_current();
    let menus =
        async_global_executor::spawn_blocking(move || query_menu(span, conn, userid)).await?;

    let body = Body::from_json(&menus)?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

fn query_menu(
    span: Span,
    conn: PooledConnection<SqliteConnectionManager>,
    userid: usize,
) -> Result<Vec<Menu>> {
    let _enter = span.enter();
    info!(
        "select lm.menu_id,lm.parent_id,lm.menu_type,lm.menu_name,lm.page_id from ld_user lu
           left join ld_user_role lur on lu.role_id = lur.role_id and lur.role_type=1
           left join ld_menu lm on lur.privilege_id=lm.menu_id
         where lu.user_id={userid} and lm.menu_status=1"
    );
    let mut stmt = conn.prepare(
        "select lm.menu_id,lm.parent_id,lm.menu_type,lm.menu_name,lm.page_id from ld_user lu
               left join ld_user_role lur on lu.role_id = lur.role_id and lur.role_type=1
               left join ld_menu lm on lur.privilege_id=lm.menu_id
             where lu.user_id=? and lm.menu_status=1",
    )?;
    let mut rows = stmt.query([&userid])?;

    let mut menus = Vec::new();
    while let Some(row) = rows.next()? {
        menus.push(Menu::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ));
    }
    debug!("结果： {menus:#?}");

    Ok(menus)
}
