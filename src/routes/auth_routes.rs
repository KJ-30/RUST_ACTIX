// 导入上层命名空间中的handlers模块
use super::handlers;
// 导入Actix Web框架的web模块
use actix_web::web;

/// 配置路由和处理器
///
/// # Parameters
///
/// * `config`: 一个可变引用到`web::ServiceConfig`，用于配置HTTP服务
pub fn config(config: &mut web::ServiceConfig) {
    // 配置"/auth"范围下的服务，包括注册和登录
    config.service(
        web::scope("/auth")
            .service(handlers::auth_handler::register) // 注册服务
            .service(handlers::auth_handler::login), // 登录服务
    );
}
