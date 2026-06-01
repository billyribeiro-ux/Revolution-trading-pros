//! Integration-test harness.
//!
//! Stage 1b: the admin/user token helpers no longer return the fake
//! strings `"test_admin_token"` / `"test_user_token"` (which the real
//! auth middleware could never accept, making every admin integration
//! test non-functional). They now seed a real `users` row and mint a
//! real HS256 JWT via the production `revolution_api::utils::create_jwt`
//! whose `sub` is that row's id — so the request flows through the
//! genuine `User` / `AdminUser` extractors against a real database.

use axum::response::Response;
use axum::Router;
use http_body_util::BodyExt;
use serde_json::Value;
use std::sync::Once;

use revolution_api::cache::{CacheInvalidator, CacheService};
use revolution_api::routes::realtime::EventBroadcaster;
use revolution_api::routes::websocket::WsConnectionManager;
use revolution_api::{config::Config, db::Database, routes, services::Services, AppState};

static INIT: Once = Once::new();

/// Initialize test environment variables once.
fn init_test_env() {
    INIT.call_once(|| {
        dotenvy::from_filename(".env.test").ok();
        dotenvy::dotenv().ok();

        if std::env::var("JWT_SECRET").is_err() {
            std::env::set_var(
                "JWT_SECRET",
                "test-jwt-secret-minimum-32-characters-for-testing",
            );
        }
        if std::env::var("DATABASE_URL").is_err() {
            std::env::set_var(
                "DATABASE_URL",
                "postgres://test:test@localhost:5432/test_db",
            );
        }
        if std::env::var("REDIS_URL").is_err() {
            std::env::set_var("REDIS_URL", "redis://localhost:6379");
        }
    });
}

/// JWT secret the harness signs with — the exact value the app verifies
/// against (`Config::jwt_secret`), so a minted token is genuinely valid.
fn jwt_secret() -> String {
    init_test_env();
    std::env::var("JWT_SECRET").expect("JWT_SECRET set by init_test_env")
}

/// Setup test application with all routes and a migrated schema.
pub async fn setup_test_app() -> Router {
    init_test_env();

    let config = Config::from_env().expect("Failed to load test config");

    let db = Database::new(&config)
        .await
        .expect("Failed to connect to test database");

    // Ensure the schema exists. `sqlx::migrate!().run()` takes an
    // advisory lock and skips already-applied migrations, so this is
    // safe to call from every test, including in parallel.
    db.migrate()
        .await
        .expect("Failed to run migrations on the test database");

    let services = Services::new(&config)
        .await
        .expect("Failed to initialize services");

    let event_broadcaster = EventBroadcaster::new();
    let ws_manager = WsConnectionManager::new();
    let cache = CacheService::new(None, None);
    let cache_invalidator = CacheInvalidator::new(cache.clone());

    let state = AppState {
        db,
        services,
        config,
        event_broadcaster,
        ws_manager,
        cache,
        cache_invalidator,
    };

    Router::new()
        .merge(routes::health::router())
        .nest("/api", routes::api_router())
        .with_state(state)
}

/// Idempotently seed a user with a real Argon2id password hash and the
/// given role; returns the row id. Safe under parallel tests (upsert on
/// the unique `email`).
pub async fn seed_user(pool: &sqlx::PgPool, email: &str, name: &str, role: &str) -> i64 {
    let password_hash = revolution_api::utils::hash_password("TestPassword123!@#")
        .expect("argon2 hashing must succeed");

    let row: (i64,) = sqlx::query_as(
        r"
        INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at)
        VALUES ($1, $2, $3, $4, NOW(), NOW(), NOW())
        ON CONFLICT (email) DO UPDATE SET
            role = EXCLUDED.role,
            password_hash = EXCLUDED.password_hash,
            email_verified_at = COALESCE(users.email_verified_at, NOW()),
            updated_at = NOW()
        RETURNING id
        ",
    )
    .bind(email)
    .bind(&password_hash)
    .bind(name)
    .bind(role)
    .fetch_one(pool)
    .await
    .expect("failed to seed test user");

    row.0
}

/// Mint a real, signed access JWT for `user_id` using the production
/// token factory and the secret the app verifies against.
pub fn mint_access_token(user_id: i64) -> String {
    revolution_api::utils::create_jwt(user_id, &jwt_secret(), 1)
        .expect("create_jwt must succeed for tests")
}

/// A small pool to the same database the app uses, for seeding rows.
/// `Router` does not expose its `AppState`, so seeding goes through an
/// independent connection to `DATABASE_URL` (the same value the app
/// reads), which is the intended single source of truth in tests.
async fn seeding_pool() -> sqlx::PgPool {
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL set by init_test_env");
    sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&url)
        .await
        .expect("failed to open seeding pool")
}

/// Setup test app + a seeded verified non-admin user (in-memory struct
/// for tests that only need credentials, not a live row).
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

/// Setup test app with a REAL admin user + a REAL signed admin JWT.
pub async fn setup_test_app_with_admin() -> (Router, String) {
    let app = setup_test_app().await;
    let pool = seeding_pool().await;
    let id = seed_user(
        &pool,
        "e2e-admin@revolutiontradingpros.test",
        "E2E Admin",
        "admin",
    )
    .await;
    (app, mint_access_token(id))
}

/// Setup test app with a REAL non-admin user + a REAL signed JWT.
pub async fn setup_test_app_with_user_token() -> (Router, String) {
    let app = setup_test_app().await;
    let pool = seeding_pool().await;
    let id = seed_user(
        &pool,
        "e2e-member@revolutiontradingpros.test",
        "E2E Member",
        "user",
    )
    .await;
    (app, mint_access_token(id))
}

/// Convert response body to JSON.
pub async fn body_to_json(response: Response) -> Value {
    let body = response.into_body();
    let bytes = body.collect().await.unwrap().to_bytes();
    serde_json::from_slice(&bytes).unwrap()
}

/// Test user struct.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct TestUser {
    pub id: i64,
    pub email: String,
    pub name: String,
    pub role: Option<String>,
}
