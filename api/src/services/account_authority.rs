//! PostgreSQL is the authority for account state, roles, token epochs and revocation.
use crate::{models::User, utils::Claims};
use axum::http::StatusCode;
use serde_json::Value;
use sqlx::{PgConnection, PgPool};
use uuid::Uuid;

pub type Failure = (StatusCode, &'static str);
/// Trusted extractor context; clients cannot supply or forge this request extension.
#[derive(Clone, Copy)]
pub struct VerifiedAuthority {
    pub subject: i64,
    pub version: i64,
    jti: Uuid,
    expires_at: i64,
    mfa_assured: bool,
}
pub struct Account {
    pub user: User,
    pub version: i64,
    pub mfa_verified: bool,
}
fn db(error: sqlx::Error) -> Failure {
    tracing::error!(%error, "Durable account authority is unavailable");
    (
        StatusCode::SERVICE_UNAVAILABLE,
        "Authentication is temporarily unavailable",
    )
}
fn unauthorized() -> Failure {
    (
        StatusCode::UNAUTHORIZED,
        "Account or token is no longer active",
    )
}

// JSON conversion avoids SQLx's unchecked timestamp decoder for legal PostgreSQL wide/infinite
// values. An unrepresentable User receives an explicit error, never a fabricated current date.
const ACCOUNT: &str = r#"SELECT jsonb_build_object('user',jsonb_build_object(
 'id',u.id,'email',u.email,'password_hash',u.password_hash,'name',u.name,'role',u.role,
 'is_active',u.is_active,'email_verified_at',u.email_verified_at,'avatar_url',u.avatar_url,
 'mfa_enabled',u.mfa_enabled,'created_at',u.created_at,'updated_at',u.updated_at),
 'mfa_verified',EXISTS(SELECT 1 FROM user_mfa_secrets s WHERE s.user_id=u.id AND s.totp_verified_at IS NOT NULL),'version',u.auth_version,'deleted',u.deleted_at IS NOT NULL OR COALESCE(LOWER(u.role),'')='deleted',
 'restricted',EXISTS(SELECT 1 FROM user_status s WHERE s.user_id=u.id AND s.status IN('banned','suspended') AND(s.ban_expires_at IS NULL OR s.ban_expires_at>NOW())))
 FROM users u WHERE u.id=$1"#;

pub async fn current_on(connection: &mut PgConnection, id: i64) -> Result<Account, Failure> {
    if id <= 0 {
        return Err(unauthorized());
    }
    let row: Value = sqlx::query_scalar(ACCOUNT)
        .bind(id)
        .fetch_optional(connection)
        .await
        .map_err(db)?
        .ok_or_else(unauthorized)?;
    if row["deleted"] == true || row["restricted"] == true || row["user"]["is_active"] == false {
        return Err(unauthorized());
    }
    let version = row["version"].as_i64().filter(|v| *v >= 0).ok_or((
        StatusCode::SERVICE_UNAVAILABLE,
        "Account authority is invalid",
    ))?;
    let user = serde_json::from_value(row["user"].clone()).map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "The account has a recorded value that authentication cannot represent",
        )
    })?;
    Ok(Account {
        user,
        version,
        mfa_verified: row["mfa_verified"] == true,
    })
}
pub async fn current(pool: &PgPool, id: i64) -> Result<Account, Failure> {
    let mut connection = pool.acquire().await.map_err(db)?;
    current_on(&mut connection, id).await
}
/// Captured only after signature/type validation by the actual User extractor.
pub(crate) fn capture(claims: &Claims) -> Result<VerifiedAuthority, Failure> {
    if claims.authority_version != 1
        || claims.token_type != "access"
        || claims.sub <= 0
        || claims.token_version < 0
    {
        return Err(unauthorized());
    }
    let jti = Uuid::parse_str(&claims.jti).map_err(|_| unauthorized())?;
    Ok(VerifiedAuthority {
        subject: claims.sub,
        version: claims.token_version,
        jti,
        expires_at: claims.exp,
        mfa_assured: claims.mfa_assured,
    })
}
/// Recheck the original access token, current account and exact DB-clock expiry.
/// Effectful callers first stabilize users with the account table/row lock order.
pub async fn verified_on(
    connection: &mut PgConnection,
    verified: &VerifiedAuthority,
) -> Result<Account, Failure> {
    let account = current_on(connection, verified.subject).await?;
    if account.version != verified.version {
        return Err(unauthorized());
    }
    require_mfa_state(&account)?;
    if account.user.mfa_enabled == Some(true) && !verified.mfa_assured {
        return Err(unauthorized());
    }
    let (revoked,unexpired):(bool,bool)=sqlx::query_as("SELECT EXISTS(SELECT 1 FROM auth_revoked_tokens WHERE jti=$1),$2::bigint>FLOOR(EXTRACT(EPOCH FROM clock_timestamp()))::bigint").bind(verified.jti).bind(verified.expires_at).fetch_one(connection).await.map_err(db)?;
    if revoked || !unexpired {
        return Err(unauthorized());
    }
    Ok(account)
}
pub async fn authenticate(pool: &PgPool, claims: &Claims) -> Result<Account, Failure> {
    let verified = capture(claims)?;
    let mut tx = pool.begin().await.map_err(db)?;
    sqlx::query("SET TRANSACTION ISOLATION LEVEL REPEATABLE READ READ ONLY")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    let account = verified_on(&mut tx, &verified).await?;
    tx.commit().await.map_err(db)?;
    Ok(account)
}
/// Explicit epoch revocation is durable even when Redis is absent or has lost every key.
pub async fn revoke_all(pool: &PgPool, verified: &VerifiedAuthority) -> Result<i64, Failure> {
    let mut tx = pool.begin().await.map_err(db)?;
    sqlx::query("LOCK TABLE users IN ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR NO KEY UPDATE")
        .bind(verified.subject)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(unauthorized)?;
    let current = verified_on(&mut tx, verified).await?;
    let version = sqlx::query_scalar("UPDATE users SET auth_version=auth_version+1 WHERE id=$1 AND auth_version=$2 RETURNING auth_version")
        .bind(current.user.id).bind(verified.version).fetch_optional(&mut *tx).await.map_err(db)?
        .ok_or_else(unauthorized)?;
    tx.commit().await.map_err(db)?;
    Ok(version)
}
/// One refresh token may rotate once. Reuse durably strands its entire token family.
pub async fn consume_refresh(pool: &PgPool, claims: &Claims) -> Result<Account, Failure> {
    if claims.authority_version != 1 || claims.token_type != "refresh" {
        return Err(unauthorized());
    }
    let jti = Uuid::parse_str(&claims.jti).map_err(|_| unauthorized())?;
    let mut tx = pool.begin().await.map_err(db)?;
    sqlx::query("LOCK TABLE users IN ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR UPDATE")
        .bind(claims.sub)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(unauthorized)?;
    let account = current_on(&mut tx, claims.sub).await?;
    if account.version != claims.token_version {
        return Err(unauthorized());
    }
    require_mfa_state(&account)?;
    if account.user.mfa_enabled == Some(true) && !claims.mfa_assured {
        return Err(unauthorized());
    }
    let unexpired: bool = sqlx::query_scalar(
        "SELECT $1::bigint>FLOOR(EXTRACT(EPOCH FROM clock_timestamp()))::bigint",
    )
    .bind(claims.exp)
    .fetch_one(&mut *tx)
    .await
    .map_err(db)?;
    if !unexpired {
        return Err(unauthorized());
    }
    let inserted=sqlx::query("INSERT INTO auth_revoked_tokens(jti,user_id,token_type,reason,expires_at) VALUES($1,$2,'refresh','rotated',to_timestamp($3::bigint::double precision)) ON CONFLICT(jti) DO NOTHING")
        .bind(jti).bind(claims.sub).bind(claims.exp).execute(&mut *tx).await.map_err(db)?.rows_affected();
    if inserted == 0 {
        sqlx::query("UPDATE users SET auth_version=auth_version+1 WHERE id=$1")
            .bind(claims.sub)
            .execute(&mut *tx)
            .await
            .map_err(db)?;
        tx.commit().await.map_err(db)?;
        return Err((
            StatusCode::UNAUTHORIZED,
            "Refresh token was reused; authenticate again",
        ));
    }
    tx.commit().await.map_err(db)?;
    Ok(account)
}
/// Single-device logout persists the exact access/refresh identifiers in one transaction.
pub async fn revoke_tokens(pool: &PgPool, member: i64, tokens: &[Claims]) -> Result<(), Failure> {
    let mut tx = pool.begin().await.map_err(db)?;
    // Serialize exact-JTI logout with account effects that stabilize the original actor.
    sqlx::query("LOCK TABLE users IN ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .map_err(db)?;
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR NO KEY UPDATE")
        .bind(member)
        .fetch_optional(&mut *tx)
        .await
        .map_err(db)?
        .ok_or_else(unauthorized)?;
    for token in tokens {
        if token.sub != member
            || token.authority_version != 1
            || !matches!(token.token_type.as_str(), "access" | "refresh")
        {
            return Err(unauthorized());
        }
        let jti = Uuid::parse_str(&token.jti).map_err(|_| unauthorized())?;
        sqlx::query("INSERT INTO auth_revoked_tokens(jti,user_id,token_type,reason,expires_at) VALUES($1,$2,$3,'logout',to_timestamp($4::bigint::double precision)) ON CONFLICT(jti) DO NOTHING")
            .bind(jti).bind(member).bind(&token.token_type).bind(token.exp).execute(&mut *tx).await.map_err(db)?;
    }
    tx.commit().await.map_err(db)
}

/// Checked adapter for authentication-specific reads of historical PostgreSQL dates.
pub fn checked_user(value: Value) -> Result<User, Failure> {
    serde_json::from_value(value).map_err(|_| {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            "The account has a recorded value that authentication cannot represent",
        )
    })
}

/// A verified secret and the recorded required flag must agree; inconsistent state is unavailable.
pub fn require_mfa_state(account: &Account) -> Result<bool, Failure> {
    if account.mfa_verified != (account.user.mfa_enabled == Some(true)) {
        return Err((
            StatusCode::SERVICE_UNAVAILABLE,
            "Recorded MFA state requires repair; authentication was not granted",
        ));
    }
    Ok(account.mfa_verified)
}
