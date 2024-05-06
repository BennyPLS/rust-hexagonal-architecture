use sqlite::{ConnectionThreadSafe};

pub mod user_repository_sqlite;

pub fn init() -> Result<ConnectionThreadSafe, sqlite::Error> {
    let conn = sqlite::Connection::open_thread_safe("database.sqlite")?;

    conn.execute(r#"
    CREATE TABLE users (
        id TEXT PRIMARY KEY,
        name TEXT,
        password TEXT,
        email TEXT
    )
    "#)?;

    Ok(conn)
}
