//! Admin Abandoned Carts Management
//! Revolution Trading Pros - Full CRM & Cart Recovery Endpoints

use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{AppState, middleware::admin::AdminUser};

#[derive(Debug, Deserialize)]
pub struct CartListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub status: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct AbandonedCartRow {
    pub id: i64,
    pub contact_id: Option<i64>,
    pub email: String,
    pub cart_total: Option<Decimal>,
    pub cart_items: Option<serde_json::Value>,
    pub status: Option<String>,
    pub recovery_email_sent: Option<bool>,
    pub recovered: Option<bool>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

/// GET /api/admin/abandoned-carts/dashboard
pub async fn get_dashboard_stats(
    State(state): State<AppState>,
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let total_abandoned: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM crm_abandoned_carts WHERE status = 'abandoned'")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let total_value: (Option<Decimal>,) =
        sqlx::query_as("SELECT SUM(cart_total) FROM crm_abandoned_carts")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((None,));

    let recovered_count: (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM crm_abandoned_carts WHERE status = 'recovered' OR recovered = true",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((0,));

    let recovered_value: (Option<Decimal>,) = sqlx::query_as(
        "SELECT SUM(cart_total) FROM crm_abandoned_carts WHERE status = 'recovered' OR recovered = true",
    )
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or((None,));

    let total_all: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM crm_abandoned_carts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let recovery_rate = if total_all.0 > 0 {
        (recovered_count.0 as f64 / total_all.0 as f64) * 100.0
    } else {
        0.0
    };

    let total_val_f64 = total_value
        .0
        .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);
    let recovered_val_f64 = recovered_value
        .0
        .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);
    let avg_cart_value = if total_all.0 > 0 {
        total_val_f64 / total_all.0 as f64
    } else {
        0.0
    };

    let email_sent_count: (i64,) =
        sqlx::query_as("SELECT COUNT(*) FROM crm_abandoned_carts WHERE recovery_email_sent = true")
            .fetch_one(&state.db.pool)
            .await
            .unwrap_or((0,));

    let stats = json!({
        "total_abandoned": total_abandoned.0,
        "total_value": total_val_f64,
        "recovered_count": recovered_count.0,
        "recovered_value": recovered_val_f64,
        "recovery_rate": (recovery_rate * 10.0).round() / 10.0,
        "avg_cart_value": (avg_cart_value * 100.0).round() / 100.0,
        "pending_recovery": total_abandoned.0,
        "email_sent_count": email_sent_count.0,
        "clicked_count": 0,
        "by_status": {
            "pending": total_abandoned.0,
            "email_sent": email_sent_count.0,
            "clicked": 0,
            "recovered": recovered_count.0,
            "expired": 0,
            "unsubscribed": 0
        },
        "trend_7_days": []
    });

    Ok(Json(stats))
}

/// GET /api/admin/abandoned-carts
pub async fn list_abandoned_carts(
    State(state): State<AppState>,
    _admin: AdminUser,
    Query(query): Query<CartListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).clamp(1, 100);
    let offset = (page - 1) * per_page;

    let rows: Vec<AbandonedCartRow> = sqlx::query_as(
        "SELECT id, contact_id, email, cart_total, cart_items, status, recovery_email_sent, recovered, created_at, updated_at \
         FROM crm_abandoned_carts \
         ORDER BY created_at DESC \
         LIMIT $1 OFFSET $2",
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .map_err(crate::utils::errors::db_error)?;

    let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM crm_abandoned_carts")
        .fetch_one(&state.db.pool)
        .await
        .unwrap_or((0,));

    let last_page = ((total.0 as f64) / (per_page as f64)).ceil() as i64;

    let data: Vec<serde_json::Value> = rows
        .into_iter()
        .map(|r| {
            let cart_val = r
                .cart_total
                .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
                .unwrap_or(0.0);
            let items = r.cart_items.unwrap_or_else(|| json!([]));
            let created_iso = r
                .created_at
                .map(|d| d.to_rfc3339())
                .unwrap_or_else(|| Utc::now().to_rfc3339());
            json!({
                "id": r.id,
                "user_id": r.contact_id,
                "session_id": format!("sess_{}", r.id),
                "user_name": null,
                "user_email": r.email,
                "cart_data": {
                    "items": items,
                    "total": cart_val
                },
                "cart_value": cart_val,
                "status": r.status.unwrap_or_else(|| "abandoned".to_string()),
                "recovery_attempts": if r.recovery_email_sent.unwrap_or(false) { 1 } else { 0 },
                "last_email_at": if r.recovery_email_sent.unwrap_or(false) { Some(&created_iso) } else { None },
                "clicked_at": null,
                "recovered_at": if r.recovered.unwrap_or(false) { Some(&created_iso) } else { None },
                "abandoned_at": created_iso,
                "created_at": created_iso
            })
        })
        .collect();

    Ok(Json(json!({
        "data": data,
        "current_page": page,
        "last_page": last_page.max(1),
        "per_page": per_page,
        "total": total.0
    })))
}

/// GET /api/admin/abandoned-carts/templates
pub async fn get_templates(
    _admin: AdminUser,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let templates = json!([
        {
            "id": "reminder_1",
            "name": "Friendly Reminder",
            "subject": "Did you leave something behind?",
            "preview": "We noticed you left some items in your cart...",
            "timing": "1 hour after abandonment",
            "has_discount": false
        },
        {
            "id": "reminder_2",
            "name": "Special Offer",
            "subject": "Complete your order and get 10% off",
            "preview": "Use discount code SAVE10 to complete your purchase...",
            "timing": "24 hours after abandonment",
            "has_discount": true
        },
        {
            "id": "reminder_3",
            "name": "Final Notice",
            "subject": "Your cart is about to expire",
            "preview": "This is your last chance to complete your checkout...",
            "timing": "72 hours after abandonment",
            "has_discount": true
        }
    ]);
    Ok(Json(templates))
}

/// GET /api/admin/abandoned-carts/{id}
pub async fn get_cart(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let row: Option<AbandonedCartRow> = sqlx::query_as(
        "SELECT id, contact_id, email, cart_total, cart_items, status, recovery_email_sent, recovered, created_at, updated_at \
         FROM crm_abandoned_carts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    let Some(r) = row else {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Cart not found" })),
        ));
    };

    let cart_val = r
        .cart_total
        .map(|d| d.to_string().parse::<f64>().unwrap_or(0.0))
        .unwrap_or(0.0);
    let items = r.cart_items.unwrap_or_else(|| json!([]));
    let created_iso = r
        .created_at
        .map(|d| d.to_rfc3339())
        .unwrap_or_else(|| Utc::now().to_rfc3339());

    Ok(Json(json!({
        "id": r.id,
        "user_id": r.contact_id,
        "session_id": format!("sess_{}", r.id),
        "user_name": null,
        "user_email": r.email,
        "cart_data": {
            "items": items,
            "total": cart_val
        },
        "cart_value": cart_val,
        "status": r.status.unwrap_or_else(|| "abandoned".to_string()),
        "recovery_attempts": if r.recovery_email_sent.unwrap_or(false) { 1 } else { 0 },
        "last_email_at": if r.recovery_email_sent.unwrap_or(false) { Some(&created_iso) } else { None },
        "clicked_at": null,
        "recovered_at": if r.recovered.unwrap_or(false) { Some(&created_iso) } else { None },
        "abandoned_at": created_iso,
        "created_at": created_iso
    })))
}

/// POST /api/admin/abandoned-carts/{id}/send-recovery
pub async fn send_recovery(
    State(state): State<AppState>,
    _admin: AdminUser,
    Path(id): Path<i64>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let result = sqlx::query(
        "UPDATE crm_abandoned_carts SET recovery_email_sent = true, updated_at = NOW() WHERE id = $1",
    )
    .bind(id)
    .execute(&state.db.pool)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({ "error": e.to_string() }))))?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            Json(json!({ "error": "Cart not found" })),
        ));
    }

    Ok(Json(json!({
        "message": "Recovery email sent successfully",
        "cart_id": id,
        "email_sent": true
    })))
}

#[derive(Debug, Deserialize)]
pub struct BulkDeleteInput {
    pub ids: Vec<i64>,
}

/// POST /api/admin/abandoned-carts/bulk-delete
pub async fn bulk_delete(
    State(state): State<AppState>,
    _admin: AdminUser,
    Json(input): Json<BulkDeleteInput>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let deleted = sqlx::query("DELETE FROM crm_abandoned_carts WHERE id = ANY($1)")
        .bind(&input.ids)
        .execute(&state.db.pool)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": e.to_string() })),
            )
        })?;

    Ok(Json(json!({
        "message": "Abandoned carts deleted successfully",
        "deleted_count": deleted.rows_affected()
    })))
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/dashboard", get(get_dashboard_stats))
        .route("/templates", get(get_templates))
        .route("/bulk-delete", post(bulk_delete))
        .route("/", get(list_abandoned_carts))
        .route("/{id}", get(get_cart))
        .route("/{id}/send-recovery", post(send_recovery))
}
