use minitrace::trace;
use serde::Deserialize;
use tide::{Response, StatusCode};

use crate::web::WebRequest;
use rusqlite::Connection;

#[derive(Deserialize)]
struct Args {
    url: Option<String>,
    statement: String,
    max_rows: Option<usize>,
}

pub(crate) async fn query(mut req: WebRequest) -> tide::Result {
    let args = req.body_json::<Args>().await?;

    let reply = match async_global_executor::spawn_blocking(|| search(args)).await {
        Ok(o) => o,
        Err(e) => e.to_string(),
    };

    let mut res = Response::new(StatusCode::Ok);
    res.set_body(reply);
    Ok(res)
}

#[trace("search")]
fn search(args: Args) -> anyhow::Result<String> {
    const SEPARATOR: char = '\t';
    const LINE_END: char = '\n';
    let url = args
        .url
        .as_ref()
        .map_or("./temporary/temp.db", |s| s.as_str());
    let conn = Connection::open(url)?;
    let mut stmt = conn.prepare(&*args.statement)?;
    let mut res = String::new();

    let col_idx = stmt.column_count() - 1;
    for col in 0..col_idx {
        res.push_str(stmt.column_name(col)?);
        res.push(SEPARATOR);
    }
    res.push_str(stmt.column_name(col_idx)?);
    res.push(LINE_END);

    let max_rows = args.max_rows.unwrap_or(100);
    let mut i = 1;

    let mut rows = stmt.query([])?;
    while let Some(row) = rows.next()? {
        if i > max_rows {
            res.push_str("WARN: 超出最大行数");
            break;
        }

        for i in 0..col_idx {
            res.push_str(&*row.get::<usize, String>(i)?);
            res.push(SEPARATOR);
        }
        res.push_str(&*row.get::<usize, String>(col_idx)?);
        res.push(LINE_END);
        i += 1;
    }

    Ok(res)
}
