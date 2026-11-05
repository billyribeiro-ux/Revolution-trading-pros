//! Original logout-all authority is rechecked after the actual mutation lock waits.
use super::request;
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::time::Duration;
use tokio::task::JoinHandle;
use uuid::Uuid;

const LOCK_QUERY: &str = "SELECT id FROM users WHERE id=$1 FOR NO KEY UPDATE";
fn access(id: i64, version: i64) -> String {
    revolution_api::utils::create_jwt_versioned(
        id,
        &std::env::var("JWT_SECRET").unwrap(),
        1,
        version,
    )
    .unwrap()
}
fn pending(api: &Router, token: &str) -> JoinHandle<(StatusCode, Value)> {
    let api = api.clone();
    let token = token.to_owned();
    tokio::spawn(async move {
        request(
            &api,
            "POST",
            "/api/auth/logout-all",
            Some(&token),
            Value::Null,
        )
        .await
    })
}
async fn blocked(pool: &PgPool, count: i64) {
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    loop {
        let actual: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM pg_stat_activity WHERE datname=current_database() AND pid<>pg_backend_pid() AND query=$1 AND wait_event_type='Lock'")
            .bind(LOCK_QUERY).fetch_one(pool).await.unwrap();
        if actual >= count {
            return;
        }
        assert!(
            tokio::time::Instant::now() < deadline,
            "logout-all never reached its held mutation lock; expected {count}, observed {actual}"
        );
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}
async fn epoch(pool: &PgPool, id: i64) -> i64 {
    sqlx::query_scalar("SELECT auth_version FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}
async fn hold(pool: &PgPool, id: i64) -> sqlx::Transaction<'_, sqlx::Postgres> {
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR UPDATE")
        .bind(id)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    tx
}
async fn joined(handle: JoinHandle<(StatusCode, Value)>) -> (StatusCode, Value) {
    tokio::time::timeout(Duration::from_secs(10), handle)
        .await
        .unwrap()
        .unwrap()
}
pub async fn proof(api: &Router, pool: &PgPool) {
    let id: i64 = sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active) VALUES('Delayed logout fixture',$1,'not-a-login-password','user',true) RETURNING id")
        .bind(format!("logout{}@example.test", Uuid::new_v4().simple())).fetch_one(pool).await.unwrap();
    let original = access(id, 0);
    let mut held = hold(pool, id).await;
    let delayed = pending(api, &original);
    blocked(pool, 1).await;
    sqlx::query("UPDATE users SET auth_version=auth_version+1 WHERE id=$1")
        .bind(id)
        .execute(&mut *held)
        .await
        .unwrap();
    held.commit().await.unwrap();
    let denied = joined(delayed).await;
    assert_eq!(denied.0, StatusCode::UNAUTHORIZED, "{denied:?}");
    assert_eq!(
        epoch(pool, id).await,
        1,
        "stale request must not revoke the newer epoch"
    );
    let current = access(id, 1);
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&current), Value::Null)
            .await
            .0,
        StatusCode::OK
    );

    let claims = revolution_api::utils::verify_jwt(
        &current,
        &std::env::var("JWT_SECRET").unwrap(),
        "access",
    )
    .unwrap();
    let mut held = hold(pool, id).await;
    let delayed = pending(api, &current);
    blocked(pool, 1).await;
    sqlx::query("INSERT INTO auth_revoked_tokens(jti,user_id,token_type,reason,expires_at) VALUES($1,$2,'access','logout',to_timestamp($3::bigint::double precision))")
        .bind(Uuid::parse_str(&claims.jti).unwrap()).bind(id).bind(claims.exp).execute(&mut *held).await.unwrap();
    held.commit().await.unwrap();
    let denied = joined(delayed).await;
    assert_eq!(denied.0, StatusCode::UNAUTHORIZED, "{denied:?}");
    assert_eq!(
        epoch(pool, id).await,
        1,
        "exact JTI revocation prevents a delayed all-session mutation"
    );
    let first = access(id, 1);
    let second = access(id, 1);
    let held = hold(pool, id).await;
    let left = pending(api, &first);
    let right = pending(api, &second);
    blocked(pool, 2).await;
    held.commit().await.unwrap();
    let left = joined(left).await;
    let right = joined(right).await;
    assert!(
        (left.0 == StatusCode::OK && right.0 == StatusCode::UNAUTHORIZED)
            || (right.0 == StatusCode::OK && left.0 == StatusCode::UNAUTHORIZED),
        "one authority generation revokes once: {left:?}, {right:?}"
    );
    assert_eq!(epoch(pool, id).await, 2);

    // A real clock boundary is tested while PostgreSQL confirms the request is
    // already past its extractor and waiting for the protected actor lock.
    let now = chrono::Utc::now().timestamp();
    let expiry = now + 4;
    let expiring = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &json!({"sub":id,"iat":now,"exp":expiry,"token_type":"access","tv":2,"av":1,"jti":Uuid::new_v4()}), &jsonwebtoken::EncodingKey::from_secret(std::env::var("JWT_SECRET").unwrap().as_bytes())).unwrap();
    let held = hold(pool, id).await;
    let delayed = pending(api, &expiring);
    blocked(pool, 1).await;
    while chrono::Utc::now().timestamp() <= expiry {
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
    held.commit().await.unwrap();
    let denied = joined(delayed).await;
    assert_eq!(denied.0, StatusCode::UNAUTHORIZED, "{denied:?}");
    assert_eq!(
        epoch(pool, id).await,
        2,
        "JWT leeway and transaction NOW cannot authorize after real expiry"
    );
    assert_eq!(
        request(
            api,
            "GET",
            "/api/auth/me",
            Some(&access(id, 2)),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
