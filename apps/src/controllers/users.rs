mod criteria;
mod delete;
mod find;
mod register;
mod update;

pub use delete::user_delete;
pub use find::{user_get, user_get_all};
pub use register::user_register;
pub use update::user_update;

use contexts::users::application::delete::UserDelete;
use contexts::users::application::find::UserFind;
use contexts::users::application::register::UserRegister;
use contexts::users::application::update::UserUpdate;
use contexts::users::domain::users::{User, UserErrors};
use garde::Validate;
use rocket::http::Status;
use serde::{Deserialize, Serialize};

use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};

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
    password: &'a str,
    #[garde(email)]
    email: &'a str,
}

#[derive(Debug, Serialize)]
struct UserResponse {
    uuid: String,
    name: String,
    #[serde(rename = "password")]
    password: String,
    email: String,
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
