use shaku::{Component, Interface};
use sqlite::ConnectionThreadSafe;

pub mod container;
mod user_repository_sqlite;
mod mappers;

const DATABASE_FILE: &str = "database.sqlite";

// language=SQL
const SQL_TABLE_USERS: &str = r#"
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL
)"#;

pub fn init() {
    let result = sqlite::Connection::open_thread_safe(DATABASE_FILE)
        .expect("Couldn't connect to the database")
        .execute(SQL_TABLE_USERS);

    if let Err(err) = result {
        if err.code.unwrap() != 1 {
            panic!("Database couldn't be initialized.")
        }
    }
}
