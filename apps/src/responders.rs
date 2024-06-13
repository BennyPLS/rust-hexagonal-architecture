#[allow(dead_code)]
use std::io::Cursor;

use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use rocket::{Request, Response};
use serde::Serialize;

pub mod problem_detail;

pub struct JsonResponse<T: Serialize> {
    body: T,
    status: Status,
}

impl<T: Serialize> JsonResponse<T> {
    #[allow(dead_code)]
    pub fn new(body: T, status: Status) -> JsonResponse<T> {
        JsonResponse { body, status }
    }

    pub fn ok(body: T) -> JsonResponse<T> {
        JsonResponse {
            body,
            status: Status::Ok,
        }
    }

    #[allow(dead_code)]
    pub fn created(body: T) -> JsonResponse<T> {
        JsonResponse {
            body,
            status: Status::Created,
        }
    }

    #[allow(dead_code)]
    pub fn accepted(body: T) -> JsonResponse<T> {
        JsonResponse {
            body,
            status: Status::Accepted,
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for JsonResponse<T> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self.body).unwrap();

        Response::build()
            .status(self.status)
            .header(ContentType::JSON)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}
