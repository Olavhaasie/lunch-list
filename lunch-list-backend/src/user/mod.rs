use actix_web::web;

mod routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/user").service(routes::get_user));
}
