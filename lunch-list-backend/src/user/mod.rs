use actix_web::web;

mod routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::get_user);
}
