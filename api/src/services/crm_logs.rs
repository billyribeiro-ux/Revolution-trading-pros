//! Record operational facts in the same transaction as their durable effects.
use serde_json::Value;
use sqlx::{Postgres, Transaction};
pub async fn record(
    tx: &mut Transaction<'_, Postgres>,
    level: &str,
    category: &str,
    message: &str,
    context: Value,
    actor: Option<i64>,
) -> Result<(), sqlx::Error> {
    sqlx::query("INSERT INTO crm_operational_logs(level,category,message,context,user_id) VALUES($1,$2,$3,$4,$5)")
        .bind(level).bind(category).bind(message).bind(context).bind(actor).execute(&mut **tx).await?;
    Ok(())
}
