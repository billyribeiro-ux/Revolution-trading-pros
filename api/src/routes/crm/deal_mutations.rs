//! Validated deal writes. A deal, its stage history and audit activity commit together.
use super::deals_pipelines::{
    CreateDealInput, CrmError, DEAL_COLS, DealRow, LoseDealInput, UpdateDealInput,
    UpdateDealStageInput, WinDealInput, crm_db_error, deal_json,
};
use crate::AppState;
use axum::{Json, http::StatusCode};
use chrono::NaiveDate;
use serde_json::{Value, json};
use sqlx::{Postgres, Transaction};

type Tx<'a> = Transaction<'a, Postgres>;
type Response = Result<Json<Value>, CrmError>;
fn invalid(message: &str) -> CrmError {
    (StatusCode::BAD_REQUEST, Json(json!({"error":message})))
}
pub(super) fn text(value: &str, field: &str, max: usize, required: bool) -> Result<(), CrmError> {
    if value.contains('\0') || value.chars().count() > max || (required && value.trim().is_empty())
    {
        return Err(invalid(&format!(
            "{field} must contain {}–{max} characters without null characters",
            if required { 1 } else { 0 }
        )));
    }
    Ok(())
}
fn date(value: &str) -> Result<NaiveDate, CrmError> {
    let parsed = NaiveDate::parse_from_str(value, "%Y-%m-%d")
        .map_err(|_| invalid("Date must be a valid YYYY-MM-DD date"))?;
    if parsed.format("%Y-%m-%d").to_string() != value {
        return Err(invalid("Date must use YYYY-MM-DD"));
    }
    Ok(parsed)
}
fn clean_json(value: &Value, depth: usize) -> bool {
    if depth > 32 {
        return false;
    }
    match value {
        Value::String(s) => !s.contains('\0'),
        Value::Array(items) => items.iter().all(|v| clean_json(v, depth + 1)),
        Value::Object(items) => items
            .iter()
            .all(|(k, v)| !k.contains('\0') && clean_json(v, depth + 1)),
        _ => true,
    }
}
fn fields(
    name: Option<&str>,
    cents: Option<i64>,
    currency: Option<&str>,
    probability: Option<i32>,
    priority: Option<&str>,
    tags: Option<&Vec<String>>,
    custom: Option<&Value>,
) -> Result<(), CrmError> {
    if let Some(v) = name {
        text(v, "Name", 255, true)?;
    }
    if cents.is_some_and(|v| !(0..=999_999_999_999_999).contains(&v)) {
        return Err(invalid(
            "Amount must be nonnegative integer cents within the supported range",
        ));
    }
    if currency.is_some_and(|v| v.len() != 3 || !v.bytes().all(|b| b.is_ascii_uppercase())) {
        return Err(invalid("Currency must be a three-letter uppercase code"));
    }
    if probability.is_some_and(|v| !(0..=100).contains(&v)) {
        return Err(invalid("Probability must be between 0 and 100"));
    }
    if priority.is_some_and(|v| !["low", "medium", "high", "urgent"].contains(&v)) {
        return Err(invalid("Invalid priority"));
    }
    if let Some(tags) = tags {
        if tags.len() > 50 {
            return Err(invalid("At most 50 tags are allowed"));
        }
        for tag in tags {
            text(tag, "Tag", 100, true)?;
        }
    }
    if let Some(v) = custom
        && (!v.is_object() || v.to_string().len() > 65_536 || !clean_json(v, 0))
    {
        return Err(invalid(
            "Custom fields must be an object of at most 64 KiB without null characters or excessive nesting",
        ));
    }
    Ok(())
}
#[derive(sqlx::FromRow)]
struct Stage {
    id: i64,
    pipeline_id: i64,
    probability: i32,
    is_closed_won: bool,
    is_closed_lost: bool,
}
async fn selected_stage(tx: &mut Tx<'_>, pipeline: i64, stage: i64) -> Result<Stage, CrmError> {
    let active: Option<i64> =
        sqlx::query_scalar("SELECT id FROM crm_pipelines WHERE id=$1 AND is_active=true FOR SHARE")
            .bind(pipeline)
            .fetch_optional(&mut **tx)
            .await
            .map_err(crm_db_error)?;
    if active.is_none() {
        return Err(invalid("Select an active pipeline"));
    }
    let stage:Stage=sqlx::query_as("SELECT id,pipeline_id,probability,is_closed_won,is_closed_lost FROM crm_pipeline_stages WHERE id=$1 AND pipeline_id=$2 FOR SHARE").bind(stage).bind(pipeline).fetch_optional(&mut **tx).await.map_err(crm_db_error)?.ok_or_else(||invalid("Stage does not belong to the selected pipeline"))?;
    if stage.is_closed_won && stage.is_closed_lost {
        return Err(invalid("Stage cannot be both won and lost"));
    }
    Ok(stage)
}
async fn associations(
    tx: &mut Tx<'_>,
    contact: Option<i64>,
    company: Option<i64>,
    owner: Option<i64>,
) -> Result<(), CrmError> {
    // Each optional reference is checked and locked until the write commits.
    for (id, query, label) in [
        (
            contact,
            "SELECT id FROM contacts WHERE id=$1 FOR KEY SHARE",
            "Contact",
        ),
        (
            company,
            "SELECT id FROM crm_companies WHERE id=$1 FOR KEY SHARE",
            "Company",
        ),
        (
            owner,
            "SELECT id FROM users WHERE id=$1 FOR KEY SHARE",
            "Owner",
        ),
    ] {
        if let Some(id) = id {
            let found: Option<i64> = sqlx::query_scalar(sqlx::AssertSqlSafe(query))
                .bind(id)
                .fetch_optional(&mut **tx)
                .await
                .map_err(crm_db_error)?;
            if found.is_none() {
                return Err(invalid(&format!("{label} does not exist")));
            }
        }
    }
    Ok(())
}
async fn fetch(tx: &mut Tx<'_>, id: i64, lock: bool) -> Result<DealRow, CrmError> {
    let sql = format!(
        "SELECT {DEAL_COLS} FROM crm_deals WHERE id=$1 {}",
        if lock { "FOR UPDATE" } else { "" }
    );
    sqlx::query_as(sqlx::AssertSqlSafe(sql.as_str()))
        .bind(id)
        .fetch_optional(&mut **tx)
        .await
        .map_err(crm_db_error)?
        .ok_or_else(|| {
            (
                StatusCode::NOT_FOUND,
                Json(json!({"error":"Deal not found"})),
            )
        })
}
async fn activity(
    tx: &mut Tx<'_>,
    id: i64,
    actor: i64,
    kind: &str,
    title: &str,
    metadata: Value,
) -> Result<(), CrmError> {
    sqlx::query("INSERT INTO crm_deal_activities(deal_id,activity_type,title,created_by,metadata,created_at,updated_at) VALUES($1,$2,$3,$4,$5,NOW(),NOW())").bind(id).bind(kind).bind(title).bind(actor).bind(metadata).execute(&mut **tx).await.map_err(crm_db_error)?;
    Ok(())
}
async fn finish(mut tx: Tx<'_>, id: i64) -> Response {
    let result = deal_json(&fetch(&mut tx, id, false).await?)?;
    tx.commit().await.map_err(crm_db_error)?;
    Ok(Json(result))
}
pub(super) async fn create(state: &AppState, actor: i64, input: CreateDealInput) -> Response {
    fields(
        Some(&input.name),
        input.amount_cents,
        input.currency.as_deref(),
        input.probability,
        input.priority.as_deref(),
        input.tags.as_ref(),
        input.custom_fields.as_ref(),
    )?;
    if let Some(v) = &input.source_channel {
        text(v, "Source channel", 100, false)?;
    }
    if let Some(v) = &input.source_campaign {
        text(v, "Source campaign", 255, false)?;
    }
    let expected = input.expected_close_date.as_deref().map(date).transpose()?;
    let mut tx = state.db.pool.begin().await.map_err(crm_db_error)?;
    let stage = selected_stage(&mut tx, input.pipeline_id, input.stage_id).await?;
    if stage.is_closed_won || stage.is_closed_lost {
        return Err(invalid(
            "Create the deal in an open stage, then record its outcome",
        ));
    }
    associations(&mut tx, input.contact_id, input.company_id, input.owner_id).await?;
    let id:i64=sqlx::query_scalar("INSERT INTO crm_deals(name,contact_id,company_id,owner_id,pipeline_id,stage_id,amount,currency,probability,expected_close_date,priority,source_channel,source_campaign,tags,custom_fields,status,stage_entered_at,created_at,updated_at) VALUES($1,$2,$3,$4,$5,$6,$7::bigint/100.0,$8,$9,$10,$11,$12,$13,$14,$15,'open',NOW(),NOW(),NOW()) RETURNING id")
        .bind(input.name.trim()).bind(input.contact_id).bind(input.company_id).bind(input.owner_id).bind(input.pipeline_id).bind(input.stage_id).bind(input.amount_cents.unwrap_or(0)).bind(input.currency.as_deref().unwrap_or("USD")).bind(input.probability.unwrap_or(stage.probability)).bind(expected).bind(input.priority.as_deref().unwrap_or("medium")).bind(input.source_channel).bind(input.source_campaign).bind(json!(input.tags.unwrap_or_default())).bind(input.custom_fields.unwrap_or(json!({}))).fetch_one(&mut *tx).await.map_err(crm_db_error)?;
    activity(
        &mut tx,
        id,
        actor,
        "created",
        "Deal created",
        json!({"pipeline_id":input.pipeline_id,"stage_id":input.stage_id}),
    )
    .await?;
    finish(tx, id).await
}
struct Transition<'a> {
    stage: Stage,
    reason: Option<&'a str>,
    lost: Option<&'a str>,
    won: Option<&'a str>,
    close: Option<NaiveDate>,
}
async fn transition(
    tx: &mut Tx<'_>,
    actor: i64,
    current: &DealRow,
    input: Transition<'_>,
) -> Result<(), CrmError> {
    let status = if input.stage.is_closed_won {
        "won"
    } else if input.stage.is_closed_lost {
        "lost"
    } else {
        "open"
    };
    let moved = current.stage_id != input.stage.id;
    let changed = current.status != status;
    if let Some(v) = input.reason {
        text(v, "Stage change reason", 10_000, false)?;
    }
    if let Some(v) = input.won {
        text(v, "Won details", 10_000, false)?;
    }
    if status == "lost" {
        text(
            input.lost.or(current.lost_reason.as_deref()).unwrap_or(""),
            "Lost reason",
            10_000,
            true,
        )?;
    }
    if moved {
        let seconds = chrono::Utc::now()
            .naive_utc()
            .signed_duration_since(current.stage_entered_at)
            .num_seconds()
            .clamp(0, i64::from(i32::MAX)) as i32;
        sqlx::query("INSERT INTO crm_deal_stage_history(deal_id,from_stage_id,to_stage_id,changed_by,reason,duration_in_stage,created_at) VALUES($1,$2,$3,$4,$5,$6,NOW())").bind(current.id).bind(current.stage_id).bind(input.stage.id).bind(actor).bind(input.reason).bind(seconds).execute(&mut **tx).await.map_err(crm_db_error)?;
    }
    sqlx::query("UPDATE crm_deals SET pipeline_id=$2,stage_id=$3,status=$4,probability=CASE WHEN $4='won' THEN 100 WHEN $4='lost' THEN 0 WHEN $5 THEN $6 ELSE probability END,stage_entered_at=CASE WHEN $5 THEN NOW() ELSE stage_entered_at END,closed_at=CASE WHEN $4='open' THEN NULL WHEN $7 THEN NOW() ELSE closed_at END,close_date=CASE WHEN $4='open' THEN NULL ELSE COALESCE($8,CASE WHEN $7 THEN CURRENT_DATE ELSE close_date END) END,lost_reason=CASE WHEN $4='lost' THEN COALESCE($9,lost_reason) ELSE NULL END,won_details=CASE WHEN $4='won' THEN COALESCE($10,won_details) ELSE NULL END,updated_at=NOW() WHERE id=$1")
        .bind(current.id).bind(input.stage.pipeline_id).bind(input.stage.id).bind(status).bind(moved).bind(input.stage.probability).bind(changed).bind(input.close).bind(input.lost).bind(input.won).execute(&mut **tx).await.map_err(crm_db_error)?;
    if changed {
        activity(
            tx,
            current.id,
            actor,
            if status == "open" { "reopened" } else { status },
            if status == "open" {
                "Deal reopened"
            } else if status == "won" {
                "Deal won"
            } else {
                "Deal lost"
            },
            json!({"from_status":current.status,"to_status":status}),
        )
        .await?;
    }
    Ok(())
}
pub(super) async fn update(
    state: &AppState,
    actor: i64,
    id: i64,
    input: UpdateDealInput,
) -> Response {
    fields(
        input.name.as_deref(),
        input.amount_cents,
        input.currency.as_deref(),
        input.probability,
        input.priority.as_deref(),
        input.tags.as_ref(),
        input.custom_fields.as_ref(),
    )?;
    let expected = input
        .expected_close_date
        .as_ref()
        .and_then(|v| v.as_deref())
        .map(date)
        .transpose()?;
    if let Some(Some(v)) = &input.source_channel {
        text(v, "Source channel", 100, false)?;
    }
    if let Some(Some(v)) = &input.source_campaign {
        text(v, "Source campaign", 255, false)?;
    }
    let mut tx = state.db.pool.begin().await.map_err(crm_db_error)?;
    let current = fetch(&mut tx, id, true).await?;
    associations(
        &mut tx,
        input.contact_id.unwrap_or(current.contact_id),
        input.company_id.unwrap_or(current.company_id),
        input.owner_id.unwrap_or(current.owner_id),
    )
    .await?;
    if input.stage_id.is_some() || input.pipeline_id.is_some() {
        let target = selected_stage(
            &mut tx,
            input.pipeline_id.unwrap_or(current.pipeline_id),
            input.stage_id.unwrap_or(current.stage_id),
        )
        .await?;
        transition(
            &mut tx,
            actor,
            &current,
            Transition {
                stage: target,
                reason: input.stage_change_reason.as_deref(),
                lost: input.lost_reason.as_deref(),
                won: None,
                close: None,
            },
        )
        .await?;
    }
    // Presence flags distinguish an omitted property from an explicit JSON null.
    sqlx::query("UPDATE crm_deals SET name=COALESCE($2,name),amount=COALESCE($3::bigint/100.0,amount),currency=COALESCE($4,currency),probability=CASE WHEN status='won' THEN 100 WHEN status='lost' THEN 0 ELSE COALESCE($5,probability) END,priority=COALESCE($6,priority),expected_close_date=CASE WHEN $7 THEN $8 ELSE expected_close_date END,contact_id=CASE WHEN $9 THEN $10 ELSE contact_id END,company_id=CASE WHEN $11 THEN $12 ELSE company_id END,owner_id=CASE WHEN $13 THEN $14 ELSE owner_id END,source_channel=CASE WHEN $15 THEN $16 ELSE source_channel END,source_campaign=CASE WHEN $17 THEN $18 ELSE source_campaign END,tags=COALESCE($19,tags),custom_fields=COALESCE($20,custom_fields),updated_at=NOW() WHERE id=$1")
        .bind(id).bind(input.name.as_deref().map(str::trim)).bind(input.amount_cents).bind(input.currency).bind(input.probability).bind(input.priority).bind(input.expected_close_date.is_some()).bind(expected).bind(input.contact_id.is_some()).bind(input.contact_id.flatten()).bind(input.company_id.is_some()).bind(input.company_id.flatten()).bind(input.owner_id.is_some()).bind(input.owner_id.flatten()).bind(input.source_channel.is_some()).bind(input.source_channel.flatten()).bind(input.source_campaign.is_some()).bind(input.source_campaign.flatten()).bind(input.tags.map(|v|json!(v))).bind(input.custom_fields).execute(&mut *tx).await.map_err(crm_db_error)?;
    activity(&mut tx, id, actor, "updated", "Deal updated", json!({})).await?;
    finish(tx, id).await
}
pub(super) async fn stage(
    state: &AppState,
    actor: i64,
    id: i64,
    input: UpdateDealStageInput,
) -> Response {
    let mut tx = state.db.pool.begin().await.map_err(crm_db_error)?;
    let current = fetch(&mut tx, id, true).await?;
    let target = selected_stage(&mut tx, current.pipeline_id, input.stage_id).await?;
    transition(
        &mut tx,
        actor,
        &current,
        Transition {
            stage: target,
            reason: input.reason.as_deref(),
            lost: None,
            won: None,
            close: None,
        },
    )
    .await?;
    finish(tx, id).await
}
async fn closing_stage(
    tx: &mut Tx<'_>,
    pipeline: i64,
    stage: Option<i64>,
    won: bool,
) -> Result<Stage, CrmError> {
    let id = if let Some(id) = stage {
        id
    } else {
        sqlx::query_scalar("SELECT id FROM crm_pipeline_stages WHERE pipeline_id=$1 AND is_closed_won=$2 AND is_closed_lost=$3 ORDER BY position,id LIMIT 1").bind(pipeline).bind(won).bind(!won).fetch_optional(&mut **tx).await.map_err(crm_db_error)?.ok_or_else(||invalid("Pipeline has no matching closing stage"))?
    };
    let target = selected_stage(tx, pipeline, id).await?;
    if target.is_closed_won != won || target.is_closed_lost == won {
        return Err(invalid("Select a matching closing stage"));
    }
    Ok(target)
}
pub(super) async fn win(state: &AppState, actor: i64, id: i64, input: WinDealInput) -> Response {
    let close = input.close_date.as_deref().map(date).transpose()?;
    let mut tx = state.db.pool.begin().await.map_err(crm_db_error)?;
    let current = fetch(&mut tx, id, true).await?;
    let stage = closing_stage(&mut tx, current.pipeline_id, input.stage_id, true).await?;
    transition(
        &mut tx,
        actor,
        &current,
        Transition {
            stage,
            reason: None,
            lost: None,
            won: input.won_details.as_deref(),
            close,
        },
    )
    .await?;
    finish(tx, id).await
}
pub(super) async fn lose(state: &AppState, actor: i64, id: i64, input: LoseDealInput) -> Response {
    text(&input.lost_reason, "Lost reason", 10_000, true)?;
    let mut tx = state.db.pool.begin().await.map_err(crm_db_error)?;
    let current = fetch(&mut tx, id, true).await?;
    let stage = closing_stage(&mut tx, current.pipeline_id, input.stage_id, false).await?;
    transition(
        &mut tx,
        actor,
        &current,
        Transition {
            stage,
            reason: None,
            lost: Some(input.lost_reason.trim()),
            won: None,
            close: None,
        },
    )
    .await?;
    finish(tx, id).await
}
