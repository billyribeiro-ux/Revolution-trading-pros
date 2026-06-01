//! Stripe-synchronised indicator price changes.
//!
//! Money is integer cents end-to-end (`amount_cents: i64`). The handler
//! creates a new Stripe Price (and Product if needed), then flips the
//! DB pointer + writes a `security_events` audit row inside a single
//! `BEGIN/COMMIT` transaction so the row and pointer move together.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::middleware::admin::AdminUser;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub(super) struct ChangeIndicatorPrice {
    pub(super) amount_cents: i64,
    pub(super) currency: Option<String>,
}

pub(super) async fn change_indicator_price(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Path(indicator_id): Path<i64>,
    Json(input): Json<ChangeIndicatorPrice>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if input.amount_cents <= 0 {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error": "amount_cents must be > 0"})),
        ));
    }
    let currency = input.currency.unwrap_or_else(|| "usd".to_string());

    #[derive(sqlx::FromRow)]
    struct IndicatorForPriceChange {
        id: i64,
        name: String,
        stripe_price_id: Option<String>,
        stripe_product_id: Option<String>,
        price_cents: Option<i64>,
    }

    let indicator: IndicatorForPriceChange = sqlx::query_as(
        r"SELECT id, name, stripe_price_id, stripe_product_id,
                  COALESCE(price_cents, (price * 100)::BIGINT) AS price_cents
           FROM indicators WHERE id = $1",
    )
    .bind(indicator_id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, indicator_id = %indicator_id, "DB error fetching indicator");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Database error"})))
    })?
    .ok_or_else(|| (StatusCode::NOT_FOUND, Json(json!({"error": "Indicator not found"}))))?;

    let old_price_id = indicator.stripe_price_id.clone();

    let env_scope = state.config.environment.clone();
    let stripe = state
        .services
        .credentials
        .stripe_client(&state.db.pool, &env_scope)
        .await;

    let product_id = match indicator.stripe_product_id.clone() {
        Some(pid) => pid,
        None => stripe
            .create_product(&indicator.name)
            .await
            .map_err(|e| {
                tracing::error!(target: "stripe_price", error = %e, indicator_id = %indicator_id, "Failed to create Stripe product");
                (StatusCode::BAD_GATEWAY, Json(json!({"error": format!("Stripe product create failed: {}", e)})))
            })?,
    };

    let new_price = stripe
        .create_price(&product_id, input.amount_cents, &currency, "one_time")
        .await
        .map_err(|e| {
            tracing::error!(target: "stripe_price", error = %e, indicator_id = %indicator_id, "Failed to create Stripe price");
            (StatusCode::BAD_GATEWAY, Json(json!({"error": format!("Stripe price create failed: {}", e)})))
        })?;

    let new_price_id = new_price.id.clone();

    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, "Failed to begin transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to start transaction"})),
        )
    })?;

    sqlx::query(
        r"UPDATE indicators
           SET stripe_price_id = $1,
               stripe_product_id = $2,
               price_cents = $3::BIGINT,
               updated_at = NOW()
           WHERE id = $4",
    )
    .bind(&new_price_id)
    .bind(&product_id)
    .bind(input.amount_cents)
    .bind(indicator_id)
    .execute(&mut *tx)
    .await
    .map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, indicator_id = %indicator_id, "Failed to update indicator");
        (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": "Failed to update indicator"})))
    })?;

    let details = json!({
        "indicator_id": indicator_id,
        "old_stripe_price_id": old_price_id,
        "new_stripe_price_id": &new_price_id,
        "old_amount_cents": indicator.price_cents,
        "new_amount_cents": input.amount_cents,
        "currency": &currency,
        "changed_by_user_id": admin.id,
    });
    if let Err(e) = sqlx::query(
        r"INSERT INTO security_events (user_id, event_type, event_category, severity, details)
           VALUES ($1, 'indicator_price_changed', 'billing', 'medium', $2)",
    )
    .bind(admin.id)
    .bind(&details)
    .execute(&mut *tx)
    .await
    {
        tracing::warn!(target: "stripe_price", error = %e, "Failed to insert security_event for indicator price change");
    }

    tx.commit().await.map_err(|e| {
        tracing::error!(target: "stripe_price", error = %e, "Failed to commit transaction");
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Failed to commit DB changes"})),
        )
    })?;

    tracing::info!(
        target: "stripe_price",
        event = "indicator_price_changed",
        indicator_id = %indicator_id,
        old_price_id = ?old_price_id,
        new_price_id = %new_price_id,
        amount_cents = %input.amount_cents,
        admin_id = %admin.id,
        "Indicator price updated; Stripe Price created and DB pointer flipped"
    );

    Ok(Json(json!({
        "success": true,
        "indicator_id": indicator_id,
        "old_stripe_price_id": old_price_id,
        "new_stripe_price_id": new_price_id,
        "amount_cents": input.amount_cents,
        "currency": currency,
    })))
}
