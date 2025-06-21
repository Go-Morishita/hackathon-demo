use axum::{
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};
use sqlx::{mysql::MySqlPoolOptions, MySql, MySqlPool};

// ---------- DB 初期化 --------------------------------------------------------

/// 環境変数 `DATABASE_URL` から MySQL へ接続するプールを生成
async fn init_db_pool() -> MySqlPool {
    // .env も Docker の environment: も両方読める
    dotenv().ok();
    let url = std::env::var("DATABASE_URL")
        .expect("環境変数 DATABASE_URL が設定されていません");

    MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .expect("MySQL への接続に失敗しました")
}

// ---------- リクエスト／レスポンス型 ----------------------------------------

#[derive(Serialize, Deserialize, Debug)]
struct CreateUserRequest {
    name: String,
    email: String,
}

#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
    email: String,
}

// ---------- ルートハンドラ ---------------------------------------------------

/// POST /users  
/// ユーザを新規作成して JSON で返す
async fn create_user(
    State(pool): State<MySqlPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    // DB に挿入
    let result = sqlx::query!(
        r#"INSERT INTO users (name, email) VALUES (?, ?)"#,
        payload.name,
        payload.email
    )
    .execute(&pool)
    .await
    .map_err(|e| {
        eprintln!("DB insert error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = UserResponse {
        id: result.last_insert_id(),
        name: payload.name,
        email: payload.email,
    };

    Ok((StatusCode::CREATED, Json(user)))
}

/// GET /users/{id}  
/// ID でユーザを取得して JSON で返す
async fn get_user(
    Path(id): Path<u64>,
    State(pool): State<MySqlPool>,
) -> Result<Json<UserResponse>, StatusCode> {
    let rec = sqlx::query_as!(
        UserResponse,
        r#"
        SELECT
            id   AS "id!: u64",
            name AS "name!",
            email
        FROM users
        WHERE id = ?
        "#,
        id
    )
    .fetch_optional(&pool)
    .await
    .map_err(|e| {
        eprintln!("DB select error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match rec {
        Some(user) => Ok(Json(user)),
        None => Err(StatusCode::NOT_FOUND),
    }
}

/// GET /  
async fn root_handler() -> &'static str {
    "Hello World!!!!!!!"
}

// ---------- エントリポイント --------------------------------------------------

#[tokio::main]
async fn main() {
    // DB プールを初期化して共有状態に入れる
    let pool = init_db_pool().await;

    // ルータ定義
    let app = Router::new()
        .route("/", get(root_handler))
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .with_state(pool); // ← ここでプールを共有

    // サーバ起動
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080")
        .await
        .expect("ポート 8080 をバインドできませんでした");
    println!("listening on {}", listener.local_addr().unwrap());

    axum::serve(listener, app).await.unwrap();
}
