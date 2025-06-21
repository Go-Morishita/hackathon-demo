use dotenvy::dotenv;
use sqlx::{mysql::MySqlPoolOptions, MySqlPool};

/// .env / 環境変数 `DATABASE_URL` から MySQL プールを生成
pub async fn init_db_pool() -> MySqlPool {
    dotenv().ok();                                       // .env 読込
    let url = std::env::var("DATABASE_URL")
        .expect("環境変数 DATABASE_URL が未設定です");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("MySQL への接続に失敗しました")
}
