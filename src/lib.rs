mod auth;
mod errors;
pub mod list;
pub mod user;

type Pool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;
