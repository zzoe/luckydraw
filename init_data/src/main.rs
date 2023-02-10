fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    let conn = rusqlite::Connection::open("sqlite.db")?;

    match conn.execute(
        &std::fs::read_to_string("init_data/sql/tables/ld_user.sql")?,
        (),
    ) {
        Ok(effected) => println!("{effected} rows were effected"),
        Err(err) => println!("execute failed: {err}"),
    }

    Ok(())
}
