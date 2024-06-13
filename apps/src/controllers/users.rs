use garde::Validate;
use rocket::http::Status;
use rocket::post;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use contexts::users::application::delete::UserDelete;
use contexts::users::application::find::UserFind;
use contexts::users::application::register::UserRegisterErrors::AlreadyExists;
use contexts::users::application::register::{UserRegister, UserRegisterErrors};
use contexts::users::application::update::{UserUpdate, UserUpdateErrors};
use contexts::users::domain::users::{User, UserErrors};

use crate::guard::Json;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::responders::JsonResponse;
use crate::Inject;

pub const BASE_URL: &str = "/users";

#[derive(Debug, Deserialize, Validate, Default)]
#[garde(allow_unvalidated)]
pub struct UserRequest<'a> {
    // language=RegExp
    #[garde(pattern(
        r"^[0-9a-fA-F]{8}-[0-9a-fA-F]{4}-7[0-9a-fA-F]{3}-[0-9a-fA-F]{4}-[0-9a-fA-F]{12}$"
    ))]
    uuid: &'a str,
    #[garde(length(chars, min = 8))]
    name: &'a str,
    // language=RegExp
    #[garde(length(chars, min = 8), pattern(r"\d.*[\W_]|[\W_].*\d"))]
    #[serde(rename = "password")]
    pub(crate) password: &'a str,
    #[garde(email)]
    email: &'a str,
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    uuid: String,
    name: String,
    #[serde(rename = "password")]
    password: String,
    email: String,
}

#[derive(Debug, Deserialize, Validate, Default)]
pub struct UserUpdateRequest<'a> {
    #[garde(skip)]
    uuid: Uuid,
    #[garde(skip)]
    name: Option<&'a str>,
    #[garde(skip)]
    password: Option<&'a str>,
    #[garde(skip)]
    email: Option<&'a str>,
}

impl From<User> for UserResponse {
    fn from(value: User) -> Self {
        UserResponse {
            uuid: value.get_id(),
            name: value.get_name().to_owned(),
            password: value.get_password().to_owned(),
            email: value.get_email().to_owned(),
        }
    }
}

impl From<UserErrors> for ProblemDetail {
    fn from(value: UserErrors) -> Self {
        match value {
            UserErrors::UserIDError { source } => {
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build()
            }
            UserErrors::UserNameError { source } => {
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build()
            }
            UserErrors::UserPasswordError { source } => {
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build()
            }
            UserErrors::UserEmailError { source } => {
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build()
            }
        }
    }
}

impl From<UserRegisterErrors> for ProblemDetail {
    fn from(value: UserRegisterErrors) -> Self {
        match value {
            AlreadyExists => ProblemDetailBuilder::from(Status::Conflict)
                .detail("The uuid for the user trying to register, is already registered.")
                .build(),
            UserRegisterErrors::InternalServerError { source } => {
                let mut err = ProblemDetailBuilder::from(Status::InternalServerError);

                if let Some(source) = source {
                    err = err.detail(source.to_string());
                }

                err.build()
            }
            UserRegisterErrors::UserError { source } => ProblemDetail::from(source),
        }
    }
}

impl From<UserUpdateErrors> for ProblemDetail {
    fn from(value: UserUpdateErrors) -> Self {
        match value {
            UserUpdateErrors::InternalServerError { source } => {
                let mut err = ProblemDetailBuilder::from(Status::InternalServerError);

                if let Some(source) = source {
                    err = err.detail(source.to_string());
                }

                err.build()
            }
            UserUpdateErrors::UserError { source } => ProblemDetail::from(source),
            UserUpdateErrors::NotFound => ProblemDetailBuilder::from(Status::NotFound)
                .detail(UserUpdateErrors::NotFound.to_string())
                .build(),
        }
    }
}

#[post("/register", data = "<new_user>")]
pub fn user_register(
    new_user: Json<UserRequest>,
    register_service: Inject<'_, dyn UserRegister>,
) -> Result<Status, ProblemDetail> {
    let user = new_user.into_inner();

    register_service.register(
        &user.uuid.to_string(),
        &user.name,
        &user.password,
        &user.email,
    )?;

    Ok(Status::Created)
}

#[get("/")]
pub fn user_get_all(user_service: Inject<'_, dyn UserFind>) -> JsonResponse<Vec<UserResponse>> {
    JsonResponse::ok(
        user_service
            .get_all()
            .into_iter()
            .map(UserResponse::from)
            .collect(),
    )
}

#[get("/<uuid>")]
pub fn user_get(
    uuid: String,
    user_service: Inject<'_, dyn UserFind>,
) -> Result<JsonResponse<UserResponse>, ProblemDetail> {
    match user_service.find_by(&uuid) {
        Some(user) => Ok(JsonResponse::ok(UserResponse::from(user))),
        None => Err(ProblemDetail::from(Status::NotFound)),
    }
}

#[put("/", data = "<updated_user>")]
pub fn user_update(
    updated_user: Json<UserUpdateRequest>,
    update_service: Inject<'_, dyn UserUpdate>,
) -> Result<Status, ProblemDetail> {
    let user = updated_user.into_inner();

    update_service.update(
        &user.uuid.to_string(),
        user.name,
        user.password,
        user.email,
    )?;

    Ok(Status::NoContent)
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
