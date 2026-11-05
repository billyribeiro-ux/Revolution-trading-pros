//! Actual member journals and receipt history, with one paged snapshot per read.
use crate::{
    AppState,
    services::crm_logs,
    utils::errors::{self, ApiError},
};
use serde::Deserialize;
use serde_json::{Value, json};
use sqlx::{PgConnection, Postgres, Transaction};

#[derive(Default, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct HistoryQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub action: Option<String>,
}
#[derive(Clone, Copy)]
pub(crate) enum Kind {
    Notes,
    Activity,
    Emails,
}
pub(super) fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error, "Member profile/history persistence failed");
    ApiError::new(
        axum::http::StatusCode::SERVICE_UNAVAILABLE,
        "Member history is unavailable; retry after storage recovers",
    )
}
pub(super) fn positive(id: i64) -> Result<(), ApiError> {
    if id <= 0 {
        return Err(ApiError::validation_error(
            "Member identifiers must be positive",
        ));
    }
    Ok(())
}
pub(super) async fn exists_on(connection: &mut PgConnection, id: i64) -> Result<(), ApiError> {
    positive(id)?;
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE id=$1 AND deleted_at IS NULL)")
            .bind(id)
            .fetch_one(connection)
            .await
            .map_err(db)?;
    if !exists {
        return Err(ApiError::not_found("Member not found"));
    }
    Ok(())
}
async fn lock_on(connection: &mut PgConnection, id: i64) -> Result<(), ApiError> {
    positive(id)?;
    // Stabilize soft deletion without blocking the actor's FK KEY SHARE locks.
    let id: Option<i64> = sqlx::query_scalar(
        "SELECT id FROM users WHERE id=$1 AND deleted_at IS NULL FOR NO KEY UPDATE",
    )
    .bind(id)
    .fetch_optional(connection)
    .await
    .map_err(db)?;
    id.ok_or_else(|| ApiError::not_found("Member not found"))?;
    Ok(())
}
impl HistoryQuery {
    fn limits(&self, kind: Kind) -> Result<(i64, i64, i64), ApiError> {
        let page = self.page.unwrap_or(1);
        let size = self.per_page.unwrap_or(25);
        if page < 1 || !(1..=100).contains(&size) {
            return Err(ApiError::validation_error(
                "Use a positive page and a page size from1 to100",
            ));
        }
        if let Some(action) = &self.action {
            if !matches!(kind, Kind::Activity)
                || action.trim().is_empty()
                || action.chars().count() > 100
                || action.chars().any(char::is_control)
            {
                return Err(ApiError::validation_error(
                    "Use a bounded recorded activity action",
                ));
            }
        }
        let offset = (page - 1)
            .checked_mul(size)
            .ok_or_else(|| ApiError::validation_error("Requested history page is too large"))?;
        Ok((page, size, offset))
    }
}

const NOTE_SELECT: &str = r#"SELECT jsonb_build_object('id',n.id,'user_id',n.user_id,'content',n.note,'created_by',n.created_by,'created_by_name',a.name,'created_at',to_char(n.created_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'updated_at',to_char(n.updated_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"')) FROM member_notes n LEFT JOIN users a ON a.id=n.created_by WHERE n.user_id=$1 AND $2::text IS NULL ORDER BY n.created_at DESC NULLS LAST,n.id DESC LIMIT $3 OFFSET $4"#;
const ACTIVITY_SELECT: &str = r#"SELECT jsonb_build_object('id',n.id,'user_id',n.user_id,'action',n.action,'description',n.details->>'description','metadata',n.details,'ip_address',n.ip_address::text,'user_agent',n.user_agent,'created_at',to_char(n.created_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"')) FROM user_activity_log n WHERE n.user_id=$1 AND ($2::text IS NULL OR n.action=$2) ORDER BY n.created_at DESC NULLS LAST,n.id DESC LIMIT $3 OFFSET $4"#;
const EMAIL_SELECT: &str = r#"SELECT jsonb_build_object('id',n.id,'user_id',n.user_id,'recipient',n.to_email,'subject',n.subject,'status',n.status,'provider',n.provider,'provider_message_id',n.provider_message_id,'sent_at',to_char(n.sent_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'queued_at',to_char(n.queued_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'created_at',n.created_at,'opened_at',n.opened_at,'clicked_at',n.clicked_at,'bounced_at',n.bounced_at,'error',COALESCE(n.error,n.error_message)) FROM email_logs n WHERE n.user_id=$1 AND $2::text IS NULL ORDER BY n.created_at DESC NULLS LAST,n.id DESC LIMIT $3 OFFSET $4"#;

pub(crate) async fn read_on(
    connection: &mut PgConnection,
    member: i64,
    query: &HistoryQuery,
    kind: Kind,
) -> Result<Value, ApiError> {
    let (page, size, offset) = query.limits(kind)?;
    exists_on(connection, member).await?;
    let (name, count_sql, select_sql) = match kind {
        Kind::Notes => (
            "notes",
            "SELECT COUNT(*) FROM member_notes WHERE user_id=$1 AND $2::text IS NULL",
            NOTE_SELECT,
        ),
        Kind::Activity => (
            "activity",
            "SELECT COUNT(*) FROM user_activity_log WHERE user_id=$1 AND ($2::text IS NULL OR action=$2)",
            ACTIVITY_SELECT,
        ),
        Kind::Emails => (
            "emails",
            "SELECT COUNT(*) FROM email_logs WHERE user_id=$1 AND $2::text IS NULL",
            EMAIL_SELECT,
        ),
    };
    let total: i64 = sqlx::query_scalar(sqlx::AssertSqlSafe(count_sql))
        .bind(member)
        .bind(&query.action)
        .fetch_one(&mut *connection)
        .await
        .map_err(db)?;
    let rows: Vec<Value> = sqlx::query_scalar(sqlx::AssertSqlSafe(select_sql))
        .bind(member)
        .bind(&query.action)
        .bind(size)
        .bind(offset)
        .fetch_all(&mut *connection)
        .await
        .map_err(db)?;
    let pages = (total / size + i64::from(total % size != 0)).max(1);
    let mut result = json!({
        "schema_version":1,"metrics_scope":format!("member_{name}"),"member_id":member,
        "pagination":{"total":total,"per_page":size,"current_page":page,"last_page":pages},
        "timestamp_basis":if matches!(kind,Kind::Emails){"created/open/click/bounce values are recorded timestamps without timezone; sent/queued instants are UTC"}else{"recorded UTC instants"}
    });
    result[name] = json!(rows);
    if matches!(kind, Kind::Emails) {
        result["coverage"] = json!(
            "Only email_logs explicitly linked by recorded user_id; addresses are not guessed or reassigned"
        );
        result["observations"] = json!(
            "Accepted/sent is a recorded provider outcome; opens/clicks are signals, not proof of human engagement"
        );
    }
    Ok(result)
}
pub(crate) async fn read(
    state: &AppState,
    member: i64,
    query: &HistoryQuery,
    kind: Kind,
) -> Result<Value, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let result = read_on(&mut tx, member, query, kind).await?;
    tx.commit().await.map_err(db)?;
    Ok(result)
}
async fn journal(
    tx: &mut Transaction<'_, Postgres>,
    actor: i64,
    member: i64,
    note: i64,
    action: &str,
) -> Result<(), ApiError> {
    let context = json!({"operation":action,"member_id":member,"note_id":note,"actor_id":actor,"description":if action=="member_note_created"{"Member note added"}else{"Member note removed"}});
    sqlx::query("INSERT INTO user_activity_log(user_id,action,entity_type,entity_id,details,created_at) VALUES($1,$2,'member_note',$3,$4,NOW())")
        .bind(member).bind(action).bind(note).bind(&context).execute(&mut **tx).await.map_err(db)?;
    crm_logs::record(
        tx,
        "info",
        "system",
        if action == "member_note_created" {
            "Member note added"
        } else {
            "Member note removed"
        },
        context,
        Some(actor),
    )
    .await
    .map_err(db)
}
pub(crate) async fn add_note(
    state: &AppState,
    actor: i64,
    member: i64,
    content: String,
) -> Result<Value, ApiError> {
    if content.trim().is_empty()
        || content.chars().count() > 10_000
        || content.len() > 40_000
        || content
            .chars()
            .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    {
        return Err(ApiError::validation_error(
            "Provide a nonempty note within10000 characters, without control characters",
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    lock_on(&mut tx, member).await?;
    let note:Value=sqlx::query_scalar(r#"WITH saved AS(INSERT INTO member_notes(user_id,note,created_by,created_at,updated_at) VALUES($1,$2,$3,NOW(),NOW()) RETURNING *) SELECT jsonb_build_object('id',n.id,'user_id',n.user_id,'content',n.note,'created_by',n.created_by,'created_by_name',a.name,'created_at',to_char(n.created_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"'),'updated_at',to_char(n.updated_at AT TIME ZONE 'UTC','YYYY-MM-DD"T"HH24:MI:SS.US"Z"')) FROM saved n LEFT JOIN users a ON a.id=n.created_by"#)
        .bind(member).bind(content).bind(actor).fetch_one(&mut *tx).await.map_err(db)?;
    let id = note["id"]
        .as_i64()
        .ok_or_else(|| ApiError::internal_error("Saved note identity is unavailable"))?;
    journal(&mut tx, actor, member, id, "member_note_created").await?;
    tx.commit().await.map_err(db)?;
    Ok(
        json!({"schema_version":1,"metrics_scope":"member_note_mutation","member_id":member,"note":note}),
    )
}
pub(crate) async fn delete_note(
    state: &AppState,
    actor: i64,
    member: i64,
    note: i64,
) -> Result<Value, ApiError> {
    positive(note)?;
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    lock_on(&mut tx, member).await?;
    let owner: Option<i64> =
        sqlx::query_scalar("SELECT user_id FROM member_notes WHERE id=$1 FOR UPDATE")
            .bind(note)
            .fetch_optional(&mut *tx)
            .await
            .map_err(db)?;
    if owner.is_some_and(|id| id != member) {
        return Err(ApiError::not_found("Member note not found"));
    }
    let removed = if owner.is_some() {
        let rows = sqlx::query("DELETE FROM member_notes WHERE id=$1 AND user_id=$2")
            .bind(note)
            .bind(member)
            .execute(&mut *tx)
            .await
            .map_err(db)?
            .rows_affected();
        if rows != 1 {
            return Err(errors::conflict(
                "Member note changed; reload before retrying",
            ));
        }
        journal(&mut tx, actor, member, note, "member_note_deleted").await?;
        true
    } else {
        false
    };
    tx.commit().await.map_err(db)?;
    Ok(
        json!({"schema_version":1,"metrics_scope":"member_note_mutation","member_id":member,"note_id":note,"removed":removed}),
    )
}
