use std::panic::resume_unwind;

use rocket::{post, routes};
use rocket::http::Status;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;
use validator::Validate;
use contexts::users::application::delete::UserDelete;

use contexts::users::application::find::UserFind;
use contexts::users::application::register::{UserRegister, UserRegisterErrors};
use contexts::users::application::register::UserRegisterErrors::AlreadyExists;
use contexts::users::application::update::UserUpdate;
use contexts::users::domain::users::{User, UserEmail, UserID, UserName, UserPassword};

use crate::guard::Json;
use crate::Inject;
use crate::responders::JsonResponse;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};

pub const BASE_URL: &str = "/users";

#[derive(Debug, Serialize, Deserialize, Validate, Default)]
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
            uuid: value.get_id().parse().unwrap(),
            name: value.get_name().to_string(),
            plain_password: value.get_password().to_string(),
            email: value.get_email().to_string(),
        }
    }
}

impl Into<User> for UserRequest {
    fn into(self) -> User {
        User::new(
            UserID::new(self.uuid.to_string()),
            UserName::new(self.name),
            UserPassword::new(self.plain_password),
            UserEmail::new(self.email),
        )
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
    JsonResponse::ok(
        user_service
            .get_all()
            .into_iter()
            .map(UserRequest::from)
            .collect(),
    )
}

#[get("/<uuid>")]
pub fn user_get(
    uuid: String,
    user_service: Inject<'_, dyn UserFind>,
) -> Result<JsonResponse<UserRequest>, ProblemDetail> {
    match user_service.find_by(&uuid) {
        Some(user) => Ok(JsonResponse::ok(UserRequest::from(user))),
        None => Err(ProblemDetail::from(Status::NotFound))
    }
}

#[put("/", data = "<updated_user>")]
pub fn user_update(
    updated_user: Json<UserRequest>,
    update_service: Inject<'_, dyn UserUpdate>,
) -> Result<Status, ProblemDetail> {
    let user: User = updated_user.into_inner().into();

    let result = update_service.update(&user);

    match result {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(ProblemDetail::from(Status::InternalServerError)),
    }
}

#[delete("/<uuid>")]
pub fn user_delete(
    uuid: String,
    delete_service: Inject<'_, dyn UserDelete>,
) -> Result<Status, ProblemDetail> {
    let result = delete_service.delete_by(&uuid);

    match result {
        Ok(_) => Ok(Status::NoContent),
        Err(_) => Err(ProblemDetail::from(Status::InternalServerError)),
    }
}