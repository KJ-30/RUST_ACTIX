// 引入上层模块中的 handlers 和 middlewares 模块
use super::{handlers, middlewares};

// 引入 Actix Web 的 web 模块
use actix_web::web;

// 引入 Actix Web Lab 的中间件工具，用于从函数创建中间件
use actix_web_lab::middleware::from_fn;

// 配置服务端点和中间件
// 该函数接收一个 web::ServiceConfig 的可变引用作为参数，用于配置服务
pub fn config(config: &mut web::ServiceConfig) {
    // 配置一个名为 "/user" 的作用域，并在其上应用认证中间件
    // 使用 from_fn 函数从中间件函数创建一个 Actix Web Lab 中间件
    config.service(
        web::scope("/user")
            .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
            // 在该作用域下添加用户信息处理的服务
            .service(handlers::user_handler::user)
            // 在该作用域下添加更新用户信息的服务
            .service(handlers::user_handler::update_user),
    );
}
