mod routes;
mod utils;
use actix_web::{middleware::Logger, web, App, HttpServer};
use sea_orm::Database;
use std::{error::Error, fmt::Display};
use utils::api_state::AppState;
#[derive(Debug)]
struct MainError {
    message: String,
}
impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error{}", self.message)
    }
}
impl Error for MainError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        &self.message
    }
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}
#[actix_web::main] // or #[tokio::main]

async fn main() -> Result<(), MainError> {
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }
    dotenv::dotenv().ok();
    env_logger::init();

    let address = (*utils::constants::ADDRESS).clone();
    let port = (*utils::constants::PORT).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();
    println!("database_url: {:?}", database_url);
    let db = Database::connect(database_url)
        .await
        .map_err(|err| MainError {
            message: err.to_string(),
        })?;
    println!("port: {}", port);
    println!("address: {}", address);
    println!("db: {:?}", db);
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
    })
    .bind((address, port))
    .map_err(|err| MainError {
        message: err.to_string(),
    })?
    .run()
    .await
    .map_err(|err| MainError {
        message: err.to_string(),
    })
}
