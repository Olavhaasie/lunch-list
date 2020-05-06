use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

use crate::auth::Claims;

#[get("/user")]
pub async fn get_user(claims: Claims) -> impl Responder {
    HttpResponse::Ok().json(json!({ "id": claims.user_id, "username": claims.sub }))
}
