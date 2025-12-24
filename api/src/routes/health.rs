//! Health check routes

use axum::{routing::{get, post}, Json, Router};
use serde::Serialize;

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
async fn setup_db(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<SetupResponse>, (axum::http::StatusCode, Json<SetupResponse>)> {
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

    // First, try to update existing superadmin
    let update_result = sqlx::query(
        r#"
        UPDATE users 
        SET password = '$2b$10$ZVtDbp8nFLBzi4LTpMiiqe33JlwMdDmPf9.yguzf09cH1iDthQi16',
            role = 'super_admin',
            email_verified_at = COALESCE(email_verified_at, NOW()),
            updated_at = NOW()
        WHERE email = 'welberribeirodrums@gmail.com'
        "#,
    )
    .execute(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update superadmin: {}", e);
        (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            Json(SetupResponse {
                success: false,
                message: format!("Failed to update superadmin: {}", e),
            }),
        )
    })?;

    // If no rows were updated, insert a new superadmin
    if update_result.rows_affected() == 0 {
        sqlx::query(
            r#"
            INSERT INTO users (email, password, name, role, email_verified_at, created_at, updated_at)
            VALUES (
                'welberribeirodrums@gmail.com',
                '$2b$10$ZVtDbp8nFLBzi4LTpMiiqe33JlwMdDmPf9.yguzf09cH1iDthQi16',
                'Welber Ribeiro',
                'super_admin',
                NOW(),
                NOW(),
                NOW()
            )
            "#,
        )
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to create superadmin: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupResponse {
                    success: false,
                    message: format!("Failed to create superadmin: {}", e),
                }),
            )
        })?;
        tracing::info!("Created new superadmin user");
    } else {
        tracing::info!("Updated existing superadmin user");
    }

    tracing::info!("Database setup completed successfully");

    Ok(Json(SetupResponse {
        success: true,
        message: "Database setup completed: email_verification_tokens table created, superadmin user configured".to_string(),
    }))
}

/// Seed membership plans and assign to super admin
/// POST /seed-memberships
async fn seed_memberships(
    axum::extract::State(state): axum::extract::State<AppState>,
) -> Result<Json<SetupResponse>, (axum::http::StatusCode, Json<SetupResponse>)> {
    // Seed all 6 membership plans
    let seed_plans = r#"
        INSERT INTO membership_plans (name, slug, description, price, billing_cycle, is_active, metadata, features, trial_days)
        VALUES
            ('Day Trading Room', 'day-trading-room', 'Live day trading sessions with real-time trade alerts.', 197.00, 'monthly', true, '{"type": "trading-room", "icon": "chart-line", "room_label": "Day Trading Room"}', '["Live Day Trading Sessions", "Discord Access", "Real-Time Trade Alerts", "Daily Market Analysis"]', 7),
            ('Swing Trading Room', 'swing-trading-room', 'Live swing trading sessions with swing trade alerts.', 147.00, 'monthly', true, '{"type": "trading-room", "icon": "trending-up", "room_label": "Swing Trading Room"}', '["Live Swing Trading Sessions", "Discord Access", "Swing Trade Alerts", "Weekly Watchlist"]', 7),
            ('Small Account Mentorship', 'small-account-mentorship', 'Personalized mentorship for small accounts.', 97.00, 'monthly', true, '{"type": "trading-room", "icon": "wallet", "room_label": "Small Account Mentorship"}', '["Small Account Strategies", "Personalized Mentorship", "Risk Management", "Position Sizing"]', 7),
            ('Alerts Only', 'alerts-only', 'Real-time trade alerts with entry and exit points.', 97.00, 'monthly', true, '{"type": "alert-service", "icon": "bell"}', '["Real-Time Trade Alerts", "Entry & Exit Points", "Stop Loss Levels", "Mobile Notifications"]', 0),
            ('Explosive Swing', 'explosive-swing', 'High momentum swing trade alerts.', 147.00, 'monthly', true, '{"type": "alert-service", "icon": "rocket"}', '["Explosive Swing Trade Alerts", "High Momentum Plays", "Detailed Analysis", "Risk/Reward Ratios"]', 0),
            ('SPX Profit Pulse', 'spx-profit-pulse', 'Premium SPX options alerts.', 197.00, 'monthly', true, '{"type": "alert-service", "icon": "activity"}', '["SPX Options Alerts", "Intraday Opportunities", "Premium Analysis", "High Win Rate Setups"]', 0)
        ON CONFLICT (slug) DO UPDATE SET
            name = EXCLUDED.name,
            description = EXCLUDED.description,
            price = EXCLUDED.price,
            metadata = EXCLUDED.metadata,
            features = EXCLUDED.features,
            updated_at = NOW()
    "#;

    sqlx::raw_sql(seed_plans)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to seed membership plans: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupResponse {
                    success: false,
                    message: format!("Failed to seed plans: {}", e),
                }),
            )
        })?;

    tracing::info!("Seeded membership plans");

    // Assign all memberships to super admin
    let assign_memberships = r#"
        INSERT INTO user_memberships (user_id, plan_id, starts_at, expires_at, status, current_period_start, current_period_end)
        SELECT
            u.id,
            p.id,
            NOW(),
            NOW() + INTERVAL '100 years',
            'active',
            NOW(),
            NOW() + INTERVAL '100 years'
        FROM users u
        CROSS JOIN membership_plans p
        WHERE u.email = 'welberribeirodrums@gmail.com'
        ON CONFLICT DO NOTHING
    "#;

    sqlx::raw_sql(assign_memberships)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to assign memberships: {}", e);
            (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                Json(SetupResponse {
                    success: false,
                    message: format!("Failed to assign memberships: {}", e),
                }),
            )
        })?;

    tracing::info!("Assigned all memberships to super admin");

    Ok(Json(SetupResponse {
        success: true,
        message: "Membership plans seeded and assigned to super admin".to_string(),
    }))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health_check))
        .route("/ready", get(ready_check))
        .route("/setup-db", post(setup_db))
        .route("/seed-memberships", post(seed_memberships))
}
