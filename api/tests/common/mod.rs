//! Test utilities and helpers
//! ICT 11+ Principal Engineer Grade

use axum::response::Response;
use axum::{body::Body, Router};
use http_body_util::BodyExt;
use serde_json::Value;

use revolution_api::routes::realtime::EventBroadcaster;
use revolution_api::{config::Config, db::Database, routes, services::Services, AppState};

/// Setup test application with all routes
pub async fn setup_test_app() -> Router {
    // Load test environment
    dotenvy::from_filename(".env.test").ok();
    dotenvy::dotenv().ok();

    // Create test config
    let config = Config::from_env().expect("Failed to load test config");

    // Create database connection
    let db = Database::new(&config)
        .await
        .expect("Failed to connect to test database");

    // Create services (will use mock/dev mode where possible)
    let services = Services::new(&config)
        .await
        .expect("Failed to initialize services");

    // Create event broadcaster
    let event_broadcaster = EventBroadcaster::new();

    // Create app state
    let state = AppState {
        db,
        services,
        config,
        event_broadcaster,
    };

    // Build router with actual routes
    Router::new()
        .merge(routes::health::router())
        .nest("/api", routes::api_router())
        .with_state(state)
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
