use rocket::{post, routes};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use contexts::users::application::user_register_service::UserRegister;

use crate::guard::Json;
use crate::Inject;
use crate::responders::JsonResponse;

pub const BASE_URL: &str = "/users";

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct UserRequest {
    uuid: Uuid,
    #[validate(length(min = 10))]
    name: String,
    #[serde(rename = "password")]
    plain_password: String,
    #[validate(email)]
    email: String,
}

#[post("/register", data = "<new_user>")]
pub fn user_register(new_user: Json<UserRequest>, register_service: Inject<'_, dyn UserRegister>) -> JsonResponse<String> {
    let user = new_user.into_inner();

    register_service.register(user.uuid.to_string(), user.name, user.plain_password, user.email);

    JsonResponse::created(String::from("User Created"))
}
