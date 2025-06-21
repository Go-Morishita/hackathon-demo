use sqlx::mysql::MySqlPoolOptions;
use sqlx::MySqlPool;
use dotenvy::dotenv;
use std::env;

/// 環境変数 DATABASE_URL を読み込み, MySQLコネクションプールを返す
pub async fn init_db_pool() -> MySqlPool {
    // .env ではなく Docker Compose の environment: からも読み込める
    dotenv().ok();
    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL must be set in environment");
    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to MySQL")
}
