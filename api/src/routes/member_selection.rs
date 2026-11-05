//! Shared member predicates for listings and downloadable snapshots.
use super::members::MemberQuery;
use crate::{
    services::member_segmentation::{self as evaluation, Condition, Definition},
    utils::errors::ApiError,
};
use chrono::NaiveDate;
use sqlx::{PgPool, Postgres, QueryBuilder};

pub(crate) struct MemberSelection {
    params: MemberQuery,
    definition: Definition,
    date_from: Option<NaiveDate>,
    date_to: Option<NaiveDate>,
}
fn date(value: Option<&str>, field: &str) -> Result<Option<NaiveDate>, ApiError> {
    value
        .map(|value| {
            let parsed = NaiveDate::parse_from_str(value, "%Y-%m-%d").map_err(|_| {
                ApiError::validation_error(&format!("{field} must be a valid YYYY-MM-DD date"))
            })?;
            if parsed.format("%Y-%m-%d").to_string() != value {
                return Err(ApiError::validation_error(&format!(
                    "{field} must be a valid YYYY-MM-DD date"
                )));
            }
            Ok(parsed)
        })
        .transpose()
}
impl MemberSelection {
    pub(crate) async fn new(pool: &PgPool, params: MemberQuery) -> Result<Self, ApiError> {
        if params
            .search
            .as_ref()
            .is_some_and(|v| v.chars().count() > 100 || v.contains('\0'))
        {
            return Err(ApiError::validation_error(
                "Member search exceeds100 characters or contains invalid characters",
            ));
        }
        let date_from = date(params.date_from.as_deref(), "date_from")?;
        let date_to = date(params.date_to.as_deref(), "date_to")?;
        if params.status.as_deref().is_some_and(|status| {
            !matches!(
                status,
                "" | "active" | "trial" | "churned" | "never_subscribed"
            )
        }) {
            return Err(ApiError::validation_error("Invalid member status"));
        }
        if date_from.zip(date_to).is_some_and(|(from, to)| from > to) {
            return Err(ApiError::validation_error(
                "Start date must not follow end date",
            ));
        }
        let mut definition = if let Some(id) = params.segment {
            evaluation::load(pool, id, false).await?
        } else {
            Definition {
                conditions: vec![],
                static_id: None,
                is_system: false,
                version: 1,
            }
        };
        if let Some(status) = &params.status
            && !status.is_empty()
        {
            definition.conditions.push(Condition {
                field: "status".into(),
                operator: "eq".into(),
                value: serde_json::json!(status),
            });
        }
        if let Some(tag) = params.tag {
            if tag <= 0 {
                return Err(ApiError::validation_error("Invalid member tag"));
            }
            definition.conditions.push(Condition {
                field: "tag".into(),
                operator: "has".into(),
                value: serde_json::json!(tag),
            });
        }
        if let Some(tier) = params.spending_tier.as_deref().filter(|v| !v.is_empty()) {
            let (min, max) = match tier {
                "whale" => (5000, None),
                "high" => (1000, Some(5000)),
                "medium" => (100, Some(1000)),
                "low" => (0, Some(100)),
                _ => return Err(ApiError::validation_error("Invalid spending tier")),
            };
            definition.conditions.push(Condition {
                field: "total_spent".into(),
                operator: "gte".into(),
                value: serde_json::json!(min),
            });
            if let Some(max) = max {
                definition.conditions.push(Condition {
                    field: "total_spent".into(),
                    operator: "lt".into(),
                    value: serde_json::json!(max),
                });
            }
        }

        if params.product_id.is_some_and(|id| id <= 0) || params.segment.is_some_and(|id| id <= 0) {
            return Err(ApiError::validation_error(
                "Member product and segment identifiers must be positive",
            ));
        }
        let selection = Self {
            params,
            definition,
            date_from,
            date_to,
        };
        selection.order()?;
        Ok(selection)
    }
    pub(crate) fn query(&self, prefix: &'static str) -> Result<QueryBuilder<Postgres>, ApiError> {
        let params = &self.params;
        let date_from = self.date_from;
        let date_to = self.date_to;
        let mut query = evaluation::segment_query(prefix, &self.definition)?;
        if let Some(search) = params.search.as_ref().filter(|v| !v.is_empty()) {
            let pattern = format!("%{search}%");
            query
                .push(" AND (m.name ILIKE ")
                .push_bind(pattern.clone())
                .push(" OR m.email ILIKE ")
                .push_bind(pattern)
                .push(")");
        }
        if let Some(date) = date_from {
            query.push(" AND m.created_at>=").push_bind(date);
        }
        if let Some(date) = date_to {
            query
                .push(" AND m.created_at<")
                .push_bind(date)
                .push("+INTERVAL '1 day'");
        }
        if let Some(plan) = params.plan_id {
            if plan <= 0 {
                return Err(ApiError::validation_error("Invalid membership plan"));
            }
            query.push(" AND EXISTS(SELECT 1 FROM user_memberships s WHERE s.user_id=m.id AND s.plan_id=").push_bind(plan).push(" AND s.deleted_at IS NULL)");
        }
        if let Some(product) = params.product_id {
            query.push(" AND EXISTS(SELECT 1 FROM order_items item JOIN orders o ON o.id=item.order_id WHERE o.user_id=m.id AND item.product_id=").push_bind(product).push(" AND o.status IN ('completed','paid'))");
        }
        Ok(query)
    }
    pub(crate) fn order(&self) -> Result<(&'static str, &'static str), ApiError> {
        let params = &self.params;
        let sort = match params.sort_by.as_deref().unwrap_or("created_at") {
            "id" => "m.id",
            "name" => "m.name",
            "email" => "m.email",
            "total_spent" => "m.total_spent",
            "updated_at" => "m.updated_at",
            "joined_at" | "created_at" => "m.created_at",
            _ => return Err(ApiError::validation_error("Unsupported member sort field")),
        };
        let direction = match params.sort_dir.as_deref().unwrap_or("desc") {
            "asc" | "ASC" => " ASC",
            "desc" | "DESC" => " DESC",
            _ => return Err(ApiError::validation_error("Invalid member sort direction")),
        };

        Ok((sort, direction))
    }
}
