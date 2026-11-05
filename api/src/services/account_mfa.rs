//! Canonical MFA state transitions and one-use challenges. Secrets never leave initial setup.
use crate::{
    AppState,
    models::{AuthResponse, UserResponse},
    services::{
        account_authority::{self, Account, VerifiedAuthority},
        crm_logs, mfa,
    },
    utils::{
        self,
        errors::{self, ApiError},
    },
};
use serde_json::{Value, json};
use sqlx::{Postgres, Transaction};
use std::{collections::HashMap, net::IpAddr};
use uuid::Uuid;
#[derive(Clone, Copy)]
pub enum Proof<'a> {
    Totp(&'a str),
    Backup(&'a str),
}
fn unavailable() -> ApiError {
    errors::service_unavailable("MFA storage or recorded configuration is unavailable")
}
fn db(error: sqlx::Error) -> ApiError {
    tracing::error!(%error,"MFA persistence failed");
    unavailable()
}
fn account_error((status, message): account_authority::Failure) -> ApiError {
    ApiError::new(status, message)
}
async fn begin(
    state: &AppState,
    id: i64,
    version: i64,
) -> Result<(Transaction<'_, Postgres>, Account), ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    // Match the account administrator lock order before taking user or secret row locks.
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR NO KEY UPDATE")
        .bind(id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(|| errors::unauthorized("Account is unavailable"))?;
    let account = account_authority::current_on(&mut tx, id)
        .await
        .map_err(account_error)?;
    if account.version != version {
        return Err(errors::unauthorized(
            "Account authority changed; authenticate again",
        ));
    }
    Ok((tx, account))
}
async fn rate(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    ip: Option<IpAddr>,
) -> Result<(), ApiError> {
    let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM mfa_attempts WHERE (user_id=$1 OR ip_address=$2::text::inet) AND NOT success AND created_at>NOW()-INTERVAL '15 minutes'").bind(id).bind(ip.map(|v|v.to_string())).fetch_one(&mut **tx).await.map_err(db)?;
    if count >= 5 {
        return Err(ApiError::new(
            axum::http::StatusCode::TOO_MANY_REQUESTS,
            "Too many MFA failures; wait before trying again",
        ));
    }
    Ok(())
}
async fn attempt(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    ip: Option<IpAddr>,
    success: bool,
    kind: &str,
) -> Result<(), ApiError> {
    sqlx::query("INSERT INTO mfa_attempts(user_id,ip_address,success,attempt_type,created_at) VALUES($1,$2::text::inet,$3,$4,NOW())").bind(id).bind(ip.map(|v|v.to_string())).bind(success).bind(kind).execute(&mut **tx).await.map_err(db)?;
    Ok(())
}
async fn audit(
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    action: &str,
    context: Value,
) -> Result<(), ApiError> {
    let context = json!({"operation":action,"member_id":id,"actor_id":id,"source":"self_mfa","description":action.replace('_'," "),"details":context});
    sqlx::query("INSERT INTO user_activity_log(user_id,action,entity_type,entity_id,details,created_at) VALUES($1,$2,'member_mfa',$1,$3,NOW())").bind(id).bind(action).bind(&context).execute(&mut **tx).await.map_err(db)?;
    crm_logs::record(
        tx,
        "info",
        "system",
        &action.replace('_', " "),
        context,
        Some(id),
    )
    .await
    .map_err(db)
}
async fn password(password: &str, account: &Account) -> Result<bool, ApiError> {
    if password.is_empty() || password.len() > 128 {
        return Err(ApiError::validation_error(
            "Provide the current bounded account password",
        ));
    }
    utils::verify_password_async(password, &account.user.password_hash)
        .await
        .map_err(|_| unavailable())
}
fn encrypted(state: &AppState, secret: &str) -> Result<String, ApiError> {
    if state.config.credentials_encryption_key.len() < 32 {
        return Err(unavailable());
    }
    let blob = utils::crypto::encrypt_map(
        &state.config.credentials_encryption_key,
        &HashMap::from([("totp".to_owned(), secret.to_owned())]),
    );
    if !blob.starts_with("v1:") {
        return Err(unavailable());
    }
    Ok(blob)
}
fn plain(state: &AppState, blob: &str) -> Result<String, ApiError> {
    if blob.starts_with("v1:") {
        utils::crypto::decrypt_map(&state.config.credentials_encryption_key, blob)
            .remove("totp")
            .ok_or_else(unavailable)
    } else if blob.len() == 32
        && blob
            .bytes()
            .all(|b| b.is_ascii_uppercase() || matches!(b, b'2'..=b'7'))
    {
        Ok(blob.to_owned())
    } else {
        Err(unavailable())
    }
}
async fn secret(tx: &mut Transaction<'_, Postgres>, id: i64) -> Result<Option<Value>, ApiError> {
    sqlx::query_scalar("SELECT jsonb_build_object('id',id,'secret',totp_secret,'verified',totp_verified_at IS NOT NULL,'hashes',backup_codes,'used',backup_codes_used,'last_step',last_totp_timestep) FROM user_mfa_secrets WHERE user_id=$1 FOR UPDATE").bind(id).fetch_optional(&mut **tx).await.map_err(db)
}
fn numeric(value: &Value, key: &str) -> Result<i64, ApiError> {
    value[key]
        .as_i64()
        .filter(|v| *v >= 0)
        .ok_or_else(unavailable)
}
fn normalize_backup(value: &str) -> Result<String, ApiError> {
    let code = value.trim().replace('-', "").to_ascii_uppercase();
    if !(8..=64).contains(&code.len()) || !code.bytes().all(|v| v.is_ascii_alphanumeric()) {
        return Err(ApiError::validation_error(
            "Provide one valid recovery code",
        ));
    }
    Ok(code)
}
/// Consumption is protected by the exact locked secret row, not an earlier snapshot.
async fn consume(
    state: &AppState,
    tx: &mut Transaction<'_, Postgres>,
    id: i64,
    row: &Value,
    proof: Proof<'_>,
) -> Result<bool, ApiError> {
    match proof {
        Proof::Totp(code) => {
            if code.len() != 6 || !code.bytes().all(|v| v.is_ascii_digit()) {
                return Err(ApiError::validation_error(
                    "Provide a six-digit authenticator code",
                ));
            }
            let raw = row["secret"].as_str().ok_or_else(unavailable)?;
            let key = plain(state, raw)?;
            let now = chrono::Utc::now().timestamp();
            let mut found = None;
            for offset in [1i64, 0, -1] {
                let at = now
                    .checked_add(offset * 30)
                    .filter(|v| *v >= 0)
                    .ok_or_else(unavailable)?;
                let candidate =
                    mfa::generate_totp(&key, u64::try_from(at).map_err(|_| unavailable())?)?;
                if utils::constant_time_compare(code, &candidate) {
                    found = Some(at / 30);
                    break;
                }
            }
            let Some(step) = found else { return Ok(false) };
            if !row["last_step"].is_null() && numeric(row, "last_step")? >= step {
                return Ok(false);
            }
            sqlx::query("UPDATE user_mfa_secrets SET last_totp_timestep=$2,updated_at=NOW() WHERE user_id=$1").bind(id).bind(step).execute(&mut **tx).await.map_err(db)?;
            if !raw.starts_with("v1:") {
                sqlx::query("UPDATE user_mfa_secrets SET totp_secret=$2 WHERE user_id=$1")
                    .bind(id)
                    .bind(encrypted(state, &key)?)
                    .execute(&mut **tx)
                    .await
                    .map_err(db)?;
            }
            Ok(true)
        }
        Proof::Backup(code) => {
            let code = normalize_backup(code)?;
            let hashes = row["hashes"].as_array().ok_or_else(unavailable)?;
            let used = numeric(row, "used")?;
            if used >= i64::from(i32::MAX) {
                return Err(unavailable());
            }
            let mut found = None;
            for (index, hash) in hashes.iter().enumerate() {
                let hash = hash
                    .as_str()
                    .filter(|v| {
                        v.starts_with("$argon2id$")
                            || v.starts_with("$2a$")
                            || v.starts_with("$2b$")
                            || v.starts_with("$2y$")
                    })
                    .ok_or_else(unavailable)?;
                if utils::verify_password_async(&code, hash)
                    .await
                    .map_err(|_| unavailable())?
                {
                    found = Some(index);
                    break;
                }
            }
            let Some(index) = found else { return Ok(false) };
            let mut remaining = hashes.clone();
            remaining.remove(index);
            sqlx::query("UPDATE user_mfa_secrets SET backup_codes=$2,backup_codes_used=backup_codes_used+1,updated_at=NOW() WHERE user_id=$1 AND totp_verified_at IS NOT NULL").bind(id).bind(json!(remaining)).execute(&mut **tx).await.map_err(db)?;
            Ok(true)
        }
    }
}
pub async fn credentials(
    state: &AppState,
    account: Account,
    assured: bool,
    ip: Option<IpAddr>,
) -> Result<AuthResponse, ApiError> {
    let required = account_authority::require_mfa_state(&account).map_err(account_error)?;
    if required && !assured {
        return Err(errors::unauthorized("An actual MFA challenge is required"));
    }
    let token = utils::create_jwt_authenticated(
        account.user.id,
        &state.config.jwt_secret,
        state.config.jwt_expires_in,
        account.version,
        assured,
    )
    .map_err(|_| unavailable())?;
    let refresh_token = utils::create_refresh_token_authenticated(
        account.user.id,
        &state.config.jwt_secret,
        account.version,
        assured,
    )
    .map_err(|_| unavailable())?;
    let session_id = utils::generate_session_id();
    if let Some(redis) = &state.services.redis
        && let Err(error) = redis
            .create_session(
                &session_id,
                account.user.id,
                &account.user.email,
                ip.map(|v| v.to_string()),
                None,
            )
            .await
    {
        tracing::warn!(%error,"Authenticated tokens issued; optional device session bookkeeping failed")
    }
    Ok(AuthResponse {
        token: token.clone(),
        access_token: token,
        refresh_token,
        session_id,
        user: UserResponse::from_authenticated(account.user, &state.config),
        expires_in: state.config.jwt_expires_in * 3600,
    })
}
pub async fn setup(
    state: &AppState,
    verified: VerifiedAuthority,
    password_input: &str,
    ip: Option<IpAddr>,
) -> Result<Value, ApiError> {
    let (mut tx, account) = begin(state, verified.subject, verified.version).await?;
    account_authority::verified_on(&mut tx, &verified)
        .await
        .map_err(account_error)?;
    rate(&mut tx, verified.subject, ip).await?;
    if !password(password_input, &account).await? {
        attempt(&mut tx, verified.subject, ip, false, "setup_password").await?;
        tx.commit().await.map_err(db)?;
        return Err(errors::unauthorized("The current password is incorrect"));
    }
    if account.user.mfa_enabled == Some(true) || account.mfa_verified {
        return Err(errors::conflict(
            "Disable existing MFA through its authenticated workflow before replacing it",
        ));
    }
    let recent:i64=sqlx::query_scalar("SELECT COUNT(*) FROM mfa_attempts WHERE user_id=$1 AND success AND attempt_type='setup_password' AND created_at>NOW()-INTERVAL '15 minutes'").bind(verified.subject).fetch_one(&mut *tx).await.map_err(db)?;
    if recent >= 3 {
        return Err(ApiError::new(
            axum::http::StatusCode::TOO_MANY_REQUESTS,
            "Too many MFA setup replacements; wait before trying again",
        ));
    }
    let key = mfa::generate_totp_secret();
    let codes: Vec<String> = (0..10)
        .map(|_| {
            let raw = Uuid::new_v4().simple().to_string().to_ascii_uppercase();
            format!(
                "{}-{}-{}-{}",
                &raw[0..8],
                &raw[8..16],
                &raw[16..24],
                &raw[24..32]
            )
        })
        .collect();
    let mut hashes = Vec::new();
    for code in &codes {
        hashes.push(
            utils::hash_password_async(&normalize_backup(code)?)
                .await
                .map_err(|_| unavailable())?,
        )
    }
    sqlx::query("INSERT INTO user_mfa_secrets(user_id,totp_secret,totp_verified_at,backup_codes,backup_codes_generated_at,backup_codes_used,last_totp_timestep,created_at,updated_at) VALUES($1,$2,NULL,$3,NOW(),0,NULL,NOW(),NOW()) ON CONFLICT(user_id) DO UPDATE SET totp_secret=EXCLUDED.totp_secret,totp_verified_at=NULL,backup_codes=EXCLUDED.backup_codes,backup_codes_generated_at=NOW(),backup_codes_used=0,last_totp_timestep=NULL,updated_at=NOW()").bind(verified.subject).bind(encrypted(state,&key)?).bind(json!(hashes)).execute(&mut *tx).await.map_err(db)?;
    attempt(&mut tx, verified.subject, ip, true, "setup_password").await?;
    audit(
        &mut tx,
        verified.subject,
        "mfa_setup_started",
        json!({"backup_count":codes.len()}),
    )
    .await?;
    let account = account_authority::current_on(&mut tx, verified.subject)
        .await
        .map_err(account_error)?;
    tx.commit().await.map_err(db)?;
    let uri = mfa::generate_qr_uri(&key, &account.user.email, "RevolutionTradingPros");
    let auth = credentials(state, account, false, ip).await?;
    Ok(
        json!({"schema_version":1,"metrics_scope":"self_mfa","operation":"setup_started","secret":key,"qr_code":uri,"qr_code_uri":uri,"backup_codes":codes,"auth":auth,"replace_session_tokens":true,"message":"MFA setup is pending. Save recovery codes now; these plaintext values are not stored."}),
    )
}
pub async fn enable(
    state: &AppState,
    verified: VerifiedAuthority,
    code: &str,
    ip: Option<IpAddr>,
) -> Result<Value, ApiError> {
    let (mut tx, account) = begin(state, verified.subject, verified.version).await?;
    account_authority::verified_on(&mut tx, &verified)
        .await
        .map_err(account_error)?;
    rate(&mut tx, verified.subject, ip).await?;
    if account.user.mfa_enabled == Some(true) {
        return Err(errors::conflict("MFA is already enabled"));
    }
    let row = secret(&mut tx, verified.subject)
        .await?
        .filter(|row| row["verified"] == false)
        .ok_or_else(|| ApiError::validation_error("Start MFA setup first"))?;
    if !consume(state, &mut tx, verified.subject, &row, Proof::Totp(code)).await? {
        attempt(&mut tx, verified.subject, ip, false, "setup_totp").await?;
        tx.commit().await.map_err(db)?;
        return Err(ApiError::validation_error(
            "Authenticator code is invalid or already used",
        ));
    }
    sqlx::query(
        "UPDATE user_mfa_secrets SET totp_verified_at=NOW(),updated_at=NOW() WHERE user_id=$1",
    )
    .bind(verified.subject)
    .execute(&mut *tx)
    .await
    .map_err(db)?;
    sqlx::query(
        "UPDATE users SET mfa_enabled=true,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1",
    )
    .bind(verified.subject)
    .execute(&mut *tx)
    .await
    .map_err(db)?;
    attempt(&mut tx, verified.subject, ip, true, "setup_totp").await?;
    audit(
        &mut tx,
        verified.subject,
        "mfa_enabled",
        json!({"method":"totp"}),
    )
    .await?;
    let account = account_authority::current_on(&mut tx, verified.subject)
        .await
        .map_err(account_error)?;
    tx.commit().await.map_err(db)?;
    let auth = credentials(state, account, true, ip).await?;
    Ok(
        json!({"schema_version":1,"metrics_scope":"self_mfa","operation":"enabled","success":true,"message":"MFA enabled using the verified authenticator code","auth":auth,"replace_session_tokens":true}),
    )
}
pub async fn verify_login(
    state: &AppState,
    id: i64,
    version: i64,
    proof: Proof<'_>,
    ip: Option<IpAddr>,
) -> Result<Account, ApiError> {
    let (mut tx, account) = begin(state, id, version).await?;
    account_authority::require_mfa_state(&account).map_err(account_error)?;
    if account.user.mfa_enabled != Some(true) {
        return Err(ApiError::validation_error(
            "MFA is not enabled for this account",
        ));
    }
    rate(&mut tx, id, ip).await?;
    let row = secret(&mut tx, id)
        .await?
        .filter(|row| row["verified"] == true)
        .ok_or_else(unavailable)?;
    let method = match proof {
        Proof::Totp(_) => "totp",
        Proof::Backup(_) => "backup",
    };
    if !consume(state, &mut tx, id, &row, proof).await? {
        attempt(&mut tx, id, ip, false, method).await?;
        tx.commit().await.map_err(db)?;
        return Err(errors::unauthorized("MFA code is invalid or already used"));
    }
    attempt(&mut tx, id, ip, true, method).await?;
    audit(&mut tx, id, "mfa_authenticated", json!({"method":method})).await?;
    let account = account_authority::current_on(&mut tx, id)
        .await
        .map_err(account_error)?;
    tx.commit().await.map_err(db)?;
    Ok(account)
}
pub async fn disable(
    state: &AppState,
    verified: VerifiedAuthority,
    password_input: &str,
    ip: Option<IpAddr>,
) -> Result<Value, ApiError> {
    let (mut tx, account) = begin(state, verified.subject, verified.version).await?;
    account_authority::verified_on(&mut tx, &verified)
        .await
        .map_err(account_error)?;
    rate(&mut tx, verified.subject, ip).await?;
    if !password(password_input, &account).await? {
        attempt(&mut tx, verified.subject, ip, false, "disable_password").await?;
        tx.commit().await.map_err(db)?;
        return Err(errors::unauthorized("The current password is incorrect"));
    }
    let row = secret(&mut tx, verified.subject).await?;
    if row.is_none() && account.user.mfa_enabled != Some(true) {
        return Err(errors::conflict("MFA is not configured"));
    }
    sqlx::query("DELETE FROM user_mfa_secrets WHERE user_id=$1")
        .bind(verified.subject)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query(
        "UPDATE users SET mfa_enabled=false,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1",
    )
    .bind(verified.subject)
    .execute(&mut *tx)
    .await
    .map_err(db)?;
    attempt(&mut tx, verified.subject, ip, true, "disable_password").await?;
    audit(
        &mut tx,
        verified.subject,
        "mfa_disabled",
        json!({"method":"current_password"}),
    )
    .await?;
    let account = account_authority::current_on(&mut tx, verified.subject)
        .await
        .map_err(account_error)?;
    tx.commit().await.map_err(db)?;
    let auth = credentials(state, account, false, ip).await?;
    Ok(
        json!({"schema_version":1,"metrics_scope":"self_mfa","operation":"disabled","success":true,"message":"MFA disabled and previous tokens revoked","auth":auth,"replace_session_tokens":true}),
    )
}

/// Cheap bounded AES conversion before accepting requests; no credential value is logged.
pub async fn upgrade_legacy_storage(state: &AppState) -> Result<usize, ApiError> {
    let mut converted = 0usize;
    loop {
        let mut tx = state.db.pool.begin().await.map_err(db)?;
        sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
            .execute(&mut *tx)
            .await
            .map_err(db)?;
        let rows:Vec<(i64,String)>=sqlx::query_as("SELECT user_id,totp_secret FROM user_mfa_secrets WHERE totp_secret~'^[A-Z2-7]{32}$' ORDER BY user_id LIMIT 32 FOR UPDATE SKIP LOCKED").fetch_all(&mut *tx).await.map_err(db)?;
        if rows.is_empty() {
            tx.commit().await.map_err(db)?;
            break;
        }
        for (id, secret) in rows {
            sqlx::query("UPDATE user_mfa_secrets SET totp_secret=$2 WHERE user_id=$1")
                .bind(id)
                .bind(encrypted(state, &secret)?)
                .execute(&mut *tx)
                .await
                .map_err(db)?;
            converted += 1;
        }
        tx.commit().await.map_err(db)?;
    }
    Ok(converted)
}
