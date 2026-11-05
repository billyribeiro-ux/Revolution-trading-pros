//! Bounded member snapshots with actual CSV, OOXML and multipage PDF downloads.
use super::ExportQuery;
use crate::{AppState, middleware::admin::AdminUser, services::crm_logs, utils::errors::ApiError};
use axum::{
    extract::{Query, State},
    http::{HeaderValue, StatusCode, header},
    response::{IntoResponse, Response},
};
use chrono::NaiveDateTime;
use serde_json::json;
use sha2::{Digest, Sha256};
use sqlx::FromRow;
use std::sync::LazyLock;
use tokio::sync::Semaphore;

#[path = "export_formats.rs"]
mod formats;
const MAX_ROWS: usize = 10_000;
const MAX_SOURCE_BYTES: usize = 16 * 1024 * 1024;
static EXPORT_SLOTS: LazyLock<Semaphore> = LazyLock::new(|| Semaphore::new(2));

#[derive(Debug, FromRow)]
struct Member {
    id: i64,
    name: Option<String>,
    email: String,
    role: Option<String>,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime,
}
impl Member {
    fn cells(&self) -> [String; 5] {
        [
            self.id.to_string(),
            self.name.clone().unwrap_or_default(),
            self.email.clone(),
            self.role.clone().unwrap_or_default(),
            self.created_at.format("%Y-%m-%d %H:%M:%S%.f").to_string(),
        ]
    }
}
fn invalid(message: impl Into<String>) -> ApiError {
    ApiError::validation_error(&message.into())
}
fn database(error: sqlx::Error) -> ApiError {
    tracing::error!(%error, "Member export persistence failed");
    ApiError::database_error("Member export storage is unavailable")
}
pub(super) async fn export_members(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(query): Query<ExportQuery>,
) -> Result<Response, ApiError> {
    export(state, admin, query, false).await
}

pub(crate) async fn export_members_csv(
    State(state): State<AppState>,
    admin: AdminUser,
    Query(query): Query<ExportQuery>,
) -> Result<Response, ApiError> {
    export(state, admin, query, true).await
}

async fn export(
    state: AppState,
    admin: AdminUser,
    query: ExportQuery,
    legacy_csv: bool,
) -> Result<Response, ApiError> {
    let format = query.format.as_deref().unwrap_or("csv");
    if legacy_csv && format != "csv" {
        return Err(invalid(
            "This endpoint exports CSV; use member-management/export for XLSX or PDF",
        ));
    }
    if !matches!(format, "csv" | "xlsx" | "pdf") {
        return Err(invalid("format must be csv, xlsx or pdf"));
    }
    let selection = super::super::member_selection::MemberSelection::new(
        &state.db.pool,
        query.member_filters(),
    )
    .await?;
    let permit = EXPORT_SLOTS.try_acquire().map_err(|_| {
        ApiError::new(
            StatusCode::TOO_MANY_REQUESTS,
            "Member exports are busy; retry shortly",
        )
    })?;
    // A single statement gives a consistent snapshot, using the same current,
    // non-deleted facts as member lists and segments. Preserve naive timestamps
    // as recorded, without inventing a timezone for historical values.
    let mut snapshot = selection.query("SELECT m.id,m.name,m.email,(SELECT u.role FROM users u WHERE u.id=m.id) AS role,m.created_at,m.updated_at")?;
    let (sort, direction) = selection.order()?;
    snapshot
        .push(" ORDER BY ")
        .push(sort)
        .push(direction)
        .push(",m.id LIMIT 10001");
    let members: Vec<Member> = snapshot
        .build_query_as()
        .fetch_all(&state.db.pool)
        .await
        .map_err(database)?;
    if members.len() > MAX_ROWS {
        return Err(invalid(
            "More than 10000 members match. Narrow the status or date range; no partial export was generated",
        ));
    }
    let source_bytes: usize = members
        .iter()
        .map(|member| {
            member.name.as_ref().map_or(0, String::len)
                + member.email.len()
                + member.role.as_ref().map_or(0, String::len)
                + 64
        })
        .sum();
    if source_bytes > MAX_SOURCE_BYTES {
        return Err(invalid(
            "Member export exceeds 16 MiB of source text; narrow the date range",
        ));
    }
    let count = members.len();
    let kind = if legacy_csv { "csv_members" } else { format }.to_owned();
    let bytes = tokio::task::spawn_blocking(move || {
        let _permit = permit;
        formats::generate(&kind, &members)
    })
    .await
    .map_err(|error| {
        tracing::error!(%error, "Member export worker failed");
        ApiError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "Member export generation failed",
        )
    })??;
    let digest = hex::encode(Sha256::digest(&bytes));
    let mut tx = state.db.pool.begin().await.map_err(database)?;
    crm_logs::record(
        &mut tx,
        "info",
        "system",
        "Member export generated",
        json!({"operation":"member_export","format":format,"rows":count,"bytes":bytes.len(),"sha256":digest,
            "filters":query,
            "scope":"current_member_facts","timestamp_basis":"recorded_without_timezone"}),
        Some(admin.0.id),
    )
    .await
    .map_err(database)?;
    tx.commit().await.map_err(database)?;
    let (mime, filename) = match format {
        "xlsx" => (
            "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet",
            "attachment; filename=\"members.xlsx\"",
        ),
        "pdf" => ("application/pdf", "attachment; filename=\"members.pdf\""),
        _ => (
            "text/csv; charset=utf-8",
            "attachment; filename=\"members.csv\"",
        ),
    };
    let mut response = (
        [
            (header::CONTENT_TYPE, mime),
            (header::CONTENT_DISPOSITION, filename),
            (header::CACHE_CONTROL, "private, no-store"),
            (header::X_CONTENT_TYPE_OPTIONS, "nosniff"),
        ],
        bytes,
    )
        .into_response();
    response.headers_mut().insert(
        "x-export-row-count",
        HeaderValue::from_str(&count.to_string())
            .map_err(|_| invalid("Cannot encode export count"))?,
    );
    response.headers_mut().insert(
        "x-export-sha256",
        HeaderValue::from_str(&digest).map_err(|_| invalid("Cannot encode export digest"))?,
    );
    Ok(response)
}
