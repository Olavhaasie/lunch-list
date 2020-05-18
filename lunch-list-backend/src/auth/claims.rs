use actix_web::{dev, http::HeaderMap, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use futures::future::{err, ok, Ready};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::errors::ServiceError;

const TOKEN_ISSUER: &str = "lunch-list";

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    exp: usize,
    iat: usize,
    iss: String,
    pub sub: usize,
    pub name: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshClaims {
    exp: usize,
    iat: usize,
    iss: String,
    pub sub: usize,
}

impl Claims {
    pub fn new(id: usize, name: String) -> Self {
        let now = Utc::now();
        let exp = now + Duration::minutes(10);
        Self {
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: TOKEN_ISSUER.to_string(),
            sub: id,
            name,
        }
    }
}

impl RefreshClaims {
    pub fn new(id: usize) -> Self {
        let now = Utc::now();
        let exp = now + Duration::weeks(1);
        Self {
            exp: exp.timestamp() as usize,
            iat: now.timestamp() as usize,
            iss: TOKEN_ISSUER.to_string(),
            sub: id,
        }
    }
}

/// Returns an access token and refresh token pair.
pub fn get_token_pair(
    id: usize,
    name: String,
    secret: &[u8],
) -> Result<(String, String), ServiceError> {
    let claims = Claims::new(id, name);
    let refresh_claims = RefreshClaims::new(id);

    Ok((encode(&claims, secret)?, encode(&refresh_claims, secret)?))
}

fn encode<T: Serialize>(claims: &T, secret: &[u8]) -> Result<String, ServiceError> {
    let encoding_key = EncodingKey::from_secret(secret);
    jsonwebtoken::encode(&Header::default(), claims, &encoding_key).map_err(ServiceError::from)
}

pub fn decode<T: DeserializeOwned>(token: &str, secret: &[u8]) -> Result<T, ServiceError> {
    let decoding_key = DecodingKey::from_secret(secret);
    let validation = Validation {
        validate_exp: true,
        iss: Some(TOKEN_ISSUER.to_string()),
        ..Default::default()
    };
    jsonwebtoken::decode::<T>(token, &decoding_key, &validation)
        .map(|t| t.claims)
        .map_err(ServiceError::from)
}

fn get_bearer_token(headers: &HeaderMap) -> Result<String, ServiceError> {
    match headers.get("Authorization") {
        Some(auth) => auth.to_str().map_err(ServiceError::from).and_then(|value| {
            match value.split("Bearer").nth(1) {
                Some(t) => Ok(t.trim().to_string()),
                None => Err(ServiceError::MissingAuthHeader),
            }
        }),
        None => Err(ServiceError::MissingAuthHeader),
    }
}

#[derive(Debug, Default)]
pub struct ClaimsConfig {
    secret: String,
}

impl ClaimsConfig {
    pub fn secret(mut self, secret: &str) -> Self {
        self.secret = secret.to_string();
        self
    }
}

impl FromRequest for Claims {
    type Error = ServiceError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ClaimsConfig;

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let token = match get_bearer_token(req.headers()) {
            Ok(token) => token,
            Err(e) => return err(e),
        };

        let config = match req
            .app_data::<Self::Config>()
            .ok_or(ServiceError::InternalError)
        {
            Ok(s) => s,
            Err(e) => return err(e),
        };

        match decode::<Self>(&token, config.secret.as_bytes()) {
            Ok(claims) => ok(claims),
            Err(e) => err(e),
        }
    }
}
