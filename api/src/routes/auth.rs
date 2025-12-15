//! Authentication routes

use axum::{
    extract::State,
    http::StatusCode,
    routing::post,
    Json, Router,
};
use serde_json::json;

use crate::{
    models::{AuthResponse, CreateUser, LoginUser, User, UserResponse},
    utils::{hash_password, verify_password, create_jwt},
    AppState,
};

/// Register a new user
async fn register(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Check if email exists
    let existing: Option<User> = sqlx::query_as(
        "SELECT * FROM users WHERE email = $1"
    )
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    if existing.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "Email already registered"})),
        ));
    }

    // Hash password
    let password_hash = hash_password(&input.password)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    // Create user
    let user: User = sqlx::query_as(
        r#"
        INSERT INTO users (id, email, password_hash, name, role, created_at, updated_at)
        VALUES ($1, $2, $3, $4, 'user', NOW(), NOW())
        RETURNING *
        "#,
    )
        .bind(uuid::Uuid::new_v4())
        .bind(&input.email)
        .bind(&password_hash)
        .bind(&input.name)
        .fetch_one(&state.db.pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?;

    // Create JWT
    let token = create_jwt(&user.id, &state.config.jwt_secret, state.config.jwt_expires_in)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// Login user
async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginUser>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<serde_json::Value>)> {
    // Find user
    let user: User = sqlx::query_as(
        "SELECT * FROM users WHERE email = $1"
    )
        .bind(&input.email)
        .fetch_optional(&state.db.pool)
        .await
        .map_err(|e| {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()})))
        })?
        .ok_or_else(|| {
            (StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"})))
        })?;

    // Verify password
    if !verify_password(&input.password, &user.password_hash)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?
    {
        return Err((StatusCode::UNAUTHORIZED, Json(json!({"error": "Invalid credentials"}))));
    }

    // Create JWT
    let token = create_jwt(&user.id, &state.config.jwt_secret, state.config.jwt_expires_in)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": e.to_string()}))))?;

    Ok(Json(AuthResponse {
        token,
        user: user.into(),
    }))
}

/// Get current user (requires auth)
async fn me(
    State(state): State<AppState>,
    user: User, // Extracted by auth middleware
) -> Json<UserResponse> {
    Json(user.into())
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        // .route("/me", get(me).layer(auth_layer))
}
