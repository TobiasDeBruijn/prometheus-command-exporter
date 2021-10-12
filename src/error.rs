use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use serde::Serialize;
use thiserror::Error;

pub type HttpResult = Result<HttpResponse, Error>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("An error occurred in the Prometheus library: {0}")]
    PrometheusLibrary(#[from] prometheus::Error),
    #[error("IO Exception: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse command output to a floating point value")]
    Parse(#[from] std::num::ParseFloatError),
    #[error("Failed to convert data to UTF-8")]
    FromUtf8(#[from] std::string::FromUtf8Error)
}

impl Error {
    fn log(&self) {
        log::warn!("{}", self)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse {
        self.log();

        let error_response = ErrorResponse::from(self);
        HttpResponse::build(self.status_code()).json(&error_response)
    }
}

#[derive(Serialize)]
struct ErrorResponse {
    code:       u16,
    message:    String
}

impl From<&Error> for ErrorResponse {
    fn from(e: &Error) -> Self {
        Self {
            code:       e.status_code().as_u16(),
            message:    format!("{}", e)
        }
    }
}

