use axum::{
    extract::Request,
    http::{HeaderValue, StatusCode},
    middleware::Next,
    response::Response,
};
use std::env;

pub async fn require_api_key(request: Request, next: Next) -> Result<Response, StatusCode> {
    const API_KEY_HEADER_NAME: &str = "API-KEY";

    let provided_key = request
        .headers()
        .get(API_KEY_HEADER_NAME)
        .and_then(|value| value.to_str().ok());

    let expected_key = env::var("API-KEY").expect("環境変数 'API-KEY' が設定されていません。");

    println!("Expected: {:?}", env::var("API-KEY"));
    println!("Received: {:?}", request.headers().get("API-KEY"));


    match provided_key {
        Some(key) if key == expected_key => {
            Ok(next.run(request).await)
        }
        _ => {
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}