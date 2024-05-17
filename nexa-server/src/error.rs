use std::fmt::Display;
use axum::extract::multipart::MultipartError;
use axum::response::{IntoResponse, Response};

use axum::http::StatusCode;
use log::error;

pub(crate) struct Error {
    pub(crate) status_code: StatusCode,
    pub(crate) message: String,
}

impl Error {
    pub(crate) fn internal(message: impl ToString) -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: message.to_string(),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}", self.status_code, self.message)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> Response {
        if self.status_code == StatusCode::INTERNAL_SERVER_ERROR {
            error!("internal server error: {}", self.message);
        }
        if self.message.is_empty() {
            self.status_code.into_response()
        } else {
            (self.status_code, self.message).into_response()
        }
    }
}

impl From<hyper_util::client::legacy::Error> for Error {
    fn from(other: hyper_util::client::legacy::Error) -> Self {
        Self::internal(other)
    }
}

impl From<serde_json::Error> for Error {
    fn from(other: serde_json::Error) -> Self {
        Self::internal(other)
    }
}

impl From<hyper::Error> for Error {
    fn from(other: hyper::Error) -> Self {
        Self::internal(other)
    }
}

impl From<axum::http::Error> for Error {
    fn from(other: axum::http::Error) -> Self {
        Self::internal(other)
    }
}

impl From<MultipartError> for Error {
    fn from(other: MultipartError) -> Self {
        Self::internal(other)
    }
}

impl From<std::io::Error> for Error {
    fn from(other: std::io::Error) -> Self {
        Self::internal(other)
    }
}
