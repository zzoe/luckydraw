#[macro_use]
extern crate log;

use log::LevelFilter::Trace;
use std::path::{Path, PathBuf};
use std::{fs, io};

// one possible implementation of walking a directory and return files
fn visit_dirs(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.append(&mut visit_dirs(&path)?);
            } else {
                files.push(path);
            }
        }
    }
    Ok(files)
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(Trace)
        .init();

    let conn = rusqlite::Connection::open("sqlite.db")?;
    let files = visit_dirs(Path::new("init_data/sql"))?;

    for file in files {
        info!("{file:?}");
        fs::read_to_string(file)?.split(';').for_each(|sql| {
            let sql = sql.trim();
            if sql.is_empty() {
                return;
            }

            info!("{sql}");
            match conn.execute(sql, ()) {
                Ok(effected) => info!("成功: {effected} rows effected\n"),
                Err(err) => error!("失败: {err}\n"),
            }
        });
    }

    Ok(())
}
