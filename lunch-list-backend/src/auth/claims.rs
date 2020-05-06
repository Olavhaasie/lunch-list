use actix_web::{dev, http::HeaderMap, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use futures::future::{err, ok, Ready};

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
pub fn get_token_pair(id: usize, name: String) -> Result<(String, String), ServiceError> {
    let claims = Claims::new(id, name);
    let refresh_claims = RefreshClaims::new(id);

    Ok((encode(&claims)?, encode(&refresh_claims)?))
}

fn encode<T: Serialize>(claims: &T) -> Result<String, ServiceError> {
    let secret = std::env::var("LUNCH_LIST_SECRET")?;
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    jsonwebtoken::encode(&Header::default(), claims, &encoding_key).map_err(ServiceError::from)
}

pub fn decode<T: DeserializeOwned>(token: &str) -> Result<T, ServiceError> {
    let secret = std::env::var("LUNCH_LIST_SECRET")?;
    let decoding_key = DecodingKey::from_secret(secret.as_bytes());
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

impl FromRequest for Claims {
    type Error = ServiceError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let token = match get_bearer_token(req.headers()) {
            Ok(token) => token,
            Err(e) => return err(e),
        };
        let result = decode::<Self>(&token);
        match result {
            Ok(claims) => ok(claims),
            Err(e) => err(e),
        }
    }
}
