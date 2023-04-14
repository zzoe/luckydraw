use anyhow::Result;
use r2d2::PooledConnection;
use r2d2_sqlite::SqliteConnectionManager;
use rusqlite::params_from_iter;
use rusqlite::types::ToSqlOutput;
use serde::{Deserialize, Serialize};
use tide::{Body, Response, StatusCode};
use tracing::{debug, info, info_span, Span};
use util::JsonDisplay;

use crate::web::WebRequest;

#[derive(Clone, Debug, Default, Deserialize, Serialize, JsonDisplay)]
#[serde(default)]
struct User {
    user_account: String,
    user_nickname: String,
    user_name: String,
    user_phone: i64,
    user_email: String,
    #[serde(skip_deserializing)]
    role_id: usize,
}

impl User {
    // Convert User to iterator over (field_name, field_value) pairs
    fn fields(&self) -> impl Iterator<Item = (&'static str, ToSqlOutput)> {
        let mut fields = Vec::new();

        if !self.user_account.is_empty() {
            fields.push(("user_account", ToSqlOutput::from(&*self.user_account)));
        }
        if !self.user_nickname.is_empty() {
            fields.push(("user_nickname", ToSqlOutput::from(&*self.user_nickname)));
        }
        if !self.user_name.is_empty() {
            fields.push(("user_name", ToSqlOutput::from(&*self.user_name)));
        }
        if self.user_phone != 0 {
            fields.push(("user_phone", ToSqlOutput::from(self.user_phone)));
        }
        if !self.user_email.is_empty() {
            fields.push(("user_email", ToSqlOutput::from(&*self.user_email)));
        }

        fields.into_iter()
    }
}

pub(crate) async fn get(req: WebRequest) -> tide::Result {
    let user: User = req.query()?;
    info!("user: {user}");

    let conn = req.state().pool.get()?;
    let span = info_span!(parent: Span::current(), "查询用户").or_current();

    let users =
        async_global_executor::spawn_blocking(move || query_user(span, &conn, &user)).await?;
    let body = Body::from_json(&users)?;

    Ok(Response::builder(StatusCode::Ok).body(body).build())
}

fn query_user(
    span: Span,
    conn: &PooledConnection<SqliteConnectionManager>,
    user: &User,
) -> Result<Vec<User>> {
    let _enter = span.enter();

    // Use a vector to store the WHERE clauses and corresponding parameters
    let mut where_clauses = Vec::new();
    let mut params = Vec::new();
    for (name, value) in user.fields() {
        where_clauses.push(format!("lu.{} = ?", name));
        params.push(value);
    }

    // Append WHERE clause to SQL query if necessary
    let sql = format!(
        "SELECT lu.user_account, lu.user_nickname, lu.user_name, lu.user_phone, lu.user_email, lu.role_id FROM ld_user lu {}",
        if where_clauses.is_empty() {
            "".to_string()
        } else {
            format!("WHERE {}", where_clauses.join(" AND "))
        }
    );

    info!("sql: {}", sql);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params_from_iter(params))?;

    let mut users = Vec::new();
    while let Some(row) = rows.next()? {
        users.push(User {
            user_account: row.get(0)?,
            user_nickname: row.get(1)?,
            user_name: row.get(2)?,
            user_phone: row.get(3)?,
            user_email: row.get(4)?,
            role_id: row.get(5)?,
        });
    }

    debug!("结果: {users:#?}");

    Ok(users)
}
