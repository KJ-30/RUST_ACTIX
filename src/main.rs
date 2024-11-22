// 导入路由和实用工具模块
mod routes;
mod utils;
// 使用Actix Web框架的功能
use actix_web::{middleware::Logger, web, App, HttpServer};
// 使用Sea ORM进行数据库迁移
use migration::{Migrator, MigratorTrait};
// 使用Sea ORM的数据库连接功能
use sea_orm::Database;
// 使用标准库中的错误处理和格式化显示
use std::{error::Error, fmt::Display};
// 引入应用状态管理模块
use utils::api_state::AppState;
// 定义主错误类型，用于处理和显示程序中的错误
#[derive(Debug)]
struct MainError {
    message: String,
}
// 实现Display trait以支持MainError类型的格式化输出
impl Display for MainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Error{}", self.message)
    }
}
// 实现Error trait以支持MainError类型作为错误处理
impl Error for MainError {
    // 返回错误的源头，这里没有源头，所以返回None
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
    // 返回错误的描述信息，该信息是MainError实例的一部分
    fn description(&self) -> &str {
        &self.message
    }
    // 返回错误的直接原因，这里使用source方法表示没有更深层次的原因
    fn cause(&self) -> Option<&dyn Error> {
        self.source()
    }
}

// 主函数，异步运行HTTP服务器
#[actix_web::main] // or #[tokio::main]
async fn main() -> Result<(), MainError> {
    // 设置Rust日志环境变量，如果未设置的话
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "actix_web=info");
    }

    // 加载环境变量文件
    dotenv::dotenv().ok();

    // 初始化日志记录器
    env_logger::init();

    // 从环境变量中获取服务器地址、端口和数据库URL
    let address = (*utils::constants::ADDRESS).clone();
    let port = (*utils::constants::PORT).clone();
    let database_url = (*utils::constants::DATABASE_URL).clone();

    // 打印数据库URL以供调试
    println!("database_url: {:?}", database_url);

    // 连接数据库
    let db = Database::connect(database_url)
        .await
        .map_err(|err| MainError {
            message: err.to_string(),
        })?;

    // 执行数据库迁移
    Migrator::up(&db, None).await.map_err(|err| MainError {
        message: err.to_string(),
    })?;

    // 打印服务器地址和端口以供调试
    println!("port: {}", port);
    println!("address: {}", address);
    println!("db: {:?}", db);

    // 创建和运行HTTP服务器
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState { db: db.clone() }))
            .wrap(Logger::default())
            .configure(routes::home_routes::config)
            .configure(routes::auth_routes::config)
            .configure(routes::user_routes::config)
            .configure(routes::post_routes::config)
    })
    .bind((address, port))
    .map_err(|err| MainError {
        message: err.to_string(),
    })?
    .run()
    .await
    .map_err(|err| MainError {
        message: err.to_string(),
    })
}
