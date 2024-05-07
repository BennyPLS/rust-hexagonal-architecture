use std::error::Error;

use sqlite::ConnectionThreadSafe;

pub mod user_repository_sqlite;

pub fn init() -> Result<ConnectionThreadSafe, sqlite::Error> {
    let conn = sqlite::Connection::open_thread_safe("database.sqlite")?;

    let result = conn.execute(
        r#"
        CREATE TABLE users (
        id TEXT PRIMARY KEY,
        name TEXT,
        password TEXT,
        email TEXT
    )"#,
    );

    if let Err(err) = result {
        if err.code != Some(1) {
            return Err(err);
        }
    }

    Ok(conn)
}

trait FromRow