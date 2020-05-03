use actix_files::{Files, NamedFile};
use actix_web::{middleware, web, App, HttpServer};
use clap::Clap;
use mobc_redis::{redis, RedisConnectionManager};

use std::io;

use lunch_list_backend::list;
use lunch_list_backend::not_found;
use lunch_list_backend::user;

const ASSETS_DIR: &str = "static";
const INDEX_HTML: &str = "index.html";

#[derive(Clap)]
#[clap(
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
    about = env!("CARGO_PKG_DESCRIPTION"),
    setting = clap::AppSettings::ColoredHelp
)]
struct Opts {
    #[clap(
        short = "a",
        long = "address",
        env = "LUNCH_LIST_ADDR",
        default_value = "localhost"
    )]
    address: String,
    #[clap(
        short = "p",
        long = "port",
        env = "LUNCH_LIST_PORT",
        default_value = "8080"
    )]
    port: u16,
    #[clap(
        long = "redis-host",
        env = "LUNCH_LIST_REDIS",
        default_value = "localhost"
    )]
    redis_host: String,
}

async fn serve_index_html() -> Result<NamedFile, std::io::Error> {
    let index_file = format!("{}/{}", ASSETS_DIR, INDEX_HTML);

    NamedFile::open(index_file)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let opts = Opts::parse();

    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let pool = build_pool(&opts)?;

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
    .bind((opts.address.as_str(), opts.port))?
    .run()
    .await
}

fn build_pool(opts: &Opts) -> std::io::Result<mobc::Pool<RedisConnectionManager>> {
    let client = redis::Client::open(format!("redis://{}/", opts.redis_host))
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let manager = RedisConnectionManager::new(client);
    Ok(mobc::Pool::new(manager))
}
