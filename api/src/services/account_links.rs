//! One-use, recipient-bound account capabilities; no role or verified identity is inferred.
use crate::{
    services::crm_logs,
    utils::{
        self,
        errors::{self, ApiError},
    },
};
use serde_json::{Value, json};
use sqlx::{PgConnection, PgPool, Postgres, Transaction};
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"Account capability persistence failed");
    errors::service_unavailable("Account capability storage is unavailable")
}
fn invalid() -> ApiError {
    ApiError::validation_error("This link is invalid or expired; request a new link")
}
fn bounded(raw: &str) -> Result<(), ApiError> {
    if !(32..=128).contains(&raw.len()) || !raw.bytes().all(|b| b.is_ascii_alphanumeric()) {
        return Err(invalid());
    }
    Ok(())
}
async fn begin(pool: &PgPool) -> Result<Transaction<'_, Postgres>, ApiError> {
    let mut tx = pool.begin().await.map_err(db)?;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    Ok(tx)
}
async fn current(connection: &mut PgConnection, id: i64) -> Result<(String, i64), ApiError> {
    sqlx::query_as("SELECT email,auth_version FROM users WHERE id=$1 AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted' FOR NO KEY UPDATE").bind(id).fetch_optional(connection).await.map_err(db)?.ok_or_else(invalid)
}
async fn audit(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    operation: &str,
) -> Result<(), ApiError> {
    let context = json!({"operation":operation,"member_id":id,"actor_id":null,"source":"one_use_account_capability","description":operation.replace('_'," ")});
    sqlx::query("INSERT INTO user_activity_log(user_id,action,entity_type,entity_id,details,created_at) VALUES($1,$2,'member_account',$1,$3,NOW())").bind(id).bind(operation).bind(&context).execute(&mut **tx).await.map_err(db)?;
    crm_logs::record(
        tx,
        "info",
        "system",
        &operation.replace('_', " "),
        context,
        None,
    )
    .await
    .map_err(db)
}
/// Returns the raw value once to the sender, never to logs or a stored template model.
pub async fn verification_on(
    connection: &mut PgConnection,
    id: i64,
) -> Result<(String, String, String), ApiError> {
    let (email, version) = current(connection, id).await?;
    let name: String = sqlx::query_scalar("SELECT name FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(&mut *connection)
        .await
        .map_err(db)?;
    let (raw, hash) = utils::generate_verification_token();
    sqlx::query("DELETE FROM email_verification_tokens WHERE user_id=$1")
        .bind(id)
        .execute(&mut *connection)
        .await
        .map_err(db)?;
    sqlx::query("INSERT INTO email_verification_tokens(user_id,token,recipient,issued_auth_version,expires_at,created_at) VALUES($1,$2,$3,$4,NOW()+INTERVAL '24 hours',NOW())").bind(id).bind(hash).bind(&email).bind(version).execute(connection).await.map_err(db)?;
    Ok((raw, email, name))
}
pub async fn password_on(
    connection: &mut PgConnection,
    id: i64,
) -> Result<(String, String, String), ApiError> {
    let (email, version) = current(connection, id).await?;
    let name: String = sqlx::query_scalar("SELECT name FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(&mut *connection)
        .await
        .map_err(db)?;
    let (raw, hash) = utils::generate_verification_token();
    sqlx::query("DELETE FROM password_resets WHERE user_id=$1 OR email=$2")
        .bind(id)
        .bind(&email)
        .execute(&mut *connection)
        .await
        .map_err(db)?;
    sqlx::query("INSERT INTO password_resets(user_id,email,token,issued_auth_version,expires_at,created_at) VALUES($1,$2,$3,$4,(NOW() AT TIME ZONE 'UTC')+INTERVAL '1 hour',NOW() AT TIME ZONE 'UTC')").bind(id).bind(&email).bind(hash).bind(version).execute(connection).await.map_err(db)?;
    Ok((raw, email, name))
}
pub async fn verify(pool: &PgPool, raw: &str) -> Result<Value, ApiError> {
    bounded(raw)?;
    let hash = utils::hash_token(raw);
    let mut tx = begin(pool).await?;
    // The table's actual primary key is UUID; deletion uses its unique hashed token instead.
    let row:Option<(i64,String,i64)>=sqlx::query_as("SELECT user_id,recipient,issued_auth_version FROM email_verification_tokens WHERE token=$1 AND recipient IS NOT NULL AND issued_auth_version IS NOT NULL AND expires_at>NOW() FOR UPDATE").bind(&hash).fetch_optional(&mut *tx).await.map_err(db)?;
    let Some((id, recipient, version)) = row else {
        return Err(invalid());
    };
    let (email, current) = current(&mut tx, id).await?;
    if email != recipient || version != current {
        return Err(invalid());
    }
    sqlx::query("UPDATE users SET email_verified_at=COALESCE(email_verified_at,NOW() AT TIME ZONE 'UTC'),updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db)?;
    let removed = sqlx::query("DELETE FROM email_verification_tokens WHERE token=$1")
        .bind(hash)
        .execute(&mut *tx)
        .await
        .map_err(db)?
        .rows_affected();
    if removed != 1 {
        return Err(invalid());
    }
    audit(&mut tx, id, "email_verified").await?;
    tx.commit().await.map_err(db)?;
    Ok(
        json!({"success":true,"message":"Email verified. Sign in again using the account's current password and MFA requirements.","email_verified":true,"reauthentication_required":true}),
    )
}
pub async fn reset(
    pool: &PgPool,
    raw: &str,
    expected_email: &str,
    password_hash: String,
) -> Result<Value, ApiError> {
    bounded(raw)?;
    let hash = utils::hash_token(raw);
    let mut tx = begin(pool).await?;
    let row:Option<(i64,i64,String,i64)>=sqlx::query_as("SELECT id,user_id,email,issued_auth_version FROM password_resets WHERE token=$1 AND LOWER(email)=LOWER($2) AND user_id IS NOT NULL AND issued_auth_version IS NOT NULL AND expires_at>NOW() AT TIME ZONE 'UTC' FOR UPDATE").bind(hash).bind(expected_email).fetch_optional(&mut *tx).await.map_err(db)?;
    let Some((_, id, recipient, version)) = row else {
        return Err(invalid());
    };
    let (email, current) = current(&mut tx, id).await?;
    if email != recipient || version != current {
        return Err(invalid());
    }
    sqlx::query(
        "UPDATE users SET password_hash=$2,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1",
    )
    .bind(id)
    .bind(password_hash)
    .execute(&mut *tx)
    .await
    .map_err(db)?;
    sqlx::query("DELETE FROM password_resets WHERE user_id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    audit(&mut tx, id, "password_reset").await?;
    tx.commit().await.map_err(db)?;
    Ok(
        json!({"success":true,"message":"Password changed. Sign in again using the account's current MFA and access requirements.","reauthentication_required":true,"tokens_revoked":true}),
    )
}
