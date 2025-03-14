use actix_web::{get, HttpResponse, Responder};
use serde_json::json;

use crate::auth::Claims;

#[get("")]
pub async fn get_user(claims: Claims) -> impl Responder {
    HttpResponse::Ok().json(json!({ "id": claims.sub, "username": claims.name }))
}
