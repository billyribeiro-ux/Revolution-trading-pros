//! All recorded orders for one live member, bounded and audited before download.
use super::history;
use crate::{AppState, middleware::admin::AdminUser, services::crm_logs, utils::errors::ApiError};
use axum::{
    extract::{Path, Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};

const MAX_ROWS: usize = 10_000;
const MAX_BYTES: usize = 16 * 1024 * 1024;
const HEADERS: [&str; 12] = [
    "id",
    "member_id",
    "order_number",
    "recorded_status",
    "currency",
    "total_units",
    "refund_units",
    "net_recorded_units",
    "created_at_recorded",
    "completed_at_recorded",
    "refunded_at_recorded",
    "timestamp_basis",
];
#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub(crate) struct Options {
    format: Option<String>,
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn text(value: &Value) -> Result<String, ApiError> {
    match value {
        Value::Null => Ok(String::new()),
        Value::String(value) => Ok(value.clone()),
        _ => Err(ApiError::internal_error(
            "Stored order export field has an invalid type",
        )),
    }
}
fn literal(value: &str) -> String {
    if value.trim_start().starts_with(['=', '+', '-', '@']) || value.starts_with(['\t', '\r', '\n'])
    {
        format!("'{value}")
    } else {
        value.to_owned()
    }
}
fn file(rows: &[Value], member: i64) -> Result<Vec<u8>, ApiError> {
    let mut output = csv::WriterBuilder::new()
        .terminator(csv::Terminator::CRLF)
        .from_writer(vec![0xEF, 0xBB, 0xBF]);
    output
        .write_record(HEADERS)
        .map_err(|_| ApiError::internal_error("Order CSV could not be generated"))?;
    for row in rows {
        let id = row["id"]
            .as_i64()
            .filter(|id| *id > 0)
            .ok_or_else(|| ApiError::internal_error("Stored order identity is invalid"))?;
        let mut record = vec![id.to_string(), member.to_string()];
        for key in ["order_number", "status", "currency"] {
            record.push(literal(&text(&row[key])?));
        }
        for key in ["total_units", "refund_units", "net_recorded_units"] {
            record.push(text(&row[key])?);
        }
        for key in ["created_at", "completed_at", "refunded_at"] {
            record.push(literal(&text(&row[key])?));
        }
        record.push("Recorded timestamps without timezone; historical timezone unverified".into());
        output
            .write_record(record)
            .map_err(|_| ApiError::internal_error("Order CSV could not be generated"))?;
    }
    let bytes = output
        .into_inner()
        .map_err(|_| ApiError::internal_error("Order CSV could not be generated"))?;
    if bytes.len() > MAX_BYTES {
        return Err(invalid(
            "Order export exceeds 16 MiB; no partial file was generated",
        ));
    }
    Ok(bytes)
}
pub(crate) async fn download(
    State(state): State<AppState>,
    admin: AdminUser,
    Path(member): Path<i64>,
    Query(options): Query<Options>,
) -> Result<Response, ApiError> {
    if options
        .format
        .as_deref()
        .is_some_and(|format| format != "csv")
    {
        return Err(invalid("The member order export format must be csv"));
    }
    let mut tx = state.db.pool.begin().await.map_err(history::db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ")
        .execute(&mut *tx)
        .await
        .map_err(history::db)?;
    history::exists_on(&mut tx, member).await?;
    let rows: Vec<Value> = sqlx::query_scalar("SELECT jsonb_build_object('id',id,'order_number',order_number,'status',status,'currency',currency,'total_units',total::text,'refund_units',refund_amount::text,'net_recorded_units',GREATEST(total-COALESCE(refund_amount,0),0)::text,'created_at',created_at,'completed_at',completed_at,'refunded_at',refunded_at) FROM orders WHERE user_id=$1 ORDER BY created_at DESC NULLS LAST,id DESC LIMIT 10001")
        .bind(member).fetch_all(&mut *tx).await.map_err(history::db)?;
    if rows.len() > MAX_ROWS {
        return Err(invalid(
            "More than 10000 recorded orders match; no partial file was generated",
        ));
    }
    let bytes = file(&rows, member)?;
    let count = rows.len();
    let digest = hex::encode(Sha256::digest(&bytes));
    crm_logs::record(&mut tx, "info", "system", "Member order export generated",
        json!({"operation":"member_order_export","member_id":member,"actor_id":admin.0.id,"rows":count,"bytes":bytes.len(),"sha256":digest,"scope":"all_recorded_member_orders","currency_policy":"per_order_recorded_currency; no cross_currency_sum","timestamp_basis":"recorded_without_timezone","formula_policy":"apostrophe_prefix_for_unsafe_text"}),Some(admin.0.id))
        .await.map_err(history::db)?;
    tx.commit().await.map_err(history::db)?;
    let mut response = (StatusCode::OK, bytes).into_response();
    let headers = response.headers_mut();
    headers.insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static("text/csv; charset=utf-8"),
    );
    headers.insert(
        header::CACHE_CONTROL,
        HeaderValue::from_static("private, no-store"),
    );
    headers.insert(
        header::VARY,
        HeaderValue::from_static("Authorization, Cookie"),
    );
    for (name, value) in [
        (
            "content-disposition",
            format!("attachment; filename=\"member-{member}-orders.csv\""),
        ),
        ("x-export-member-id", member.to_string()),
        ("x-export-row-count", count.to_string()),
        ("x-export-sha256", digest),
        ("x-export-scope", "all_recorded_member_orders".to_owned()),
    ] {
        headers.insert(
            axum::http::HeaderName::from_static(name),
            HeaderValue::from_str(&value)
                .map_err(|_| ApiError::internal_error("Order export receipt header is invalid"))?,
        );
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn csv_retains_recorded_units_null_dates_and_escaped_text() {
        let bytes=file(&[json!({"id":7,"order_number":"=UNTRUSTED(1)","status":"paid","currency":"EUR","total_units":"12.34","refund_units":"2.00","net_recorded_units":"10.34","created_at":null,"completed_at":"12000-01-01T00:00:00","refunded_at":null})],9).unwrap();
        assert!(bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
        let mut reader = csv::Reader::from_reader(&bytes[3..]);
        assert_eq!(
            reader.headers().unwrap().iter().collect::<Vec<_>>(),
            HEADERS
        );
        let row = reader.records().next().unwrap().unwrap();
        assert_eq!(&row[2], "'=UNTRUSTED(1)");
        assert_eq!(&row[4], "EUR");
        assert_eq!(&row[5], "12.34");
        assert_eq!(&row[7], "10.34");
        assert_eq!(&row[8], "");
        assert_eq!(&row[9], "12000-01-01T00:00:00");
    }
}
