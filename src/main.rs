use actix_web::{middleware, web, App, HttpServer};
use r2d2_redis::{r2d2, RedisConnectionManager};

use lunch_list::list;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let manager = RedisConnectionManager::new("redis://localhost").unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(web::scope("/api").configure(list::config))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
