use rocket::{post, routes, Route};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

use crate::controllers::guard::Json;
use crate::controllers::responders::problem_detail::ProblemDetail;
use crate::controllers::responders::JsonResponse;

pub const BASE_URL: &str = "/users";

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct User {
    uuid: Uuid,
    #[validate(length(min = 10))]
    name: String,
    #[serde(rename = "password")]
    plain_password: String,
    #[validate(email)]
    email: String,
}

#[post("/register", data = "<new_user>")]
pub fn user_register(new_user: Json<User>) -> Result<JsonResponse<String>, Box<ProblemDetail>> {
    let user = new_user.into_inner();

    Ok(JsonResponse::created(String::from("User Created")))
}
