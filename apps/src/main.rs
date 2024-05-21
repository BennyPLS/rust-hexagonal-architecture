#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

use contexts::shared::infrastructure::dependency_container::{
    build_sqlite_container, SQLiteImplementation,
};

use crate::controllers::users;

use contexts::users::infrastructure::sqlite;

pub type Inject<'r, I> = shaku_rocket::Inject<'r, SQLiteImplementation, I>;

mod controllers;
mod guard;
mod handlers;
mod responders;

#[launch]
async fn rocket() -> Rocket<Build> {
    let conn = sqlite::init().expect("Couldn't initialize the database.");

    rocket::build()
        .manage(Box::new(build_sqlite_container(conn).build()))
        .register(
            "/",
            catchers![
                handlers::not_found,
                handlers::bad_request,
                handlers::conflict,
                handlers::payload_too_large,
                handlers::unprocessable_entity,
                handlers::internal_error_server,
            ],
        )
        .mount(users::BASE_URL, routes![users::user_register])
}
