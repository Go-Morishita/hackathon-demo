use axum::{
    http::StatusCode,
    routing::{get, post}, 
    extract::Path, 
    Json,
    Router
};
use serde::{Deserialize, Serialize};

// 'Deserialize' を追加するとJSONからこの構造体に変換できる
#[derive(Serialize, Deserialize, Debug)]
struct CreateUserRequest{
    name: String,
    email: String,
}

// POSTリクエストを受け取るハンドラー
// Json(payload) でリクエストボディを受け取る
async fn create_user(
    Json(payload): Json<CreateUserRequest>,
) -> (StatusCode, Json<UserResponse>){
    // 実際にはここでデータベースに保存する
    println!("Received user: {:?}", payload);

    let user = UserResponse{
        id: 1337,
        name: payload.name,
        email: payload.email,
    };

    // ステータスコード201（Created）とユーザー情報を返す
    (StatusCode::CREATED, Json(user))
}

// 作成したユーザー情報を返すための構造体
#[derive(Serialize)]
struct UserResponse {
    id: u64,
    name: String,
    email: String,
}

// `#[derive(Serialize)]` をつけるとJSONに変換可能になる
#[derive(Serialize)]
struct User {
    id: u32,
    name: String,
    email: String,
}

async fn create_user_json() -> Json<User> {
    let user = User {
        id: 1,
        name: "Aoyama Taro".to_string(),
        email: "taro@example.com".to_string(),
    };
    Json(user)
}

#[tokio::main]
async fn main() {
    async fn root_handler() -> String{
        format!("Hello World!")
    }
    

    async fn user_handler(Path(user_id): Path<String>) -> String{
        format!("user id is: {}", user_id)
    } 

    // ルートを定義
    let app = Router::new().route("/", get(root_handler)).route("/users/{id}", get(user_handler)).route("/json-user", get(create_user_json));

    // 指定したポートにサーバーを開く
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
