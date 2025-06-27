use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde_json::json;
use crate::models::{AppState, CreateUserRequest, UserResponse};
use sqlx::{query, query_as, mysql::MySqlQueryResult};

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let result: MySqlQueryResult = query(
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

    let id = result.last_insert_id();

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": format!("User with id {} created successfully", id)
        })),
    ))
}

pub async fn get_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, StatusCode> {
    let rec = query_as::<_, UserResponse>(
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

pub async fn get_all_users(
    State(state): State<AppState>
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    let users = query_as::<_, UserResponse>(
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

pub async fn update_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let result = query(
        "UPDATE users SET name = ?, email = ? WHERE id = ?",
    )
    .bind(&payload.name)
    .bind(&payload.email)
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("DB update error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": format!("User with id {} updated successfully", id)
        })),
    ))
}

pub async fn delete_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<serde_json::Value>), StatusCode> {
    let result = query(
        "DELETE FROM users WHERE id = ?",
    )
    .bind(id)
    .execute(&state.pool)
    .await
    .map_err(|e| {
        eprintln!("DB delete error: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok((
        StatusCode::OK,
        Json(json!({
            "message": format!("User with id {} deleted successfully", id)
        })),
    ))
}
