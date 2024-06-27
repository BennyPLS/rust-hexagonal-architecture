mod models;
mod schema;

use dotenvy::dotenv;
use lazy_static::lazy_static;
use std::env;

lazy_static! {
    static ref DataBaseUrl: String = get_database_url();
}

fn get_database_url() -> String {
    dotenv().expect("The file '.env' couldn't be loaded");

    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}
