use crate::utils::{api_response, api_state::AppState};
use actix_web::{get, web, Responder};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};

// 定义User结构体，用于序列化和反序列化用户信息
#[derive(Serialize, Deserialize)]
struct User {
    id: i32,
    name: String,
    email: String,
}

// 定义UserList结构体，包含一个用户列表
#[derive(Serialize, Deserialize)]
struct UserList {
    users: Vec<User>,
}

// 定义一个GET路由，用于问候用户
#[get("/hello/{name}")]
pub async fn greet(name: web::Path<String>) -> impl Responder {
    api_response::ApiResponse::new(200, format!("Hello {}!", name))
}

// 定义一个GET路由，用于测试API响应
#[get("/test")]
/// 异步测试函数，用于获取用户列表
///
/// 此函数从数据库中检索所有用户信息，并将其转换为JSON格式的响应返回
/// 它利用了Actix Web框架的`web::Data`来共享应用状态，包括数据库连接等信息
///
/// # Arguments
/// * `api_state` - 一个包含应用状态的`web::Data`实例，用于访问数据库等资源
///
/// # Returns
/// * `Result<api_response::ApiResponse, api_response::ApiResponse>` - 返回一个结果，包含成功或失败的`ApiResponse`实例
///   成功时，返回状态码200和用户列表的JSON字符串；失败时，返回相应的错误信息
pub async fn test(
    api_state: web::Data<AppState>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    // 从数据库中查找所有用户
    let users: Vec<User> = entity::user::Entity::find()
        .all(&api_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|user| User {
            id: user.id,
            name: user.name,
            email: user.email,
        })
        .collect();

    // 将用户列表封装到`UserList`结构中
    let user_list = UserList { users };
    // 将用户列表序列化为JSON字符串
    let user_list_json = serde_json::to_string(&user_list).unwrap();

    // 返回包含用户列表JSON字符串的成功响应
    Ok(api_response::ApiResponse::new(200, user_list_json))
}
