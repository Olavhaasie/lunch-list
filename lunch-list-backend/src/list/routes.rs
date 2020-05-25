use std::ops::DerefMut;

use actix_web::{delete, get, put, web, HttpResponse, Responder};
use chrono::Datelike;
use mobc_redis::{redis, redis::AsyncCommands};
use serde_json::json;

use super::{list_model::List, list_query::ListQuery, list_type::ListType};
use crate::{auth::Claims, errors::ServiceError, Pool};

#[get("/{id}")]
async fn get_list(
    id: web::Path<usize>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    let mut conn = db.get().await?;
    redis::pipe()
        .hgetall(&format!("list:{}", id))
        .smembers(&format!("users:{}", id))
        .query_async(conn.deref_mut())
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

#[get("")]
async fn get_lists(
    query: web::Query<ListQuery>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let mut conn = db.get().await?;
    let (start, stop) = query.to_range();
    let ids: Vec<usize> = if query.rev() {
        conn.zrange("dates", start as isize, stop as isize).await?
    } else {
        conn.zrevrange("dates", start as isize, stop as isize)
            .await?
    };

    let mut lists = Vec::new();
    for id in ids {
        let list = conn.hgetall(&format!("list:{}", id)).await?;
        let card = conn.scard(&format!("users:{}", id)).await?;
        lists.push(List::from_hash(id, list).unwrap().with_size(card))
    }
    Ok(HttpResponse::Ok().json(json!({ "lists": lists })))
}

#[delete("/{id}")]
async fn delete_list(
    id: web::Path<usize>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let mut conn = db.get().await?;
    let id = id.into_inner();
    let (_, _, b): (bool, bool, bool) = redis::pipe()
        .zrem("dates", id)
        .del(&format!("users:{}", id))
        .del(&format!("list:{}", id))
        .query_async(conn.deref_mut())
        .await?;
    if b {
        Ok(HttpResponse::NoContent())
    } else {
        Ok(HttpResponse::NotFound())
    }
}

#[put("")]
async fn put_list(
    list: web::Json<List>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let days = list.date.num_days_from_ce();
    let mut conn = db.get().await?;
    let list_id: Vec<usize> = conn.zrangebyscore("dates", days, days).await?;
    let id: Option<usize> = match list_id.as_slice() {
        [id] => {
            let list_type = conn
                .hget::<&str, &str, String>(&format!("list:{}", id), "type")
                .await?
                .parse::<ListType>()
                .unwrap();
            if list_type != list.list_type {
                let id: usize = conn.incr("next_list_id", 1 as usize).await?;
                redis::pipe()
                    .hset_multiple(
                        &format!("list:{}", id),
                        &[
                            ("type", list.list_type.to_string()),
                            ("date", list.date.to_string()),
                        ],
                    )
                    .zadd("dates", id, days)
                    .query_async(conn.deref_mut())
                    .await?;
                Some(id)
            } else {
                None
            }
        }
        [] => {
            let id: usize = conn.incr("next_list_id", 1 as usize).await?;
            redis::pipe()
                .hset_multiple(
                    &format!("list:{}", id),
                    &[
                        ("type", list.list_type.to_string()),
                        ("date", list.date.to_string()),
                    ],
                )
                .zadd("dates", id, days)
                .query_async(conn.deref_mut())
                .await?;
            Some(id)
        }
        _ => None,
    };

    if let Some(id) = id {
        Ok(HttpResponse::Created().json(json!({ "id": id })))
    } else {
        Ok(HttpResponse::NoContent().finish())
    }
}

#[put("/{id}/user")]
async fn add_user(
    id: web::Path<usize>,
    claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    let mut conn = db.get().await?;

    let exists = conn.exists(&format!("list:{}", id)).await?;

    if exists {
        let added = conn.sadd(&format!("users:{}", id), claims.name).await?;
        if added {
            Ok(HttpResponse::Created())
        } else {
            Ok(HttpResponse::NoContent())
        }
    } else {
        Ok(HttpResponse::NotFound())
    }
}

#[delete("/{id}/user")]
async fn remove_user(
    id: web::Path<usize>,
    claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    let mut conn = db.get().await?;

    let exists = conn.exists(&format!("list:{}", id)).await?;

    if exists {
        conn.srem(&format!("users:{}", id), claims.sub).await?;
        Ok(HttpResponse::NoContent())
    } else {
        Ok(HttpResponse::NotFound())
    }
}
