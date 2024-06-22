use crate::guard::Json;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::Inject;
use contexts::users::application::update::{UserUpdate, UserUpdateErrors};
use garde::Validate;
use rocket::http::Status;
use rocket::serde::Deserialize;
use uuid::Uuid;

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
            UserUpdateErrors::NotFound => ProblemDetailBuilder::from(Status::NotFound)
                .detail(UserUpdateErrors::NotFound.to_string())
                .build(),
            UserUpdateErrors::UserError { source } => ProblemDetail::from(source),
        }
    }
}

#[put("/", data = "<updated_user>")]
pub fn user_update(
    updated_user: Json<UserUpdateRequest>,
    update_service: Inject<'_, dyn UserUpdate>,
) -> Result<Status, ProblemDetail> {
    let user = updated_user.into_inner();

    update_service.update(&user.uuid.to_string(), user.name, user.password, user.email)?;

    Ok(Status::NoContent)
}
