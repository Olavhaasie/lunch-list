pub mod errors;
pub mod list;

type Pool = r2d2_redis::r2d2::Pool<r2d2_redis::RedisConnectionManager>;
