// 引入上层模块中的 handlers 和 middlewares 模块
use super::{handlers, middlewares};

// 引入 Actix Web 的 web 模块
use actix_web::web;

// 引入 Actix Web Lab 的中间件实用函数
use actix_web_lab::middleware::from_fn;

// 配置服务端点和中间件
//
// 该函数接收一个 mut 的 web::ServiceConfig 引用，用于配置服务端点和中间件
pub fn config(config: &mut web::ServiceConfig) {
    config
        // 配置 "/post" 路径下的服务端点
        .service(
            web::scope("/post")
                // 应用认证中间件以检查请求的合法性
                .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                // 注册处理创建帖子请求的端点
                .service(handlers::post_handler::create_post)
                // 注册处理获取当前用户帖子请求的端点
                .service(handlers::post_handler::get_my_post),
        )
        .service(
            web::scope("/post")
                // 注册处理获取所有帖子请求的端点
                .service(handlers::post_handler::get_all_post)
                // 注册处理获取单个帖子请求的端点
                .service(handlers::post_handler::get_one_post),
        );
}
