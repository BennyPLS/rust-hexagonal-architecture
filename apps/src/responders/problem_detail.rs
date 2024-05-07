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
    #[serde(skip_serializing_if = "HashMap::is_empty", flatten)]
    extensions: HashMap<String, serde_json::Value>,
}

impl ProblemDetail {
    pub fn from(status: Status) -> ProblemDetail {
        ProblemDetail {
            r#type: String::from("about:blank"),
            status,
            title: String::from(status.reason_lossy()),
            detail: None,
            instance: None,
            extensions: HashMap::new(),
        }
    }

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
    pub fn from(status: Status) -> ProblemDetailBuilder {
        ProblemDetailBuilder {
            r#type: None,
            status,
            title: String::from(status.reason_lossy()),
            detail: None,
            instance: None,
            extensions: HashMap::new(),
        }
    }

    pub fn r#type<T: Into<String>>(mut self, r#type: T) -> Self {
        self.r#type = Some(r#type.into());
        self
    }

    pub fn status(mut self, status: Status) -> Self {
        self.status = status;
        self
    }

    pub fn title<T: Into<String>>(mut self, title: T) -> Self {
        self.title = title.into();
        self
    }

    pub fn detail<T: Into<String>>(mut self, detail: T) -> Self {
        self.detail = Some(detail.into());
        self
    }

    pub fn instance<T: Into<String>>(mut self, instance: T) -> Self {
        self.instance = Some(instance.into());
        self
    }

    pub fn extensions<T: IntoIterator<Item = (String, serde_json::Value)>>(
        mut self,
        extension: T,
    ) -> Self {
        self.extensions.extend(extension);
        self
    }

    pub fn add_extension<T: Into<String>>(mut self, key: T, value: serde_json::Value) -> Self {
        self.extensions.insert(key.into(), value);
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
