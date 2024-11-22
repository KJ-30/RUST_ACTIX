// 导入lazy_static宏和环境变量处理模块
use lazy_static::lazy_static;
use std::env;

// 使用lazy_static宏来延迟静态变量的初始化，确保在第一次使用时才进行初始化
lazy_static! {
    // 定义静态引用变量ADDRESS，存储服务器地址
    pub static ref ADDRESS: String = set_address();
    // 定义静态引用变量PORT，存储服务器端口
    pub static ref PORT: u16 = set_port();
    // 定义静态引用变量DATABASE_URL，存储数据库连接URL
    pub static ref DATABASE_URL: String = set_database_url();
    // 定义静态引用变量JWT_SECRET，存储JWT密钥
    pub static ref JWT_SECRET: String = set_jwt_secret();
    // 定义静态引用变量MAX_FILE_SIZE，存储允许的最大文件大小
    pub static ref MAX_FILE_SIZE: u64 = set_max_file_size();
}

// 设置服务器地址的函数
fn set_address() -> String {
    // 加载环境变量文件，如果失败则忽略错误
    dotenv::dotenv().ok();
    // 从环境变量中获取ADDRESS的值，如果未设置，则默认为"localhost"
    env::var("ADDRESS").unwrap_or("localhost".to_owned())
}

// 设置服务器端口的函数
fn set_port() -> u16 {
    // 加载环境变量文件，如果失败则忽略错误
    dotenv::dotenv().ok();
    // 从环境变量中获取PORT的值，如果未设置，则默认为"5050"，并将其解析为u16类型
    std::env::var("PORT")
        .unwrap_or("5050".to_owned())
        .parse::<u16>()
        .expect("Can't parse the port")
}

// 设置数据库连接URL的函数
fn set_database_url() -> String {
    // 加载环境变量文件，如果失败则忽略错误
    dotenv::dotenv().ok();
    // 从环境变量中获取DATABASE_URL的值，如果未设置，则期望为"postgres://postgres:665712@localhost:5432/NewBlogDB"
    env::var("DATABASE_URL").expect("postgres://postgres:665712@localhost:5432/NewBlogDB")
}

// 设置JWT密钥的函数
fn set_jwt_secret() -> String {
    // 加载环境变量文件，如果失败则忽略错误
    dotenv::dotenv().ok();
    // 从环境变量中获取JWT_SECRET的值，如果未设置，则期望为"secret"
    env::var("JWT_SECRET").expect("secret")
}

// 设置允许的最大文件大小的函数
fn set_max_file_size() -> u64 {
    // 加载环境变量文件，如果失败则忽略错误
    dotenv::dotenv().ok();
    // 从环境变量中获取MAX_FILE_SIZE的值，如果未设置，则默认为"5050"，并将其解析为u64类型
    env::var("MAX_FILE_SIZE")
        .unwrap_or("5050".to_owned())
        .parse::<u64>()
        .expect("Cant parse the max file size")
}