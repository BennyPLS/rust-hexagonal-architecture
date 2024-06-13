use std::collections::HashMap;

use rocket::http::Status;
use rocket::Request;

use crate::responders::problem_detail::{ProblemDetail, ProblemDetailBuilder};

#[catch(400)]
pub fn bad_request(req: &Request) -> ProblemDetail {
    let err = req.local_cache::<Option<HashMap<String, serde_json::Value>>, _>(|| None);

    let mut builder = ProblemDetailBuilder::from(Status::BadRequest)
        .detail("The server cannot or will not process the request due to something that is perceived to be a client error");

    if let Some(err) = err {
        builder = builder.extensions(err.clone());
    }

    builder.build()
}

/// Handles a 404 error by returning a JSON response with an error message.
#[catch(404)]
pub fn not_found(req: &Request) -> ProblemDetail {
    ProblemDetailBuilder::from(Status::NotFound)
        .detail(format!(
            "The requested uri '{}' was not found",
            req.uri().path()
        ))
        .build()
}

/// Handles a 409 error by returning a JSON response with an error message.
#[catch(409)]
pub fn conflict(_: &Request) -> ProblemDetail {
    ProblemDetailBuilder::from(Status::Conflict).build()
}

#[catch(413)]
pub fn payload_too_large() -> ProblemDetail {
    ProblemDetailBuilder::from(Status::PayloadTooLarge)
        .detail("Request entity is larger than limits defined by server")
        .build()
}

/// Handles a 422 error by returning a JSON response with an error message.
#[catch(422)]
pub fn unprocessable_entity(req: &Request) -> ProblemDetail {
    let err = req.local_cache::<Option<HashMap<String, serde_json::Value>>, _>(|| None);

    let mut builder = ProblemDetailBuilder::from(Status::UnprocessableEntity)
        .detail("The request was well-formed but was unable to be followed due to semantic errors");

    if let Some(err) = err {
        builder = builder.extensions(err.clone());
    }

    builder.build()
}

#[catch(500)]
pub fn internal_error_server(req: &Request) -> ProblemDetail {
    let err = req.local_cache::<Option<HashMap<String, serde_json::Value>>, _>(|| None);

    let mut builder = ProblemDetailBuilder::from(Status::InternalServerError)
        .detail("The server has encountered a situation it does not know how to handle");

    if let Some(err) = err {
        builder = builder.extensions(err.clone());
    }

    builder.build()
}
