//! Authorization and rate-limiting for the CMS SEO endpoint.
//!
//! Two gates that every request to `/validate` must pass:
//! 1. `require_cms_editor` — role allow-list (admin / editor / marketing /
//!    developer). RBAC failure returns 403.
//! 2. `check_rate_limit` — 30 requests / 60 seconds per user, backed by
//!    Redis. Failure to reach Redis fails *open* (logs an error, allows the
//!    request) so a Redis outage doesn't break content authoring.

use axum::{http::StatusCode, Json};
use serde_json::{json, Value as JsonValue};

use crate::{models::User, AppState};

use super::{
    api_error, api_error_with_details, RATE_LIMIT_MAX_REQUESTS, RATE_LIMIT_WINDOW_SECONDS,
};

pub(super) fn require_cms_editor(user: &User) -> Result<(), (StatusCode, Json<JsonValue>)> {
    let role = user.role.as_deref().unwrap_or("user");
    if matches!(
        role,
        "admin" | "super-admin" | "super_admin" | "editor" | "marketing" | "developer"
    ) {
        Ok(())
    } else {
        Err(api_error(
            StatusCode::FORBIDDEN,
            "Editor access required for SEO validation",
        ))
    }
}

pub(super) async fn check_rate_limit(
    state: &AppState,
    user_id: i64,
) -> Result<(), (StatusCode, Json<JsonValue>)> {
    if let Some(ref redis) = state.services.redis {
        let key = format!("cms_seo_rate_limit:{user_id}");
        match redis
            .check_rate_limit(&key, RATE_LIMIT_MAX_REQUESTS, RATE_LIMIT_WINDOW_SECONDS)
            .await
        {
            Ok(allowed) if allowed => Ok(()),
            Ok(_) => {
                tracing::warn!(
                    target: "security",
                    event = "cms_seo_rate_limited",
                    user_id = %user_id,
                    "CMS SEO rate limit exceeded"
                );
                Err(api_error_with_details(
                    StatusCode::TOO_MANY_REQUESTS,
                    "Rate limit exceeded. Please wait before making more requests.",
                    json!({
                        "limit": RATE_LIMIT_MAX_REQUESTS,
                        "window_seconds": RATE_LIMIT_WINDOW_SECONDS,
                        "retry_after": RATE_LIMIT_WINDOW_SECONDS
                    }),
                ))
            }
            Err(e) => {
                tracing::error!(
                    target: "cms_seo",
                    error = %e,
                    "Failed to check rate limit"
                );
                // Allow request if rate limit check fails (fail open for availability)
                Ok(())
            }
        }
    } else {
        // No Redis available, allow request
        Ok(())
    }
}
