use actix_web::{web::HttpResponse, Responder};
use serde_json::json;

mod auth;
mod errors;
pub mod list;
pub mod user;

type Pool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().json(json!({ "error": "Resource not found" }))
}
