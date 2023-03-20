use std::fs::File;
use std::sync::Arc;

use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub(crate) static GLOBAL_CONFIG: Lazy<ArcSwap<Config>> =
    Lazy::new(|| ArcSwap::from_pointee(Config::default()));

/// 初始化配置
pub(crate) fn reload() {
    if let Ok(file) = File::open("config.json") {
        if let Ok(c) = serde_json::from_reader::<File, Config>(file) {
            GLOBAL_CONFIG.store(Arc::new(c));
            return;
        }
    }

    if let Ok(c) = serde_json::to_string_pretty(&Config::default()) {
        std::fs::write("config.json", c).ok();
    }
}

#[derive(Deserialize, Serialize, Default)]
pub(crate) struct Config {
    pub(crate) log: LogCfg,
    pub(crate) web: WebCfg,
    pub(crate) sqlite: SqliteCfg,
}

#[derive(Deserialize, Serialize)]
pub(crate) struct LogCfg {
    pub(crate) directory: String,
    pub(crate) file_name_prefix: String,
    pub(crate) level: String,
}

impl Default for LogCfg {
    fn default() -> Self {
        LogCfg {
            directory: "./logs/".to_owned(),
            file_name_prefix: "lucky-draw.log".to_owned(),
            level: "INFO".to_owned(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct WebCfg {
    pub(crate) address: String,
    pub(crate) cert: String,
    pub(crate) key: String,
}

impl Default for WebCfg {
    fn default() -> Self {
        WebCfg {
            address: "127.0.0.1:1314".to_owned(),
            cert: "cert.pem".to_string(),
            key: "key.pem".to_string(),
        }
    }
}

#[derive(Deserialize, Serialize)]
pub(crate) struct SqliteCfg {
    pub(crate) path: String,
}

impl Default for SqliteCfg {
    fn default() -> Self {
        SqliteCfg {
            path: "sqlite.db".to_owned(),
        }
    }
}
