use std::io::Cursor;

use rocket::{Request, Response};
use rocket::http::Status;
use rocket::response::Responder;
use serde::Serialize;

pub mod problem_detail;

pub struct JsonResponse<T: Serialize> {
    body: T,
    status: Status,
}

impl<T: Serialize + Default> JsonResponse<T> {
    pub fn new(body: T, status: Status) -> JsonResponse<T> {
        JsonResponse { body, status }
    }

    pub fn empty_body(status: Status) -> JsonResponse<T> {
        JsonResponse {
            body: T::default(),
            status,
        }
    }

    pub fn ok(body: T) -> JsonResponse<T> {
        JsonResponse {
            body,
            status: Status::Ok,
        }
    }

    pub fn created(body: T) -> JsonResponse<T> {
        JsonResponse {
            body,
            status: Status::Created,
        }
    }

    pub fn accepted(body: T) -> JsonResponse<T> {
        JsonResponse {
            body,
            status: Status::Accepted,
        }
    }

    pub fn no_content() -> JsonResponse<String> {
        JsonResponse {
            body: String::new(),
            status: Status::NoContent,
        }
    }
}

impl<'r, T: Serialize> Responder<'r, 'static> for JsonResponse<T> {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let json = serde_json::to_string(&self.body).unwrap();

        Response::build()
            .status(self.status)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}
