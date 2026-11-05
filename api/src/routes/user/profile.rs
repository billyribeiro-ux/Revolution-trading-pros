//! Self-owned profile and credential changes use actual metadata and current DB authority.
use crate::{
    AppState,
    models::{User, UserResponse},
    services::{
        account_authority::{self, VerifiedAuthority},
        account_links, account_mfa,
        campaign_mailer::{RejectedSubmission, UncertainSubmission},
        crm_logs,
    },
    utils::{
        self,
        errors::{self, ApiError},
    },
};
use axum::{Extension, Json, extract::State};
use serde::Deserialize;
use serde_json::{Value, json};
use validator::ValidateEmail;
fn db(error: sqlx::Error) -> ApiError {
    if error
        .as_database_error()
        .is_some_and(|e| e.is_unique_violation())
    {
        return errors::conflict("Email is already recorded for an account");
    }
    tracing::error!(%error,"Self profile persistence failed");
    errors::service_unavailable("Profile change is unconfirmed; reload before retrying")
}
async fn projection_on(
    connection: &mut sqlx::PgConnection,
    state: &AppState,
    user: User,
) -> Result<Value, ApiError> {
    let raw:Value=sqlx::query_scalar("SELECT jsonb_build_object('first_name',CASE WHEN jsonb_typeof(metadata->'first_name')='string' THEN metadata->>'first_name' ELSE NULL END,'last_name',CASE WHEN jsonb_typeof(metadata->'last_name')='string' THEN metadata->>'last_name' ELSE NULL END,'version',account_version,'mfa_enabled',mfa_enabled) FROM users WHERE id=$1 AND deleted_at IS NULL AND COALESCE(LOWER(role),'')<>'deleted'").bind(user.id).fetch_optional(connection).await.map_err(db)?.ok_or_else(||ApiError::not_found("Account not found"))?;
    let mut value = serde_json::to_value(UserResponse::from_authenticated(user, &state.config))
        .map_err(|_| ApiError::internal_error("Profile cannot be represented"))?;
    for field in ["first_name", "last_name", "version", "mfa_enabled"] {
        value[field] = raw[field].clone()
    }
    Ok(value)
}
pub(super) async fn get_profile(
    State(state): State<AppState>,
    user: User,
    Extension(verified): Extension<VerifiedAuthority>,
) -> Result<Json<Value>, ApiError> {
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let current = account_authority::verified_on(&mut tx, &verified)
        .await
        .map_err(|(status, message)| ApiError::new(status, message))?;
    if current.version != verified.version {
        return Err(errors::unauthorized(
            "Account authority changed; authenticate again",
        ));
    }
    let profile = projection_on(&mut tx, &state, current.user).await?;
    tx.commit().await.map_err(db)?;
    Ok(Json(profile))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub(crate) struct UpdateProfileRequest {
    #[serde(alias = "expected_version")]
    expected_version: i64,
    #[serde(alias = "first_name")]
    first_name: Option<String>,
    #[serde(alias = "last_name")]
    last_name: Option<String>,
    #[serde(alias = "display_name", alias = "name")]
    display_name: Option<String>,
    email: Option<String>,
    #[serde(alias = "current_password")]
    current_password: Option<String>,
    #[serde(alias = "new_password")]
    new_password: Option<String>,
    #[serde(default, alias = "avatar_url", deserialize_with = "nullable")]
    avatar_url: Option<Option<String>>,
}
fn nullable<'de, D: serde::Deserializer<'de>>(d: D) -> Result<Option<Option<String>>, D::Error> {
    Option::<String>::deserialize(d).map(Some)
}
fn bounded(value: &str, empty: bool) -> Result<String, ApiError> {
    let value = value.trim();
    if (!empty && value.is_empty())
        || value.chars().count() > 200
        || value.chars().any(char::is_control)
    {
        return Err(ApiError::validation_error(
            "Provide bounded profile names without controls",
        ));
    }
    Ok(value.to_owned())
}
pub(crate) async fn update_profile(
    State(state): State<AppState>,
    user: User,
    Extension(verified): Extension<VerifiedAuthority>,
    Json(input): Json<UpdateProfileRequest>,
) -> Result<Json<Value>, ApiError> {
    let first = input
        .first_name
        .as_deref()
        .map(|v| bounded(v, true))
        .transpose()?;
    let last = input
        .last_name
        .as_deref()
        .map(|v| bounded(v, true))
        .transpose()?;
    let display = input
        .display_name
        .as_deref()
        .map(|v| bounded(v, false))
        .transpose()?;
    let email = input.email.map(|v| v.trim().to_lowercase());
    if email
        .as_ref()
        .is_some_and(|v| v.len() > 254 || !v.validate_email())
    {
        return Err(ApiError::validation_error("Provide a valid bounded email"));
    }
    if let Some(Some(value)) = &input.avatar_url {
        let url = reqwest::Url::parse(value)
            .map_err(|_| ApiError::validation_error("Provide an absolute avatar URL or null"))?;
        if value.len() > 500
            || !matches!(url.scheme(), "http" | "https")
            || !url.username().is_empty()
            || url.password().is_some()
            || value.chars().any(char::is_control)
        {
            return Err(ApiError::validation_error(
                "Provide a bounded avatar URL without credentials",
            ));
        }
    }
    let email_changed = email.as_ref().is_some_and(|value| value != &user.email);
    let credential_change = email_changed || input.new_password.is_some();
    if credential_change && input.current_password.is_none() {
        return Err(ApiError::validation_error(
            "The current password is required to change email or password",
        ));
    }
    if !credential_change && input.current_password.is_some() {
        return Err(ApiError::validation_error(
            "A current password is used only for a credential change",
        ));
    }
    if let Some(current) = input.current_password.as_deref() {
        if current.is_empty() || current.len() > 128 {
            return Err(ApiError::validation_error(
                "Provide the current bounded password",
            ));
        }
        if !utils::verify_password_async(current, &user.password_hash)
            .await
            .map_err(|_| errors::service_unavailable("Password verification is unavailable"))?
        {
            return Err(errors::unauthorized("The current password is incorrect"));
        }
    }
    let hash = if let Some(password) = input.new_password.as_deref() {
        utils::validate_password(password).map_err(ApiError::validation_error)?;
        if Some(password) == input.current_password.as_deref() {
            return Err(ApiError::validation_error(
                "Choose a different new password",
            ));
        }
        Some(
            utils::hash_password_async(password)
                .await
                .map_err(|_| errors::service_unavailable("Password hashing is unavailable"))?,
        )
    } else {
        None
    };
    let mut tx = state.db.pool.begin().await.map_err(db)?;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR NO KEY UPDATE")
        .bind(user.id)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(|| ApiError::not_found("Account not found"))?;
    let current = account_authority::verified_on(&mut tx, &verified)
        .await
        .map_err(|(status, message)| ApiError::new(status, message))?;
    if verified.subject != user.id
        || verified.version != current.version
        || current.user.password_hash != user.password_hash
    {
        return Err(errors::unauthorized(
            "Account authority changed; authenticate again",
        ));
    }
    let actual_version: i64 = sqlx::query_scalar("SELECT account_version FROM users WHERE id=$1")
        .bind(user.id)
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
    if input.expected_version < 1 {
        return Err(ApiError::validation_error(
            "Provide the saved positive profile version",
        ));
    }
    if input.expected_version != actual_version {
        return Err(errors::conflict(
            "Profile changed; reload it before editing",
        ));
    }
    if email_changed
        && !crate::middleware::admin::is_superadmin_role(current.user.role.as_deref())
        && state.config.is_superadmin_email_strict(
            &current.user.email,
            current.user.role.as_deref(),
            current.user.email_verified_at.is_some(),
        )
    {
        return Err(errors::forbidden(
            "A separate super administrator must change the email that establishes this account's SuperAdmin authority",
        ));
    }
    let metadata: Option<Value> = sqlx::query_scalar("SELECT metadata FROM users WHERE id=$1")
        .bind(user.id)
        .fetch_one(&mut *tx)
        .await
        .map_err(db)?;
    let mut metadata = metadata.unwrap_or_else(|| json!({}));
    let original_metadata = metadata.clone();
    if (first.is_some() || last.is_some()) && !metadata.is_object() {
        return Err(errors::conflict(
            "Recorded profile metadata requires repair before editing names",
        ));
    }
    if let Some(first) = &first {
        metadata["first_name"] = json!(first)
    }
    if let Some(last) = &last {
        metadata["last_name"] = json!(last)
    }
    let display = display.or_else(|| {
        if first.is_some() || last.is_some() {
            let full = format!(
                "{} {}",
                metadata["first_name"].as_str().unwrap_or(""),
                metadata["last_name"].as_str().unwrap_or("")
            );
            let full = full.trim();
            (!full.is_empty()).then(|| full.to_owned())
        } else {
            None
        }
    });
    if first.is_none()
        && last.is_none()
        && display.is_none()
        && email.is_none()
        && hash.is_none()
        && input.avatar_url.is_none()
    {
        return Err(ApiError::validation_error(
            "Provide a changed profile field",
        ));
    }
    let actual_change = display
        .as_ref()
        .is_some_and(|value| value != &current.user.name)
        || email
            .as_ref()
            .is_some_and(|value| value != &current.user.email)
        || hash.is_some()
        || input
            .avatar_url
            .as_ref()
            .is_some_and(|value| value.as_deref() != current.user.avatar_url.as_deref())
        || ((first.is_some() || last.is_some()) && metadata != original_metadata);
    if !actual_change {
        return Err(ApiError::validation_error(
            "Provide at least one changed profile field",
        ));
    }
    sqlx::query("UPDATE users SET name=COALESCE($2,name),email=COALESCE($3,email),password_hash=COALESCE($4,password_hash),avatar_url=CASE WHEN $5 THEN $6 ELSE avatar_url END,metadata=CASE WHEN $7 THEN $8 ELSE metadata END,email_verified_at=CASE WHEN $3::text IS NOT NULL AND email IS DISTINCT FROM $3 THEN NULL ELSE email_verified_at END,updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1").bind(user.id).bind(display).bind(&email).bind(hash).bind(input.avatar_url.is_some()).bind(input.avatar_url.flatten()).bind(first.is_some()||last.is_some()).bind(metadata).execute(&mut *tx).await.map_err(db)?;
    let verification = if email_changed {
        Some(account_links::verification_on(&mut tx, user.id).await?)
    } else {
        None
    };
    let context = json!({"operation":"self_profile_updated","member_id":user.id,"actor_id":user.id,"source":"self_service","credential_change":credential_change,"description":"Self-owned profile updated"});
    sqlx::query("INSERT INTO user_activity_log(user_id,action,entity_type,entity_id,details,created_at) VALUES($1,'self_profile_updated','member_account',$1,$2,NOW())").bind(user.id).bind(&context).execute(&mut *tx).await.map_err(db)?;
    crm_logs::record(
        &mut tx,
        "info",
        "system",
        "Self-owned profile updated",
        context,
        Some(user.id),
    )
    .await
    .map_err(db)?;
    let after = account_authority::current_on(&mut tx, user.id)
        .await
        .map_err(|(status, message)| ApiError::new(status, message))?;
    let profile = projection_on(&mut tx, &state, after.user.clone()).await?;
    tx.commit().await.map_err(db)?;
    let auth = if credential_change && !email_changed {
        let assured = after.user.mfa_enabled == Some(true);
        Some(account_mfa::credentials(&state, after, assured, None).await?)
    } else {
        None
    };
    let delivery = if let Some((raw, recipient, name)) = verification {
        let link = format!(
            "{}/verify-email?token={raw}",
            state.config.app_url.trim_end_matches('/')
        );
        match state
            .services
            .email
            .send_transactional_linked(
                &state.db.pool,
                user.id,
                &recipient,
                "email-verification",
                json!({"name":name,"verification_link":link,"expires_in_hours":24}),
            )
            .await
        {
            Ok(()) => "accepted",
            Err(error) if error.downcast_ref::<RejectedSubmission>().is_some() => "rejected",
            Err(error) if error.downcast_ref::<UncertainSubmission>().is_some() => "unknown",
            Err(_) => "unavailable",
        }
    } else {
        "not_requested"
    };
    Ok(Json(
        json!({"schema_version":1,"metrics_scope":"self_profile_mutation","success":true,"message":if email_changed{"Profile changed; verify the new email and sign in again"}else{"Profile changed"},"data":profile,"email_verification_required":email_changed,"email_delivery":{"status":delivery},"reauthentication_required":email_changed,"replace_session_tokens":auth.is_some(),"auth":auth}),
    ))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub(crate) struct PasswordRequest {
    expected_version: i64,
    #[serde(alias = "currentPassword")]
    current_password: String,
    #[serde(alias = "password", alias = "newPassword")]
    new_password: String,
    #[serde(alias = "password_confirmation", alias = "newPasswordConfirmation")]
    new_password_confirmation: String,
    revoke_sessions: Option<bool>,
}
pub(crate) async fn change_password(
    state: State<AppState>,
    user: User,
    verified: Extension<VerifiedAuthority>,
    Json(input): Json<PasswordRequest>,
) -> Result<Json<Value>, ApiError> {
    if input.new_password != input.new_password_confirmation {
        return Err(ApiError::validation_error("Passwords do not match"));
    }
    if input.revoke_sessions == Some(false) {
        return Err(ApiError::validation_error(
            "Password changes always revoke previous tokens",
        ));
    }
    update_profile(
        state,
        user,
        verified,
        Json(UpdateProfileRequest {
            expected_version: input.expected_version,
            first_name: None,
            last_name: None,
            display_name: None,
            email: None,
            current_password: Some(input.current_password),
            new_password: Some(input.new_password),
            avatar_url: None,
        }),
    )
    .await
}
