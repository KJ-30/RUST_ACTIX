use crate::utils::{api_response, api_state, jwt::Claims};
use actix_web::{get, post, web};
use sea_orm::EntityTrait;
use sea_orm::{ActiveModelTrait, IntoActiveModel, Set};
use serde::{Deserialize, Serialize};

// 定义更新用户信息的请求模型
#[derive(Serialize, Deserialize)]
struct UpdateUserModel {
    name: String,
}

// 获取用户信息的API路由
#[get("")] // 路由注解，指定HTTP GET方法和路径
pub async fn user(
    app_state: web::Data<api_state::AppState>, // 应用状态数据，包含数据库连接等
    claim_data: Claims,                        // JWT令牌中的用户声明数据
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    // 根据用户ID查询数据库中的用户记录
    let user_model = entity::user::Entity::find_by_id(claim_data.id)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User Not Found".to_owned(),
        ))?;

    // 返回用户信息的JSON响应
    Ok(api_response::ApiResponse::new(
        200,
        format!(
            " {{ 'name': '{}', 'email': '{}' }} ",
            user_model.name, user_model.email
        ),
    ))
}

// 更新用户信息的API路由
#[post("/update")] // 路由注解，指定HTTP POST方法和路径
pub async fn update_user(
    api_state: web::Data<api_state::AppState>, // 应用状态数据，包含数据库连接等
    use_data: web::Json<UpdateUserModel>,      // 更新用户的请求数据
    claims_data: Claims,                       // JWT令牌中的用户声明数据
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    // 根据用户ID查询数据库中的用户记录，并转换为可更新的模型
    let mut user_model = entity::user::Entity::find_by_id(claims_data.id)
        .one(&api_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .ok_or(api_response::ApiResponse::new(
            404,
            "User Not Found".to_owned(),
        ))?
        .into_active_model();

    // 更新用户名称
    user_model.name = Set(use_data.name.clone());
    // 将更新后的用户模型保存到数据库
    user_model
        .update(&api_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;
    // 返回用户更新成功的响应
    Ok(api_response::ApiResponse::new(
        200,
        "User Updated".to_owned(),
    ))
}
