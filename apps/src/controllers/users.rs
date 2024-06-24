mod criteria;
mod delete;
mod find;
mod register;
mod update;

pub use criteria::user_criteria;
pub use delete::user_delete;
pub use find::{user_get, user_get_all};
pub use register::user_register;
pub use update::user_update;

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
pub struct UserResponse {
    uuid: String,
    name: String,
    #[serde(rename = "password")]
    password: String,
    email: String,
}

impl From<User<'_>> for UserResponse {
    fn from(value: User) -> Self {
        let (uuid, name, password, email) = value.into_inners();
        UserResponse {
            uuid,
            name,
            password,
            email,
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
