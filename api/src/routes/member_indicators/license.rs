//! License key management.
//!
//! - `generate_license_key` — internal SHA256 16-char formatter
//! - `validate_license_key` — by-key (no session) validation, deliberate
//!   so licenses can be shared across devices
//! - `get_license_key` — paywall-gated; returns existing key or mints
//!   a fresh one for an owned indicator
//!
//! All ownership joins use `user_indicators (user_id BIGINT, indicator_id BIGINT)`
//! with `i64` binds. Money is not directly handled here, but the access
//! check upstream is the paywall gate the rest of this module depends on.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use hmac::{Hmac, KeyInit, Mac};
use serde_json::json;
use sha2::Sha256;

use crate::models::User;
use crate::AppState;

/// ICT 7: Generate a license key for an indicator.
///
/// RUST_DEEP_AUDIT_2026-06-07 (P1-3): keyed HMAC-SHA256 over
/// `user_id-indicator_id-timestamp` using the boot-validated
/// `MEMBER_LICENSE_SECRET` — was an unkeyed SHA256 of a public formula signed
/// with a possibly-empty secret, making the shareable key derivable offline.
fn generate_license_key(secret: &str, user_id: i64, indicator_id: i64) -> String {
    let timestamp = chrono::Utc::now().timestamp();
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("HMAC-SHA256 accepts a key of any length");
    mac.update(format!("{user_id}-{indicator_id}-{timestamp}").as_bytes());
    let hash = hex::encode(mac.finalize().into_bytes());
    // Format: XXXX-XXXX-XXXX-XXXX (16 chars from hash)
    let key_chars: String = hash.chars().take(16).collect();
    format!(
        "{}-{}-{}-{}",
        &key_chars[0..4].to_uppercase(),
        &key_chars[4..8].to_uppercase(),
        &key_chars[8..12].to_uppercase(),
        &key_chars[12..16].to_uppercase()
    )
}

/// ICT 7: Validate a license key
pub(super) async fn validate_license_key(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Query(params): Query<LicenseValidateParams>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let license_key = params.license_key.as_deref().unwrap_or("");

    if license_key.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "License key is required", "valid": false})),
        ));
    }

    // Get indicator
    let indicator: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

    let indicator_id = match indicator {
        Some((id,)) => id,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Indicator not found", "valid": false})),
            ));
        }
    };

    // Check if license key matches any user's ownership
    let ownership: Option<(
        i64,
        i64,
        chrono::NaiveDateTime,
        Option<chrono::NaiveDateTime>,
    )> = sqlx::query_as(
        r"
        SELECT id, user_id, purchased_at, expires_at
        FROM user_indicators
        WHERE indicator_id = $1 AND license_key = $2
        ",
    )
    .bind(indicator_id)
    .bind(license_key)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    match ownership {
        Some((id, user_id, purchased_at, expires_at)) => {
            // Check if expired
            let is_expired = expires_at
                .map(|exp| exp < chrono::Utc::now().naive_utc())
                .unwrap_or(false);

            if is_expired {
                return Ok(Json(json!({
                    "success": true,
                    "valid": false,
                    "reason": "License has expired",
                    "expires_at": expires_at
                })));
            }

            Ok(Json(json!({
                "success": true,
                "valid": true,
                "license_key": license_key,
                "user_id": user_id,
                "purchased_at": purchased_at.and_utc().to_rfc3339(),
                "expires_at": expires_at.map(|e| e.and_utc().to_rfc3339()),
                "is_lifetime": expires_at.is_none()
            })))
        }
        None => Ok(Json(json!({
            "success": true,
            "valid": false,
            "reason": "Invalid license key"
        }))),
    }
}

#[derive(Debug, serde::Deserialize)]
pub(super) struct LicenseValidateParams {
    license_key: Option<String>,
}

/// ICT 7: Get or generate license key for owned indicator
pub(super) async fn get_license_key(
    user: User,
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let user_id: i64 = user.id;

    // Get indicator
    let indicator: Option<(i64,)> = sqlx::query_as("SELECT id FROM indicators WHERE slug = $1")
        .bind(&slug)
        .fetch_optional(&state.db.pool)
        .await
        .ok()
        .flatten();

    let indicator_id = match indicator {
        Some((id,)) => id,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                Json(json!({"error": "Indicator not found"})),
            ));
        }
    };

    // Check ownership and get existing license key
    let ownership: Option<(i64, Option<String>)> = sqlx::query_as(
        "SELECT id, license_key FROM user_indicators WHERE user_id = $1 AND indicator_id = $2",
    )
    .bind(user_id)
    .bind(indicator_id)
    .fetch_optional(&state.db.pool)
    .await
    .ok()
    .flatten();

    match ownership {
        Some((ownership_id, existing_key)) => {
            let license_key = match existing_key {
                Some(key) if !key.is_empty() => key,
                _ => {
                    // Generate new license key
                    let new_key = generate_license_key(
                        &state.config.member_license_secret,
                        user_id,
                        indicator_id,
                    );
                    // Save to database
                    sqlx::query("UPDATE user_indicators SET license_key = $1 WHERE id = $2")
                        .bind(&new_key)
                        .bind(ownership_id)
                        .execute(&state.db.pool)
                        .await
                        .ok();
                    new_key
                }
            };

            Ok(Json(json!({
                "success": true,
                "data": {
                    "license_key": license_key,
                    "indicator_id": indicator_id
                }
            })))
        }
        None => Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error": "You do not own this indicator"})),
        )),
    }
}
