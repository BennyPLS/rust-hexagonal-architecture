#[macro_use]
extern crate rocket;

use rocket::{Build, launch, Rocket};

use crate::controllers::users as api_users;

mod controllers;
mod domain;
mod repositories;
mod services;

#[launch]
fn rocket() -> Rocket<Build> {
    rocket::build()
        .register(
            "/",
            catchers![
                controllers::handlers::not_found,
                controllers::handlers::bad_request,
                controllers::handlers::payload_too_large,
                controllers::handlers::unprocessable_entity,
                controllers::handlers::internal_error_server
            ],
        )
        .mount(api_users::BASE_URL, routes![api_users::user_register])
}
