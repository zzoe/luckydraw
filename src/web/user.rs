use serde::{Deserialize, Serialize};
use tide::{Body, Response, StatusCode};

use crate::web::WebRequest;

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
struct User {
    user_account: String,
    user_nickname: String,
    user_name: String,
    user_phone: usize,
    user_email: String,
    #[serde(skip_deserializing)]
    role_id: usize,
}

pub(crate) async fn get(mut _req: WebRequest) -> tide::Result {
    let mut users = Vec::new();
    for i in 1..101 {
        users.push(User {
            user_account: i.to_string(),
            user_nickname: i.to_string(),
            user_name: i.to_string(),
            user_phone: i,
            user_email: i.to_string(),
            role_id: i,
        });
    }
    let body = Body::from_json(&users)?;
    Ok(Response::builder(StatusCode::Ok).body(body).build())
}
