use actix_web::web;

mod list_model;
mod list_query;
mod list_type;
mod routes;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(routes::get_list)
        .service(routes::get_lists)
        .service(routes::delete_list)
        .service(routes::put_list);
}
