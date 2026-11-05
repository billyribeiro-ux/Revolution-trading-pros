//! Atomic account administration. Recorded account changes are distinct from provider billing.
use crate::{
    AppState,
    middleware::admin::{AdminUser, SuperAdminUser, is_admin_role, is_superadmin_role},
    services::{account_authority, crm_logs},
    utils::errors::{self, ApiError},
};
use axum::{
    Extension, Json,
    extract::{Path, State},
    http::{HeaderMap, header},
    response::{IntoResponse, Response},
};
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::Sha256;
use sqlx::{PgConnection, Postgres, Transaction};
use uuid::Uuid;
use validator::ValidateEmail;
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CreateMemberRequest {
    pub request_id: Uuid,
    pub name: String,
    pub email: String,
    pub password: Option<String>,
    pub role: Option<String>,
    pub send_welcome_email: Option<bool>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UpdateMemberRequest {
    pub expected_version: i64,
    pub name: Option<String>,
    pub email: Option<String>,
    pub role: Option<String>,
    #[serde(default, deserialize_with = "nullable_patch")]
    pub avatar_url: Option<Option<String>>,
    pub password: Option<String>,
    pub is_active: Option<bool>,
}
pub(crate) fn nullable_patch<'de, D: serde::Deserializer<'de>>(
    d: D,
) -> Result<Option<Option<String>>, D::Error> {
    Option::<String>::deserialize(d).map(Some)
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BanMemberRequest {
    pub expected_version: i64,
    pub reason: Option<String>,
    pub duration_days: Option<i32>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UnbanMemberRequest {
    pub expected_version: i64,
}
fn invalid(message: &str) -> ApiError {
    ApiError::validation_error(message)
}
fn db(error: sqlx::Error) -> ApiError {
    if error
        .as_database_error()
        .is_some_and(|e| e.is_unique_violation())
    {
        return errors::conflict("Account email or request identifier is already in use");
    }
    tracing::error!(%error,"Account administration persistence failed");
    errors::service_unavailable("Account change is unconfirmed; reload the member before retrying")
}
fn response(value: Value) -> Response {
    (
        [
            (header::CACHE_CONTROL, "private, no-store"),
            (header::VARY, "Authorization, Cookie"),
        ],
        Json(value),
    )
        .into_response()
}
fn role(value: &str) -> Result<&'static str, ApiError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "user" | "member" => Ok("user"),
        "moderator" => Ok("moderator"),
        "editor" => Ok("editor"),
        "marketing" => Ok("marketing"),
        "admin" | "administrator" => Ok("admin"),
        "developer" => Ok("developer"),
        "superadmin" | "super_admin" | "super-admin" => Ok("super_admin"),
        _ => Err(invalid("Select an established account role")),
    }
}
fn name(value: &str) -> Result<String, ApiError> {
    let name = value.trim();
    if name.is_empty() || name.chars().count() > 200 || name.chars().any(char::is_control) {
        return Err(invalid(
            "Provide a name from 1 to 200 characters without controls",
        ));
    }
    Ok(name.to_owned())
}
fn email(value: &str) -> Result<String, ApiError> {
    let email = value.trim().to_lowercase();
    if email.len() > 254 || !email.validate_email() || email.chars().any(char::is_control) {
        return Err(invalid("Provide one valid email address within 254 bytes"));
    }
    Ok(email)
}
fn version(before: &Value, expected: i64) -> Result<(), ApiError> {
    if expected < 1 {
        return Err(invalid("Provide the saved positive account version"));
    }
    if before["version"] != expected {
        return Err(errors::conflict(
            "Account changed; reload it before editing",
        ));
    }
    Ok(())
}
fn scalar(value: &Value, key: &str) -> Result<i64, ApiError> {
    value[key]
        .as_i64()
        .ok_or_else(|| errors::service_unavailable("Stored account authority is invalid"))
}
// One table write lock stabilizes the actor, target, unique-email check and last-privileged count.
// It precedes deterministic row locks and conflicts with every ordinary users INSERT/UPDATE/DELETE.
async fn locked(
    state: &AppState,
    actor: i64,
    verified: account_authority::VerifiedAuthority,
    target: Option<i64>,
) -> Result<(Transaction<'_, Postgres>, Value, Option<Value>), ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let rows: Vec<(i64,)> =
        sqlx::query_as("SELECT id FROM users WHERE id=$1 OR id=$2 ORDER BY id FOR NO KEY UPDATE")
            .bind(actor)
            .bind(target)
            .fetch_all(&mut *tx)
            .await
            .map_err(db)?;
    if !rows.iter().any(|r| r.0 == actor) {
        return Err(errors::unauthorized("Account is no longer active"));
    }
    let current = account_authority::verified_on(&mut tx, &verified)
        .await
        .map_err(|(status, message)| ApiError::new(status, message))?;
    if verified.subject != actor || verified.version != current.version {
        return Err(errors::unauthorized(
            "Actor authority changed; authenticate again",
        ));
    }
    if !is_admin_role(current.user.role.as_deref()) {
        return Err(errors::forbidden(
            "Current global administrator authority is required",
        ));
    }
    let superadmin = is_superadmin_role(current.user.role.as_deref())
        || state.config.is_superadmin_email_strict(
            &current.user.email,
            current.user.role.as_deref(),
            current.user.email_verified_at.is_some(),
        );
    let actor = json!({"id":actor,"role":current.user.role,"auth_version":current.version,"superadmin":superadmin});
    let target = if let Some(id) = target {
        if id <= 0 {
            return Err(invalid("Member identifiers must be positive"));
        }
        Some(record(&mut tx, id).await?)
    } else {
        None
    };
    Ok((tx, actor, target))
}
const ACCOUNT: &str = r#"SELECT jsonb_build_object('id',id,'name',name,'email',email,'role',role,'avatar_url',avatar_url,'is_active',is_active,'email_verified',email_verified_at IS NOT NULL,'version',account_version,'auth_version',auth_version,'deleted',deleted_at IS NOT NULL OR COALESCE(LOWER(role),'')='deleted','created_at',created_at,'updated_at',updated_at,'deleted_at',deleted_at) FROM users WHERE id=$1"#;
async fn record(connection: &mut PgConnection, id: i64) -> Result<Value, ApiError> {
    sqlx::query_scalar(ACCOUNT)
        .bind(id)
        .fetch_optional(connection)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Member not found"))
}
fn protect(
    actor: &Value,
    target: Option<&Value>,
    new_role: Option<&str>,
    destructive: bool,
) -> Result<(), ApiError> {
    let superadmin = actor["superadmin"] == true;
    if let Some(target) = target {
        if target["deleted"] == true {
            return Err(ApiError::not_found("Member not found"));
        }
        if actor["id"] == target["id"]
            && (destructive
                || new_role.is_some_and(|role| {
                    Some(role)
                        != target["role"]
                            .as_str()
                            .and_then(|value| self::role(value).ok())
                }))
        {
            return Err(invalid(
                "Use a separate privileged account for self-demotion, restriction or deletion",
            ));
        }
        if actor["id"] != target["id"] && is_admin_role(target["role"].as_str()) && !superadmin {
            return Err(errors::forbidden(
                "Only a current super administrator may change a privileged peer",
            ));
        }
    }
    if new_role.is_some_and(|role| is_admin_role(Some(role))) && !superadmin {
        return Err(errors::forbidden(
            "Only a current super administrator may grant global staff authority",
        ));
    }
    Ok(())
}
async fn last_privileged(
    connection: &mut PgConnection,
    target: &Value,
    losing_authority: bool,
) -> Result<(), ApiError> {
    if losing_authority
        && is_superadmin_role(target["role"].as_str())
        && target["is_active"] != false
        && target["deleted"] == false
    {
        let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM users u WHERE u.deleted_at IS NULL AND u.is_active IS DISTINCT FROM false AND LOWER(u.role) IN('super_admin','super-admin','superadmin') AND NOT EXISTS(SELECT 1 FROM user_status s WHERE s.user_id=u.id AND s.status IN('banned','suspended') AND(s.ban_expires_at IS NULL OR s.ban_expires_at>NOW()))").fetch_one(connection).await.map_err(db)?;
        if count <= 1 {
            return Err(errors::conflict(
                "Preserve at least one active super administrator",
            ));
        }
    }
    Ok(())
}
async fn journal(
    connection: &mut PgConnection,
    actor: Option<i64>,
    member: i64,
    action: &str,
    context: Value,
) -> Result<(), ApiError> {
    let context = json!({"operation":action,"member_id":member,"actor_id":actor,"details":context,"description":action.replace('_'," ")});
    sqlx::query("INSERT INTO user_activity_log(user_id,action,entity_type,entity_id,details,created_at) VALUES($1,$2,'member_account',$1,$3,NOW())").bind(member).bind(action).bind(&context).execute(&mut *connection).await.map_err(db)?;
    // crm_logs::record requires Transaction; caller uses the companion audit helper below.
    Ok(())
}
async fn audited(
    tx: &mut Transaction<'_, Postgres>,
    actor: Option<i64>,
    member: i64,
    action: &str,
    context: Value,
) -> Result<(), ApiError> {
    journal(tx, actor, member, action, context.clone()).await?;
    crm_logs::record(
        tx,
        "info",
        "system",
        &action.replace('_', " "),
        json!({"operation":action,"member_id":member,"actor_id":actor,"details":context}),
        actor,
    )
    .await
    .map_err(db)
}
fn ack(operation: &str, member: i64, account: Value) -> Value {
    json!({"schema_version":1,"metrics_scope":"member_account_mutation","operation":operation,"member_id":member.to_string(),"account":account,"provider_billing":"unchanged"})
}
async fn unique_email(
    connection: &mut PgConnection,
    mail: &str,
    except: Option<i64>,
) -> Result<(), ApiError> {
    let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM users WHERE LOWER(email)=$1 AND($2::bigint IS NULL OR id<>$2))").bind(mail).bind(except).fetch_one(connection).await.map_err(db)?;
    if exists {
        return Err(errors::conflict("Email is already recorded for an account"));
    }
    Ok(())
}

pub(crate) async fn create_member(
    State(state): State<AppState>,
    SuperAdminUser(admin): SuperAdminUser,
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
    Json(mut input): Json<CreateMemberRequest>,
) -> Result<Response, ApiError> {
    if input.request_id.is_nil() {
        return Err(invalid("Provide one stable nonzero creation request UUID"));
    }
    input.name = name(&input.name)?;
    input.email = email(&input.email)?;
    let target_role = role(input.role.as_deref().unwrap_or("user"))?;
    input.role = Some(target_role.into());
    if let Some(password) = input.password.as_deref() {
        crate::utils::validate_password(password).map_err(invalid)?;
    }
    let mut mac = Hmac::<Sha256>::new_from_slice(state.config.jwt_secret.as_bytes())
        .map_err(|_| ApiError::internal_error("Account request hashing is unavailable"))?;
    mac.update(b"member-create-v1\0");
    mac.update(&serde_json::to_vec(&input).map_err(|_| invalid("Invalid creation request"))?);
    let source_hash = hex::encode(mac.finalize().into_bytes());
    let generated = input.password.is_none();
    let password = input
        .password
        .clone()
        .unwrap_or_else(|| format!("Aa1!{}", Uuid::new_v4().simple()));
    crate::utils::validate_password(&password).map_err(invalid)?;
    let password_hash = crate::utils::hash_password_async(&password)
        .await
        .map_err(|_| errors::service_unavailable("Password hashing is unavailable"))?;
    let (mut tx, actor, _) = locked(&state, admin.id, verified, None).await?;
    if actor["superadmin"] != true {
        return Err(errors::forbidden(
            "Current super administrator authority is required",
        ));
    }
    protect(&actor, None, Some(target_role), false)?;
    let saved:Option<(i64,String,i64,Value)>=sqlx::query_as("SELECT original_actor_id,source_hash,original_member_id,response_snapshot FROM member_admin_creations WHERE request_id=$1 FOR UPDATE").bind(input.request_id).fetch_optional(&mut *tx).await.map_err(db)?;
    if let Some((owner, hash, id, snapshot)) = saved {
        if owner != admin.id || hash != source_hash {
            return Err(errors::conflict(
                "Request UUID belongs to a different creation intent",
            ));
        }
        let current = record(&mut tx, id).await?;
        let welcome = super::welcome::state_on(&mut tx, id).await?;
        tx.commit().await.map_err(db)?;
        let mut result = ack("created", id, current);
        result["created_account_snapshot"] = snapshot;
        result["request_id"] = json!(input.request_id);
        result["replayed"] = json!(true);
        result["temporary_password"] = Value::Null;
        result["temporary_password_recovery"] =
            json!("Use the existing password-reset workflow if the first response was lost");
        result["welcome"] = welcome;
        return Ok(response(result));
    }
    unique_email(&mut tx, &input.email, None).await?;
    let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,created_at,updated_at) VALUES($1,$2,$3,$4,true,NOW() AT TIME ZONE 'UTC',NOW() AT TIME ZONE 'UTC') RETURNING id").bind(&input.name).bind(&input.email).bind(password_hash).bind(target_role).fetch_one(&mut *tx).await.map_err(db)?;
    let saved = record(&mut tx, id).await?;
    sqlx::query("INSERT INTO member_admin_creations(request_id,created_by,original_actor_id,user_id,original_member_id,source_hash,response_snapshot) VALUES($1,$2,$2,$3,$3,$4,$5)").bind(input.request_id).bind(admin.id).bind(id).bind(source_hash).bind(&saved).execute(&mut *tx).await.map_err(db)?;
    let welcome = if input.send_welcome_email.unwrap_or(false) {
        super::welcome::queue_on(
            &mut tx,
            &state,
            id,
            input.request_id,
            &input.name,
            &input.email,
        )
        .await?
    } else {
        json!({"requested":false,"status":"not_requested"})
    };
    audited(&mut tx,Some(admin.id),id,"account_created",json!({"request_id":input.request_id,"role":target_role,"welcome_requested":input.send_welcome_email.unwrap_or(false)})).await?;
    tx.commit().await.map_err(db)?;
    let mut result = ack("created", id, saved.clone());
    result["created_account_snapshot"] = saved;
    result["request_id"] = json!(input.request_id);
    result["replayed"] = json!(false);
    result["temporary_password"] = if generated {
        json!(password)
    } else {
        Value::Null
    };
    result["welcome"] = welcome;
    Ok(response(result))
}
pub(crate) async fn update_member(
    State(state): State<AppState>,
    SuperAdminUser(admin): SuperAdminUser,
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
    Path(id): Path<i64>,
    Json(input): Json<UpdateMemberRequest>,
) -> Result<Response, ApiError> {
    let next_name = input.name.as_deref().map(name).transpose()?;
    let next_email = input.email.as_deref().map(email).transpose()?;
    let next_role = input.role.as_deref().map(role).transpose()?;
    if let Some(Some(avatar)) = &input.avatar_url {
        let url = reqwest::Url::parse(avatar)
            .map_err(|_| invalid("Avatar must be an absolute HTTP or HTTPS URL"))?;
        if avatar.len() > 500
            || !matches!(url.scheme(), "https" | "http")
            || !url.username().is_empty()
            || url.password().is_some()
            || avatar.chars().any(char::is_control)
        {
            return Err(invalid(
                "Avatar must be a bounded HTTP or HTTPS URL without credentials",
            ));
        }
    }
    let hash = if let Some(password) = &input.password {
        crate::utils::validate_password(password).map_err(invalid)?;
        Some(
            crate::utils::hash_password_async(password)
                .await
                .map_err(|_| errors::service_unavailable("Password hashing is unavailable"))?,
        )
    } else {
        None
    };
    let (mut tx, actor, before) = locked(&state, admin.id, verified, Some(id)).await?;
    if actor["superadmin"] != true {
        return Err(errors::forbidden(
            "Current super administrator authority is required",
        ));
    }
    let before = before.ok_or_else(|| ApiError::not_found("Member not found"))?;
    protect(
        &actor,
        Some(&before),
        next_role,
        input.is_active == Some(false),
    )?;
    if actor["id"] == before["id"]
        && actor["superadmin"] == true
        && !is_superadmin_role(before["role"].as_str())
        && next_email
            .as_ref()
            .is_some_and(|email| before["email"] != *email)
    {
        return Err(errors::forbidden(
            "A separate super administrator must change the email that establishes this account's SuperAdmin authority",
        ));
    }
    version(&before, input.expected_version)?;
    last_privileged(
        &mut tx,
        &before,
        next_role.is_some_and(|r| !is_superadmin_role(Some(r))) || input.is_active == Some(false),
    )
    .await?;
    if let Some(mail) = &next_email {
        unique_email(&mut tx, mail, Some(id)).await?
    }
    if input.is_active == Some(true) {
        let restricted:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM user_status WHERE user_id=$1 AND status IN('banned','suspended','expired_requires_review'))").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
        if restricted {
            return Err(errors::conflict(
                "Remove the recorded restriction through the unban workflow before activating this account",
            ));
        }
    }
    let mut changed = Vec::new();
    for (key, present) in [
        (
            "name",
            next_name
                .as_ref()
                .is_some_and(|value| before["name"] != *value),
        ),
        (
            "email",
            next_email
                .as_ref()
                .is_some_and(|value| before["email"] != *value),
        ),
        (
            "role",
            next_role.is_some_and(|value| before["role"] != value),
        ),
        (
            "avatar_url",
            input
                .avatar_url
                .as_ref()
                .is_some_and(|value| value.as_deref() != before["avatar_url"].as_str()),
        ),
        ("password", hash.is_some()),
        (
            "is_active",
            input
                .is_active
                .is_some_and(|value| before["is_active"].as_bool() != Some(value)),
        ),
    ] {
        if present {
            changed.push(key)
        }
    }
    if changed.is_empty() {
        return Err(invalid("Provide at least one changed account field"));
    }
    sqlx::query("UPDATE users SET name=COALESCE($2,name),email=COALESCE($3,email),role=COALESCE($4,role),avatar_url=CASE WHEN $5 THEN $6 ELSE avatar_url END,password_hash=COALESCE($7,password_hash),is_active=COALESCE($8,is_active),email_verified_at=CASE WHEN $3::text IS NOT NULL AND email IS DISTINCT FROM $3 THEN NULL ELSE email_verified_at END,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1")
        .bind(id).bind(next_name).bind(next_email).bind(next_role).bind(input.avatar_url.is_some()).bind(input.avatar_url.flatten()).bind(hash).bind(input.is_active).execute(&mut *tx).await.map_err(db)?;
    let after = record(&mut tx, id).await?;
    audited(&mut tx,Some(admin.id),id,"profile_updated_by_admin",json!({"fields":changed,"before_version":before["version"],"after_version":after["version"],"tokens_revoked":before["auth_version"]!=after["auth_version"]})).await?;
    tx.commit().await.map_err(db)?;
    Ok(response(ack("updated", id, after)))
}
pub(crate) async fn delete_member(
    State(state): State<AppState>,
    SuperAdminUser(admin): SuperAdminUser,
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
    Path(id): Path<i64>,
    headers: HeaderMap,
) -> Result<Response, ApiError> {
    let expected = headers
        .get(header::IF_MATCH)
        .and_then(|v| v.to_str().ok())
        .and_then(|v| v.strip_prefix('"').and_then(|v| v.strip_suffix('"')))
        .filter(|v| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()))
        .and_then(|v| v.parse::<i64>().ok())
        .filter(|v| *v > 0)
        .ok_or_else(|| invalid("Provide the saved account version in a quoted If-Match header"))?;
    let (mut tx, actor, before) = locked(&state, admin.id, verified, Some(id)).await?;
    if actor["superadmin"] != true {
        return Err(errors::forbidden(
            "Current super administrator authority is required",
        ));
    }
    let before = before.ok_or_else(|| ApiError::not_found("Member not found"))?;
    if before["deleted"] == true {
        tx.commit().await.map_err(db)?;
        let mut result = ack("deleted", id, before);
        result["removed"] = json!(false);
        return Ok(response(result));
    }
    protect(&actor, Some(&before), None, true)?;
    version(&before, expected)?;
    last_privileged(&mut tx, &before, true).await?;
    sqlx::query("UPDATE users SET deleted_at=NOW(),is_active=false,name='Deleted User',email=$2,role='deleted',password_hash='DELETED',avatar_url=NULL,metadata=NULL,email_verified_at=NULL,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).bind(format!("deleted_{id}_{}@removed.invalid",Uuid::new_v4().simple())).execute(&mut *tx).await.map_err(db)?;
    let cancelled=sqlx::query("UPDATE user_memberships SET status='cancelled',cancelled_at=NOW() AT TIME ZONE 'UTC',expires_at=NOW() AT TIME ZONE 'UTC',cancel_at_period_end=false,updated_at=NOW() AT TIME ZONE 'UTC' WHERE user_id=$1 AND deleted_at IS NULL AND status IN('active','trial','trialing','past_due')").bind(id).execute(&mut *tx).await.map_err(db)?.rows_affected();
    sqlx::query("DELETE FROM email_verification_tokens WHERE user_id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query("DELETE FROM password_resets WHERE user_id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query("DELETE FROM user_mfa_secrets WHERE user_id=$1")
        .bind(id)
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let after = record(&mut tx, id).await?;
    audited(&mut tx,Some(admin.id),id,"account_deleted",json!({"local_memberships_cancelled":cancelled,"provider_billing":"unchanged","before_version":before["version"],"after_version":after["version"]})).await?;
    tx.commit().await.map_err(db)?;
    let mut result = ack("deleted", id, after);
    result["removed"] = json!(true);
    result["local_memberships_cancelled"] = json!(cancelled);
    Ok(response(result))
}
pub(crate) async fn restrict(
    state: AppState,
    actor_id: i64,
    verified: account_authority::VerifiedAuthority,
    id: i64,
    input: BanMemberRequest,
    status: &str,
    require_super: bool,
) -> Result<Response, ApiError> {
    if input
        .duration_days
        .is_some_and(|days| !(1..=3650).contains(&days))
    {
        return Err(invalid(
            "Restriction duration must be from 1 to 3650 days, or omitted",
        ));
    }
    let reason = input.reason.map(|v| v.trim().to_owned());
    if reason.as_ref().is_some_and(|v| {
        v.is_empty()
            || v.chars().count() > 2000
            || v.chars()
                .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
    }) {
        return Err(invalid("Provide a nonempty reason within 2000 characters"));
    }
    let (mut tx, actor, before) = locked(&state, actor_id, verified, Some(id)).await?;
    if require_super && actor["superadmin"] != true {
        return Err(errors::forbidden(
            "Current super administrator authority is required",
        ));
    }
    let before = before.ok_or_else(|| ApiError::not_found("Member not found"))?;
    protect(&actor, Some(&before), None, true)?;
    version(&before, input.expected_version)?;
    last_privileged(&mut tx, &before, true).await?;
    sqlx::query("UPDATE users SET is_active=false,account_version=account_version+1,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db)?;
    let after = record(&mut tx, id).await?;
    let prior = before["is_active"].as_bool();
    sqlx::query("INSERT INTO user_status(user_id,status,status_reason,banned_at,banned_by,ban_expires_at,ban_reason,restriction_original_is_active,restriction_prior_recorded,restriction_auth_version,created_at,updated_at) VALUES($1,$2,$3,NOW(),$4,CASE WHEN $5::integer IS NULL THEN NULL ELSE NOW()+make_interval(days=>$5) END,$3,$6,true,$7,NOW(),NOW()) ON CONFLICT(user_id) DO UPDATE SET status=EXCLUDED.status,status_reason=EXCLUDED.status_reason,banned_at=EXCLUDED.banned_at,banned_by=EXCLUDED.banned_by,ban_expires_at=EXCLUDED.ban_expires_at,ban_reason=EXCLUDED.ban_reason,restriction_original_is_active=CASE WHEN user_status.status IN('banned','suspended') AND user_status.restriction_prior_recorded THEN user_status.restriction_original_is_active ELSE EXCLUDED.restriction_original_is_active END,restriction_prior_recorded=true,restriction_auth_version=EXCLUDED.restriction_auth_version,updated_at=NOW()")
        .bind(id).bind(status).bind(reason).bind(actor_id).bind(input.duration_days).bind(prior).bind(scalar(&after,"auth_version")?).execute(&mut *tx).await.map_err(db)?;
    let restriction:Value=sqlx::query_scalar("SELECT jsonb_build_object('status',status,'reason',status_reason,'expires_at',to_char(ban_expires_at AT TIME ZONE 'UTC','YYYY-MM-DD\"T\"HH24:MI:SS.US\"Z\"')) FROM user_status WHERE user_id=$1").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
    let action = if status == "banned" {
        "account_banned"
    } else {
        "account_suspended"
    };
    audited(&mut tx,Some(actor_id),id,action,json!({"restriction":restriction,"before_version":before["version"],"after_version":after["version"]})).await?;
    tx.commit().await.map_err(db)?;
    let mut result = ack(status, id, after);
    result["restriction"] = restriction;
    Ok(response(result))
}
pub(crate) async fn ban_member(
    State(state): State<AppState>,
    AdminUser(actor): AdminUser,
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
    Path(id): Path<i64>,
    Json(input): Json<BanMemberRequest>,
) -> Result<Response, ApiError> {
    restrict(state, actor.id, verified, id, input, "banned", false).await
}
pub(crate) async fn suspend_member(
    State(state): State<AppState>,
    AdminUser(actor): AdminUser,
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
    Path(id): Path<i64>,
    Json(input): Json<BanMemberRequest>,
) -> Result<Response, ApiError> {
    restrict(state, actor.id, verified, id, input, "suspended", false).await
}
pub(crate) async fn unban_member(
    State(state): State<AppState>,
    AdminUser(admin): AdminUser,
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
    Path(id): Path<i64>,
    Json(input): Json<UnbanMemberRequest>,
) -> Result<Response, ApiError> {
    unban(state, admin.id, verified, id, input, false).await
}
pub(crate) async fn unban(
    state: AppState,
    actor_id: i64,
    verified: account_authority::VerifiedAuthority,
    id: i64,
    input: UnbanMemberRequest,
    require_super: bool,
) -> Result<Response, ApiError> {
    let (mut tx, actor, before) = locked(&state, actor_id, verified, Some(id)).await?;
    if require_super && actor["superadmin"] != true {
        return Err(errors::forbidden(
            "Current super administrator authority is required",
        ));
    }
    let before = before.ok_or_else(|| ApiError::not_found("Member not found"))?;
    protect(&actor, Some(&before), None, false)?;
    version(&before, input.expected_version)?;
    let changed=sqlx::query("UPDATE user_status SET status='active',status_reason=NULL,ban_expires_at=NULL,ban_reason=NULL,restriction_auth_version=NULL,updated_at=NOW() WHERE user_id=$1 AND status IN('banned','suspended','expired_requires_review')").bind(id).execute(&mut *tx).await.map_err(db)?.rows_affected();
    if changed == 0 {
        return Err(errors::conflict(
            "No recorded restriction remains; reload the current account",
        ));
    }
    // This explicit administrator action enables the account; automatic expiry below restores
    // only a recorded pre-restriction value and never guesses a legacy active state.
    sqlx::query("UPDATE users SET is_active=true,account_version=account_version+1,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db)?;
    let after = record(&mut tx, id).await?;
    audited(&mut tx,Some(actor_id),id,"account_unbanned",json!({"before_version":before["version"],"after_version":after["version"],"is_active":after["is_active"]})).await?;
    tx.commit().await.map_err(db)?;
    Ok(response(ack("restriction_removed", id, after)))
}
/// Called by the existing worker. Every observed expiry and its account effect commit together.
pub(crate) async fn expire_restrictions(pool: &sqlx::PgPool) -> Result<usize, ApiError> {
    let mut tx = pool.begin().await.map_err(db)?;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let ids:Vec<i64>=sqlx::query_scalar("SELECT s.user_id FROM user_status s JOIN users u ON u.id=s.user_id WHERE u.deleted_at IS NULL AND s.status IN('banned','suspended') AND s.ban_expires_at<=NOW() ORDER BY s.user_id LIMIT 50 FOR UPDATE OF u,s SKIP LOCKED").fetch_all(&mut *tx).await.map_err(db)?;
    for id in &ids {
        let provenance:(bool,Option<bool>,Option<i64>,i64)=sqlx::query_as("SELECT s.restriction_prior_recorded,s.restriction_original_is_active,s.restriction_auth_version,u.auth_version FROM user_status s JOIN users u ON u.id=s.user_id WHERE s.user_id=$1").bind(id).fetch_one(&mut *tx).await.map_err(db)?;
        let restored = provenance.0 && provenance.2 == Some(provenance.3);
        if restored {
            sqlx::query("UPDATE users SET is_active=$2,account_version=account_version+1,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).bind(provenance.1).execute(&mut *tx).await.map_err(db)?;
        } else {
            sqlx::query("UPDATE users SET account_version=account_version+1,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(id).execute(&mut *tx).await.map_err(db)?;
        }
        sqlx::query("UPDATE user_status SET status=CASE WHEN $2 THEN 'active' ELSE 'expired_requires_review' END,ban_expires_at=NULL,restriction_auth_version=NULL,updated_at=NOW() WHERE user_id=$1").bind(id).bind(restored).execute(&mut *tx).await.map_err(db)?;
        audited(&mut tx,None,*id,"account_restriction_expired",json!({"source":"expiry_worker","prior_state_restored":restored,"manual_review_required":!restored})).await?;
    }
    tx.commit().await.map_err(db)?;
    Ok(ids.len())
}
