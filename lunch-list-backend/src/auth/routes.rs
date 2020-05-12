use std::ops::DerefMut;

use actix_web::{
    cookie::{Cookie, SameSite},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use digest::Digest;
use mobc_redis::{redis, redis::AsyncCommands};
use serde_json::json;
use validator::Validate;

use super::{
    claims::{decode, get_token_pair, RefreshClaims},
    login::Login,
    logout::LogoutRequest,
};
use crate::{errors::ServiceError, Pool};

type Hasher = blake2::Blake2b;

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
                let digest = Hasher::digest(refresh_token.as_bytes());

                conn.sadd(&format!("refresh_tokens:{}", id), digest.as_slice())
                    .await?;

                let refresh_cookie = Cookie::build("refresh_token", refresh_token)
                    .http_only(true)
                    .same_site(SameSite::Strict)
                    .finish();
                Ok(HttpResponse::Ok()
                    .cookie(refresh_cookie)
                    .json(json!({ "token": access_token })))
            } else {
                Err(ServiceError::Unauthorized)
            }
        }
        None => Err(ServiceError::Unauthorized),
    }
}

#[get("/refresh")]
pub async fn refresh(
    req: HttpRequest,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let token = req
        .cookie("refresh_token")
        .map(|c| c.value().to_string())
        .ok_or(ServiceError::Unauthorized)?;
    let claims = decode::<RefreshClaims>(&token)?;

    let mut conn = db.get().await?;
    let digest = Hasher::digest(token.as_bytes());
    let valid: bool = conn
        .srem(&format!("refresh_tokens:{}", claims.sub), digest.as_slice())
        .await?;
    if !valid {
        conn.del(&format!("refresh_tokens:{}", claims.sub)).await?;
        return Err(ServiceError::InvalidRefreshToken);
    }

    let name = conn
        .hget(&format!("user:{}", claims.sub), "username")
        .await?;

    let (access_token, refresh_token) = get_token_pair(claims.sub, name)?;

    let digest = Hasher::digest(refresh_token.as_bytes());
    conn.sadd(&format!("refresh_tokens:{}", claims.sub), digest.as_slice())
        .await?;

    let refresh_cookie = Cookie::build("refresh_token", refresh_token)
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish();
    Ok(HttpResponse::Ok()
        .cookie(refresh_cookie)
        .json(json!({ "token": access_token })))
}

#[post("/logout")]
pub async fn logout(
    req: HttpRequest,
    query: web::Query<LogoutRequest>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let refresh_cookie = req
        .cookie("refresh_token")
        .ok_or(ServiceError::Unauthorized)?;
    let claims = decode::<RefreshClaims>(refresh_cookie.value())?;

    let mut conn = db.get().await?;

    if query.all {
        conn.del(&format!("refresh_tokens:{}", claims.sub)).await?;
    } else {
        let token = refresh_cookie.value();
        let digest = Hasher::digest(token.as_bytes());
        conn.srem(&format!("refresh_tokens:{}", claims.sub), digest.as_slice())
            .await?;
    }

    Ok(HttpResponse::NoContent()
        .del_cookie(&refresh_cookie)
        .finish())
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
