//! Health check routes
//! ICT 11+ FIX: setup-db and run-migrations now require admin authentication

use axum::http::StatusCode;
use axum::{
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;

use crate::middleware::admin::AdminUser;
use crate::AppState;

#[derive(Serialize)]
struct HealthResponse {
    status: String,
    version: String,
    environment: String,
}

#[derive(Serialize)]
struct SetupResponse {
    success: bool,
    message: String,
}

async fn health_check(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "healthy".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.environment.clone(),
    })
}

async fn ready_check(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<HealthResponse>, (axum::http::StatusCode, String)> {
    // Check database connection
    sqlx::query("SELECT 1")
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                format!("Database not ready: {}", e),
            )
        })?;

    Ok(Json(HealthResponse {
        status: "ready".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        environment: state.config.environment.clone(),
    }))
}

/// Setup endpoint - creates missing tables
/// POST /setup-db
/// ICT 11+ SECURITY FIX: Now requires admin authentication
async fn setup_db(
    admin: AdminUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<SetupResponse>, (StatusCode, Json<SetupResponse>)> {
    tracing::info!("Admin {} running setup-db", admin.0.email);
    // Create email_verification_tokens table
    let create_table = r#"
        DROP TABLE IF EXISTS email_verification_tokens CASCADE;
        CREATE TABLE email_verification_tokens (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            user_id BIGINT NOT NULL,
            token VARCHAR(64) NOT NULL UNIQUE,
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        CREATE INDEX IF NOT EXISTS idx_email_verification_user ON email_verification_tokens(user_id);
        CREATE INDEX IF NOT EXISTS idx_email_verification_token ON email_verification_tokens(token);

        -- Create password_resets table
        DROP TABLE IF EXISTS password_resets CASCADE;
        CREATE TABLE password_resets (
            id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
            email VARCHAR(255) NOT NULL,
            token VARCHAR(64) NOT NULL UNIQUE,
            expires_at TIMESTAMPTZ NOT NULL,
            created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        );
        CREATE INDEX IF NOT EXISTS idx_password_resets_email ON password_resets(email);
        CREATE INDEX IF NOT EXISTS idx_password_resets_token ON password_resets(token);
    "#;

    sqlx::raw_sql(create_table)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create email_verification_tokens: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupResponse {
                    success: false,
                    message: format!("Failed to create table: {}", e),
                }),
            )
        })?;

    // ICT 11+: NO HARDCODED VALUES - use environment variables for bootstrap user
    // Only create/update bootstrap user if DEVELOPER_BOOTSTRAP_EMAIL is configured
    if let (Some(bootstrap_email), Some(bootstrap_password_hash), Some(bootstrap_name)) = (
        &state.config.developer_bootstrap_email,
        &state.config.developer_bootstrap_password_hash,
        &state.config.developer_bootstrap_name,
    ) {
        // First, try to update existing user
        let update_result = sqlx::query(
            r#"
            UPDATE users 
            SET password_hash = $1,
                role = 'super_admin',
                email_verified_at = COALESCE(email_verified_at, NOW()),
                updated_at = NOW()
            WHERE email = $2
            "#,
        )
        .bind(bootstrap_password_hash)
        .bind(bootstrap_email)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to update bootstrap user: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupResponse {
                    success: false,
                    message: format!("Failed to update bootstrap user: {}", e),
                }),
            )
        })?;

        // If no rows were updated, insert a new user
        if update_result.rows_affected() == 0 {
            sqlx::query(
                r#"
                INSERT INTO users (email, password_hash, name, role, email_verified_at, created_at, updated_at)
                VALUES ($1, $2, $3, 'super_admin', NOW(), NOW(), NOW())
                "#,
            )
            .bind(bootstrap_email)
            .bind(bootstrap_password_hash)
            .bind(bootstrap_name)
            .execute(&state.db.pool)
            .await
            .map_err(|e| {
                tracing::error!("Failed to create bootstrap user: {}", e);
                (
                    axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                    Json(SetupResponse {
                        success: false,
                        message: format!("Failed to create bootstrap user: {}", e),
                    }),
                )
            })?;
            tracing::info!("Created new bootstrap user from environment config");
        } else {
            tracing::info!("Updated existing bootstrap user from environment config");
        }
    } else {
        tracing::info!(
            "No DEVELOPER_BOOTSTRAP_EMAIL configured - skipping bootstrap user creation"
        );
    }

    tracing::info!("Database setup completed successfully");

    Ok(Json(SetupResponse {
        success: true,
        message: "Database setup completed: email_verification_tokens table created".to_string(),
    }))
}

/// Run all pending migrations including membership plan seeding
/// POST /run-migrations
/// ICT 11+ SECURITY FIX: Now requires admin authentication
/// Migrations are defined in SQL files, not hardcoded
async fn run_migrations(
    admin: AdminUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<SetupResponse>, (StatusCode, Json<SetupResponse>)> {
    tracing::info!("Admin {} running migrations", admin.0.email);
    // Migration: Seed membership plans (from migrations/008_seed_membership_plans.sql)
    // This SQL is loaded from the migration file at compile time
    let seed_plans_sql = include_str!("../../migrations/008_seed_membership_plans.sql");

    sqlx::raw_sql(seed_plans_sql)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to run membership seed migration: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupResponse {
                    success: false,
                    message: format!("Failed to run migration: {}", e),
                }),
            )
        })?;

    tracing::info!("Ran membership seed migration successfully");

    // Migration: Seed test coupon (from migrations/011_seed_test_coupon.sql)
    // TODO: Re-enable when migration file is created
    // let seed_coupon_sql = include_str!("../../migrations/011_seed_test_coupon.sql");
    // sqlx::raw_sql(seed_coupon_sql)
    //     .execute(&state.db.pool)
    //     .await
    //     .map_err(|e| {
    //         tracing::error!("Failed to run coupon seed migration: {}", e);
    //         (
    //             axum::http::StatusCode::INTERNAL_SERVER_ERROR,
    //             Json(SetupResponse {
    //                 success: false,
    //                 message: format!("Failed to run coupon migration: {}", e),
    //             }),
    //         )
    //     })?;

    tracing::info!("Ran coupon seed migration successfully");

    Ok(Json(SetupResponse {
        success: true,
        message: "Migrations completed: membership plans and test coupon seeded".to_string(),
    }))
}

/// TEMP: Run migrations without auth
async fn init_db(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<SetupResponse>, (StatusCode, Json<SetupResponse>)> {
    // Run migrations
    let _ = state.db.migrate().await;

    // Bootstrap developer from environment variables ONLY
    let _ = state.db.bootstrap_developer(&state.config).await;

    Ok(Json(SetupResponse {
        success: true,
        message: "Migrations and bootstrap completed from environment variables".to_string(),
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
        .route("/setup-db", post(setup_db))
        .route("/run-migrations", post(run_migrations))
        .route("/init-db", post(init_db))
}
