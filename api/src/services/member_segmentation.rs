//! One validated rule compiler for segment counts, previews and campaign audiences.
use crate::utils::errors::ApiError;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{PgPool, Postgres, QueryBuilder};
use std::str::FromStr;

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Condition {
    pub field: String,
    pub operator: String,
    pub value: Value,
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
pub fn rules(value: Option<Value>) -> Result<Vec<Condition>, ApiError> {
    let value = match value {
        None | Some(Value::Null) => Value::Array(vec![]),
        Some(Value::Object(object)) if object.is_empty() => Value::Array(vec![]),
        Some(Value::Object(mut object))
            if object.len() == 1 && object.contains_key("conditions") =>
        {
            object.remove("conditions").unwrap()
        }
        Some(value) => value,
    };
    let entries: Vec<Condition> = serde_json::from_value(value).map_err(|_| {
        invalid("Segment rules must be an array of field/operator/value conditions")
    })?;
    if entries.len() > 50 {
        return Err(invalid("At most50 segment conditions are allowed"));
    }
    query("SELECT COUNT(*)", &entries)?;
    Ok(entries)
}
fn number(value: &Value) -> Result<Decimal, ApiError> {
    let raw = match value {
        Value::Number(v) => v.to_string(),
        Value::String(v) => v.clone(),
        _ => {
            return Err(invalid(
                "Numeric segment values must be nonnegative numbers",
            ));
        }
    };
    Decimal::from_str(&raw)
        .ok()
        .filter(|v| *v >= Decimal::ZERO && *v <= Decimal::from(1_000_000_000))
        .ok_or_else(|| {
            invalid("Numeric segment values must be finite nonnegative numbers within1billion")
        })
}
fn text(value: &Value) -> Result<&str, ApiError> {
    value
        .as_str()
        .filter(|v| !v.trim().is_empty() && v.chars().count() <= 255 && !v.contains('\0'))
        .ok_or_else(|| {
            invalid("Segment values must contain1–255 characters without null characters")
        })
}
pub fn query(
    prefix: &'static str,
    rules: &[Condition],
) -> Result<QueryBuilder<Postgres>, ApiError> {
    let mut query = QueryBuilder::new(prefix);
    query.push(" FROM admin_member_facts m WHERE true");
    for condition in rules {
        let field = match condition.field.as_str() {
            "status" => "m.status",
            "subscription_status" => "m.subscription_status",
            "total_spent" => "m.total_spent",
            "days_since_signup" => "m.days_since_signup",
            "days_since_last_order" => "m.days_since_last_order",
            "email_engagement" => "m.email_engagement",
            "product" | "tag" => "",
            _ => return Err(invalid("Unsupported segment field")),
        };
        query.push(" AND (");
        if matches!(condition.field.as_str(), "product" | "tag") {
            if !matches!(condition.operator.as_str(), "has" | "not_has") {
                return Err(invalid("Product and tag conditions require has or not_has"));
            }
            let value = if let Some(id) = condition.value.as_i64().filter(|id| *id > 0) {
                id.to_string()
            } else {
                text(&condition.value)?.to_string()
            };
            if condition.operator == "not_has" {
                query.push("NOT ");
            }
            if condition.field == "tag" {
                query.push("EXISTS(SELECT 1 FROM user_member_tags t JOIN member_tags tag ON tag.id=t.tag_id WHERE t.user_id=m.id AND (tag.id::text=").push_bind(value.clone()).push(" OR LOWER(tag.name)=LOWER(").push_bind(value).push(")))");
            } else {
                query.push("EXISTS(SELECT 1 FROM order_items item JOIN orders o ON o.id=item.order_id JOIN products p ON p.id=item.product_id WHERE o.user_id=m.id AND o.status IN ('completed','paid') AND (p.id::text=").push_bind(value.clone()).push(" OR LOWER(p.name)=LOWER(").push_bind(value).push(")))");
            }
        } else {
            let numeric = matches!(
                condition.field.as_str(),
                "total_spent" | "days_since_signup" | "days_since_last_order"
            );
            let operator = match condition.operator.as_str() {
                "eq" => " = ",
                "neq" => " <> ",
                "gt" if numeric => " > ",
                "lt" if numeric => " < ",
                "gte" if numeric => " >= ",
                "lte" if numeric => " <= ",
                _ => return Err(invalid("Unsupported operator for segment field")),
            };
            query.push(field).push(operator);
            if numeric {
                query.push_bind(number(&condition.value)?);
            } else {
                let value = text(&condition.value)?;
                if condition.field == "email_engagement"
                    && !matches!(value, "engaged" | "not_engaged")
                {
                    return Err(invalid(
                        "Email engagement must be engaged or not_engaged; this records open signals only",
                    ));
                }
                query.push_bind(value.to_string());
            }
        }
        query.push(")");
    }
    Ok(query)
}
pub async fn count(pool: &PgPool, rules: &[Condition]) -> Result<i64, ApiError> {
    query("SELECT COUNT(*)", rules)?
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(|error| {
            tracing::error!(%error,"Member segment evaluation failed");
            ApiError::database_error("Unable to evaluate member segment")
        })
}

pub struct Definition {
    pub conditions: Vec<Condition>,
    pub static_id: Option<i64>,
    pub is_system: bool,
}
pub async fn load(pool: &PgPool, id: i64, active_only: bool) -> Result<Definition, ApiError> {
    let row:Option<(Option<Value>,bool,bool,bool)>=sqlx::query_as("SELECT filter_criteria,COALESCE(is_dynamic,false),COALESCE(is_active,false),is_system FROM member_segments WHERE id=$1").bind(id).fetch_optional(pool).await.map_err(storage)?;
    let (value, dynamic, active, is_system) = row.ok_or_else(|| ApiError::not_found("Segment"))?;
    if active_only && !active {
        return Err(invalid("Select an active member segment"));
    }
    let conditions = rules(value)?;
    Ok(Definition {
        static_id: if dynamic || !conditions.is_empty() {
            None
        } else {
            Some(id)
        },
        conditions,
        is_system,
    })
}
pub fn segment_query(
    prefix: &'static str,
    definition: &Definition,
) -> Result<QueryBuilder<Postgres>, ApiError> {
    let mut builder = query(prefix, &definition.conditions)?;
    if let Some(id) = definition.static_id {
        builder.push(" AND EXISTS(SELECT 1 FROM user_member_segments membership WHERE membership.user_id=m.id AND membership.segment_id=").push_bind(id).push(")");
    }
    Ok(builder)
}
pub fn storage(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Member segmentation persistence failed");
    if let Some(db) = error.as_database_error() {
        match db.code().as_deref() {
            Some("23505") => {
                return crate::utils::errors::conflict("A segment with this slug already exists");
            }
            Some("23503") => {
                return crate::utils::errors::conflict("The referenced record no longer exists");
            }
            _ => {}
        }
    }
    crate::utils::errors::service_unavailable("Member segmentation storage is unavailable")
}
