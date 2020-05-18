#![forbid(unsafe_code)]
use actix_web::{web::HttpResponse, Responder};
use serde_json::json;

pub mod auth;
mod errors;
pub mod list;
pub mod user;

type Pool = mobc::Pool<mobc_redis::RedisConnectionManager>;

pub struct AppState {
    pub token_secret: String,
    pub signup_secret: String,
}

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({ "error": "Resource not found" }))
}
