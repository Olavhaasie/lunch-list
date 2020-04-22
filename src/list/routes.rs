use actix_web::{delete, get, put, web, HttpResponse, Responder};
use chrono::Datelike;
use r2d2_redis::{redis, redis::Commands};
use serde_json::json;

use crate::errors::ServiceError;
use crate::Pool;

use std::ops::DerefMut;

use super::list_model::List;
use super::list_query::ListQuery;
use super::list_type::ListType;
use crate::auth::Claims;

#[get("/list/{id}")]
async fn get_list(
    id: web::Path<usize>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    web::block(move || {
        let mut conn = db.get().unwrap();
        redis::pipe()
            .hgetall(&format!("list:{}", id))
            .smembers(&format!("users:{}", id))
            .query(conn.deref_mut())
    })
    .await
    .map(|(list, users)| {
        if let Some(list) = List::from_hash(id, list) {
            HttpResponse::Ok().json(list.with_users(users))
        } else {
            HttpResponse::NotFound().finish()
        }
    })
    .map_err(ServiceError::from)
}

#[get("/list")]
async fn get_lists(
    query: web::Query<ListQuery>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        let (start, stop) = query.to_range();
        let ids: Vec<usize> = if query.rev() {
            conn.zrange("dates", start as isize, stop as isize)?
        } else {
            conn.zrevrange("dates", start as isize, stop as isize)?
        };

        ids.into_iter()
            .map(|id| {
                conn.hgetall(&format!("list:{}", id))
                    .map(|l| List::from_hash(id, l).unwrap())
            })
            .collect::<Result<Vec<List>, _>>()
    })
    .await
    .map(|lists: Vec<List>| HttpResponse::Ok().json(json!({ "lists": lists })))
    .map_err(ServiceError::from)
}

#[delete("/list/{id}")]
async fn delete_list(
    id: web::Path<usize>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        let id = id.into_inner();
        redis::pipe()
            .zrem("dates", id)
            .del(&format!("users:{}", id))
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
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    web::block(move || {
        let mut conn = db.get().unwrap();
        redis::transaction(conn.deref_mut(), &["dates"], |conn, pipe| {
            let days = list.date.num_days_from_ce();
            let list_id: Vec<usize> = conn.zrangebyscore("dates", days, days)?;
            match list_id.as_slice() {
                [id] => {
                    let list_type = conn
                        .hget::<&str, &str, String>(&format!("list:{}", id), "type")?
                        .parse::<ListType>()
                        .unwrap();
                    if list_type == list.list_type {
                        Ok(Some(None))
                    } else {
                        let id: usize = conn.incr("next_list_id", 1)?;
                        pipe.hset_multiple(
                            &format!("list:{}", id),
                            &[
                                ("type", list.list_type.to_string()),
                                ("date", list.date.to_string()),
                            ],
                        )
                        .zadd("dates", id, days)
                        .query(conn)?;
                        Ok(Some(Some(id)))
                    }
                }
                [] => {
                    let id: usize = conn.incr("next_list_id", 1)?;
                    pipe.hset_multiple(
                        &format!("list:{}", id),
                        &[
                            ("type", list.list_type.to_string()),
                            ("date", list.date.to_string()),
                        ],
                    )
                    .zadd("dates", id, days)
                    .query(conn)?;
                    Ok(Some(Some(id)))
                }
                _ => Ok(Some(None)),
            }
        })
    })
    .await
    .map(|id| {
        if let Some(id) = id {
            HttpResponse::Created().json(json!({ "id": id }))
        } else {
            HttpResponse::NoContent().finish()
        }
    })
    .map_err(ServiceError::from)
}

#[put("/list/{id}/user")]
async fn add_user(
    id: web::Path<usize>,
    claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.sadd(&format!("users:{}", id), claims.sub)
    })
    .await
    .map(|added| {
        if added {
            HttpResponse::Created()
        } else {
            HttpResponse::NoContent()
        }
    })
    .map_err(ServiceError::from)
}

#[delete("/list/{id}/user")]
async fn remove_user(
    id: web::Path<usize>,
    claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    web::block(move || {
        let mut conn = db.get().unwrap();
        conn.srem(&format!("users:{}", id), claims.sub)
    })
    .await
    .map(|_: bool| HttpResponse::NoContent())
    .map_err(ServiceError::from)
}
