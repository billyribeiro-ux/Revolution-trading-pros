//! Test utilities and helpers
//! ICT 11+ Principal Engineer Grade

use axum::response::Response;
use axum::{body::Body, Router};
use http_body_util::BodyExt;
use serde_json::Value;

/// Setup test application with all routes
pub async fn setup_test_app() -> Router {
    // Load test environment
    dotenvy::from_filename(".env.test").ok();

    // This would normally initialize the full app
    // For now, return a minimal router for compilation
    Router::new()
}

/// Setup test app with a verified user
pub async fn setup_test_app_with_user() -> (Router, TestUser, String) {
    let app = setup_test_app().await;
    let password = "TestPassword123!@#";
    let user = TestUser {
        id: 1,
        email: format!("test_{}@example.com", uuid::Uuid::new_v4()),
        name: "Test User".to_string(),
        role: Some("user".to_string()),
    };

    (app, user, password.to_string())
}

/// Setup test app with admin user and token
pub async fn setup_test_app_with_admin() -> (Router, String) {
    let app = setup_test_app().await;
    let admin_token = "test_admin_token";

    (app, admin_token.to_string())
}

/// Setup test app with regular user and token
pub async fn setup_test_app_with_user_token() -> (Router, String) {
    let app = setup_test_app().await;
    let user_token = "test_user_token";

    (app, user_token.to_string())
}

/// Convert response body to JSON
pub async fn body_to_json(response: Response) -> Value {
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

/// Test user struct
#[derive(Debug, Clone)]
pub struct TestUser {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: Option<String>,
}
