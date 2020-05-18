use actix_web::web;

mod claims;
mod login;
mod logout;
mod routes;

pub use claims::{Claims, ClaimsConfig};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .service(routes::login)
            .service(routes::refresh)
            .service(routes::logout)
            .service(routes::signup),
    );
}
