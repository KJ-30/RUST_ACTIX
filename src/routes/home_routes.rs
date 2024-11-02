use super::handlers;
use actix_web::web;
pub fn config(config: &mut web::ServiceConfig) {
    config.service(
        web::scope("/home")
            .service(handlers::home_handler::greet)
            .service(handlers::home_handler::test),
    );
}
