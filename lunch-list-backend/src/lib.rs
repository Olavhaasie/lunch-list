use actix_web::{web::HttpResponse, Responder};
use bb8_redis::RedisPool;
use serde_json::json;

mod auth;
mod errors;
pub mod list;
pub mod user;

type Pool = RedisPool;

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({ "error": "Resource not found" }))
}
