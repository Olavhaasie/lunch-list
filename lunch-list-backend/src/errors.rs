use std::{collections::HashMap, convert::From, env, fmt};

use actix_web::{
    error::{BlockingError, ResponseError},
    http::{header, StatusCode},
    HttpResponse,
};
use jsonwebtoken::errors::Error as JwtError;
use mobc_redis::redis::RedisError;
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Internal Server Error")]
    InternalError,
    #[error("Internal Server Error")]
    DatabaseError(#[from] RedisError),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid JWT")]
    InvalidJwt(#[from] JwtError),
    #[error("Internal Server Error")]
    EnvError(#[from] env::VarError),
    #[error("User with username '{username}' already exists")]
    UserAlreadyExists { username: String },
    #[error("Internal Server Error")]
    HashError(#[from] argon2::Error),
    #[error("Missing 'Authorization' header with Bearer token")]
    MissingAuthHeader,
    #[error("Invalid header value")]
    InvalidHeader,
    #[error("Invalid input")]
    ValidatorError(HashMap<String, String>),
    #[error("Re-using refresh token")]
    InvalidRefreshToken,
    #[error("Invalid signup secret")]
    InvalidSignupSecret,
}

impl ResponseError for ServiceError {
    fn error_response(&self) -> HttpResponse {
        let json = match self {
            Self::ValidatorError(errors) => json!({ "error": self.to_string(), "errors": errors }),
            _ => json!({ "error": self.to_string()}),
        };
        HttpResponse::build(self.status_code()).json(json)
    }

    fn status_code(&self) -> StatusCode {
        match self {
            Self::Unauthorized
            | Self::MissingAuthHeader
            | Self::InvalidRefreshToken
            | Self::InvalidSignupSecret => StatusCode::UNAUTHORIZED,
            Self::InvalidJwt(_)
            | Self::UserAlreadyExists { .. }
            | Self::InvalidHeader
            | Self::ValidatorError { .. } => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<header::ToStrError> for ServiceError {
    fn from(_: header::ToStrError) -> Self {
        Self::InvalidHeader
    }
}

impl From<mobc::Error<RedisError>> for ServiceError {
    fn from(e: mobc::Error<RedisError>) -> Self {
        match e {
            mobc::Error::Inner(e) => Self::DatabaseError(e),
            _ => Self::InternalError,
        }
    }
}

impl From<validator::ValidationErrors> for ServiceError {
    fn from(err: validator::ValidationErrors) -> Self {
        Self::ValidatorError(
            err.field_errors()
                .iter()
                .map(|(field, errors)| {
                    let errors = errors
                        .iter()
                        .filter_map(|e| e.message.as_ref())
                        .map(|c| c.to_string())
                        .collect::<Vec<String>>()
                        .join(", ");
                    ((*field).to_string(), errors)
                })
                .collect(),
        )
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
