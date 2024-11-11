use crate::utils::{api_response, api_state, jwt::Claims};
use actix_web::{get, post, web};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct PostModel {
    text: String,
    title: String,
}

#[post("create")]
pub async fn postHandler(
    app_state: web::Data<api_state::AppState>,
    claim_data: Claims,
    post_data: web::Json<PostModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    Ok(api_response::ApiResponse::new(
        200,
        "Post Created".to_owned(),
    ))
}
