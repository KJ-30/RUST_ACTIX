use crate::utils::{api_response, api_state::AppState};
use actix_web::{get, web, Responder};
use sea_orm::{ConnectionTrait, Statement};
#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {}!", name))
    api_response::ApiResponse::new(200, format!("Hello {}!", name))
}
#[get("/test")]
pub async fn test(api_state: web::Data<AppState>) -> impl Responder {
    let res = api_state
        .db
        .query_all(Statement::from_string(
            sea_orm::DatabaseBackend::Postgres,
            "SELECT * FROM user;",
        ))
        .await
        .unwrap();
    api_response::ApiResponse::new(200, format!("Hello test!"))
}
