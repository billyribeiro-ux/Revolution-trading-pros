//! Capability-based observations for native SMTP broadcasts.
use crate::{AppState, utils::errors::ApiError};
use axum::{
    Router,
    body::Body,
    extract::{Path, State},
    http::{Method, Response, StatusCode},
    routing::get,
};
use serde_json::json;
use uuid::Uuid;

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error, "Unable to record SMTP campaign tracking request");
    ApiError::new(
        StatusCode::SERVICE_UNAVAILABLE,
        "Unable to record campaign activity",
    )
}
async fn observe(
    state: &AppState,
    token: Uuid,
    click: bool,
    record: bool,
) -> Result<Option<String>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    let found: Option<(Uuid, i64, String, Option<String>)> = if click {
        sqlx::query_as("SELECT d.id,d.campaign_id,d.email,l.target FROM crm_campaign_tracking_links l JOIN crm_campaign_deliveries d ON d.id=l.delivery_id WHERE l.id=$1 AND d.provider='smtp'")
            .bind(token).fetch_optional(&mut *tx).await.map_err(db)?
    } else {
        sqlx::query_as("SELECT id,campaign_id,email,NULL::text FROM crm_campaign_deliveries WHERE tracking_token=$1 AND provider='smtp'")
            .bind(token).fetch_optional(&mut *tx).await.map_err(db)?
    };
    let Some((delivery, campaign, email, target)) = found else {
        return Err(ApiError::new(
            StatusCode::NOT_FOUND,
            "Campaign link not found",
        ));
    };
    if let Some(target) = &target {
        let url = reqwest::Url::parse(target)
            .map_err(|_| ApiError::validation_error("Invalid campaign link"))?;
        if !matches!(url.scheme(), "http" | "https")
            || url.host_str().is_none()
            || !url.username().is_empty()
            || url.password().is_some()
        {
            return Err(ApiError::validation_error("Invalid campaign link"));
        }
    }
    if record {
        // Same lock order as the worker/provider callbacks. Tracking does not
        // reconcile unknown SMTP acceptance or assert delivery.
        sqlx::query("SELECT id FROM crm_campaigns WHERE id=$1 FOR UPDATE")
            .bind(campaign)
            .fetch_one(&mut *tx)
            .await
            .map_err(db)?;
        let eligible: bool = sqlx::query_scalar("SELECT attempt_count>0 AND status IN ('processing','accepted','unknown') FROM crm_campaign_deliveries WHERE id=$1 FOR UPDATE")
            .bind(delivery).fetch_one(&mut *tx).await.map_err(db)?;
        if eligible {
            let kind = if click { "Click" } else { "Open" };
            sqlx::query("INSERT INTO crm_campaign_events(fingerprint,delivery_id,recipient_email,provider_message_id,record_type,occurred_at,details) SELECT $1,id,$3,provider_message_id,$4,NOW(),$5 FROM crm_campaign_deliveries WHERE id=$2 ON CONFLICT(fingerprint) DO NOTHING")
                .bind(format!("smtp:{kind}:{token}")).bind(delivery).bind(email).bind(kind)
                .bind(json!({"provider":"smtp","observation":"capability_request","link_id":if click {Some(token)} else {None}})).execute(&mut *tx).await.map_err(db)?;
            let query = if click {
                "UPDATE crm_campaign_deliveries SET clicked_at=COALESCE(clicked_at,NOW()),updated_at=NOW() WHERE id=$1"
            } else {
                "UPDATE crm_campaign_deliveries SET opened_at=COALESCE(opened_at,NOW()),updated_at=NOW() WHERE id=$1"
            };
            sqlx::query(query)
                .bind(delivery)
                .execute(&mut *tx)
                .await
                .map_err(db)?;
            crate::services::campaign_delivery::refresh(&mut tx, campaign)
                .await
                .map_err(|error| {
                    tracing::error!(%error, "Unable to update campaign activity totals");
                    ApiError::new(
                        StatusCode::SERVICE_UNAVAILABLE,
                        "Unable to update campaign activity totals",
                    )
                })?;
        }
    }
    tx.commit().await.map_err(db)?;
    Ok(target)
}
fn response(
    status: StatusCode,
    body: Body,
    content_type: Option<&str>,
    location: Option<&str>,
) -> Result<Response<Body>, ApiError> {
    let mut builder = Response::builder()
        .status(status)
        .header("cache-control", "private, no-store")
        .header("referrer-policy", "no-referrer")
        .header("x-robots-tag", "noindex, nofollow")
        .header("x-content-type-options", "nosniff");
    if let Some(value) = content_type {
        builder = builder.header("content-type", value);
    }
    if let Some(value) = location {
        builder = builder.header("location", value);
    }
    builder
        .body(body)
        .map_err(|_| ApiError::validation_error("Invalid campaign response"))
}
async fn open(
    State(state): State<AppState>,
    Path(token): Path<Uuid>,
    method: Method,
) -> Result<Response<Body>, ApiError> {
    observe(&state, token, false, method != Method::HEAD).await?;
    // A valid transparent 1x1 GIF, returned only after persistence succeeds.
    let bytes = b"GIF89a\x01\0\x01\0\x80\0\0\0\0\0\xff\xff\xff!\xf9\x04\x01\0\0\0\0,\0\0\0\0\x01\0\x01\0\0\x02\x02D\x01\0;";
    response(
        StatusCode::OK,
        if method == Method::HEAD {
            Body::empty()
        } else {
            Body::from(bytes.as_slice())
        },
        Some("image/gif"),
        None,
    )
}
async fn click(
    State(state): State<AppState>,
    Path(token): Path<Uuid>,
    method: Method,
) -> Result<Response<Body>, ApiError> {
    let target = observe(&state, token, true, method != Method::HEAD).await?;
    response(StatusCode::FOUND, Body::empty(), None, target.as_deref())
}
pub fn router() -> Router<AppState> {
    Router::new()
        .route("/open/{token}", get(open))
        .route("/click/{token}", get(click))
}
