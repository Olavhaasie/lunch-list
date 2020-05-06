use actix_web::web;

mod claims;
mod login;
mod routes;

pub use claims::Claims;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::login).service(routes::signup);
}
