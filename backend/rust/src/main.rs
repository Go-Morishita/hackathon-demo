mod db;
mod models;
mod handlers;
mod routes;

use models::AppState;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    // DB プール & 共有状態
    let pool  = db::init_db_pool().await;
    let state = AppState { pool };

    // ルータ生成
    let app = routes::create_router(state);

    // サーバ起動
    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();
    println!("listening on http://{addr}");

    axum::serve(
        tokio::net::TcpListener::bind(addr).await.unwrap(),
        app,
    )
    .await
    .unwrap();
}
