use actix_web::{
    error::{BlockingError, ResponseError},
    http::{header, StatusCode},
    HttpResponse,
};
use bcrypt::BcryptError;
use failure::Fail;
use jsonwebtoken::errors::Error as JwtError;
use r2d2_redis::redis::RedisError;
use serde_json::json;

use std::convert::From;
use std::env;
use std::fmt;

#[derive(Debug, Fail)]
pub enum ServiceError {
    #[fail(display = "Internal Server Error")]
    InternalError,
    #[fail(display = "Internal Server Error")]
    DatabaseError(RedisError),
    #[fail(display = "Unauthorized")]
    Unauthorized,
    #[fail(display = "Invalid JWT")]
    InvalidJwt(JwtError),
    #[fail(display = "Internal Server Error")]
    EnvError(env::VarError),
    #[fail(display = "User with username '{}' already exists", username)]
    UserAlreadyExists { username: String },
    #[fail(display = "Internal Server Error")]
    BcryptError(BcryptError),
    #[fail(display = "Missing 'Authorization' header with Bearer token")]
    MissingAuthHeader,
    #[fail(display = "Invalid header value")]
    InvalidHeader,
    #[fail(display = "Username can only contain alphanumeric characters or whitespaces")]
    InvalidUsername,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(json!({ "error": self.to_string()}))
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::InvalidJwt(_) => StatusCode::BAD_REQUEST,
            Self::UserAlreadyExists { .. } => StatusCode::BAD_REQUEST,
            Self::MissingAuthHeader => StatusCode::UNAUTHORIZED,
            Self::InvalidHeader => StatusCode::BAD_REQUEST,
            Self::InvalidUsername => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<RedisError> for ServiceError {
    fn from(err: RedisError) -> Self {
        Self::DatabaseError(err)
    }
}

impl From<JwtError> for ServiceError {
    fn from(err: JwtError) -> Self {
        Self::InvalidJwt(err)
    }
}

impl From<env::VarError> for ServiceError {
    fn from(err: env::VarError) -> Self {
        Self::EnvError(err)
    }
}

impl From<BcryptError> for ServiceError {
    fn from(err: BcryptError) -> Self {
        Self::BcryptError(err)
    }
}

impl From<header::ToStrError> for ServiceError {
    fn from(_: header::ToStrError) -> Self {
        Self::InvalidHeader
    }
}

impl<E: fmt::Debug + Into<ServiceError>> From<BlockingError<E>> for ServiceError {
    fn from(error: BlockingError<E>) -> Self {
        match error {
            BlockingError::Error(db_error) => db_error.into(),
            BlockingError::Canceled => Self::InternalError,
        }
    }
}
