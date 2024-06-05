use shaku::{Component, Interface};
use sqlite::ConnectionThreadSafe;

pub mod user_repository_sqlite;

pub trait Database: Interface  {
    fn get_connection(&self) -> ConnectionThreadSafe;
}

const DATABASE_FILE: &str = "database.sqlite";

const SQL_TABLE_USERS: &str = r#"
CREATE TABLE users (
    id TEXT PRIMARY KEY,
    name TEXT,
    password TEXT,
    email TEXT
)"#;

pub fn init() {
    sqlite::Connection::open_thread_safe(DATABASE_FILE)
        .expect("Couldn't connect to the database")
        .execute(SQL_TABLE_USERS)
        .expect("Tried to create User Table failed.");
}

#[derive(Component)]
#[shaku(interface = Database)]
pub struct SQLiteDatabase {}

impl Database for SQLiteDatabase {
    fn get_connection(&self) -> ConnectionThreadSafe {
        sqlite::Connection::open_thread_safe(DATABASE_FILE).unwrap()
    }
}
