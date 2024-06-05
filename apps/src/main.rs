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
    sqlite::init();

    rocket::build()
        .manage(Box::new(build_sqlite_container().build()))
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
        .mount(users::BASE_URL, routes![users::user_register, users::user_get, users::user_get_all, users::user_update, users::user_delete])
}
