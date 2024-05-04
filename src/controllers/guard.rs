use std::collections::HashMap;

use rocket::data::{FromData, Limits, Outcome};
use rocket::http::Status;
use rocket::request::local_cache;
use rocket::{Data, Request};
use serde::Deserialize;
use serde_json::error::Category;
use serde_json::json;
use thiserror::Error;
use validator::{Validate, ValidationErrors};

#[derive(Debug, Error)]
pub enum JsonValidationError {
    #[error("Validation failed")]
    Validation {
        #[from]
        source: ValidationErrors,
    },
    #[error("Body empty")]
    EmptyBody,
    #[error("Parsing failed")]
    Parse {
        #[from]
        source: serde_json::Error,
    },
    #[error("Payload too large")]
    TooLarge,
    #[error("IO error")]
    IO {
        #[from]
        source: std::io::Error,
    },
}

pub enum JsonGuardErrors<'a> {
    ValidationError(&'a ValidationErrors),
    BodyEmpty,
    ParseError(&'a serde_json::Error),
    IO(&'a std::io::Error),
}

impl<'a> JsonGuardErrors<'a> {
    pub fn get_problem_detail_extensions(&self) -> HashMap<String, serde_json::Value> {
        let mut extensions = HashMap::new();

        match self {
            JsonGuardErrors::ValidationError(errors) => {
                for (field, field_errors) in errors.field_errors() {
                    for err in field_errors {
                        extensions.insert(format!("{}_{}", field, err.code), json!(err.params));
                    }
                }
            }
            JsonGuardErrors::ParseError(serde_error) => {
                extensions.insert("parse_error".to_string(), json!(serde_error.to_string()));
            }
            JsonGuardErrors::BodyEmpty => {
                extensions.insert("parse_error".to_string(), json!("Body empty"));
            }
            JsonGuardErrors::IO(io) => {
                extensions.insert("io_error".to_string(), json!(io.kind().to_string()));
            }
        }

        extensions
    }
}

#[derive(Debug)]
pub struct Json<T>(pub T);

impl<T> Json<T> {
    pub fn into_inner(self) -> T {
        self.0
    }
}

#[rocket::async_trait]
impl<'r, T> FromData<'r> for Json<T>
where
    T: Deserialize<'r> + Validate,
{
    type Error = JsonValidationError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> Outcome<'r, Self> {
        let limit = req.limits().get("json").unwrap_or(Limits::JSON);

        let string = match data.open(limit).into_string().await {
            Err(error) => {
                req.local_cache(|| {
                    Some(JsonGuardErrors::IO(&error).get_problem_detail_extensions())
                });
                return Outcome::Error((
                    Status::InternalServerError,
                    JsonValidationError::IO { source: error },
                ));
            }
            Ok(cap_string) if cap_string.is_complete() => cap_string.into_inner(),
            Ok(_) => {
                return Outcome::Error((Status::PayloadTooLarge, JsonValidationError::TooLarge));
            }
        };

        let string = local_cache!(req, string);

        if string.is_empty() {
            req.local_cache(|| Some(JsonGuardErrors::BodyEmpty.get_problem_detail_extensions()));
            return Outcome::Error((Status::BadRequest, JsonValidationError::EmptyBody));
        }

        match serde_json::from_str::<T>(string) {
            Err(error) => match error.classify() {
                Category::Data => {
                    req.local_cache(|| {
                        Some(JsonGuardErrors::ParseError(&error).get_problem_detail_extensions())
                    });
                    Outcome::Error((
                        Status::UnprocessableEntity,
                        JsonValidationError::Parse { source: error },
                    ))
                }
                _ => {
                    req.local_cache(|| {
                        Some(JsonGuardErrors::ParseError(&error).get_problem_detail_extensions())
                    });
                    Outcome::Error((
                        Status::BadRequest,
                        JsonValidationError::Parse { source: error },
                    ))
                }
            },
            Ok(t) => match t.validate() {
                Err(error) => {
                    req.local_cache(|| {
                        Some(
                            JsonGuardErrors::ValidationError(&error)
                                .get_problem_detail_extensions(),
                        )
                    });
                    Outcome::Error((
                        Status::UnprocessableEntity,
                        JsonValidationError::Validation { source: error },
                    ))
                }
                Ok(_) => Outcome::Success(Json(t)),
            },
        }
    }
}
