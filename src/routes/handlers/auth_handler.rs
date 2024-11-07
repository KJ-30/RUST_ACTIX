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
#[derive(Serialize, Deserialize)]
struct RegisterJson {
    name: String,
    email: String,
    password: String,
}
#[derive(Serialize, Deserialize)]
struct LoginJson {
    email: String,
    password: String,
}
#[post("/register")]
pub async fn register(
    app_state: web::Data<api_state::AppState>,
    register_json: web::Json<RegisterJson>,
) -> Result<ApiResponse, ApiResponse> {
    let user_model = entity::user::ActiveModel {
        name: Set(register_json.name.clone()),
        email: Set(register_json.email.clone()),
        password: Set(digest(&register_json.password)),
        ..Default::default()
    }
    .insert(&app_state.db)
    .await
    .map_err(|err| ApiResponse::new(500, err.to_string()))?;

    Ok(ApiResponse::new(200, format!("{}", user_model.id)))
}

#[post("/login")]
/// 异步登录函数
///
/// 该函数接收登录信息的JSON对象和应用状态对象作为参数，
/// 并尝试根据提供的电子邮件和密码对用户进行身份验证。
/// 如果用户存在且密码匹配，则生成一个JWT令牌并返回。
///
/// # 参数
///
/// * `login_json` - 包含登录信息的JSON对象，包括电子邮件和密码。
/// * `app_state` - 应用状态对象，包含数据库连接等信息。
///
/// # 返回
///
/// * 成功时 - 返回包含JWT令牌的响应。
/// * 失败时 - 返回适当的错误响应，如用户未找到或数据库查询错误。
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
