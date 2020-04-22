use actix_web::{middleware, web, App, HttpServer};
use r2d2_redis::{r2d2, RedisConnectionManager};

use std::env;

use lunch_list::list;
use lunch_list::not_found;
use lunch_list::user;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let addr = env::var("LUNCH_LIST_ADDR").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("LUNCH_LIST_PORT").unwrap_or_else(|_| "8080".to_string());
    let redis_host = env::var("LUNCH_LIST_REDIS").unwrap_or_else(|_| "localhost".to_string());

    let manager = RedisConnectionManager::new(format!("redis://{}", redis_host)).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .configure(list::config)
                    .configure(user::config),
            )
            .default_service(web::route().to(not_found))
    })
    .bind(&format!("{}:{}", addr, port))?
    .run()
    .await
}
