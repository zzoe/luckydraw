use std::fmt::Debug;
use std::iter::repeat_with;

use anyhow::Result;
use async_session::MemoryStore;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tide::{Request, Server};
use tide_rustls::TlsListener;
use time::Duration;

pub(crate) mod auth;
pub(crate) mod log_ext;
pub(crate) mod menu;
pub(crate) mod session;
pub(crate) mod static_file;

#[derive(Clone, Debug)]
pub(crate) struct WebState {
    pub(crate) pool: Pool<SqliteConnectionManager>,
}

impl Default for WebState {
    fn default() -> Self {
        let sqlite = SqliteConnectionManager::file("sqlite.db");
        WebState {
            pool: Pool::new(sqlite).unwrap(),
        }
    }
}

pub(crate) type WebServer = Server<WebState>;
pub(crate) type WebRequest = Request<WebState>;

pub(crate) async fn listen() {
    let app = new(WebState::default()).unwrap();
    let address = std::env::var("LUCK_DRAW_ADDRESS").unwrap_or_else(|_| "0.0.0.0:1314".to_string());
    let listener = TlsListener::build()
        .addrs(address)
        .cert("cert.pem")
        .key("key.pem");
    if let Err(e) = app.listen(listener).await {
        eprintln!("app listen fail: {e}");
    }
}

fn new(state: WebState) -> Result<WebServer> {
    let secret = repeat_with(|| fastrand::u8(..))
        .take(64)
        .collect::<Vec<u8>>();
    let session = session::SessionMiddleware::new(MemoryStore::default(), &secret)
        .with_session_ttl(Some(Duration::seconds(3600)));

    let mut app = tide::with_state(state);

    // log
    app.with(log_ext::LogMiddleware::new());
    // session
    app.with(session);
    // authentication
    app.with(auth::Authentication::new());
    // login
    app.at("/login").post(auth::login);

    Ok(route(app))
}

pub(crate) fn route(mut app: Server<WebState>) -> Server<WebState> {
    //数据库
    // app.at("/api/sqlite/query").post(sqlite::query);

    let mut api = tide::with_state(app.state().clone());
    api.at("/menu").get(menu::get);

    let mut static_file = tide::with_state(app.state().clone());
    static_file.at("*").get(static_file::get);

    app.at("/api").nest(api);
    app.at("/").nest(static_file);
    app.at("/").get(static_file::get);

    app
}
