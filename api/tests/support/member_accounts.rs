//! Real account mutations and durable authority; never provider billing cancellation.
use super::{Adapter, request};
use axum::{
    Router,
    body::{Body, to_bytes},
    http::{Request, StatusCode, header},
};
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;
const ROOT: &str = "/api/admin/member-management";
const PASSWORD: &str = "VioletCedar9!HarborRidge";
async fn token(pool: &PgPool, id: i64) -> String {
    let epoch: i64 = sqlx::query_scalar("SELECT auth_version FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();
    revolution_api::utils::create_jwt_versioned(id, &std::env::var("JWT_SECRET").unwrap(), 1, epoch)
        .unwrap()
}
fn submitted(adapter: &Adapter) -> usize {
    let guard = adapter.requests.lock().unwrap();
    guard.len()
}
async fn remove(api: &Router, id: i64, version: i64, token: &str) -> (StatusCode, Value) {
    let response = api
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("{ROOT}/{id}"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::IF_MATCH, format!("\"{version}\""))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}
async fn snapshot(pool: &PgPool, id: i64) -> Value {
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET LOCAL TIME ZONE 'UTC'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let value:Value=sqlx::query_scalar("SELECT jsonb_build_object('user',(SELECT to_jsonb(u) FROM users u WHERE id=$1),'restriction',(SELECT to_jsonb(s) FROM user_status s WHERE user_id=$1),'memberships',(SELECT COALESCE(jsonb_agg(to_jsonb(m) ORDER BY id),'[]') FROM user_memberships m WHERE user_id=$1),'activity',(SELECT COALESCE(jsonb_agg(to_jsonb(a) ORDER BY id),'[]') FROM user_activity_log a WHERE user_id=$1),'audit',(SELECT COUNT(*) FROM crm_operational_logs WHERE context->>'member_id'=$1::text))").bind(id).fetch_one(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    value
}
async fn create(api: &Router, token: &str, email: &str, welcome: bool) -> (Uuid, Value) {
    let key = Uuid::new_v4();
    let body = json!({"request_id":key,"name":"Recorded Account <Fixture>","email":email,"password":PASSWORD,"role":"user","send_welcome_email":welcome});
    let result = request(api, "POST", ROOT, Some(token), body.clone()).await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    assert_eq!(result.1["metrics_scope"], "member_account_mutation");
    assert_eq!(result.1["request_id"], key.to_string());
    assert_eq!(result.1["account"]["email"], email);
    assert_eq!(result.1["account"]["version"], 1);
    let id = result.1["account"]["id"].as_i64().unwrap();
    let replay = request(api, "POST", ROOT, Some(token), body.clone()).await;
    assert_eq!(replay.0, StatusCode::OK, "{replay:?}");
    assert_eq!(replay.1["account"]["id"], id);
    assert_eq!(replay.1["replayed"], true);
    assert!(replay.1["temporary_password"].is_null());
    let mut different = body.clone();
    different["name"] = json!("Changed intent");
    let conflict = request(api, "POST", ROOT, Some(token), different).await;
    assert_eq!(conflict.0, StatusCode::CONFLICT, "{conflict:?}");
    (key, result.1)
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let prefix = format!("a{}", Uuid::new_v4().simple());
    let super_id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,email_verified_at) VALUES('Account super fixture',$1,'not-a-login-password','SuPeR_AdMiN',true,NOW() AT TIME ZONE 'UTC') RETURNING id").bind(format!("{prefix}-super@example.test")).fetch_one(pool).await.unwrap();
    let super_token = token(pool, super_id).await;
    let body = json!({"request_id":Uuid::new_v4(),"name":"Denied account","email":format!("{prefix}-denied@example.test"),"password":PASSWORD});
    assert_eq!(
        request(api, "POST", ROOT, None, body.clone()).await.0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(api, "POST", ROOT, Some(&tokens[0]), body.clone())
            .await
            .0,
        StatusCode::FORBIDDEN,
        "Existing frontend Superadmin write policy also holds in Rust"
    );
    for role in ["admin", "administrator", "developer", "user"] {
        let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role) VALUES('Non-super role probe',$1,'not-a-login-password',$2) RETURNING id").bind(format!("{prefix}-{role}@example.test")).bind(role).fetch_one(pool).await.unwrap();
        let denied = token(pool, id).await;
        assert_eq!(
            request(api, "POST", ROOT, Some(&denied), body.clone())
                .await
                .0,
            StatusCode::FORBIDDEN,
            "{role}"
        );
        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await
            .unwrap();
    }
    let (key, created) = create(
        api,
        &super_token,
        &format!("{prefix}-member@example.test"),
        false,
    )
    .await;
    let id = created["account"]["id"].as_i64().unwrap();
    let old = token(pool, id).await;
    let observed = chrono::NaiveDateTime::parse_from_str(
        created["account"]["created_at"].as_str().unwrap(),
        "%Y-%m-%dT%H:%M:%S%.f",
    )
    .unwrap();
    assert!(
        (chrono::Utc::now().naive_utc() - observed)
            .num_seconds()
            .abs()
            < 10,
        "New users use UTC wall time under the NY fixture"
    );
    let changed = request(
        api,
        "PUT",
        &format!("{ROOT}/{id}"),
        Some(&super_token),
        json!({"expected_version":1,"name":"Persisted actual name"}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["account"]["version"], 2);
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&old), Value::Null)
            .await
            .0,
        StatusCode::OK,
        "A name-only edit is not a token revocation"
    );
    let stale = request(
        api,
        "PUT",
        &format!("{ROOT}/{id}"),
        Some(&super_token),
        json!({"expected_version":1,"name":"Lost update"}),
    )
    .await;
    assert_eq!(stale.0, StatusCode::CONFLICT);
    let before = snapshot(pool, id).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO member_accounts_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "PUT",
        &format!("{ROOT}/{id}"),
        Some(&super_token),
        json!({"expected_version":2,"name":"Must roll back"}),
    )
    .await;
    sqlx::query("ALTER TABLE member_accounts_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    assert_eq!(
        snapshot(pool, id).await,
        before,
        "Account, version and activity roll back with required audit"
    );
    let changed = request(
        api,
        "PUT",
        &format!("{ROOT}/{id}"),
        Some(&super_token),
        json!({"expected_version":2,"email":format!("{prefix}-changed@example.test")}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["account"]["version"], 3);
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&old), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED,
        "Email authority changes durably revoke without Redis"
    );
    let fresh = token(pool, id).await;
    let refresh = revolution_api::utils::create_refresh_token_versioned(
        id,
        &std::env::var("JWT_SECRET").unwrap(),
        1,
    )
    .unwrap();
    let rotated = request(
        api,
        "POST",
        "/api/auth/refresh",
        None,
        json!({"refresh_token":refresh}),
    )
    .await;
    assert_eq!(rotated.0, StatusCode::OK, "{rotated:?}");
    let replay = request(
        api,
        "POST",
        "/api/auth/refresh",
        None,
        json!({"refresh_token":refresh}),
    )
    .await;
    assert_eq!(replay.0, StatusCode::UNAUTHORIZED);
    assert_eq!(
        request(
            api,
            "GET",
            "/api/auth/me",
            Some(rotated.1["access_token"].as_str().unwrap()),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED,
        "Refresh reuse revokes the rotated family in PostgreSQL"
    );
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&fresh), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let banned = request(
        api,
        "POST",
        &format!("{ROOT}/{id}/ban"),
        Some(&tokens[0]),
        json!({"expected_version":3,"reason":"Recorded restriction","duration_days":1}),
    )
    .await;
    assert_eq!(banned.0, StatusCode::OK, "{banned:?}");
    assert_eq!(banned.1["account"]["is_active"], false);
    assert_eq!(banned.1["account"]["version"], 4);
    sqlx::query("UPDATE user_status SET ban_expires_at=NOW()-INTERVAL '1 second' WHERE user_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    revolution_api::queue::worker::process_account_expirations(&revolution_api::db::Database {
        pool: pool.clone(),
    })
    .await
    .unwrap();
    let profile = request(
        api,
        "GET",
        &format!("{ROOT}/{id}"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(profile.0, StatusCode::OK, "{profile:?}");
    assert_eq!(profile.1["member"]["account"]["is_active"], true);
    assert_eq!(profile.1["member"]["account"]["version"], 5);
    let removed = remove(api, id, 5, &super_token).await;
    assert_eq!(removed.0, StatusCode::OK, "{removed:?}");
    assert_eq!(removed.1["removed"], true);
    assert_eq!(removed.1["provider_billing"], "unchanged");
    let deleted_at: bool =
        sqlx::query_scalar("SELECT deleted_at IS NOT NULL FROM users WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(deleted_at);
    for base in [ROOT, "/api/admin/members"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{base}/{id}"),
                Some(&tokens[0]),
                Value::Null
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        )
    }
    assert_eq!(remove(api, id, 5, &super_token).await.1["removed"], false);
    // The explicit old writer marker is excluded without inventing a historical deletion date.
    let legacy:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active) VALUES('Deleted User',$1,'DELETED','deleted',false) RETURNING id").bind(format!("{prefix}-legacy@removed.local")).fetch_one(pool).await.unwrap();
    let included: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM admin_member_facts WHERE id=$1)")
            .bind(legacy)
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(!included);
    let raw: bool = sqlx::query_scalar("SELECT deleted_at IS NULL FROM users WHERE id=$1")
        .bind(legacy)
        .fetch_one(pool)
        .await
        .unwrap();
    assert!(raw);
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/{legacy}"),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let count = submitted(adapter);
    let mut welcomes = Vec::new();
    for (scenario, outcome) in [
        ("accepted", "accepted"),
        ("rejected", "rejected"),
        ("unknown", "unknown"),
    ] {
        let email = format!("{prefix}+member-accounts-{scenario}@example.test");
        let (req, ack) = create(api, &super_token, &email, true).await;
        assert_eq!(ack["welcome"]["status"], "queued");
        let member = ack["account"]["id"].as_i64().unwrap();
        let job = Uuid::parse_str(ack["welcome"]["job_id"].as_str().unwrap()).unwrap();
        sqlx::query("UPDATE jobs SET available_at=TIMESTAMPTZ '1900-01-01' WHERE id=$1")
            .bind(job)
            .execute(pool)
            .await
            .unwrap();
        let processed =
            revolution_api::queue::worker::process_next_job(&revolution_api::db::Database {
                pool: pool.clone(),
            })
            .await
            .unwrap();
        assert_eq!(processed, Some(job));
        let log: i64 = ack["welcome"]["email_log_id"].as_i64().unwrap();
        let actual:(String,Option<String>,bool)=sqlx::query_as("SELECT status,provider_message_id,sent_at IS NOT NULL FROM email_logs WHERE id=$1 AND user_id=$2").bind(log).bind(member).fetch_one(pool).await.unwrap();
        match outcome {
            "accepted" => {
                assert_eq!(actual.0, "sent");
                assert!(actual.1.is_some());
                assert!(actual.2)
            }
            "rejected" => {
                assert_eq!(actual.0, "failed");
                assert!(!actual.2)
            }
            _ => {
                assert_eq!(actual.0, "delivery_unknown");
                assert!(!actual.2)
            }
        }
        welcomes.push((req, member, job, log));
    }
    assert_eq!(submitted(adapter), count + 3);
    for (req, member, job, log) in welcomes {
        sqlx::query("DELETE FROM jobs WHERE id=$1")
            .bind(job)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM email_logs WHERE id=$1")
            .bind(log)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(member)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM member_admin_creations WHERE request_id=$1")
            .bind(req)
            .execute(pool)
            .await
            .unwrap();
    }
    mutation_faults_and_expiry(api, pool, &tokens[0], &super_token, &prefix).await;
    welcome_faults(api, pool, &super_token, &prefix, adapter).await;
    websocket_authority(pool, &tokens[0], &super_token, &prefix).await;
    authority_edges(api, pool, &tokens[0], &super_token, &prefix).await;
    legacy_aliases(api, pool, &tokens[0], &super_token, &prefix, adapter).await;
    sqlx::query("DELETE FROM users WHERE id=ANY($1)")
        .bind(vec![id, legacy, super_id])
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_admin_creations WHERE request_id=$1")
        .bind(key)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Member account proof passed its explicit create/CAS/delete/epoch/refresh/expiry/welcome/legacy-marker scopes"
    );
}

async fn legacy_aliases(
    api: &Router,
    pool: &PgPool,
    admin: &str,
    super_token: &str,
    prefix: &str,
    adapter: &Adapter,
) {
    let before_calls = submitted(adapter);
    let root = "/api/admin/users";
    let key = Uuid::new_v4();
    let email = format!("{prefix}-alias@example.test");
    let input = json!({"request_id":key,"name":"Legacy alias actual account","email":email,"password":PASSWORD,"roles":["moderator"],"send_welcome_email":false});
    let denied = request(api, "POST", root, Some(admin), input.clone()).await;
    assert_eq!(
        denied.0,
        StatusCode::FORBIDDEN,
        "Alias create cannot bypass the existing Superadmin proxy policy"
    );
    let created = request(api, "POST", root, Some(super_token), input.clone()).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    assert_eq!(
        created.1["account"]["role"], "moderator",
        "The advertised roles array must persist the one actual stored role"
    );
    let id = created.1["account"]["id"].as_i64().unwrap();
    let replay=request(api,"POST",ROOT,Some(super_token),json!({"request_id":key,"name":"Legacy alias actual account","email":email,"password":PASSWORD,"role":"moderator","send_welcome_email":false})).await;
    assert_eq!(replay.0, StatusCode::OK, "{replay:?}");
    assert_eq!(
        replay.1["account"]["id"], id,
        "Both aliases share one durable creation intent"
    );
    let path = format!("{root}/{id}");
    let snapshot_before = snapshot(pool, id).await;
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            Some(admin),
            json!({"expected_version":1,"name":"Forbidden"})
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(api, "DELETE", &path, Some(admin), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            Some(super_token),
            json!({"expected_version":1,"roles":["user","admin"]})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST,
        "Multiple independent roles are not stored and may not be fabricated"
    );
    assert_eq!(snapshot(pool, id).await, snapshot_before);
    let changed = request(
        api,
        "PUT",
        &path,
        Some(super_token),
        json!({"expected_version":1,"roles":["administrator"],"is_active":false}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["account"]["role"], "admin");
    assert_eq!(changed.1["account"]["is_active"], false);
    assert_eq!(changed.1["account"]["version"], 2);
    let fresh = request(api, "GET", &path, Some(admin), Value::Null).await;
    assert_eq!(fresh.0, StatusCode::OK);
    assert_eq!(fresh.1["member"]["role"], "admin");
    assert_eq!(fresh.1["member"]["account"]["version"], 2);
    assert_eq!(fresh.1["member"]["account"]["is_active"], false);
    let before = snapshot(pool, id).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO member_alias_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "PUT",
        &path,
        Some(super_token),
        json!({"expected_version":2,"name":"Must roll back through alias"}),
    )
    .await;
    sqlx::query("ALTER TABLE member_alias_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    assert_eq!(snapshot(pool, id).await, before);
    let enabled = request(
        api,
        "PUT",
        &path,
        Some(super_token),
        json!({"expected_version":2,"is_active":true}),
    )
    .await;
    assert_eq!(enabled.0, StatusCode::OK, "{enabled:?}");
    assert_eq!(enabled.1["account"]["version"], 3);
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/{id}/ban"),
            Some(admin),
            json!({"expected_version":3,"duration_days":1})
        )
        .await
        .0,
        StatusCode::FORBIDDEN,
        "Current privileged peers require current Superadmin authority"
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/ban"),
            Some(admin),
            json!({"expected_version":3})
        )
        .await
        .0,
        StatusCode::FORBIDDEN,
        "The /users moderation alias preserves its stronger existing proxy boundary"
    );
    let stale = remove_from(api, root, id, 2, super_token).await;
    assert_eq!(stale.0, StatusCode::CONFLICT);
    let deleted = remove_from(api, root, id, 3, super_token).await;
    assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
    assert_eq!(deleted.1["removed"], true);
    assert_eq!(deleted.1["provider_billing"], "unchanged");
    for family in [root, ROOT, "/api/admin/members"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{family}/{id}"),
                Some(admin),
                Value::Null
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        )
    }
    assert_eq!(submitted(adapter), before_calls);
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_admin_creations WHERE request_id=$1")
        .bind(key)
        .execute(pool)
        .await
        .unwrap();
}
async fn remove_from(
    api: &Router,
    root: &str,
    id: i64,
    version: i64,
    token: &str,
) -> (StatusCode, Value) {
    let response = api
        .clone()
        .oneshot(
            Request::builder()
                .method("DELETE")
                .uri(format!("{root}/{id}"))
                .header(header::AUTHORIZATION, format!("Bearer {token}"))
                .header(header::IF_MATCH, format!("\"{version}\""))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = to_bytes(response.into_body(), 1_000_000).await.unwrap();
    (status, serde_json::from_slice(&bytes).unwrap())
}

async fn authority_edges(
    api: &Router,
    pool: &PgPool,
    admin: &str,
    super_token: &str,
    prefix: &str,
) {
    let (key, created) = create(
        api,
        super_token,
        &format!("{prefix}-authority@example.test"),
        false,
    )
    .await;
    let id = created["account"]["id"].as_i64().unwrap();
    let secret = std::env::var("JWT_SECRET").unwrap();
    let access = token(pool, id).await;
    let now = chrono::Utc::now().timestamp();
    let expired=jsonwebtoken::encode(&jsonwebtoken::Header::default(),&json!({"sub":id,"iat":now-60,"exp":now-30,"token_type":"access","tv":0,"av":1,"jti":Uuid::new_v4()}),&jsonwebtoken::EncodingKey::from_secret(secret.as_bytes())).unwrap();
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&expired), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED,
        "JWT library clock leeway cannot resurrect a token after durable revocation cleanup"
    );
    let legacy=jsonwebtoken::encode(&jsonwebtoken::Header::default(),&json!({"sub":id,"iat":now,"exp":now+3600,"token_type":"access","tv":0,"jti":Uuid::new_v4()}),&jsonwebtoken::EncodingKey::from_secret(secret.as_bytes())).unwrap();
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&legacy), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED,
        "Redis-era tokens must reauthenticate; no epoch is guessed"
    );
    for epoch in [-1, 999] {
        let forged_epoch =
            revolution_api::utils::create_jwt_versioned(id, &secret, 1, epoch).unwrap();
        assert_eq!(
            request(api, "GET", "/api/auth/me", Some(&forged_epoch), Value::Null)
                .await
                .0,
            StatusCode::UNAUTHORIZED,
            "Epochs must match exactly, not merely be above the stored value"
        )
    }
    assert_eq!(
        request(
            api,
            "POST",
            "/api/auth/refresh",
            None,
            json!({"refresh_token":access})
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED,
        "Access tokens cannot rotate as refresh tokens"
    );
    let refresh = revolution_api::utils::create_refresh_token_versioned(id, &secret, 0).unwrap();
    let logged_out = request(
        api,
        "POST",
        "/api/auth/logout",
        Some(&access),
        json!({"token":access,"refresh_token":refresh}),
    )
    .await;
    assert_eq!(logged_out.0, StatusCode::OK, "{logged_out:?}");
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&access), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            api,
            "POST",
            "/api/auth/refresh",
            None,
            json!({"refresh_token":refresh})
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let current = token(pool, id).await;
    let all = request(
        api,
        "POST",
        "/api/auth/logout-all",
        Some(&current),
        Value::Null,
    )
    .await;
    assert_eq!(all.0, StatusCode::OK, "{all:?}");
    assert_eq!(all.1["success"], true);
    assert!(
        all.1["sessions_invalidated"].is_null(),
        "With no Redis sessions the actual device count is unknown, not fabricated zero"
    );
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&current), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let current = token(pool, id).await;
    sqlx::query("UPDATE users SET created_at=TIMESTAMP 'infinity' WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&current), Value::Null)
            .await
            .0,
        StatusCode::SERVICE_UNAVAILABLE,
        "Legal unrepresentable auth dates never panic or become a forged current date"
    );
    let factual = request(
        api,
        "GET",
        &format!("{ROOT}/{id}"),
        Some(admin),
        Value::Null,
    )
    .await;
    assert_eq!(factual.0, StatusCode::OK, "{factual:?}");
    assert_eq!(factual.1["member"]["created_at"], "infinity");
    sqlx::query("UPDATE users SET created_at=$2::text::timestamp WHERE id=$1")
        .bind(id)
        .bind(created["account"]["created_at"].as_str().unwrap())
        .execute(pool)
        .await
        .unwrap();
    let before = snapshot(pool, id).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO self_account_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "POST",
        "/api/user/deactivate",
        Some(&current),
        json!({"password":PASSWORD,"reason":"Recorded self closure","delete_data":false}),
    )
    .await;
    sqlx::query("ALTER TABLE self_account_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    assert_eq!(
        snapshot(pool, id).await,
        before,
        "Self closure cannot discard its required journal/audit"
    );
    let inactive = request(
        api,
        "POST",
        "/api/user/deactivate",
        Some(&current),
        json!({"password":PASSWORD,"reason":"Recorded self closure","delete_data":false}),
    )
    .await;
    assert_eq!(inactive.0, StatusCode::OK, "{inactive:?}");
    assert_eq!(inactive.1["is_active"], false);
    assert_eq!(inactive.1["deleted"], false);
    assert_eq!(inactive.1["provider_billing"], "unchanged");
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&current), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM admin_member_facts WHERE id=$1)")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(exists, "Inactivity is not a deletion marker");
    let profile = request(
        api,
        "GET",
        &format!("{ROOT}/{id}"),
        Some(admin),
        Value::Null,
    )
    .await;
    let version = profile.1["member"]["account"]["version"].as_i64().unwrap();
    let reactivated = request(
        api,
        "PUT",
        &format!("{ROOT}/{id}"),
        Some(super_token),
        json!({"expected_version":version,"is_active":true}),
    )
    .await;
    assert_eq!(reactivated.0, StatusCode::OK, "{reactivated:?}");
    assert_eq!(
        request(api, "GET", "/api/auth/me", Some(&current), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED,
        "Reactivation does not resurrect any old token"
    );
    let fresh = token(pool, id).await;
    let deleted = request(
        api,
        "POST",
        "/api/user/deactivate",
        Some(&fresh),
        json!({"password":PASSWORD,"reason":"Recorded anonymization request","delete_data":true}),
    )
    .await;
    assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
    assert_eq!(deleted.1["deleted"], true);
    assert!(
        deleted.1["data_retention"]
            .as_str()
            .unwrap()
            .contains("retained")
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/{id}"),
            Some(admin),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_admin_creations WHERE request_id=$1")
        .bind(key)
        .execute(pool)
        .await
        .unwrap();
}

async fn websocket_authority(pool: &PgPool, admin: &str, super_token: &str, prefix: &str) {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    async fn frame(stream: &mut tokio::net::TcpStream) -> Option<Value> {
        let mut header = [0u8; 2];
        if stream.read_exact(&mut header).await.is_err() {
            return None;
        }
        let opcode = header[0] & 15;
        if opcode == 8 {
            return None;
        }
        assert_eq!(header[0] & 128, 128, "Controlled test frames are complete");
        assert_eq!(header[1] & 128, 0, "Server frames must not be masked");
        let mut size = u64::from(header[1] & 127);
        if size == 126 {
            size = u64::from(stream.read_u16().await.unwrap())
        } else if size == 127 {
            size = stream.read_u64().await.unwrap()
        }
        assert!(size <= 65536);
        let mut bytes = vec![0u8; usize::try_from(size).unwrap()];
        stream.read_exact(&mut bytes).await.unwrap();
        Some(serde_json::from_slice(&bytes).unwrap())
    }
    async fn connect(address: std::net::SocketAddr, path: &str) -> tokio::net::TcpStream {
        let mut stream = tokio::net::TcpStream::connect(address).await.unwrap();
        stream.write_all(format!("GET {path} HTTP/1.1\r\nHost: {address}\r\nConnection: Upgrade\r\nUpgrade: websocket\r\nSec-WebSocket-Version: 13\r\nSec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n\r\n").as_bytes()).await.unwrap();
        let mut response = Vec::new();
        while !response.ends_with(b"\r\n\r\n") {
            assert!(response.len() < 16384);
            response.push(stream.read_u8().await.unwrap())
        }
        assert!(
            response.starts_with(b"HTTP/1.1 101"),
            "{}",
            String::from_utf8_lossy(&response)
        );
        stream
    }
    async fn send(stream: &mut tokio::net::TcpStream, value: Value) {
        let body = serde_json::to_vec(&value).unwrap();
        assert!(body.len() <= 125);
        let mask = [17u8, 117, 29, 41];
        let mut frame = vec![129, 128 | u8::try_from(body.len()).unwrap()];
        frame.extend(mask);
        frame.extend(body.iter().enumerate().map(|(i, b)| b ^ mask[i % 4]));
        stream.write_all(&frame).await.unwrap()
    }
    let state = super::state(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let manager = state.ws_manager.clone();
    let app = Router::new()
        .nest("/api", revolution_api::routes::api_router())
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let serving = app.clone();
    let server = tokio::spawn(async move { axum::serve(listener, serving).await.unwrap() });
    let room = format!("{prefix}-ws");
    let mut anonymous = connect(address, "/api/ws/ws").await;
    send(&mut anonymous, json!({"action":"subscribe","room":room})).await;
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        loop {
            let value = frame(&mut anonymous).await.unwrap();
            assert_ne!(
                value["type"], "Subscribed",
                "An anonymous upgraded connection cannot join a private room"
            );
            if value["type"] == "Error" {
                assert_eq!(value["payload"]["code"], "AUTH_REQUIRED");
                break;
            }
        }
    })
    .await
    .unwrap();
    assert_eq!(manager.room_connection_count(&room).await, 0);
    let (key, created) = create(
        &app,
        super_token,
        &format!("{prefix}-websocket@example.test"),
        false,
    )
    .await;
    let id = created["account"]["id"].as_i64().unwrap();
    let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES($1,$1,0,'monthly') RETURNING id").bind(format!("{prefix}-ws-plan")).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at) VALUES($1,$2,'active',NOW() AT TIME ZONE 'UTC',(NOW() AT TIME ZONE 'UTC')+INTERVAL '1 day')").bind(id).bind(plan).execute(pool).await.unwrap();
    let jwt = token(pool, id).await;
    let mut connected = connect(address, &format!("/api/ws/ws?token={jwt}&rooms={room}")).await;
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        loop {
            let value = frame(&mut connected).await.unwrap();
            if value["type"] == "Subscribed" {
                assert_eq!(value["payload"]["room"], room);
                break;
            }
        }
    })
    .await
    .unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        while manager.room_connection_count(&room).await != 1 {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await
        }
    })
    .await
    .unwrap();
    manager
        .broadcast_to_room(
            &room,
            revolution_api::routes::websocket::WsMessage::Heartbeat { timestamp: 1170001 },
        )
        .await;
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        loop {
            let value = frame(&mut connected).await.unwrap();
            if value["payload"]["timestamp"] == 1170001 {
                break;
            }
        }
    })
    .await
    .unwrap();
    let banned = request(
        &app,
        "POST",
        &format!("{ROOT}/{id}/ban"),
        Some(admin),
        json!({"expected_version":1,"reason":"Socket revocation fixture"}),
    )
    .await;
    assert_eq!(banned.0, StatusCode::OK, "{banned:?}");
    manager
        .broadcast_to_room(
            &room,
            revolution_api::routes::websocket::WsMessage::Heartbeat { timestamp: 1170002 },
        )
        .await;
    tokio::time::timeout(std::time::Duration::from_secs(3),async{loop{let Some(value)=frame(&mut connected).await else{break};assert_ne!(value["payload"]["timestamp"],1170002,"Existing sockets cannot receive another private message after current DB authority is revoked")}}).await.unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(3), async {
        while manager.room_connection_count(&room).await != 0 {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await
        }
    })
    .await
    .unwrap();
    drop(anonymous);
    drop(connected);
    server.abort();
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM membership_plans WHERE id=$1")
        .bind(plan)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_admin_creations WHERE request_id=$1")
        .bind(key)
        .execute(pool)
        .await
        .unwrap();
}

async fn welcome_faults(
    api: &Router,
    pool: &PgPool,
    super_token: &str,
    prefix: &str,
    adapter: &Adapter,
) {
    let submissions = submitted(adapter);
    let mut records = Vec::new();
    for kind in ["configuration", "missing-receipt", "lost-ack"] {
        let email = format!("{prefix}-welcome-{kind}@example.test");
        let (key, created) = create(api, super_token, &email, true).await;
        let id = created["account"]["id"].as_i64().unwrap();
        let job = Uuid::parse_str(created["welcome"]["job_id"].as_str().unwrap()).unwrap();
        let log = created["welcome"]["email_log_id"].as_i64().unwrap();
        sqlx::query("UPDATE jobs SET available_at=TIMESTAMPTZ '1800-01-01' WHERE id=$1")
            .bind(job)
            .execute(pool)
            .await
            .unwrap();
        if kind == "configuration" {
            sqlx::query("ALTER TABLE settings RENAME TO welcome_configuration_unavailable")
                .execute(pool)
                .await
                .unwrap();
        }
        if kind == "missing-receipt" {
            sqlx::query("DELETE FROM email_logs WHERE id=$1")
                .bind(log)
                .execute(pool)
                .await
                .unwrap();
        }
        if kind == "lost-ack" {
            sqlx::query("CREATE FUNCTION member_accounts_reject_ack() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.template_type='admin_member_welcome' AND NEW.status='sent' THEN RAISE EXCEPTION 'controlled receipt persistence failure'; END IF; RETURN NEW; END $$").execute(pool).await.unwrap();
            sqlx::query("CREATE TRIGGER member_accounts_reject_ack BEFORE UPDATE ON email_logs FOR EACH ROW EXECUTE FUNCTION member_accounts_reject_ack()").execute(pool).await.unwrap();
        }
        let result =
            revolution_api::queue::worker::process_next_job(&revolution_api::db::Database {
                pool: pool.clone(),
            })
            .await;
        if kind == "configuration" {
            sqlx::query("ALTER TABLE welcome_configuration_unavailable RENAME TO settings")
                .execute(pool)
                .await
                .unwrap();
        }
        if kind == "lost-ack" {
            sqlx::query("DROP TRIGGER member_accounts_reject_ack ON email_logs")
                .execute(pool)
                .await
                .unwrap();
            sqlx::query("DROP FUNCTION member_accounts_reject_ack()")
                .execute(pool)
                .await
                .unwrap();
        }
        assert_eq!(result.unwrap(), Some(job));
        let state: Option<String> = sqlx::query_scalar("SELECT status FROM email_logs WHERE id=$1")
            .bind(log)
            .fetch_optional(pool)
            .await
            .unwrap();
        match kind {
            "configuration" => assert_eq!(state.as_deref(), Some("unavailable")),
            "missing-receipt" => assert!(state.is_none()),
            _ => assert_eq!(
                state.as_deref(),
                Some("delivery_unknown"),
                "Lost persistence of an actual provider ACK is not recorded success"
            ),
        }
        let replay=request(api,"POST",ROOT,Some(super_token),json!({"request_id":key,"name":"Recorded Account <Fixture>","email":email,"password":PASSWORD,"role":"user","send_welcome_email":true})).await;
        assert_eq!(replay.0, StatusCode::OK, "{replay:?}");
        assert_eq!(
            replay.1["welcome"]["status"],
            if kind == "lost-ack" {
                "unknown"
            } else {
                "unavailable"
            }
        );
        let attempts: i32 = sqlx::query_scalar("SELECT attempts FROM jobs WHERE id=$1")
            .bind(job)
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(attempts, 1);
        records.push((key, id, job, log));
    }
    assert_eq!(
        submitted(adapter),
        submissions + 1,
        "Missing configuration/receipt cannot submit; lost ACK submission happens once and creation replay never resubmits"
    );
    for (key, id, job, log) in records {
        sqlx::query("DELETE FROM email_logs WHERE id=$1")
            .bind(log)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM jobs WHERE id=$1")
            .bind(job)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM users WHERE id=$1")
            .bind(id)
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("DELETE FROM member_admin_creations WHERE request_id=$1")
            .bind(key)
            .execute(pool)
            .await
            .unwrap();
    }
}

async fn mutation_faults_and_expiry(
    api: &Router,
    pool: &PgPool,
    admin: &str,
    super_token: &str,
    prefix: &str,
) {
    let key = Uuid::new_v4();
    let email = format!("{prefix}-create-rollback@example.test");
    let before: (i64, i64) =
        sqlx::query_as("SELECT (SELECT COUNT(*) FROM jobs),(SELECT COUNT(*) FROM email_logs)")
            .fetch_one(pool)
            .await
            .unwrap();
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO account_create_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed=request(api,"POST",ROOT,Some(super_token),json!({"request_id":key,"name":"Atomic create failure","email":email,"password":PASSWORD,"send_welcome_email":true})).await;
    sqlx::query("ALTER TABLE account_create_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    let state:(bool,bool,i64,i64)=sqlx::query_as("SELECT EXISTS(SELECT 1 FROM users WHERE email=$1),EXISTS(SELECT 1 FROM member_admin_creations WHERE request_id=$2),(SELECT COUNT(*) FROM jobs),(SELECT COUNT(*) FROM email_logs)").bind(&email).bind(key).fetch_one(pool).await.unwrap();
    assert_eq!(
        state,
        (false, false, before.0, before.1),
        "Creation, welcome job, linked receipt and intent roll back with audit"
    );
    let (key, created) = create(
        api,
        super_token,
        &format!("{prefix}-faults-expiry@example.test"),
        false,
    )
    .await;
    let id = created["account"]["id"].as_i64().unwrap();
    let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES($1,$1,10,'monthly') RETURNING id").bind(format!("{prefix}-fault-plan")).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at) VALUES($1,$2,'active',NOW() AT TIME ZONE 'UTC',(NOW() AT TIME ZONE 'UTC')+INTERVAL '30 days')").bind(id).bind(plan).execute(pool).await.unwrap();
    let before = snapshot(pool, id).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO account_delete_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = remove(api, id, 1, super_token).await;
    sqlx::query("ALTER TABLE account_delete_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    assert_eq!(
        before,
        snapshot(pool, id).await,
        "Anonymization/local entitlement cancellation/journal roll back together"
    );
    sqlx::query("UPDATE users SET is_active=NULL WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let current: i64 = sqlx::query_scalar("SELECT account_version FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();
    let before = snapshot(pool, id).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO account_restrict_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "POST",
        &format!("{ROOT}/{id}/suspend"),
        Some(admin),
        json!({"expected_version":current,"duration_days":1,"reason":"Atomic restriction"}),
    )
    .await;
    sqlx::query("ALTER TABLE account_restrict_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    assert_eq!(before, snapshot(pool, id).await);
    let suspended=request(api,"POST",&format!("{ROOT}/{id}/suspend"),Some(admin),json!({"expected_version":current,"duration_days":1,"reason":"Recorded nullable prior activation"})).await;
    assert_eq!(suspended.0, StatusCode::OK, "{suspended:?}");
    assert_eq!(suspended.1["restriction"]["status"], "suspended");
    sqlx::query("UPDATE user_status SET ban_expires_at=NOW()-INTERVAL '1 second' WHERE user_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let before = snapshot(pool, id).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO account_expiry_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed =
        revolution_api::queue::worker::process_account_expirations(&revolution_api::db::Database {
            pool: pool.clone(),
        })
        .await;
    sqlx::query("ALTER TABLE account_expiry_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert!(failed.is_err());
    assert_eq!(
        before,
        snapshot(pool, id).await,
        "Expiry cannot activate before its journal/audit commits"
    );
    revolution_api::queue::worker::process_account_expirations(&revolution_api::db::Database {
        pool: pool.clone(),
    })
    .await
    .unwrap();
    let restored:(Option<bool>,String,i64)=sqlx::query_as("SELECT u.is_active,s.status,u.account_version FROM users u JOIN user_status s ON s.user_id=u.id WHERE u.id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(
        restored.0, None,
        "Actual captured nullable activation is restored without guessing true"
    );
    assert_eq!(restored.1, "active");
    let banned=request(api,"POST",&format!("{ROOT}/{id}/ban"),Some(admin),json!({"expected_version":restored.2,"duration_days":1,"reason":"Independent authority supersedes expiry"})).await;
    assert_eq!(banned.0, StatusCode::OK, "{banned:?}");
    sqlx::query("UPDATE users SET password_hash='changed-by-owned-authority-fixture' WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE user_status SET ban_expires_at=NOW()-INTERVAL '1 second' WHERE user_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    revolution_api::queue::worker::process_account_expirations(&revolution_api::db::Database {
        pool: pool.clone(),
    })
    .await
    .unwrap();
    let review:(Option<bool>,String,i64)=sqlx::query_as("SELECT u.is_active,s.status,u.account_version FROM users u JOIN user_status s ON s.user_id=u.id WHERE u.id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(review.0, Some(false));
    assert_eq!(review.1, "expired_requires_review");
    let before = snapshot(pool, id).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO account_unban_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "POST",
        &format!("{ROOT}/{id}/unban"),
        Some(admin),
        json!({"expected_version":review.2}),
    )
    .await;
    sqlx::query("ALTER TABLE account_unban_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    assert_eq!(before, snapshot(pool, id).await);
    let unbanned = request(
        api,
        "POST",
        &format!("{ROOT}/{id}/unban"),
        Some(admin),
        json!({"expected_version":review.2}),
    )
    .await;
    assert_eq!(unbanned.0, StatusCode::OK, "{unbanned:?}");
    assert_eq!(unbanned.1["account"]["is_active"], true);
    // Real legacy restrictions contain no captured prior activation or applied epoch.
    sqlx::query("UPDATE users SET is_active=false WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE user_status SET status='banned',restriction_prior_recorded=false,restriction_original_is_active=NULL,restriction_auth_version=NULL,ban_expires_at=NOW()-INTERVAL '1 second' WHERE user_id=$1").bind(id).execute(pool).await.unwrap();
    revolution_api::queue::worker::process_account_expirations(&revolution_api::db::Database {
        pool: pool.clone(),
    })
    .await
    .unwrap();
    let legacy:(Option<bool>,String,i64)=sqlx::query_as("SELECT u.is_active,s.status,u.account_version FROM users u JOIN user_status s ON s.user_id=u.id WHERE u.id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(legacy.0, Some(false));
    assert_eq!(
        legacy.1, "expired_requires_review",
        "A missing historical prior state never fabricates reactivation"
    );
    let removed = remove(api, id, legacy.2, super_token).await;
    assert_eq!(removed.0, StatusCode::OK, "{removed:?}");
    assert_eq!(removed.1["local_memberships_cancelled"], 1);
    assert_eq!(removed.1["provider_billing"], "unchanged");
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM membership_plans WHERE id=$1")
        .bind(plan)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_admin_creations WHERE request_id=$1")
        .bind(key)
        .execute(pool)
        .await
        .unwrap();
}
