use arc_swap::access::Access;
use time::macros::format_description;
use time::UtcOffset;
use tracing::Level;
use tracing_appender::non_blocking::WorkerGuard;
use tracing_subscriber::fmt::time::OffsetTime;

use crate::config::{Config, GLOBAL_CONFIG};

mod config;
mod web;

fn main() {
    let _guard = init_log();
    async_global_executor::block_on(web::listen());
}

fn init_log() -> WorkerGuard {
    //初始化配置
    config::reload();
    let log_cfg = GLOBAL_CONFIG.map(|cfg: &Config| &cfg.log).load();

    let file_appender =
        tracing_appender::rolling::daily(&*log_cfg.directory, &*log_cfg.file_name_prefix);
    let (non_blocking, guard) = tracing_appender::non_blocking(file_appender);
    let time_format =
        format_description!("[year]-[month]-[day] [hour]:[minute]:[second].[subsecond digits:3]");

    tracing_subscriber::fmt()
        .with_ansi(false)
        // .with_thread_ids(true)
        .with_max_level(log_cfg.level.parse::<Level>().expect("日志级别配置错误"))
        .with_timer(OffsetTime::new(
            UtcOffset::from_hms(8, 0, 0).unwrap(),
            time_format,
        ))
        .with_writer(non_blocking)
        .init();

    guard
}
