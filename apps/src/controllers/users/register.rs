use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use contexts::users::application::register::{UserRegister, UserRegisterErrors};
use contexts::users::application::register::UserRegisterErrors::AlreadyExists;
use rocket::http::Status;
use crate::controllers::users::UserRequest;
use crate::guard::Json;
use crate::Inject;

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