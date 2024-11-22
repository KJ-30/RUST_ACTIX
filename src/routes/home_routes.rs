// 引入上层模块中的handlers模块
use super::handlers;
// 引入Actix Web的web模块以使用其功能
use actix_web::web;

/// 配置应用程序的服务
///
/// # Parameters
///
/// * `config`: 一个可变引用到`web::ServiceConfig`，用于配置应用程序的服务
pub fn config(config: &mut web::ServiceConfig) {
    // 配置一个名为"/home"的范围，并在其下注册服务
    config.service(
        web::scope("/home")
            // 在"/home"范围内注册`greet`服务，处理对应的HTTP请求
            .service(handlers::home_handler::greet)
            // 在"/home"范围内注册`test`服务，处理对应的HTTP请求
            .service(handlers::home_handler::test),
    );
}
