use actix_web::{
    error::{BlockingError, ResponseError},
    http::StatusCode,
    HttpResponse,
};
use failure::Fail;
use r2d2_redis::redis;
use serde_json::json;

use std::convert::From;

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalError,
    #[fail(display = "Internal Server Error")]
    DatabaseError(redis::ErrorKind, String),
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        match self {
            _ => HttpResponse::InternalServerError().json(json!({ "error": self.to_string()})),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<redis::RedisError> for ServiceError {
    fn from(error: redis::RedisError) -> Self {
        Self::DatabaseError(
            error.kind(),
            error.detail().map(|s| s.to_string()).unwrap_or_default(),
        )
    }
}

impl From<BlockingError<redis::RedisError>> for ServiceError {
    fn from(error: BlockingError<redis::RedisError>) -> Self {
        match error {
            BlockingError::Error(db_error) => db_error.into(),
            BlockingError::Canceled => Self::InternalError,
        }
    }
}
