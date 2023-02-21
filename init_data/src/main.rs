#[macro_use]
extern crate log;

use log::LevelFilter::Trace;

fn main() -> anyhow::Result<()> {
    pretty_env_logger::formatted_timed_builder()
        .filter_level(Trace)
        .init();

    let conn = rusqlite::Connection::open("sqlite.db")?;
    let files = vec![
        "init_data/sql/create/ld_activity.sql",
        "init_data/sql/create/ld_dict.sql",
        "init_data/sql/create/ld_menu.sql",
        "init_data/sql/create/ld_plan.sql",
        "init_data/sql/create/ld_role.sql",
        "init_data/sql/create/ld_user.sql",
        "init_data/sql/create/ld_user_role.sql",
        "init_data/sql/data/ld_user.sql",
    ];

    for file in files {
        std::fs::read_to_string(file)?.split(';').for_each(|sql| {
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
