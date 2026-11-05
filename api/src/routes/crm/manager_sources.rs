//! Actual persisted source checks shared by CRM entry handlers. No client facts.
use super::manager_permissions::CrmUser;
use crate::utils::errors::{self, ApiError};
use serde_json::Value;
use sqlx::{PgConnection, PgPool, Postgres, Transaction};

fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error, "CRM source authorization unavailable");
    errors::service_unavailable("CRM source authorization is unavailable")
}
pub(super) async fn read_transaction(
    pool: &PgPool,
) -> Result<Transaction<'static, Postgres>, ApiError> {
    let mut tx = pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    Ok(tx)
}
pub(super) async fn campaign_on(
    connection: &mut PgConnection,
    actor: &CrmUser,
    id: i64,
    source: &Value,
) -> Result<(), ApiError> {
    if !actor.is_delegated() {
        return Ok(());
    }
    actor.record(source)?;
    if actor.needs_history()? {
        let member:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_revisions WHERE campaign_id=$1 AND (definition->'subscriber_settings'->'members'='true'::jsonb OR COALESCE(definition->'subscriber_settings'->'member_segment','null'::jsonb)<>'null'::jsonb))")
            .bind(id).fetch_one(&mut *connection).await.map_err(db)?;
        actor.history_member_source(member)?;
    }
    Ok(())
}
pub(super) async fn campaign_all_on(
    connection: &mut PgConnection,
    actor: &CrmUser,
) -> Result<(), ApiError> {
    if !actor.is_delegated() {
        return Ok(());
    }
    let sources:Vec<(i64,Value)>=sqlx::query_as("SELECT id,jsonb_build_object('subscriber_settings',subscriber_settings,'template_id',template_id,'email_template_id',email_template_id) FROM crm_campaigns WHERE native_email_campaign_id IS NULL AND NOT EXISTS(SELECT 1 FROM crm_sequence_engines e WHERE e.campaign_id=crm_campaigns.id)")
        .fetch_all(&mut *connection).await.map_err(db)?;
    for (id, source) in sources {
        campaign_on(connection, actor, id, &source).await?;
    }
    Ok(())
}
pub(super) async fn sequence_on(
    connection: &mut PgConnection,
    actor: &CrmUser,
    id: i64,
    source: &Value,
) -> Result<(), ApiError> {
    if !actor.is_delegated() {
        return Ok(());
    }
    actor.record(source)?;
    if actor.needs_history()? {
        let member:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_sequence_revisions WHERE sequence_id=$1 AND (definition->'subscriber_settings'->'members'='true'::jsonb OR COALESCE(definition->'subscriber_settings'->'member_segment','null'::jsonb)<>'null'::jsonb))")
            .bind(id).fetch_one(&mut *connection).await.map_err(db)?;
        actor.history_member_source(member)?;
    }
    Ok(())
}
pub(super) async fn sequence_all_on(
    connection: &mut PgConnection,
    actor: &CrmUser,
) -> Result<(), ApiError> {
    if !actor.is_delegated() {
        return Ok(());
    }
    let sources:Vec<(i64,Value)>=sqlx::query_as("SELECT id,jsonb_build_object('subscriber_settings',subscriber_settings) FROM crm_sequences WHERE deleted_at IS NULL")
        .fetch_all(&mut *connection).await.map_err(db)?;
    for (id, source) in sources {
        sequence_on(connection, actor, id, &source).await?;
    }
    Ok(())
}
