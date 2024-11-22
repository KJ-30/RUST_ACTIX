// 引入sea_orm库中的DatabaseConnection类型，用于数据库连接
use sea_orm::DatabaseConnection;

/// AppState 结构体定义了应用的状态。
/// 它包含一个数据库连接，使得应用可以与数据库进行交互。
pub struct AppState {
    /// 数据库连接实例，通过它可以执行数据库操作。
    pub db: DatabaseConnection,
}
