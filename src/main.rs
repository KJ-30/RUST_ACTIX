mod routes;
mod utils;
use actix_web::{get, middleware::Logger, web, App, HttpServer, Responder};
use routes::handlers::home_handler::greet;
#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let address = (utils::constants::ADDRESS).clone();
    let port = (utils::constants::PORT).clone();

    HttpServer::new(|| App::new().wrap(Logger::default()).service(greet))
        .bind((address, port))?
        .run()
        .await
}
