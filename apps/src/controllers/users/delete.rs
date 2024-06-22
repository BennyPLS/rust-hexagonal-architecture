use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::Inject;
use contexts::users::application::delete::{UserDelete, UserDeleteErrors};
use rocket::http::Status;

impl From<UserDeleteErrors> for ProblemDetail {
    fn from(value: UserDeleteErrors) -> Self {
        match value {
            UserDeleteErrors::InternalServerError { source } => {
                let mut err = ProblemDetailBuilder::from(Status::InternalServerError);

                if let Some(source) = source {
                    err = err.detail(source.to_string());
                }

                err.build()
            }
            UserDeleteErrors::UserIDError { source } => {
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build()
            }
        }
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
