use serde::{Deserialize, Serialize};
use sqlx::MySqlPool;

/// アプリ全体の共有状態
#[derive(Clone)]
pub struct AppState {
    pub pool: MySqlPool,
}

/* ---- DTO ---- */

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
}

#[derive(Serialize, Debug, sqlx::FromRow)]
pub struct UserResponse {
    pub id: u64,
    pub name: String,
    pub email: String,
}
