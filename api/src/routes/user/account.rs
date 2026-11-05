//! Self-service closure writes real account state and journals atomically; provider billing is separate.
use crate::{
    AppState,
    middleware::admin::{is_admin_role, is_superadmin_role},
    models::User,
    services::{account_authority, crm_logs},
};
use axum::{
    Extension, Json,
    extract::State,
    http::{StatusCode, header},
    response::{IntoResponse, Response},
};
use serde::Deserialize;
use serde_json::json;
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct DeactivateAccountRequest {
    password: String,
    reason: Option<String>,
    #[serde(default)]
    delete_data: bool,
}
fn failure(error: sqlx::Error) -> (StatusCode, Json<serde_json::Value>) {
    tracing::error!(%error,"Self-service account persistence failed");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        Json(
            json!({"error":"Account change is unconfirmed; reload account state before retrying"}),
        ),
    )
}
pub(super) async fn deactivate_account(
    State(state): State<AppState>,
    user: User,
    Extension(verified): Extension<account_authority::VerifiedAuthority>,
    Json(input): Json<DeactivateAccountRequest>,
) -> Result<Response, (StatusCode, Json<serde_json::Value>)> {
    if input.password.is_empty()
        || input.password.len() > 512
        || input.reason.as_ref().is_some_and(|r| {
            r.chars().count() > 2000
                || r.chars()
                    .any(|c| c.is_control() && !matches!(c, '\n' | '\r' | '\t'))
        })
    {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Provide a bounded password and optional reason"})),
        ));
    }
    let password_valid = crate::utils::verify_password_async(&input.password, &user.password_hash)
        .await
        .map_err(|_| {
            (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(json!({"error":"Password verification is unavailable"})),
            )
        })?;
    if !password_valid {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({"error":"Password is incorrect","field":"password"})),
        ));
    }
    let mut tx = state.db.pool.begin().await.map_err(failure)?;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(failure)?;
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR NO KEY UPDATE")
        .bind(user.id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(failure)?
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({"error":"Account is unavailable"})),
            )
        })?;
    let current = account_authority::verified_on(&mut tx, &verified)
        .await
        .map_err(|(status, message)| (status, Json(json!({"error":message}))))?;
    if verified.subject != user.id
        || verified.version != current.version
        || current.user.password_hash != user.password_hash
    {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({"error":"Account credentials changed; authenticate again"})),
        ));
    }
    // Staff must transfer administrative authority through the reviewed administrator workflow.
    if is_admin_role(current.user.role.as_deref())
        || is_superadmin_role(current.user.role.as_deref())
    {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({"error":"A separate super administrator must close a privileged account"})),
        ));
    }
    let local_memberships = if input.delete_data {
        sqlx::query("UPDATE user_memberships SET status='cancelled',cancelled_at=NOW() AT TIME ZONE 'UTC',expires_at=NOW() AT TIME ZONE 'UTC',cancel_at_period_end=false,updated_at=NOW() AT TIME ZONE 'UTC' WHERE user_id=$1 AND deleted_at IS NULL AND status IN('active','trial','trialing','past_due')").bind(user.id).execute(&mut *tx).await.map_err(failure)?.rows_affected()
    } else {
        0
    };
    if input.delete_data {
        sqlx::query("UPDATE users SET email=$2,name='Deleted User',role='deleted',metadata=NULL,avatar_url=NULL,password_hash='DEACTIVATED',email_verified_at=NULL,is_active=false,deleted_at=NOW(),updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1")
            .bind(user.id).bind(format!("deleted_{}_{}@removed.invalid",user.id,uuid::Uuid::new_v4().simple())).execute(&mut *tx).await.map_err(failure)?;
        sqlx::query("DELETE FROM password_resets WHERE user_id=$1")
            .bind(user.id)
            .execute(&mut *tx)
            .await
            .map_err(failure)?;
        sqlx::query("DELETE FROM user_mfa_secrets WHERE user_id=$1")
            .bind(user.id)
            .execute(&mut *tx)
            .await
            .map_err(failure)?;
        sqlx::query("DELETE FROM email_verification_tokens WHERE user_id=$1")
            .bind(user.id)
            .execute(&mut *tx)
            .await
            .map_err(failure)?;
    } else {
        sqlx::query(
            "UPDATE users SET is_active=false,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1",
        )
        .bind(user.id)
        .execute(&mut *tx)
        .await
        .map_err(failure)?;
    }
    let operation = if input.delete_data {
        "account_deleted"
    } else {
        "account_deactivated"
    };
    let context = json!({"operation":operation,"member_id":user.id,"actor_id":user.id,"source":"self_service","reason":input.reason,"description":operation.replace('_'," "),"local_memberships_cancelled":local_memberships,"provider_billing":"unchanged"});
    sqlx::query("INSERT INTO user_activity_log(user_id,action,entity_type,entity_id,details,created_at) VALUES($1,$2,'member_account',$1,$3,NOW())").bind(user.id).bind(operation).bind(&context).execute(&mut *tx).await.map_err(failure)?;
    crm_logs::record(
        &mut tx,
        "info",
        "system",
        "Self-service account state changed",
        context,
        Some(user.id),
    )
    .await
    .map_err(failure)?;
    let version: i64 = sqlx::query_scalar("SELECT auth_version FROM users WHERE id=$1")
        .bind(user.id)
        .fetch_one(&mut *tx)
        .await
        .map_err(failure)?;
    tx.commit().await.map_err(failure)?;
    Ok(([(header::CACHE_CONTROL,"private, no-store"),(header::VARY,"Authorization, Cookie")],Json(json!({"schema_version":1,"metrics_scope":"member_account_mutation","member_id":user.id.to_string(),"operation":operation,"is_active":false,"deleted":input.delete_data,"auth_version":version,"tokens_revoked":true,"local_memberships_cancelled":local_memberships,"provider_billing":"unchanged","data_retention":"Recorded financial and audit history is retained; this response does not claim permanent erasure"}))).into_response())
}
