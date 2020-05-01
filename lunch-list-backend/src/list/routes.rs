use actix_web::{delete, get, put, web, HttpResponse, Responder};
use bb8_redis::{redis, redis::AsyncCommands};
use chrono::Datelike;
use futures::future::join_all;
use serde_json::json;

use std::collections::HashMap;

use crate::errors::ServiceError;
use crate::Pool;

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
    let mut conn = db.get().await.unwrap().unwrap();
    redis::pipe()
        .hgetall(&format!("list:{}", id))
        .smembers(&format!("users:{}", id))
        .query_async(&mut conn)
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
    let mut conn = db.get().await.unwrap().unwrap();
    let (start, stop) = query.to_range();
    let ids: Vec<usize> = if query.rev() {
        conn.zrange("dates", start as isize, stop as isize).await?
    } else {
        conn.zrevrange("dates", start as isize, stop as isize)
            .await?
    };

    let futures = ids
        .into_iter()
        .map(|id| conn.hgetall(&format!("list:{}", id)));
    let lists = join_all(futures)
        .await
        .into_iter()
        .collect::<Result<Vec<HashMap<_, _>>, _>>()?;
    let lists = lists
        .into_iter()
        .zip(ids.iter())
        .map(|(l, id)| List::from_hash(*id, l).unwrap())
        .collect::<Vec<List>>();

    Ok(HttpResponse::Ok().json(json!({ "lists": lists })))
}

#[delete("/list/{id}")]
async fn delete_list(
    id: web::Path<usize>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let mut conn = db.get().await.unwrap().unwrap();
    let id = id.into_inner();
    let (_, _, b): (bool, bool, bool) = redis::pipe()
        .zrem("dates", id)
        .del(&format!("users:{}", id))
        .del(&format!("list:{}", id))
        .query_async(&mut conn)
        .await?;
    if b {
        Ok(HttpResponse::NoContent())
    } else {
        Ok(HttpResponse::NotFound())
    }
}

#[put("/list")]
async fn put_list(
    list: web::Json<List>,
    _claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let days = list.date.num_days_from_ce();
    let mut conn = db.get().await.unwrap().unwrap();
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
                    .query_async(&mut conn)
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
                .query_async(&mut conn)
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

#[put("/list/{id}/user")]
async fn add_user(
    id: web::Path<usize>,
    claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    let mut conn = db.get().await.unwrap().unwrap();
    let added = conn.sadd(&format!("users:{}", id), claims.sub).await?;
    if added {
        Ok(HttpResponse::Created())
    } else {
        Ok(HttpResponse::NoContent())
    }
}

#[delete("/list/{id}/user")]
async fn remove_user(
    id: web::Path<usize>,
    claims: Claims,
    db: web::Data<Pool>,
) -> Result<impl Responder, ServiceError> {
    let id = id.into_inner();
    let mut conn = db.get().await.unwrap().unwrap();
    let _: bool = conn.srem(&format!("users:{}", id), claims.sub).await?;
    Ok(HttpResponse::NoContent())
}
