use actix_web::{post, web, HttpResponse, Responder};
use mobc_redis::{redis, redis::AsyncCommands};
use serde_json::json;
use validator::Validate;

use std::ops::DerefMut;

use super::{claims::get_token_pair, login::Login};
use crate::errors::ServiceError;
use crate::Pool;

#[post("/login")]
pub async fn login(
    login: web::Json<Login>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let login = login.into_inner();
    let mut conn = db.get().await?;
    let id: Option<usize> = conn.hget("users", &login.username).await?;
    match id {
        Some(id) => {
            let password: String = conn.hget(&format!("user:{}", id), "password").await?;
            if login.verify_hash(&password)? {
                let (access_token, refresh_token) = get_token_pair(id, login.username)?;
                Ok(HttpResponse::Ok()
                    .json(json!({ "access_token": access_token, "refresh_token": refresh_token })))
            } else {
                Err(ServiceError::Unauthorized)
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

#[post("/signup")]
pub async fn signup(
    user: web::Json<Login>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let user = user.into_inner();
    user.validate()?;

    let mut conn = db.get().await?;
    let exists: bool = conn.hexists("users", &user.username).await?;
    if exists {
        Err(ServiceError::UserAlreadyExists {
            username: user.username,
        })
    } else {
        let user_id: usize = conn.incr("next_user_id", 1usize).await?;
        redis::pipe()
            .hset("users", &user.username, user_id)
            .hset_multiple(
                &format!("user:{}", user_id),
                &[("username", &user.username), ("password", &user.hash()?)],
            )
            .query_async(conn.deref_mut())
            .await?;
        Ok(HttpResponse::Created().json(json!({ "id": user_id })))
    }
}
