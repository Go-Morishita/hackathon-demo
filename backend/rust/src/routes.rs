use crate::{handlers, models::AppState};
use axum::{
    routing::{get, post},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::root_handler))
        .route("/users", post(handlers::create_user))
        .route("/users/{id}", get(handlers::get_user))
        .with_state(state)
}
