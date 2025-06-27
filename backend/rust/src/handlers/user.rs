use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::models::{AppState, CreateUserRequest, UserResponse};
use sqlx::mysql::MySqlQueryResult;

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    let result: MySqlQueryResult = sqlx::query(
        "INSERT INTO users (name, email) VALUES (?, ?)",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .execute(&state.pool)
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

pub async fn get_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, StatusCode> {
    let rec = sqlx::query_as::<_, UserResponse>(
        "SELECT id, name, email FROM users WHERE id = ?",
    )
    .bind(id)
    .fetch_optional(&state.pool)
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

pub async fn get_all_user(
    State(state): State<AppState>
) -> Result<Json<UserResponse>, StatusCode> {
    let rec = sqlx::query_as::<_, UserResponse>(
        "SELECT id, name, email FROM users",
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("DB select error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(users))
}
