use actix_web::{get, post, web, HttpResponse, Responder};
use jsonwebtoken::{encode, EncodingKey, Header};
use r2d2_redis::{redis, redis::Commands};
use serde_json::json;

use std::ops::DerefMut;

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
                if login.verify_hash(&password)? {
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

#[post("/user")]
pub async fn create_user(
    user: web::Json<Login>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let user = user.into_inner();
        let mut conn = db.get().unwrap();
        let exists: bool = conn.hexists("users", &user.username)?;
        if exists {
            Err(ServiceError::UserAlreadyExists {
                username: user.username,
            })
        } else {
            let user_id: usize = conn.incr("next_user_id", 1)?;
            redis::pipe()
                .hset("users", &user.username, user_id)
                .hset_multiple(
                    &format!("user:{}", user_id),
                    &[("username", &user.username), ("password", &user.hash()?)],
                )
                .query(conn.deref_mut())?;
            Ok(user_id)
        }
    })
    .await
    .map(|id| HttpResponse::Created().json(json!({ "id": id })))
    .map_err(ServiceError::from)
}

#[get("/user")]
pub async fn get_user(claims: Claims) -> impl Responder {
    HttpResponse::Ok().json(json!({ "id": claims.user_id, "username": claims.sub }))
}
