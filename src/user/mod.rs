use actix_web::web;

mod claims;
mod login;
mod routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::login);
}
