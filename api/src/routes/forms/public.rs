//! Public-facing form handlers — no AdminUser gate.
//!
//! Mounted under `/api/forms` for frontend consumption: list published
//! forms, fetch one by id/slug, submit, and track views. Spam-blocked
//! via honeypot fields (`_hp_website`, `_hp_email`, `_hp_name`).
//! Email + webhook notifications fire post-commit so we never hold a
//! DB tx across an outbound HTTP call.

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde_json::json;

use super::types::{FormListQuery, FormRow};
use crate::AppState;

/// List published forms (public) - GET /forms
pub(super) async fn list_public_forms(
    State(state): State<AppState>,
    Query(query): Query<FormListQuery>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    let page = query.page.unwrap_or(1).max(1);
    let per_page = query.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    let search_pattern = query.search.as_ref().map(|s| format!("%{s}%"));

    let forms: Vec<FormRow> = sqlx::query_as(
        r"
        SELECT id, name, slug, description, fields, settings, is_published, submission_count, created_at, updated_at
        FROM forms
        WHERE is_published = true
          AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "
    )
    .bind(search_pattern.as_deref())
    .bind(per_page)
    .bind(offset)
    .fetch_all(&state.db.pool)
    .await
    .unwrap_or_default();

    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM forms WHERE is_published = true AND ($1::text IS NULL OR name ILIKE $1 OR description ILIKE $1)"
    )
    .bind(search_pattern.as_deref())
    .fetch_one(&state.db.pool)
    .await
    .unwrap_or(0);

    Ok(Json(json!({
        "success": true,
        "data": {
            "data": forms,
            "total": total,
            "per_page": per_page,
            "current_page": page
        },
        "forms": forms,
        "total": total
    })))
}

/// Get a single published form by ID or slug (public)
pub(super) async fn get_public_form(
    State(state): State<AppState>,
    Path(id_or_slug): Path<String>,
) -> Result<Json<FormRow>, (StatusCode, Json<serde_json::Value>)> {
    let form: Option<FormRow> = if let Ok(id) = id_or_slug.parse::<i64>() {
        sqlx::query_as("SELECT * FROM forms WHERE id = $1 AND is_published = true")
            .bind(id)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten()
    } else {
        sqlx::query_as("SELECT * FROM forms WHERE slug = $1 AND is_published = true")
            .bind(&id_or_slug)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten()
    };

    form.map(Json).ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Form not found"})),
        )
    })
}

/// Submit a form (public) - POST /forms/:slug/submit
/// ICT 7 Fix: Complete implementation with validation, webhook, and email notifications
pub(super) async fn submit_form(
    State(state): State<AppState>,
    Path(slug): Path<String>,
    Json(data): Json<serde_json::Value>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // ICT 7 Spam Protection: Check honeypot field
    // If _hp_website field is filled, it's a bot (humans can't see this field)
    if let Some(honeypot) = data.get("_hp_website") {
        if let Some(hp_str) = honeypot.as_str() {
            if !hp_str.is_empty() {
                tracing::warn!(
                    target: "security",
                    event = "spam_blocked",
                    form_slug = %slug,
                    "Honeypot triggered - spam submission blocked"
                );
                // Silently accept to not tip off bots, but don't save
                return Ok(Json(json!({
                    "success": true,
                    "message": "Form submitted successfully"
                })));
            }
        }
    }

    // Also check for _hp_email honeypot variant
    if let Some(honeypot) = data.get("_hp_email") {
        if let Some(hp_str) = honeypot.as_str() {
            if !hp_str.is_empty() {
                tracing::warn!(
                    target: "security",
                    event = "spam_blocked",
                    form_slug = %slug,
                    "Honeypot triggered - spam submission blocked"
                );
                return Ok(Json(json!({
                    "success": true,
                    "message": "Form submitted successfully"
                })));
            }
        }
    }

    // Remove honeypot fields from submission data
    let mut clean_data = data.clone();
    if let Some(obj) = clean_data.as_object_mut() {
        obj.remove("_hp_website");
        obj.remove("_hp_email");
        obj.remove("_hp_name");
    }

    // Find form by slug
    let form: Option<FormRow> =
        sqlx::query_as("SELECT * FROM forms WHERE slug = $1 AND is_published = true")
            .bind(&slug)
            .fetch_optional(&state.db.pool)
            .await
            .ok()
            .flatten();

    let form = form.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({"error": "Form not found"})),
        )
    })?;

    // ICT 7 Fix: Server-side validation
    let validation_errors = validate_form_submission(&form.fields, &clean_data);
    if !validation_errors.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "Validation failed",
                "errors": validation_errors
            })),
        ));
    }

    // ICT 7 SAFETY: insert the submission and bump the counter atomically. The
    // external email + webhook fire AFTER commit so we never hold a DB tx across
    // an HTTP call (and the counter never drifts on a crash mid-flow).
    let mut tx = state.db.pool.begin().await.map_err(|e| {
        tracing::error!("tx start (submit_form): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // Insert submission
    let submission_result: (i64,) = sqlx::query_as(
        "INSERT INTO form_submissions (form_id, data, status, created_at) VALUES ($1, $2, 'unread', NOW()) RETURNING id"
    )
        .bind(form.id)
        .bind(&clean_data)
        .fetch_one(&mut *tx)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": e.to_string()})),
            )
        })?;

    // Update submission count
    sqlx::query("UPDATE forms SET submission_count = submission_count + 1 WHERE id = $1")
        .bind(form.id)
        .execute(&mut *tx)
        .await
        .map_err(|e| {
            tracing::error!("Failed to increment form submission_count: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"error": "Failed to update submission count"})),
            )
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("tx commit (submit_form): {}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"error": "Database error"})),
        )
    })?;

    // ICT 7 Fix: Email notification via Postmark (post-commit; do not hold a DB tx across HTTP)
    if let Some(settings) = form.settings.as_object() {
        if settings
            .get("send_email")
            .and_then(|v| v.as_bool())
            .unwrap_or(false)
        {
            if let Some(email_to) = settings.get("email_to").and_then(|v| v.as_str()) {
                {
                    let email_service = &state.services.email;
                    let subject = format!("New Form Submission: {}", form.name);
                    let html_body = build_form_notification_email(&form.name, &clean_data);

                    if let Err(e) = email_service
                        .send(email_to, &subject, &html_body, None)
                        .await
                    {
                        tracing::error!(
                            target: "forms",
                            event = "email_send_error",
                            form_id = %form.id,
                            error = %e,
                            "Failed to send form notification email"
                        );
                    } else {
                        tracing::info!(
                            target: "forms",
                            event = "email_notification_sent",
                            form_id = %form.id,
                            email_to = %email_to,
                            "Form submission email notification sent"
                        );
                    }
                }
            }
        }

        // ICT 7 Fix: Webhook integration
        if let Some(webhook_url) = settings.get("webhook_url").and_then(|v| v.as_str()) {
            if !webhook_url.is_empty() {
                let webhook_payload = json!({
                    "event": "form_submission",
                    "form_id": form.id,
                    "form_name": form.name,
                    "form_slug": form.slug,
                    "submission_id": submission_result.0,
                    "data": clean_data,
                    "timestamp": chrono::Utc::now().to_rfc3339()
                });

                // Fire-and-forget webhook call (don't block response)
                let webhook_url = webhook_url.to_string();
                let client = reqwest::Client::new();
                tokio::spawn(async move {
                    match client
                        .post(&webhook_url)
                        .header("Content-Type", "application/json")
                        .header("X-Webhook-Event", "form_submission")
                        .json(&webhook_payload)
                        .timeout(std::time::Duration::from_secs(10))
                        .send()
                        .await
                    {
                        Ok(response) => {
                            if response.status().is_success() {
                                tracing::info!(
                                    target: "forms",
                                    event = "webhook_success",
                                    url = %webhook_url,
                                    "Webhook delivered successfully"
                                );
                            } else {
                                tracing::warn!(
                                    target: "forms",
                                    event = "webhook_error",
                                    url = %webhook_url,
                                    status = %response.status(),
                                    "Webhook returned non-success status"
                                );
                            }
                        }
                        Err(e) => {
                            tracing::error!(
                                target: "forms",
                                event = "webhook_failed",
                                url = %webhook_url,
                                error = %e,
                                "Failed to deliver webhook"
                            );
                        }
                    }
                });
            }
        }
    }

    // Get success message from settings or use default
    let success_message = form
        .settings
        .as_object()
        .and_then(|s| s.get("success_message"))
        .and_then(|v| v.as_str())
        .unwrap_or("Form submitted successfully");

    // Get redirect URL if configured
    let redirect_url = form
        .settings
        .as_object()
        .and_then(|s| s.get("redirect_url"))
        .and_then(|v| v.as_str());

    let mut response = json!({
        "success": true,
        "message": success_message,
        "submission_id": submission_result.0.to_string()
    });

    if let Some(url) = redirect_url {
        response["redirect_url"] = json!(url);
    }

    Ok(Json(response))
}

/// ICT 7 Fix: Server-side form field validation
fn validate_form_submission(
    fields: &serde_json::Value,
    data: &serde_json::Value,
) -> std::collections::HashMap<String, Vec<String>> {
    let mut errors: std::collections::HashMap<String, Vec<String>> =
        std::collections::HashMap::new();

    if let Some(fields_array) = fields.as_array() {
        for field in fields_array {
            let field_name = field.get("name").and_then(|v| v.as_str()).unwrap_or("");
            let field_label = field
                .get("label")
                .and_then(|v| v.as_str())
                .unwrap_or(field_name);
            let required = field
                .get("required")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            let field_type = field
                .get("field_type")
                .and_then(|v| v.as_str())
                .unwrap_or("text");
            let validation = field.get("validation");

            let value = data.get(field_name);
            let value_str = value.and_then(|v| v.as_str()).unwrap_or("");
            let is_empty =
                value.is_none() || value_str.is_empty() || value.is_some_and(|v| v.is_null());

            // Required field validation
            if required && is_empty {
                errors
                    .entry(field_name.to_string())
                    .or_default()
                    .push(format!("{field_label} is required"));
                continue;
            }

            // Skip further validation if empty and not required
            if is_empty {
                continue;
            }

            // Email validation
            if field_type == "email"
                && !value_str.is_empty()
                && (!value_str.contains('@') || !value_str.contains('.'))
            {
                errors
                    .entry(field_name.to_string())
                    .or_default()
                    .push(format!("{field_label} must be a valid email address"));
            }

            // Min/max length validation
            if let Some(validation) = validation {
                if let Some(min_length) = validation.get("min_length").and_then(|v| v.as_u64()) {
                    if (value_str.len() as u64) < min_length {
                        errors
                            .entry(field_name.to_string())
                            .or_default()
                            .push(format!(
                                "{field_label} must be at least {min_length} characters"
                            ));
                    }
                }

                if let Some(max_length) = validation.get("max_length").and_then(|v| v.as_u64()) {
                    if (value_str.len() as u64) > max_length {
                        errors
                            .entry(field_name.to_string())
                            .or_default()
                            .push(format!(
                                "{field_label} must be no more than {max_length} characters"
                            ));
                    }
                }

                // Pattern validation
                if let Some(pattern) = validation.get("pattern").and_then(|v| v.as_str()) {
                    if let Ok(regex) = regex::Regex::new(pattern) {
                        if !regex.is_match(value_str) {
                            let message = validation
                                .get("pattern_message")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Invalid format");
                            errors
                                .entry(field_name.to_string())
                                .or_default()
                                .push(message.to_string());
                        }
                    }
                }

                // Number validation (min/max)
                if field_type == "number" {
                    if let Ok(num_value) = value_str.parse::<f64>() {
                        if let Some(min) = validation.get("min").and_then(|v| v.as_f64()) {
                            if num_value < min {
                                errors
                                    .entry(field_name.to_string())
                                    .or_default()
                                    .push(format!("{field_label} must be at least {min}"));
                            }
                        }
                        if let Some(max) = validation.get("max").and_then(|v| v.as_f64()) {
                            if num_value > max {
                                errors
                                    .entry(field_name.to_string())
                                    .or_default()
                                    .push(format!("{field_label} must be no more than {max}"));
                            }
                        }
                    }
                }
            }
        }
    }

    errors
}

/// ICT 7 Fix: Build HTML email for form notification
fn build_form_notification_email(form_name: &str, data: &serde_json::Value) -> String {
    let mut fields_html = String::new();

    if let Some(obj) = data.as_object() {
        for (key, value) in obj {
            let value_str = match value {
                serde_json::Value::String(s) => s.clone(),
                serde_json::Value::Null => "Not provided".to_string(),
                serde_json::Value::Array(arr) => arr
                    .iter()
                    .filter_map(|v| v.as_str())
                    .collect::<Vec<_>>()
                    .join(", "),
                _ => value.to_string(),
            };
            fields_html.push_str(&format!(
                r#"<tr><td style="padding: 10px 15px; border-bottom: 1px solid #eee; font-weight: 600; color: #333; width: 30%;">{}</td><td style="padding: 10px 15px; border-bottom: 1px solid #eee; color: #666;">{}</td></tr>"#,
                key.replace('_', " ").to_uppercase(),
                html_escape(&value_str)
            ));
        }
    }

    format!(
        r#"<!DOCTYPE html>
<html>
<head>
    <meta charset="utf-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>New Form Submission</title>
</head>
<body style="font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 0; padding: 20px; background-color: #f5f5f5;">
    <div style="max-width: 600px; margin: 0 auto; background-color: #ffffff; border-radius: 8px; overflow: hidden; box-shadow: 0 2px 4px rgba(0,0,0,0.1);">
        <div style="background: linear-gradient(135deg, #667eea 0%, #764ba2 100%); color: white; padding: 30px; text-align: center;">
            <h1 style="margin: 0; font-size: 24px;">New Form Submission</h1>
            <p style="margin: 10px 0 0; opacity: 0.9;">{}</p>
        </div>
        <div style="padding: 30px;">
            <table style="width: 100%; border-collapse: collapse;">
                {}
            </table>
            <div style="margin-top: 30px; padding-top: 20px; border-top: 1px solid #eee; text-align: center; color: #999; font-size: 12px;">
                <p>This email was sent from Revolution Trading Pros</p>
                <p>Submitted at: {}</p>
            </div>
        </div>
    </div>
</body>
</html>"#,
        form_name,
        fields_html,
        chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC")
    )
}

/// Helper to escape HTML characters
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Track form view (public) - POST /forms/:slug/view
/// ICT 7 Fix: Form analytics - track views for conversion rate calculation
pub(super) async fn track_form_view(
    State(state): State<AppState>,
    Path(slug): Path<String>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // Update view count (we'll add a view_count column if not exists)
    sqlx::query(
        r"
        UPDATE forms
        SET settings = jsonb_set(
            COALESCE(settings, '{}'::jsonb),
            '{view_count}',
            (COALESCE((settings->>'view_count')::int, 0) + 1)::text::jsonb
        )
        WHERE slug = $1 AND is_published = true
        ",
    )
    .bind(&slug)
    .execute(&state.db.pool)
    .await
    .ok();

    Ok(Json(json!({"success": true})))
}
