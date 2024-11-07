use super::handlers;
use actix_web::web;
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/auth")
            .service(handlers::auth_handler::register)
            .service(handlers::auth_handler::login),
    );
}
