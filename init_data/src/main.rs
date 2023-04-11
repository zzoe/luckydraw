#[macro_use]
extern crate log;

use std::path::{Path, PathBuf};
use std::{fs, io};

use log::LevelFilter::Trace;

// one possible implementation of walking a directory and return files
fn visit_dir(dir: &Path) -> io::Result<Vec<PathBuf>> {
    let mut files = Vec::new();
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                files.append(&mut visit_dir(&path)?);
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
    let mut files = visit_dir(Path::new("init_data/sql/table"))?;
    files.append(&mut visit_dir(Path::new("init_data/sql/data"))?);

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
