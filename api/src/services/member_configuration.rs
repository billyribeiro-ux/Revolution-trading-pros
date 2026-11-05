//! Shared validation for the established member tag and saved-filter contracts.
use crate::utils::errors::{self, ApiError};
use serde_json::{Map, Value, json};
use sqlx::PgPool;

pub fn text(value: &str, max: usize, required: bool) -> Result<(), ApiError> {
    if value.contains('\0') || value.chars().count() > max || required && value.trim().is_empty() {
        return Err(ApiError::validation_error(
            "Provide bounded nonempty text without null characters",
        ));
    }
    Ok(())
}
pub fn slug(value: &str) -> Result<String, ApiError> {
    text(value, 255, true)?;
    if !value
        .chars()
        .all(|v| v.is_ascii_lowercase() || v.is_ascii_digit() || v == '-')
    {
        return Err(ApiError::validation_error(
            "Slug must contain lowercase letters, numbers or hyphens",
        ));
    }
    Ok(value.into())
}
pub fn generated_slug(name: &str) -> String {
    let value = name
        .to_lowercase()
        .replace(' ', "-")
        .chars()
        .filter(|v| v.is_ascii_alphanumeric() || *v == '-')
        .take(210)
        .collect::<String>();
    if value.is_empty() {
        uuid::Uuid::new_v4().to_string()
    } else {
        value
    }
}
pub fn color(value: &str) -> Result<(), ApiError> {
    if value.len() != 7
        || !value.starts_with('#')
        || !value[1..].chars().all(|v| v.is_ascii_hexdigit())
    {
        return Err(ApiError::validation_error(
            "Tag color must be a six-digit hexadecimal color",
        ));
    }
    Ok(())
}
pub fn storage(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Member configuration persistence failed");
    match error
        .as_database_error()
        .and_then(|error| error.code())
        .as_deref()
    {
        Some("23505") => {
            errors::conflict("A member configuration record with this slug already exists")
        }
        Some("23503") => {
            errors::conflict("A referenced member configuration record no longer exists")
        }
        _ => errors::service_unavailable("Member configuration storage is unavailable"),
    }
}
pub async fn filters(pool: &PgPool, value: &Value) -> Result<Value, ApiError> {
    let mut connection = pool.acquire().await.map_err(storage)?;
    filters_on(&mut connection, value).await
}
pub async fn filters_on(
    connection: &mut sqlx::PgConnection,
    value: &Value,
) -> Result<Value, ApiError> {
    let fields = value.as_object().filter(|v| v.len() <= 20).ok_or_else(|| {
        ApiError::validation_error("Saved member filters must be an object of at most20 fields")
    })?;
    let mut normalized = Map::new();
    for (key, value) in fields {
        if value.is_null() || value == "" {
            continue;
        }
        if matches!(
            key.as_str(),
            "segment" | "tag" | "product_id" | "plan_id" | "page" | "per_page"
        ) {
            let id = value
                .as_i64()
                .or_else(|| value.as_str().and_then(|v| v.parse().ok()))
                .filter(|v| *v > 0)
                .ok_or_else(|| {
                    ApiError::validation_error(
                        "Member filter references must be positive identifiers",
                    )
                })?;
            if key == "page" && id > 10000 || key == "per_page" && id > 100 {
                return Err(ApiError::validation_error(
                    "Invalid saved filter pagination",
                ));
            }
            let exists = match key.as_str() {
                "segment" => {
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM member_segments WHERE id=$1)")
                        .bind(id)
                        .fetch_one(&mut *connection)
                        .await
                }
                "tag" => {
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM member_tags WHERE id=$1)")
                        .bind(id)
                        .fetch_one(&mut *connection)
                        .await
                }
                "product_id" => {
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM products WHERE id=$1)")
                        .bind(id)
                        .fetch_one(&mut *connection)
                        .await
                }
                "plan_id" => {
                    sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM membership_plans WHERE id=$1)")
                        .bind(id)
                        .fetch_one(&mut *connection)
                        .await
                }
                _ => Ok(true),
            }
            .map_err(storage)?;
            if !exists {
                return Err(ApiError::validation_error(
                    "A saved filter reference no longer exists",
                ));
            }
            normalized.insert(key.clone(), json!(id));
            continue;
        }
        let selected = value.as_str().ok_or_else(|| {
            ApiError::validation_error("Member filter text fields must be strings")
        })?;
        text(selected, if key == "search" { 100 } else { 255 }, false)?;
        let valid = match key.as_str() {
            "search" => true,
            "status" => ["active", "trial", "churned", "never_subscribed"].contains(&selected),
            "spending_tier" => ["whale", "high", "medium", "low"].contains(&selected),
            "sort_by" => [
                "id",
                "name",
                "email",
                "total_spent",
                "updated_at",
                "joined_at",
                "created_at",
            ]
            .contains(&selected),
            "sort_dir" => ["asc", "desc"].contains(&selected),
            "date_from" | "date_to" => {
                chrono::NaiveDate::parse_from_str(selected, "%Y-%m-%d").is_ok()
            }
            _ => false,
        };
        if !valid {
            return Err(ApiError::validation_error(
                "Unsupported member filter field or value",
            ));
        }
        normalized.insert(key.clone(), json!(selected));
    }
    if let (Some(from), Some(to)) = (
        normalized.get("date_from").and_then(Value::as_str),
        normalized.get("date_to").and_then(Value::as_str),
    ) && from > to
    {
        return Err(ApiError::validation_error(
            "Saved filter start date must not follow end date",
        ));
    }
    Ok(Value::Object(normalized))
}
pub async fn audit(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    actor: i64,
    action: &str,
    context: Value,
) -> Result<(), ApiError> {
    crate::services::crm_logs::record(tx, "info", "system", action, context, Some(actor))
        .await
        .map_err(storage)
}
