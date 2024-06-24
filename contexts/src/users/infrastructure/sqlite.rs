use crate::shared::domain::criteria::filter::Operator;
use crate::shared::domain::criteria::order::OrderType;

pub mod container;
mod criteria_sqlite;
mod mappers;
mod user_criteria_repository_sqlite;
mod user_repository_sqlite;

const DATABASE_FILE: &str = "database.sqlite";

// language=SQL
const SQL_TABLE_USERS: &str = r#"
CREATE TABLE users (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    password TEXT NOT NULL,
    email TEXT NOT NULL
)"#;

pub const USER_TABLE_NAME: &str = "users";
pub const USER_TABLE_FIELDS: [&'static str; 4] = ["id", "name", "password", "email"];

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

pub trait ToSQLite {
    fn to_sql(&self) -> &'static str;
}

pub const OP_LIKE: [Operator; 2] = [Operator::CO, Operator::NC];

impl ToSQLite for Operator {
    fn to_sql(&self) -> &'static str {
        match &self {
            Operator::EQ => "=",
            Operator::GT => ">",
            Operator::GE => ">=",
            Operator::LT => "<",
            Operator::LE => "<=",
            Operator::CO => "LIKE",
            Operator::NC => "NOT LIKE",
        }
    }
}

impl ToSQLite for OrderType {
    fn to_sql(&self) -> &'static str {
        match &self {
            OrderType::ASC => "ASC",
            OrderType::DESC => "DESC",
        }
    }
}
