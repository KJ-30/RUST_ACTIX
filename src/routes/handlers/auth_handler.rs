use crate::utils::{api_response::ApiResponse, api_state, jwt::encode_jwt};
use actix_web::{post, web, Responder};
use sea_orm::ActiveModelTrait;
use sea_orm::ColumnTrait;
use sea_orm::Condition;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::Set;
use serde::{Deserialize, Serialize};
use sha256::digest;

// 注册请求的JSON结构
#[derive(Serialize, Deserialize)]
struct RegisterJson {
    name: String,
    email: String,
    password: String,
}

// 登录请求的JSON结构
#[derive(Serialize, Deserialize)]
struct LoginJson {
    email: String,
    password: String,
}

// 注册用户的异步函数
#[post("/register")]
pub async fn register(
    app_state: web::Data<api_state::AppState>,
    register_json: web::Json<RegisterJson>,
) -> Result<ApiResponse, ApiResponse> {
    // 创建用户模型并插入数据库
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    // 返回用户ID
    Ok(ApiResponse::new(200, format!("{}", user_model.id)))
}

// 登录用户的异步函数
#[post("/login")]
pub async fn login(
    login_json: web::Json<LoginJson>,
    app_state: web::Data<api_state::AppState>,
) -> Result<impl Responder, ApiResponse> {
    // 根据提供的电子邮件和密码尝试从数据库中查找用户
    let user_data = entity::user::Entity::find()
        .filter(
            Condition::all()
                .add(entity::user::Column::Email.eq(&login_json.email))
                .add(entity::user::Column::Password.eq(digest(&login_json.password))),
        )
        .one(&app_state.db)
        .await
        // 如果数据库查询失败，返回500错误码和错误信息
        .map_err(|err| ApiResponse::new(500, err.to_string()))?
        // 如果用户未找到，返回404错误码和错误信息
        .ok_or(ApiResponse::new(404, "User Not Found".to_owned()))?;

    // 为找到的用户生成JWT令牌
    let token = encode_jwt(user_data.email, user_data.id)
        // 如果令牌生成失败，返回500错误码和错误信息
        .map_err(|err| ApiResponse::new(500, err.to_string()))?;
    // 打印生成的令牌（用于调试目的）
    println!("token: {}", token);
    // 返回包含生成令牌的响应
    Ok(ApiResponse::new(200, format!("{{ 'token':'{}' }}", token)))
}