use crate::utils::{api_response, api_state, jwt::Claims};
use actix_web::{get, post, web};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct UpdateUserModel {
    name: String,
}

#[get("")]
pub async fn user(
    app_state: web::Data<api_state::AppState>,
    claim_data: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User Not Found".to_owned(),
        ))?;

    Ok(api_response::ApiResponse::new(
        200,
        format!(
            " {{ 'name': '{}', 'email': '{}' }} ",
            user_model.name, user_model.email
        ),
    ))
}

#[post("/update")]
pub async fn update_user(
    api_state: web::Data<api_state::AppState>,
    use_data: web::Json<UpdateUserModel>,
    Claims_data: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let mut user_model = entity::user::Entity::find_by_id(Claims_data.id)
        .one(&api_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User Not Found".to_owned(),
        ))?
        .into_active_model();

    user_model.name = Set(use_data.name.clone());
    user_model
        .update(&api_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;
    Ok(api_response::ApiResponse::new(
        200,
        "User Updated".to_owned(),
    ))
}
