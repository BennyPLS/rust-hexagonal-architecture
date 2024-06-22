use rocket::http::Status;
use contexts::users::application::find::{UserFind, UserFindErrors};
use crate::controllers::users::UserResponse;
use crate::Inject;
use crate::responders::JsonResponse;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};

impl From<UserFindErrors> for ProblemDetail {
    fn from(value: UserFindErrors) -> Self {
        match value {
            UserFindErrors::InternalServerError { source } => {
                let mut err = ProblemDetailBuilder::from(Status::InternalServerError);

                if let Some(source) = source {
                    err = err.detail(source.to_string());
                }

                err.build()
            }
            UserFindErrors::UserIDError { source } => {
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build()
            }
        }
    }
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
    match user_service.find_by(&uuid)? {
        Some(user) => Ok(JsonResponse::ok(UserResponse::from(user))),
        None => Err(ProblemDetail::from(Status::NotFound)),
    }
}