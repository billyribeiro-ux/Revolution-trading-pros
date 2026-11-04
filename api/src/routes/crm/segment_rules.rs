//! Validated segment predicates. Identifiers/operators are selected from literals;
//! every user value is bound, including text that contains SQL metacharacters.
use crate::utils::errors::ApiError;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::{Postgres, QueryBuilder};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub(super) struct Condition {
    pub field: String,
    pub operator: String,
    #[serde(default)]
    pub value: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub logic: Option<String>,
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn text(value: &Value) -> Result<String, ApiError> {
    value
        .as_str()
        .filter(|s| s.len() <= 500 && !s.contains('\0'))
        .map(str::to_owned)
        .ok_or_else(|| invalid("Rule text values must be strings up to 500 bytes"))
}
fn number(value: &Value) -> Result<f64, ApiError> {
    value
        .as_f64()
        .or_else(|| value.as_str().and_then(|s| s.parse().ok()))
        .filter(|v| v.is_finite())
        .ok_or_else(|| invalid("Rule value must be a finite number"))
}
fn date(value: &Value) -> Result<chrono::DateTime<chrono::Utc>, ApiError> {
    let text = text(value)?;
    chrono::DateTime::parse_from_rfc3339(&text)
        .map(|d| d.with_timezone(&chrono::Utc))
        .or_else(|_| {
            chrono::NaiveDate::parse_from_str(&text, "%Y-%m-%d")
                .map(|d| d.and_time(chrono::NaiveTime::MIN).and_utc())
        })
        .map_err(|_| invalid("Date rules require YYYY-MM-DD or an RFC3339 timestamp"))
}
fn array(value: &Value) -> Result<&[Value], ApiError> {
    value
        .as_array()
        .filter(|v| !v.is_empty() && v.len() <= 100)
        .map(Vec::as_slice)
        .ok_or_else(|| invalid("Rule lists require 1–100 values"))
}

pub(super) fn query(
    prefix: &'static str,
    rules: &[Condition],
    match_type: &str,
) -> Result<QueryBuilder<Postgres>, ApiError> {
    if !matches!(match_type, "all" | "any") || rules.len() > 50 {
        return Err(invalid("Use match_type all/any and at most 50 rules"));
    }
    let mut query = QueryBuilder::new(prefix);
    query.push(" WHERE ");
    if rules.is_empty() {
        query.push(if match_type == "all" { "TRUE" } else { "FALSE" });
        return Ok(query);
    }
    // Mixed connectors are evaluated left-to-right; a rule's logic connects it
    // to the preceding group. Without overrides, match_type supplies all/any.
    for _ in rules {
        query.push("(");
    }
    for (index, rule) in rules.iter().enumerate() {
        if let Some(logic) = rule.logic.as_deref()
            && !matches!(logic, "and" | "or")
        {
            return Err(invalid("Rule logic must be and/or"));
        }
        if index > 0 {
            query.push(match rule.logic.as_deref() {
                Some("and") => " AND ",
                Some("or") => " OR ",
                _ if match_type == "all" => " AND ",
                _ => " OR ",
            });
        }
        append(&mut query, rule)?;
        query.push(")");
    }
    Ok(query)
}

fn append(query: &mut QueryBuilder<Postgres>, rule: &Condition) -> Result<(), ApiError> {
    let (column, kind) = match rule.field.as_str() {
        "email" => ("c.email", 0),
        "first_name" => ("c.first_name", 0),
        "last_name" => ("c.last_name", 0),
        "phone" => ("c.phone", 0),
        "company" | "company_name" => ("c.company", 0),
        "job_title" => ("c.job_title", 0),
        "status" => ("c.status", 0),
        "source" => ("c.source", 0),
        "score" | "lead_score" => ("c.score", 1),
        "created_at" => ("(c.created_at AT TIME ZONE 'UTC')", 2),
        "updated_at" => ("(c.updated_at AT TIME ZONE 'UTC')", 2),
        "last_contacted_at" => ("(c.last_contacted_at AT TIME ZONE 'UTC')", 2),
        "tags" => ("c.tags", 3),
        _ => return Err(invalid("Unsupported contact rule field")),
    };
    let op = rule.operator.as_str();
    if matches!(op, "is_null" | "is_not_null") {
        query.push(column).push(if op == "is_null" {
            " IS NULL"
        } else {
            " IS NOT NULL"
        });
        return Ok(());
    }
    if kind == 3 {
        match op {
            "equals" | "contains" | "not_equals" | "not_contains" => {
                if matches!(op, "not_equals" | "not_contains") {
                    query.push("NOT (");
                }
                query
                    .push("COALESCE(c.tags,'[]'::jsonb) @> ")
                    .push_bind(serde_json::json!([text(&rule.value)?]));
                if matches!(op, "not_equals" | "not_contains") {
                    query.push(")");
                }
            }
            "in" | "not_in" => {
                if op == "not_in" {
                    query.push("NOT (");
                }
                let values = array(&rule.value)?
                    .iter()
                    .map(text)
                    .collect::<Result<Vec<_>, _>>()?;
                query
                    .push("COALESCE(c.tags,'[]'::jsonb) ?| ")
                    .push_bind(values);
                if op == "not_in" {
                    query.push(")");
                }
            }
            _ => {
                return Err(invalid(
                    "Tag rules support contains, in, their negations, and null checks",
                ));
            }
        }
        return Ok(());
    }
    if matches!(op, "contains" | "not_contains") {
        if kind != 0 {
            return Err(invalid("Contains rules require a text field"));
        }
        query
            .push("POSITION(LOWER(")
            .push_bind(text(&rule.value)?)
            .push(") IN LOWER(")
            .push(column)
            .push("))")
            .push(if op == "contains" { ">0" } else { "=0" });
        return Ok(());
    }
    query.push(column);
    if matches!(op, "in" | "not_in") {
        let values = array(&rule.value)?;
        query.push(if op == "in" { " = ANY(" } else { " <> ALL(" });
        match kind {
            0 => {
                query.push_bind(values.iter().map(text).collect::<Result<Vec<_>, _>>()?);
            }
            1 => {
                query.push_bind(values.iter().map(number).collect::<Result<Vec<_>, _>>()?);
            }
            _ => {
                query.push_bind(values.iter().map(date).collect::<Result<Vec<_>, _>>()?);
            }
        }
        query.push(")");
        return Ok(());
    }
    let operator = match op {
        "equals" => " IS NOT DISTINCT FROM ",
        "not_equals" => " IS DISTINCT FROM ",
        "greater_than" if kind > 0 => " > ",
        "less_than" if kind > 0 => " < ",
        "between" if kind > 0 => " BETWEEN ",
        _ => return Err(invalid("Unsupported operator for this rule field")),
    };
    query.push(operator);
    let values = if op == "between" {
        let values = array(&rule.value)?;
        if values.len() != 2 {
            return Err(invalid("Between rules require exactly two values"));
        }
        if (kind == 1 && number(&values[0])? > number(&values[1])?)
            || (kind == 2 && date(&values[0])? > date(&values[1])?)
        {
            return Err(invalid(
                "Between rules require an ordered lower and upper value",
            ));
        }
        values.to_vec()
    } else {
        vec![rule.value.clone()]
    };
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            query.push(" AND ");
        }
        match kind {
            0 => {
                query.push_bind(text(value)?);
            }
            1 => {
                query.push_bind(number(value)?);
            }
            _ => {
                query.push_bind(date(value)?);
            }
        }
    }
    Ok(())
}
