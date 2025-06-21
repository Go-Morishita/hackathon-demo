use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use crate::models::{AppState, CreateUserRequest, UserResponse};

/// POST /users
pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<UserResponse>), StatusCode> {
    // 挿入
    let result = sqlx::query!(
        r#"INSERT INTO users (name, email) VALUES (?, ?)"#,
        payload.name,
        payload.email
    )
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

/// GET /users/:id
pub async fn get_user(
    Path(id): Path<u64>,
    State(state): State<AppState>,
) -> Result<Json<UserResponse>, StatusCode> {
    let rec = sqlx::query_as!(
        UserResponse,
        r#"
        SELECT
            id   AS "id!: u64",
            name AS "name!",
            email AS "email!"
        FROM users
        WHERE id = ?
        "#,
        id
    )
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
