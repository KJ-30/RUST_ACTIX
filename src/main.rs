mod routes;
mod utils;
use actix_web::{middleware::Logger, web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
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
// 实现Error trait以支持MainError类型作为错误处理
impl Error for MainError {
    // 返回错误的源头，这里没有源头，所以返回None
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    // 返回错误的描述信息，该信息是MainError实例的一部分
    fn description(&self) -> &str {
        &self.message
    }
    // 返回错误的直接原因，这里使用source方法表示没有更深层次的原因
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
    Migrator::up(&db,None).await.map_err(|err| MainError {
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
            .configure(routes::auth_routes::config)
            .configure(routes::user_routes::config)
            .configure(routes::post_routes::config)
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
