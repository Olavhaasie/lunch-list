use actix_files::{Files, NamedFile};
use actix_web::{http::ContentEncoding, middleware, web, App, HttpServer};
use r2d2_redis::{r2d2, RedisConnectionManager};

use std::env;

use lunch_list_backend::list;
use lunch_list_backend::not_found;
use lunch_list_backend::user;

const ASSETS_DIR: &str = "target/deploy";
const INDEX_HTML: &str = "index.html";

async fn serve_index_html() -> Result<NamedFile, std::io::Error> {
    let index_file = format!("{}/{}", ASSETS_DIR, INDEX_HTML);

    NamedFile::open(index_file)
}

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
            .wrap(middleware::Compress::default())
            .service(
                web::scope("/api")
                    .configure(list::config)
                    .configure(user::config)
                    .default_service(web::route().to(not_found)),
            )
            .service(Files::new("/", ASSETS_DIR).index_file(INDEX_HTML))
            .default_service(web::get().to(serve_index_html))
    })
    .bind(&format!("{}:{}", addr, port))?
    .run()
    .await
}
