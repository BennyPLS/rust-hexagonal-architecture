#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};

use contexts::shared::infrastructure::dependency_container::{default_module, DependencyContainer};

use crate::controllers::users;
pub type Inject<'r, I> = shaku_rocket::Inject<'r, DependencyContainer, I>;

mod controllers;
mod guard;
mod handlers;
mod responders;

#[launch]
async fn rocket() -> Rocket<Build> {
    rocket::build()
        .manage(Box::new(default_module().await.build()))
        .register(
            "/",
            catchers![
                handlers::not_found,
                handlers::bad_request,
                handlers::payload_too_large,
                handlers::unprocessable_entity,
                handlers::internal_error_server
            ],
        )
        .mount(users::BASE_URL, routes![users::user_register])
}
