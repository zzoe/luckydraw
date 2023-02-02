use minitrace::Span;
use serde::Serialize;
use tide::prelude::Deserialize;
use tide::{Body, Response, StatusCode};

use crate::web::WebRequest;

#[derive(Default, Deserialize)]
#[serde(default)]
struct MenuReq {
    sys: usize,
    userid: usize,
}

#[derive(Clone, Debug, Default, Serialize)]
pub struct MenuNode {
    pub menu_id: u32,
    pub parent_id: u32,
    pub menu_type: u32,
    pub menu_name: String,
    pub func_id: u32,
}

pub(crate) async fn get(req: WebRequest) -> tide::Result {
    let mut span = Span::enter_with_local_parent("menu");
    let menu_req: MenuReq = req.query()?;
    span.add_properties(|| {
        vec![
            ("sys", menu_req.sys.to_string()),
            ("userid", menu_req.userid.to_string()),
        ]
    });

    let body = Body::from_json(&mock_menu())?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

fn mock_menu() -> Vec<MenuNode> {
    vec![
        MenuNode {
            menu_id: 1,
            parent_id: 0,
            menu_type: 0,
            menu_name: "系统管理".to_string(),
            func_id: 0,
        },
        MenuNode {
            menu_id: 2,
            parent_id: 1,
            menu_type: 2,
            menu_name: "用户管理".to_string(),
            func_id: 1001,
        },
        MenuNode {
            menu_id: 3,
            parent_id: 1,
            menu_type: 2,
            menu_name: "角色管理".to_string(),
            func_id: 1002,
        },
        MenuNode {
            menu_id: 4,
            parent_id: 0,
            menu_type: 0,
            menu_name: "XX中心".to_string(),
            func_id: 0,
        },
    ]
}
