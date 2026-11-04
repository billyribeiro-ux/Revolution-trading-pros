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
    pub version: i64,
}
pub async fn load(pool: &PgPool, id: i64, active_only: bool) -> Result<Definition, ApiError> {
    let row:Option<(Option<Value>,bool,bool,bool,i64)>=sqlx::query_as("SELECT filter_criteria,COALESCE(is_dynamic,false),COALESCE(is_active,false),is_system,authoring_version FROM member_segments WHERE id=$1").bind(id).fetch_optional(pool).await.map_err(storage)?;
    let (value, dynamic, active, is_system, version) =
        row.ok_or_else(|| ApiError::not_found("Segment"))?;
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
        version,
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

pub async fn observe(
    pool: &PgPool,
    id: i64,
    definition: &Definition,
    total: i64,
) -> Result<(), ApiError> {
    let saved=sqlx::query("INSERT INTO member_segment_observations(segment_id,authoring_version,member_count) SELECT id,authoring_version,$3 FROM member_segments WHERE id=$1 AND authoring_version=$2 ON CONFLICT(segment_id,authoring_version,observed_date) DO UPDATE SET member_count=EXCLUDED.member_count,observed_at=NOW()")
        .bind(id).bind(definition.version).bind(total).execute(pool).await.map_err(storage)?;
    if saved.rows_affected() != 1 {
        return Err(crate::utils::errors::conflict(
            "Segment changed; reload before reviewing its analytics",
        ));
    }
    Ok(())
}
pub async fn analytics(pool: &PgPool, id: i64) -> Result<Value, ApiError> {
    let definition = load(pool, id, false).await?;
    let mut query = segment_query("WITH selected AS (SELECT m.*", &definition)?;
    query.push("), messages AS (SELECT d.opened_at FROM crm_campaign_deliveries d WHERE d.status='accepted' AND EXISTS(SELECT 1 FROM selected s WHERE LOWER(s.email)=LOWER(d.email)) UNION ALL SELECT e.opened_at FROM email_logs e WHERE e.status IN ('sent','delivered','opened','clicked','bounced') AND EXISTS(SELECT 1 FROM selected s WHERE LOWER(s.email)=LOWER(e.to_email))) SELECT jsonb_build_object('total_members',COUNT(*),'avg_spending',ROUND(AVG(total_spent),2),'avg_lifetime_days',ROUND(AVG(days_since_signup),1),'conversion_rate',ROUND(100.0*COUNT(*) FILTER(WHERE total_spent>0)/NULLIF(COUNT(*),0),1),'email_open_rate',(SELECT ROUND(100.0*COUNT(*) FILTER(WHERE opened_at IS NOT NULL)/NULLIF(COUNT(*),0),1) FROM messages),'accepted_messages',(SELECT COUNT(*) FROM messages),'open_signals',(SELECT COUNT(*) FROM messages WHERE opened_at IS NOT NULL)) FROM selected");
    let mut data: Value = query
        .build_query_scalar()
        .fetch_one(pool)
        .await
        .map_err(storage)?;
    let total = data["total_members"]
        .as_i64()
        .ok_or_else(|| crate::utils::errors::internal_error("Invalid member analytics count"))?;
    observe(pool, id, &definition, total).await?;
    let baseline:Option<i64>=sqlx::query_scalar("SELECT member_count FROM member_segment_observations WHERE segment_id=$1 AND authoring_version=$2 AND observed_at<=NOW()-INTERVAL '30 days' AND observed_at>=NOW()-INTERVAL '31 days' ORDER BY observed_at DESC LIMIT 1").bind(id).bind(definition.version).fetch_optional(pool).await.map_err(storage)?;
    data["growth_30d"] = baseline
        .filter(|count| *count > 0)
        .map(|count| {
            serde_json::json!((((total - count) as f64 / count as f64) * 1000.0).round() / 10.0)
        })
        .unwrap_or(Value::Null);
    data["baseline_count"] = serde_json::json!(baseline);
    data["observed_at"] = serde_json::json!(chrono::Utc::now());
    data["authoring_version"] = serde_json::json!(definition.version);
    let mut products = segment_query("WITH selected AS (SELECT m.id", &definition)?;
    products.push(") SELECT jsonb_build_object('name',p.name,'count',COUNT(DISTINCT o.user_id)) FROM selected s JOIN orders o ON o.user_id=s.id JOIN order_items item ON item.order_id=o.id JOIN products p ON p.id=item.product_id WHERE o.status IN ('completed','paid') GROUP BY p.id,p.name ORDER BY COUNT(DISTINCT o.user_id) DESC,p.id LIMIT 10");
    data["top_products"] = Value::Array(
        products
            .build_query_scalar()
            .fetch_all(pool)
            .await
            .map_err(storage)?,
    );
    Ok(data)
}
pub async fn run_observations(pool: PgPool) {
    let mut interval = tokio::time::interval(std::time::Duration::from_secs(3600));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        interval.tick().await;
        let ids: Vec<i64> = match sqlx::query_scalar(
            "SELECT id FROM member_segments WHERE is_active=true ORDER BY id",
        )
        .fetch_all(&pool)
        .await
        {
            Ok(ids) => ids,
            Err(error) => {
                tracing::error!(%error,"Unable to list member segment observation jobs");
                continue;
            }
        };
        for id in ids {
            let result = async {
                let definition = load(&pool, id, true).await?;
                let total = segment_query("SELECT COUNT(*)", &definition)?
                    .build_query_scalar()
                    .fetch_one(&pool)
                    .await
                    .map_err(storage)?;
                observe(&pool, id, &definition, total).await
            }
            .await;
            if let Err(error) = result {
                tracing::error!(segment_id=id,message=%error.message,"Member segment observation failed");
            }
        }
    }
}
