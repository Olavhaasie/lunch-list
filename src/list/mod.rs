use actix_web::{delete, get, put, web, HttpResponse, Responder};
use r2d2_redis::redis::Commands;
use serde_json::json;

use crate::errors::ServiceError;
use crate::Pool;

mod list_model;
mod list_type;

use list_model::List;

#[get("/list/{id}")]
async fn get_list(
    id: web::Path<usize>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.hgetall(&format!("list:{}", id))
    })
    .await
    .map(List::from_hash)
    .map(|list| {
        if let Some(list) = list {
            HttpResponse::Ok().json(list)
        } else {
            HttpResponse::NotFound().finish()
        }
    })
    .map_err(ServiceError::from)
}

#[delete("/list/{id}")]
async fn delete_list(
    id: web::Path<usize>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.del::<&str, bool>(&format!("list:{}", id))
    })
    .await
    .map(|b| {
        if b {
            HttpResponse::NoContent()
        } else {
            HttpResponse::NotFound()
        }
    })
    .map_err(ServiceError::from)
}

#[put("/list")]
async fn put_list(
    list: web::Json<List>,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        let next_list_id: usize = conn.incr("next_list_id", 1)?;
        conn.hset_multiple(
            &format!("list:{}", next_list_id),
            &[
                ("type", list.list_type.to_string()),
                ("date", list.date.to_string()),
            ],
        )?;
        Ok(next_list_id)
    })
    .await
    .map(|id| HttpResponse::Created().json(json!({ "id": id })))
    .map_err(ServiceError::from)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_list).service(delete_list).service(put_list);
}
