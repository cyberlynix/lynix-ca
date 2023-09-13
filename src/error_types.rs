use std::fmt::{Display, Formatter};
use actix_web::{HttpResponse, ResponseError};
use actix_web::http::StatusCode;
use derive_more::{Display};

#[allow(dead_code)]
#[derive(Debug, Display)]
pub(crate) enum ErrorKind {
    #[display(fmt = "A templating error has occurred: {}", _0)]
    TemplateError(String),
}

impl ResponseError for ErrorKind {
    fn status_code(&self) -> StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }

    fn error_response(&self) -> HttpResponse {
        let error_message = format!("[FloofRender] ERROR: {}", self);

        HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(error_message)
    }
}

// Implement From trait to wrap an underlying error
impl From<std::io::Error> for ErrorKind {
    fn from(err: std::io::Error) -> Self {
        ErrorKind::TemplateError(err.to_string())
    }
}
