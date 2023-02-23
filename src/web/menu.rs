use anyhow::Result;
use minitrace::Span;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use serde::Serialize;
use tide::prelude::Deserialize;
use tide::{Body, Response, StatusCode};

use crate::web::session::SessionExt;
use crate::web::WebRequest;

#[derive(Default, Deserialize)]
#[serde(default)]
struct MenuReq {
    sys: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct MenuNode {
    pub menu_id: usize,
    pub parent_id: usize,
    pub menu_type: usize,
    pub menu_name: String,
    pub page_id: usize,
}

impl MenuNode {
    fn new(
        menu_id: usize,
        parent_id: usize,
        menu_type: usize,
        menu_name: String,
        page_id: usize,
    ) -> Self {
        MenuNode {
            menu_id,
            parent_id,
            menu_type,
            menu_name,
            page_id,
        }
    }
}

pub(crate) async fn get(req: WebRequest) -> tide::Result {
    let mut span = Span::enter_with_local_parent("接口:菜单查询");
    let userid: usize = match req.session().get("userid") {
        Some(id) => id,
        None => return Ok(Response::from(StatusCode::Unauthorized)),
    };
    let menu_req: MenuReq = req.query()?;

    span.add_properties(|| {
        vec![
            ("sys", menu_req.sys.to_string()),
            ("userid", userid.to_string()),
        ]
    });

    let conn = req.state().pool.get()?;
    let mut span = Span::enter_with_parent("数据库:查询用户菜单", &span);
    let menus = async_global_executor::spawn_blocking(move || query_menu(conn, userid)).await?;
    span.add_properties(|| {
        vec![
            ("查询SQL", format!("select lm.menu_id,lm.parent_id,lm.menu_type,lm.menu_name,lm.page_id from ld_user lu
           left join ld_user_role lur on lu.role_id = lur.role_id and lur.role_type=1
           left join ld_menu lm on lur.privilege_id=lm.menu_id
         where lu.user_id={userid} and lm.menu_status=1")),
            ("返回菜单数", menus.len().to_string()),
        ]
    });

    let body = Body::from_json(&menus)?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

fn query_menu(
    conn: PooledConnection<SqliteConnectionManager>,
    userid: usize,
) -> Result<Vec<MenuNode>> {
    let mut stmt = conn.prepare(
        "select lm.menu_id,lm.parent_id,lm.menu_type,lm.menu_name,lm.page_id from ld_user lu
               left join ld_user_role lur on lu.role_id = lur.role_id and lur.role_type=1
               left join ld_menu lm on lur.privilege_id=lm.menu_id
             where lu.user_id=? and lm.menu_status=1",
    )?;
    let mut rows = stmt.query([&userid])?;

    let mut menus = Vec::<MenuNode>::new();
    while let Some(row) = rows.next()? {
        menus.push(MenuNode::new(
            row.get(0)?,
            row.get(1)?,
            row.get(2)?,
            row.get(3)?,
            row.get(4)?,
        ));
    }

    Ok(menus)
}
