use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::responders::JsonResponse;
use crate::Inject;
use contexts::users::application::delete::{UserDelete, UserDeleteErrors};
use rocket::http::Status;

impl From<UserDeleteErrors> for Box<ProblemDetail> {
    fn from(value: UserDeleteErrors) -> Self {
        match value {
            UserDeleteErrors::InternalServerError { source } => {
                dbg!(source);
                Box::from(ProblemDetail::from(Status::InternalServerError))
            }
            UserDeleteErrors::UserIDError { source } => Box::from(
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build(),
            ),
        }
    }
}

#[delete("/<uuid>")]
pub fn user_delete(
    uuid: String,
    delete_service: Inject<'_, dyn UserDelete>,
) -> Result<JsonResponse<()>, Box<ProblemDetail>> {
    delete_service.delete_by(&uuid)?;

    Ok(JsonResponse::new((), Status::NoContent))
}
