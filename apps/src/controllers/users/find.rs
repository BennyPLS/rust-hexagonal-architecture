use crate::controllers::users::UserResponse;
use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};
use crate::responders::JsonResponse;
use crate::Inject;
use contexts::users::application::find::{UserFind, UserFindErrors, UserListErrors};
use rocket::http::Status;

impl From<UserFindErrors> for Box<ProblemDetail> {
    fn from(value: UserFindErrors) -> Self {
        match value {
            UserFindErrors::InternalServerError { source } => {
                dbg!(source);
                Box::from(ProblemDetail::from(Status::InternalServerError))
            }
            UserFindErrors::UserIDError { source } => Box::from(
                ProblemDetailBuilder::from(Status::UnprocessableEntity)
                    .detail(source.to_string())
                    .build(),
            ),
        }
    }
}

impl From<UserListErrors> for Box<ProblemDetail> {
    fn from(value: UserListErrors) -> Self {
        match value {
            UserListErrors::InternalServerError { source } => {
                dbg!(source);
                Box::from(ProblemDetail::from(Status::InternalServerError))
            }
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
