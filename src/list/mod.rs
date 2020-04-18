use actix_web::{delete, get, put, web, HttpResponse, Responder};
use chrono::Datelike;
use r2d2_redis::{redis, redis::Commands};
use serde_json::json;

use crate::errors::ServiceError;
use crate::Pool;

use std::ops::DerefMut;

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
        let id = id.into_inner();
        redis::pipe()
            .zrem("dates:lunch", id)
            .zrem("dates:dinner", id)
            .del(&format!("list:{}", id))
            .query(conn.deref_mut())
    })
    .await
    .map(|(_, _, b): (bool, bool, bool)| {
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
        let dates_key = format!("dates:{}", list.list_type);
        redis::transaction(conn.deref_mut(), &[&dates_key], |conn, pipe| {
            let days = list.date.num_days_from_ce();
            let list_id: Vec<usize> = conn.zrangebyscore(&dates_key, days, days)?;
            match list_id.first() {
                Some(&id) => Ok(Some(id)),
                None => {
                    let id: usize = conn.incr("next_list_id", 1)?;
                    pipe.hset_multiple(
                        &format!("list:{}", id),
                        &[
                            ("type", list.list_type.to_string()),
                            ("date", list.date.to_string()),
                        ],
                    )
                    .zadd(&dates_key, id, days)
                    .query(conn)?;
                    Ok(Some(id))
                }
            }
        })
    })
    .await
    .map(|id| HttpResponse::Created().json(json!({ "id": id })))
    .map_err(ServiceError::from)
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_list).service(delete_list).service(put_list);
}
