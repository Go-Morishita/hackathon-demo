use crate::{auth, handlers, models::AppState};
use axum::{
    middleware,
    routing::{get, post},
    Router,
};

pub fn create_router(state: AppState) -> Router {
    let api_routes = Router::new()
        .route("/users", post(handlers::create_user))
        .route("/users", get(handlers::get_all_user))
        .route("/users/{id}", get(handlers::get_user))
        .layer(middleware::from_fn(auth::require_api_key));

    Router::new()
        .route("/", get(handlers::root_handler))
        .merge(api_routes)
        .with_state(state)
}
