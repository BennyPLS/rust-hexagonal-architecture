use std::collections::HashMap;
use std::io::Cursor;

use rocket::{Request, response, Response};
use rocket::http::{ContentType, Status};
use rocket::response::Responder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, PartialEq)]
pub struct ProblemDetail {
    #[serde(rename = "type")]
    r#type: String,
    status: Status,
    title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    instance: Option<String>,
    #[serde(flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

impl ProblemDetail {
    pub fn builder() -> ProblemDetailBuilder {
        ProblemDetailBuilder::default()
    }
}

pub struct ProblemDetailBuilder {
    r#type: Option<String>,
    status: Status,
    title: String,
    detail: Option<String>,
    instance: Option<String>,
    extensions: HashMap<String, serde_json::Value>,
}

impl Default for ProblemDetailBuilder {
    fn default() -> Self {
        let status = Status::default();

        ProblemDetailBuilder {
            r#type: None,
            status,
            title: String::from(status.reason_lossy()),
            detail: None,
            instance: None,
            extensions: HashMap::new(),
        }
    }
}

impl ProblemDetailBuilder {
    pub fn new(status: Status) -> ProblemDetailBuilder {
        ProblemDetailBuilder {
            r#type: None,
            status,
            title: String::from(status.reason_lossy()),
            detail: None,
            instance: None,
            extensions: HashMap::new(),
        }
    }

    pub fn r#type(mut self, r#type: String) -> Self {
        self.r#type = Some(r#type);
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn title(mut self, title: String) -> Self {
        self.title = title;
        self
    }

    pub fn detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }

    pub fn instance(mut self, instance: String) -> Self {
        self.instance = Some(instance);
        self
    }

    pub fn extensions<T: IntoIterator<Item = (String, serde_json::Value)>>(mut self, extension: T) -> Self {
        self.extensions.extend(extension);
        self
    }

    pub fn add_extension(mut self, key: String, value: serde_json::Value) -> Self {
        self.extensions.insert(key, value);
        self
    }

    pub fn build(self) -> ProblemDetail {
        ProblemDetail {
            r#type: self.r#type.unwrap_or(String::from("about:blank")),
            status: self.status,
            title: self.title,
            detail: self.detail,
            instance: self.instance,
            extensions: self.extensions,
        }
    }
}

impl<'r> Responder<'r, 'static> for ProblemDetail {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        let json = serde_json::to_string(&self).unwrap();

        Response::build()
            .status(self.status)
            .header(ContentType::new("application", "problem+json"))
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}
