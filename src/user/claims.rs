use actix_web::{dev, FromRequest, HttpRequest};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, DecodingKey, Validation};
use serde::{Deserialize, Serialize};

use futures::future::{err, ok, Ready};
use std::env;

use crate::errors::ServiceError;

#[derive(Debug, Deserialize, Serialize)]
pub struct Claims {
    sub: String,
    exp: usize,
    user_id: usize,
}

impl Claims {
    pub fn new(username: String, id: usize) -> Self {
        let date = Utc::now() + Duration::hours(1);
        Self {
            sub: username,
            exp: date.timestamp() as usize,
            user_id: id,
        }
    }
}

impl FromRequest for Claims {
    type Error = ServiceError;
    type Future = Ready<Result<Self, Self::Error>>;
    type Config = ();

    fn from_request(req: &HttpRequest, _: &mut dev::Payload) -> Self::Future {
        let auth = req.headers().get("Authorization");
        match auth {
            Some(auth) => {
                let secret = match env::var("LUNCH_LIST_SECRET") {
                    Ok(s) => s,
                    Err(e) => return err(e.into()),
                };
                let token = match auth.to_str() {
                    Ok(value) => match value.split("Bearer").nth(1) {
                        Some(t) => t,
                        None => return err(ServiceError::MissingAuthHeader),
                    },
                    Err(e) => return err(e.into()),
                };
                let result = decode::<Claims>(
                    token,
                    &DecodingKey::from_secret(secret.as_bytes()),
                    &Validation::default(),
                );
                match result {
                    Ok(token_data) => ok(token_data.claims),
                    Err(e) => err(ServiceError::from(e)),
                }
            }
            None => err(ServiceError::MissingAuthHeader),
        }
    }
}
