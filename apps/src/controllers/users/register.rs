use crate::controllers::users::UserRequest;
use crate::guard::Json;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::responders::JsonResponse;
use crate::Inject;
use contexts::users::application::register::{UserRegister, UserRegisterErrors};
use rocket::http::Status;

impl From<UserRegisterErrors> for Box<ProblemDetail> {
    fn from(value: UserRegisterErrors) -> Self {
        match value {
            UserRegisterErrors::InternalServerError { source } => {
                dbg!(source);
                Box::from(ProblemDetail::from(Status::InternalServerError))
            }
            UserRegisterErrors::AlreadyExists => Box::from(
                ProblemDetailBuilder::from(Status::Conflict)
                    .detail("The uuid for the user trying to register, is already registered.")
                    .build(),
            ),
            UserRegisterErrors::UserError { source } => Box::from(ProblemDetail::from(source)),
        }
    }
}

#[post("/register", data = "<new_user>")]
pub fn user_register(
    new_user: Json<UserRequest>,
    register_service: Inject<'_, dyn UserRegister>,
) -> Result<JsonResponse<()>, Box<ProblemDetail>> {
    let user = new_user.into_inner();

    register_service.register(
        &user.uuid.to_string(),
        &user.name,
        &user.password,
        &user.email,
    )?;

    Ok(JsonResponse::created(()))
}
