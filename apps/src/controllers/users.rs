use std::panic::resume_unwind;

use contexts::users::application::find::UserFind;
use rocket::http::Status;
use rocket::{post, routes};
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;

use contexts::users::application::register::UserRegisterErrors::AlreadyExists;
use contexts::users::application::register::{UserRegister, UserRegisterErrors};
use contexts::users::domain::users::User;

use crate::guard::Json;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::responders::JsonResponse;
use crate::Inject;

pub const BASE_URL: &str = "/users";

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRequest {
    uuid: Uuid,
    #[validate(length(min = 10))]
    name: String,
    #[serde(rename = "password")]
    plain_password: String,
    #[validate(email)]
    email: String,
}

impl From<User> for UserRequest {
    fn from(value: User) -> Self {
        UserRequest {
            uuid: value.get_id(),
        }
    }
}

#[post("/register", data = "<new_user>")]
pub fn user_register(
    new_user: Json<UserRequest>,
    register_service: Inject<'_, dyn UserRegister>,
) -> Result<Status, ProblemDetail> {
    let user = new_user.into_inner();

    let result = register_service.register(
        user.uuid.to_string(),
        user.name,
        user.plain_password,
        user.email,
    );

    if let Err(err) = result {
        return if let AlreadyExists = err {
            Err(ProblemDetailBuilder::from(Status::Conflict)
                .detail("The uuid for the user trying to register, is already registered.")
                .add_extension("uuid", json!(user.uuid))
                .build())
        } else {
            Err(ProblemDetail::from(Status::InternalServerError))
        };
    }

    Ok(Status::Created)
}

#[get("/")]
pub fn user_get_all(user_service: Inject<'_, dyn UserFind>) -> JsonResponse<Vec<UserRequest>> {
    user_service.get_all()
}
