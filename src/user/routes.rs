use actix_web::{post, web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use r2d2_redis::redis::Commands;
use serde_json::json;

use super::claims::Claims;
use super::login::Login;
use crate::errors::ServiceError;
use crate::Pool;

#[post("/user/login")]
pub async fn login(
    login: web::Json<Login>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let login = login.into_inner();
        let mut conn = db.get().unwrap();
        let id: Option<usize> = conn.hget("users", &login.username)?;
        match id {
            Some(id) => {
                let password: String = conn.hget(&format!("user:{}", id), "password")?;
                if password == login.password {
                    let claims = Claims::new(login.username, id);
                    let secret = std::env::var("LUNCH_LIST_SECRET")?;
                    encode(
                        &Header::default(),
                        &claims,
                        &EncodingKey::from_secret(secret.as_bytes()),
                    )
                    .map_err(ServiceError::from)
                } else {
                    Err(ServiceError::Unauthorized)
                }
            }
            None => Err(ServiceError::Unauthorized),
        }
    })
    .await
    .map(|token| HttpResponse::Ok().json(json!({ "token": token })))
    .map_err(ServiceError::from)
}
