use std::fmt::Debug;
use std::iter::repeat_with;

use anyhow::Result;
use arc_swap::access::Access;
use async_session::MemoryStore;
use r2d2::Pool;
use r2d2_sqlite::SqliteConnectionManager;
use tide::{Request, Server};
use tide_rustls::TlsListener;
use time::Duration;

use crate::config::{Config, GLOBAL_CONFIG};

pub(crate) mod auth;
pub(crate) mod log_ext;
pub(crate) mod menu;
pub(crate) mod session;
pub(crate) mod static_file;
pub(crate) mod user;

#[derive(Clone, Debug)]
pub(crate) struct WebState {
    pub(crate) pool: Pool<SqliteConnectionManager>,
}

impl Default for WebState {
    fn default() -> Self {
        let sqlite_cfg = GLOBAL_CONFIG.map(|cfg: &Config| &cfg.sqlite).load();
        let sqlite = SqliteConnectionManager::file(&*sqlite_cfg.path);
        WebState {
            pool: Pool::new(sqlite).unwrap(),
        }
    }
}

pub(crate) type WebServer = Server<WebState>;
pub(crate) type WebRequest = Request<WebState>;

pub(crate) async fn listen() {
    let app = new(WebState::default()).unwrap();

    let web_cfg = GLOBAL_CONFIG.map(|cfg: &Config| &cfg.web).load();
    let listener = TlsListener::build()
        .addrs(&*web_cfg.address)
        .cert(&*web_cfg.cert)
        .key(&*web_cfg.key);
    if let Err(e) = app.listen(listener).await {
        if e.kind() == std::io::ErrorKind::NotFound {
            println!(
                "install https://github.com/FiloSottile/mkcert
$ mkcert -key-file key.pem -cert-file cert.pem localhost 127.0.0.1 ::1"
            );
        }
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
    api.at("/user").get(user::get);
    api.at("/user").post(user::get);
    api.at("/user").delete(user::get);

    let mut static_file = tide::with_state(app.state().clone());
    static_file.at("*").get(static_file::get);

    app.at("/api").nest(api);
    app.at("/").nest(static_file);
    app.at("/").get(static_file::get);

    app
}
