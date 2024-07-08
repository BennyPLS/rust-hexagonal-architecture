use crate::controllers::users::UserResponse;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::responders::JsonResponse;
use crate::Inject;
use contexts::users::application::find::{UserFind, UserFindErrors};
use rocket::http::Status;

impl From<UserFindErrors> for Box<ProblemDetail> {
    fn from(value: UserFindErrors) -> Self {
        match value {
            UserFindErrors::InternalServerError { source } => {
                let mut err = ProblemDetailBuilder::from(Status::InternalServerError);

                if let Some(source) = source {
                    err = err.detail(source.to_string());
                }

                Box::from(err.build())
            }
            UserFindErrors::UserIDError { source } => Box::from(
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build(),
            ),
        }
    }
}

#[get("/")]
pub fn user_get_all(
    user_service: Inject<'_, dyn UserFind>,
) -> Result<JsonResponse<Vec<UserResponse>>, Box<ProblemDetail>> {
    Ok(JsonResponse::ok(
        user_service
            .get_all()?
            .into_iter()
            .map(UserResponse::from)
            .collect(),
    ))
}

#[get("/<uuid>")]
pub fn user_get(
    uuid: &str,
    user_service: Inject<'_, dyn UserFind>,
) -> Result<JsonResponse<Option<UserResponse>>, Box<ProblemDetail>> {
    Ok(JsonResponse::ok(
        user_service.find_by(uuid)?.map(UserResponse::from),
    ))
}
