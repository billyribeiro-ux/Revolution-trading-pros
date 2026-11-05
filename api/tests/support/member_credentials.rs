//! Real password/MFA/link routes with a separately owned Redis and the loopback mail adapter.
use super::{Adapter, member_account_upgrade_proof};
use axum::{
    Router,
    body::{Body, to_bytes},
    extract::ConnectInfo,
    http::{Request, StatusCode},
};
use redis::AsyncCommands;
use revolution_api::{
    AppState,
    db::Database,
    routes,
    services::{account_links, mfa, redis::RedisService},
    utils,
};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::{
    net::SocketAddr,
    sync::atomic::{AtomicU32, Ordering},
};
use tower::ServiceExt;
use uuid::Uuid;
const PASSWORD: &str = "CedarHarbor8!VioletRidge";
const NEW_PASSWORD: &str = "CypressHill7!MapleBrook";
static PEER: AtomicU32 = AtomicU32::new(1);

async fn request(
    api: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    value: Value,
) -> (StatusCode, Value) {
    let n = PEER.fetch_add(1, Ordering::Relaxed);
    assert!(n < 60_000);
    let peer = SocketAddr::from(([127, 42, (n / 250) as u8, (n % 250 + 1) as u8], 45002));
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .extension(ConnectInfo(peer));
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let response = api
        .clone()
        .oneshot(builder.body(Body::from(value.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let body = to_bytes(response.into_body(), 2_000_000).await.unwrap();
    (
        status,
        serde_json::from_slice(&body)
            .unwrap_or_else(|_| json!({"text":String::from_utf8_lossy(&body)})),
    )
}
fn token(value: &Value) -> &str {
    value["access_token"].as_str().unwrap()
}
async fn login(
    api: &Router,
    email: &str,
    password: &str,
    code: Option<(&str, bool)>,
) -> (StatusCode, Value) {
    let mut body = json!({"email":email,"password":password});
    let path = if let Some((code, backup)) = code {
        body[if backup { "backup_code" } else { "mfa_code" }] = json!(code);
        "/api/auth/login/mfa"
    } else {
        "/api/auth/login"
    };
    request(api, "POST", path, None, body).await
}
async fn epoch(pool: &PgPool, id: i64) -> i64 {
    sqlx::query_scalar("SELECT auth_version FROM users WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}
async fn profile_version(api: &Router, signed: &str) -> i64 {
    let response = request(api, "GET", "/api/user/profile", Some(signed), Value::Null).await;
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    response.1["version"].as_i64().unwrap()
}
async fn snapshot(pool: &PgPool, id: i64) -> Value {
    sqlx::query_scalar("SELECT jsonb_build_object('account',(SELECT jsonb_build_object('auth_version',auth_version,'account_version',account_version,'mfa_enabled',mfa_enabled,'password_hash',password_hash,'email_verified_at',email_verified_at) FROM users WHERE id=$1),'secret',(SELECT to_jsonb(s) FROM user_mfa_secrets s WHERE user_id=$1),'attempts',(SELECT COALESCE(jsonb_agg(to_jsonb(a) ORDER BY id),'[]') FROM mfa_attempts a WHERE user_id=$1),'activity',(SELECT COALESCE(jsonb_agg(to_jsonb(a) ORDER BY id),'[]') FROM user_activity_log a WHERE user_id=$1),'audit',(SELECT COUNT(*) FROM crm_operational_logs WHERE context->>'member_id'=$1::text))")
        .bind(id).fetch_one(pool).await.unwrap()
}
async fn account(pool: &PgPool, email: &str) -> i64 {
    let hash = utils::hash_password_async(PASSWORD).await.unwrap();
    sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,email_verified_at) VALUES('Credential proof',$1,$2,'user',true,NOW() AT TIME ZONE 'UTC') RETURNING id")
        .bind(email).bind(hash).fetch_one(pool).await.unwrap()
}
fn current_code(secret: &str) -> String {
    mfa::generate_totp(
        secret,
        u64::try_from(chrono::Utc::now().timestamp()).unwrap(),
    )
    .unwrap()
}
fn invalid_code(secret: &str) -> String {
    let now = chrono::Utc::now().timestamp();
    let valid: Vec<String> = [-30, 0, 30]
        .into_iter()
        .map(|offset| mfa::generate_totp(secret, u64::try_from(now + offset).unwrap()).unwrap())
        .collect();
    (0..1_000_000)
        .map(|v| format!("{v:06}"))
        .find(|code| !valid.contains(code))
        .unwrap()
}
fn extract_link(adapter: &Adapter, email: &str, marker: &str) -> String {
    fn find(value: &Value, marker: &str) -> Option<String> {
        match value {
            Value::String(value) => value.find(marker).map(|start| {
                value[start + marker.len()..]
                    .chars()
                    .take_while(char::is_ascii_alphanumeric)
                    .collect()
            }),
            Value::Array(values) => values.iter().find_map(|v| find(v, marker)),
            Value::Object(values) => values.values().find_map(|v| find(v, marker)),
            _ => None,
        }
    }
    let requests = adapter.requests.lock().unwrap();
    requests
        .iter()
        .rev()
        .filter(|value| value["To"] == email)
        .find_map(|value| find(value, marker))
        .filter(|value| value.len() >= 32)
        .expect("Actual controlled provider request contains the new recipient-bound link")
}
async fn actual_state(pool: &PgPool) -> (AppState, redis::aio::MultiplexedConnection) {
    let url = std::env::var("ADMIN_VERIFY_OWNED_REDIS_URL")
        .expect("Run the isolated account-authority wrapper");
    let marker = std::env::var("ADMIN_VERIFY_OWNED_REDIS_TOKEN")
        .expect("An explicit owned Redis token is required");
    assert_eq!(marker.len(), 32);
    assert!(marker.bytes().all(|b| b.is_ascii_hexdigit()));
    let parsed = reqwest::Url::parse(&url).unwrap();
    assert_eq!(parsed.scheme(), "redis");
    assert_eq!(parsed.host_str(), Some("127.0.0.1"));
    assert_eq!(parsed.path(), "/0");
    assert!(
        parsed.username().is_empty()
            && parsed.password().is_none()
            && parsed.query().is_none()
            && parsed.fragment().is_none()
    );
    let port = parsed.port().unwrap();
    assert!(port >= 1024);
    assert_ne!(port, 16388, "Never use the running browser fixture Redis");
    let mut connection = redis::Client::open(url.as_str())
        .unwrap()
        .get_multiplexed_async_connection()
        .await
        .unwrap();
    let actual: String = connection.get("revolution:fixture:owner").await.unwrap();
    assert_eq!(actual, marker);
    let mut state = super::state(Database { pool: pool.clone() }, None).await;
    state.services.redis = Some(RedisService::new(&url).await.unwrap());
    state.config.redis_url = url;
    (state, connection)
}
fn build_api(state: AppState) -> Router {
    Router::new()
        .nest("/api", routes::api_router())
        .with_state(state)
}

pub async fn proof(pool: &PgPool, adapter: &Adapter) {
    let upgrade = member_account_upgrade_proof::proof(pool).await;
    assert_eq!(upgrade.duplicate_ids.len(), 2);
    assert_eq!(upgrade.duplicate_emails.len(), 2);
    assert!(
        upgrade
            .database_name
            .starts_with("admin_move_verify_upgrade_")
    );
    let (upgrade_state, _upgrade_redis) = actual_state(&upgrade.pool).await;
    let converted = revolution_api::services::account_mfa::upgrade_legacy_storage(&upgrade_state)
        .await
        .unwrap();
    assert!(converted >= 6);
    let remaining: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM user_mfa_secrets WHERE totp_secret NOT LIKE 'v1:%'",
    )
    .fetch_one(&upgrade.pool)
    .await
    .unwrap();
    assert_eq!(
        remaining, 0,
        "Real startup conversion removes valid legacy plaintext secret storage"
    );
    privilege_invariants(&build_api(upgrade_state), &upgrade.pool).await;
    upgrade.pool.close().await; // Preserve the owned DB and context for 119; never delete a legacy collision.
    let (state, mut redis) = actual_state(pool).await;
    let api = build_api(state.clone());
    let email = format!("mfa-{}@example.test", Uuid::new_v4());
    let id = account(pool, &email).await;
    let initial = login(&api, &email, PASSWORD, None).await;
    assert_eq!(initial.0, StatusCode::OK, "{initial:?}");
    let old = token(&initial.1).to_owned();
    assert_eq!(
        request(
            &api,
            "POST",
            "/api/auth/me/mfa/enable",
            Some(&old),
            json!({})
        )
        .await
        .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    let before = snapshot(pool, id).await;
    let wrong = request(
        &api,
        "POST",
        "/api/auth/me/mfa/enable",
        Some(&old),
        json!({"password":"WrongBounded9!Secret"}),
    )
    .await;
    assert_eq!(wrong.0, StatusCode::UNAUTHORIZED, "{wrong:?}");
    let after = snapshot(pool, id).await;
    assert_eq!(before["secret"], after["secret"]);
    assert_eq!(before["account"], after["account"]);
    assert_eq!(after["attempts"].as_array().unwrap().len(), 1);
    let setup = request(
        &api,
        "POST",
        "/api/auth/me/mfa/enable",
        Some(&old),
        json!({"password":PASSWORD}),
    )
    .await;
    assert_eq!(setup.0, StatusCode::OK, "{setup:?}");
    let pending = token(&setup.1["auth"]).to_owned();
    let secret = setup.1["secret"].as_str().unwrap();
    assert_eq!(secret.len(), 32);
    assert!(
        setup.1["qr_code"]
            .as_str()
            .unwrap()
            .starts_with("otpauth://totp/")
    );
    assert_eq!(setup.1["backup_codes"].as_array().unwrap().len(), 10);
    assert_eq!(
        request(&api, "GET", "/api/user/profile", Some(&old), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    let stored = snapshot(pool, id).await;
    assert!(
        stored["secret"]["totp_secret"]
            .as_str()
            .unwrap()
            .starts_with("v1:")
    );
    for value in setup.1["backup_codes"].as_array().unwrap() {
        assert!(
            !stored.to_string().contains(value.as_str().unwrap()),
            "Plain recovery code is never persisted"
        );
    }
    let rotated = request(
        &api,
        "POST",
        "/api/auth/me/mfa/enable",
        Some(&pending),
        json!({"password":PASSWORD}),
    )
    .await;
    assert_eq!(rotated.0, StatusCode::OK, "{rotated:?}");
    let pending = token(&rotated.1["auth"]).to_owned();
    let secret = rotated.1["secret"].as_str().unwrap();
    let codes = rotated.1["backup_codes"].as_array().unwrap();
    let before = snapshot(pool, id).await;
    let invalid = request(
        &api,
        "POST",
        "/api/auth/me/mfa/verify",
        Some(&pending),
        json!({"code":invalid_code(secret)}),
    )
    .await;
    assert_eq!(invalid.0, StatusCode::BAD_REQUEST, "{invalid:?}");
    assert_eq!(before["account"], snapshot(pool, id).await["account"]);
    let setup_code = current_code(secret);
    let enabled = request(
        &api,
        "POST",
        "/api/auth/me/mfa/verify",
        Some(&pending),
        json!({"code":setup_code}),
    )
    .await;
    assert_eq!(enabled.0, StatusCode::OK, "{enabled:?}");
    let enabled_token = token(&enabled.1["auth"]).to_owned();
    assert!(
        utils::verify_jwt(&enabled_token, &state.config.jwt_secret, "access")
            .unwrap()
            .mfa_assured
    );
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/user/profile",
            Some(&pending),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let password_only = login(&api, &email, PASSWORD, None).await;
    assert_eq!(password_only.0, StatusCode::OK, "{password_only:?}");
    assert_eq!(password_only.1["mfa_required"], true);
    for key in ["token", "access_token", "refresh_token", "user"] {
        assert!(password_only.1.get(key).is_none());
    }
    let reused = login(&api, &email, PASSWORD, Some((&setup_code, false))).await;
    assert_eq!(
        reused.0,
        StatusCode::UNAUTHORIZED,
        "TOTP setup proof cannot be replayed: {reused:?}"
    );
    let unassured =
        utils::create_jwt_versioned(id, &state.config.jwt_secret, 1, epoch(pool, id).await)
            .unwrap();
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/user/profile",
            Some(&unassured),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let unassured_refresh =
        utils::create_refresh_token_versioned(id, &state.config.jwt_secret, epoch(pool, id).await)
            .unwrap();
    assert_eq!(
        request(
            &api,
            "POST",
            "/api/auth/refresh",
            None,
            json!({"refresh_token":unassured_refresh})
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let before = snapshot(pool, id).await;
    assert_eq!(
        request(
            &api,
            "POST",
            "/api/auth/me/mfa/enable",
            Some(&enabled_token),
            json!({"password":PASSWORD})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        before,
        snapshot(pool, id).await,
        "Enabled secret cannot be overwritten by setup"
    );
    let version = epoch(pool, id).await;
    let code = codes[0].as_str().unwrap();
    let (one, two) = tokio::join!(
        login(&api, &email, PASSWORD, Some((code, true))),
        login(&api, &email, PASSWORD, Some((code, true)))
    );
    let mut statuses = [one.0.as_u16(), two.0.as_u16()];
    statuses.sort_unstable();
    assert_eq!(statuses, [200, 401], "{one:?} {two:?}");
    let active = if one.0 == StatusCode::OK {
        token(&one.1)
    } else {
        token(&two.1)
    }
    .to_owned();
    let value = snapshot(pool, id).await;
    assert_eq!(value["secret"]["backup_codes_used"], 1);
    assert_eq!(value["secret"]["backup_codes"].as_array().unwrap().len(), 9);
    assert_eq!(
        epoch(pool, id).await,
        version,
        "Consumption does not churn the credential epoch"
    );
    let (one, two) = tokio::join!(
        login(
            &api,
            &email,
            PASSWORD,
            Some((codes[1].as_str().unwrap(), true))
        ),
        login(
            &api,
            &email,
            PASSWORD,
            Some((codes[2].as_str().unwrap(), true))
        )
    );
    assert_eq!(one.0, StatusCode::OK, "{one:?}");
    assert_eq!(two.0, StatusCode::OK, "{two:?}");
    let value = snapshot(pool, id).await;
    assert_eq!(value["secret"]["backup_codes_used"], 3);
    assert_eq!(value["secret"]["backup_codes"].as_array().unwrap().len(), 7);
    let before = snapshot(pool, id).await;
    sqlx::raw_sql("CREATE FUNCTION fixture_mfa_audit_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.context->>'operation'='mfa_authenticated' THEN RAISE EXCEPTION 'owned MFA audit rollback fixture'; END IF; RETURN NEW; END $$; CREATE TRIGGER fixture_mfa_audit_failure BEFORE INSERT ON crm_operational_logs FOR EACH ROW EXECUTE FUNCTION fixture_mfa_audit_failure();").execute(pool).await.unwrap();
    let failed = login(
        &api,
        &email,
        PASSWORD,
        Some((codes[3].as_str().unwrap(), true)),
    )
    .await;
    sqlx::raw_sql("DROP TRIGGER fixture_mfa_audit_failure ON crm_operational_logs; DROP FUNCTION fixture_mfa_audit_failure();").execute(pool).await.unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    assert_eq!(
        before,
        snapshot(pool, id).await,
        "No code/attempt/journal commits when audit fails"
    );
    let retried = login(
        &api,
        &email,
        PASSWORD,
        Some((codes[3].as_str().unwrap(), true)),
    )
    .await;
    assert_eq!(retried.0, StatusCode::OK, "{retried:?}");
    let saved_version = profile_version(&api, &active).await;
    let changed=request(&api,"PUT","/api/user/password",Some(&active),json!({"expected_version":saved_version,"current_password":PASSWORD,"new_password":NEW_PASSWORD,"new_password_confirmation":NEW_PASSWORD})).await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["replace_session_tokens"], true);
    assert!(
        utils::verify_jwt(
            token(&changed.1["auth"]),
            &state.config.jwt_secret,
            "access"
        )
        .unwrap()
        .mfa_assured
    );
    assert_eq!(
        request(&api, "GET", "/api/user/profile", Some(&active), Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        login(&api, &email, NEW_PASSWORD, None).await.1["mfa_required"],
        true,
        "Changing password cannot bypass MFA"
    );
    let rotation_code = "C7A8D9E0F1A2B3C4D5E6F7A8B9C0D1E2";
    let rotation_hash = utils::hash_password_async(rotation_code).await.unwrap();
    let previous_epoch = epoch(pool, id).await;
    sqlx::query("UPDATE user_mfa_secrets SET backup_codes=$2 WHERE user_id=$1")
        .bind(id)
        .bind(json!([rotation_hash]))
        .execute(pool)
        .await
        .unwrap();
    assert!(
        epoch(pool, id).await > previous_epoch,
        "Rotation revokes even if generated_at was not changed"
    );
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/user/profile",
            Some(token(&changed.1["auth"])),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let authenticated = login(&api, &email, NEW_PASSWORD, Some((rotation_code, true))).await;
    assert_eq!(authenticated.0, StatusCode::OK, "{authenticated:?}");
    let disabled = request(
        &api,
        "POST",
        "/api/auth/me/mfa/disable",
        Some(token(&authenticated.1)),
        json!({"password":NEW_PASSWORD}),
    )
    .await;
    assert_eq!(disabled.0, StatusCode::OK, "{disabled:?}");
    assert_eq!(disabled.1["auth"]["user"]["mfa_enabled"], false);
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/user/profile",
            Some(token(&authenticated.1)),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let none: bool =
        sqlx::query_scalar("SELECT NOT EXISTS(SELECT 1 FROM user_mfa_secrets WHERE user_id=$1)")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(none);
    assert!(token(&login(&api, &email, NEW_PASSWORD, None).await.1).len() > 20);
    delayed_authority(&state, pool).await;
    links(&api, pool, adapter).await;
    mfa_rate(&api, pool).await;
    mfa_races(&api, pool).await;
    // Redis is exclusively owned and attested above. Erase its keys to prove it has no authority.
    let marker = std::env::var("ADMIN_VERIFY_OWNED_REDIS_TOKEN").unwrap();
    let _: () = redis::cmd("FLUSHDB").query_async(&mut redis).await.unwrap();
    let _: () = redis.set("revolution:fixture:owner", marker).await.unwrap();
    let fresh = token(&disabled.1["auth"]);
    let mut no_redis = state.clone();
    no_redis.services.redis = None;
    let restarted = build_api(no_redis);
    assert_eq!(
        request(
            &restarted,
            "GET",
            "/api/user/profile",
            Some(fresh),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        login(&restarted, &email, NEW_PASSWORD, None).await.0,
        StatusCode::SERVICE_UNAVAILABLE,
        "Password rate protection fails closed during Redis loss"
    );
    sqlx::query("UPDATE users SET role='moderator' WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            &restarted,
            "GET",
            "/api/user/profile",
            Some(fresh),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED,
        "Fresh DB role authority survives Redis loss/reconstructed app state"
    );
    let post_role =
        utils::create_jwt_versioned(id, &state.config.jwt_secret, 1, epoch(pool, id).await)
            .unwrap();
    let shutdown: redis::RedisResult<()> = redis::cmd("SHUTDOWN")
        .arg("NOSAVE")
        .query_async(&mut redis)
        .await;
    let _shutdown_result = shutdown;
    let ping: redis::RedisResult<String> = redis::cmd("PING").query_async(&mut redis).await;
    assert!(
        ping.is_err(),
        "The owned Redis process is actually unavailable"
    );
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/user/profile",
            Some(&post_role),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK,
        "Actual Redis process loss cannot replace DB account authority"
    );
    let unavailable = tokio::time::timeout(
        std::time::Duration::from_secs(6),
        login(&api, &email, NEW_PASSWORD, None),
    )
    .await
    .expect("The real Redis outage must produce a bounded fail-closed response");
    assert_eq!(
        unavailable.0,
        StatusCode::SERVICE_UNAVAILABLE,
        "{unavailable:?}"
    );
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

async fn links(api: &Router, pool: &PgPool, adapter: &Adapter) {
    let email = format!("registered-{}@example.test", Uuid::new_v4());
    let input = json!({"name":"Actual Registered Fixture","email":email,"password":PASSWORD,"password_confirmation":PASSWORD,"terms_accepted":true,"marketing_consent":false});
    let registered = request(api, "POST", "/api/auth/register", None, input).await;
    assert_eq!(registered.0, StatusCode::OK, "{registered:?}");
    assert_eq!(registered.1["requires_verification"], true);
    assert!(registered.1.get("access_token").is_none());
    let id: i64 = sqlx::query_scalar("SELECT id FROM users WHERE email=$1")
        .bind(&email)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(
        login(api, &email, PASSWORD, None).await.0,
        StatusCode::FORBIDDEN
    );
    let raw = extract_link(adapter, &email, "/verify-email?token=");
    let pending_epoch = epoch(pool, id).await;
    let pending_token =
        utils::create_jwt_versioned(id, &std::env::var("JWT_SECRET").unwrap(), 1, pending_epoch)
            .unwrap();
    let issued:Value=sqlx::query_scalar("SELECT jsonb_build_object('recipient',recipient,'version',issued_auth_version,'id_type',pg_typeof(id)::text) FROM email_verification_tokens WHERE user_id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(issued["id_type"], "uuid");
    assert_eq!(issued["recipient"], email);
    let verified = request(
        api,
        "GET",
        &format!("/api/auth/verify-email?token={raw}"),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(verified.0, StatusCode::OK, "{verified:?}");
    assert_eq!(verified.1["reauthentication_required"], true);
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/auth/verify-email?token={raw}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            api,
            "GET",
            "/api/user/profile",
            Some(&pending_token),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED,
        "An actual verification transition revokes prior unverified authority"
    );
    let login_result = login(api, &email, PASSWORD, None).await;
    assert_eq!(login_result.0, StatusCode::OK, "{login_result:?}");
    let profile = profile_version(api, token(&login_result.1)).await;
    let prior_epoch = epoch(pool, id).await;
    let (a, b) = tokio::join!(
        request(
            api,
            "PUT",
            "/api/user/profile",
            Some(token(&login_result.1)),
            json!({"expected_version":profile,"display_name":"Actual Registered Fixture","first_name":"First Concurrent"})
        ),
        request(
            api,
            "PUT",
            "/api/user/profile",
            Some(token(&login_result.1)),
            json!({"expected_version":profile,"display_name":"Actual Registered Fixture","first_name":"Second Concurrent"})
        )
    );
    let mut statuses = [a.0.as_u16(), b.0.as_u16()];
    statuses.sort_unstable();
    assert_eq!(
        statuses,
        [200, 409],
        "Metadata-only first-name writes use CAS even with the same display name: {a:?} {b:?}"
    );
    assert_eq!(
        profile_version(api, token(&login_result.1)).await,
        profile + 1
    );
    assert_eq!(
        epoch(pool, id).await,
        prior_epoch,
        "Names do not revoke auth authority"
    );
    sqlx::query(
        "UPDATE users SET last_login_at=NOW(),updated_at=NOW() AT TIME ZONE 'UTC' WHERE id=$1",
    )
    .bind(id)
    .execute(pool)
    .await
    .unwrap();
    assert_eq!(
        profile_version(api, token(&login_result.1)).await,
        profile + 1,
        "Operational observations do not churn the form version"
    );

    assert_eq!(
        request(
            api,
            "POST",
            "/api/auth/forgot-password",
            None,
            json!({"email":email})
        )
        .await
        .0,
        StatusCode::OK
    );
    let old_reset = extract_link(adapter, &email, "/reset-password?token=");
    let new_email = format!("updated-{}@example.test", Uuid::new_v4());
    let no_op_version = profile_version(api, token(&login_result.1)).await;
    let no_op = request(
        api,
        "PUT",
        "/api/user/profile",
        Some(token(&login_result.1)),
        json!({"expected_version":no_op_version,"display_name":"Actual Registered Fixture"}),
    )
    .await;
    assert_eq!(
        no_op.0,
        StatusCode::BAD_REQUEST,
        "Identical facts do not fabricate a change journal: {no_op:?}"
    );
    let saved_version = profile_version(api, token(&login_result.1)).await;
    let changed=request(api,"PUT","/api/user/profile",Some(token(&login_result.1)),json!({"expected_version":saved_version,"email":new_email,"current_password":PASSWORD,"first_name":"Recorded","last_name":"Owner","avatar_url":null})).await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["reauthentication_required"], true);
    assert!(changed.1["auth"].is_null());
    assert_eq!(changed.1["data"]["first_name"], "Recorded");
    assert_eq!(changed.1["data"]["email"], new_email);
    assert_eq!(
        request(
            api,
            "GET",
            "/api/user/profile",
            Some(token(&login_result.1)),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(request(api,"POST","/api/auth/reset-password",None,json!({"email":email,"token":old_reset,"password":NEW_PASSWORD,"password_confirmation":NEW_PASSWORD})).await.0,StatusCode::BAD_REQUEST,"The former recipient cannot reset the changed account");
    let new_verify = extract_link(adapter, &new_email, "/verify-email?token=");
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/auth/verify-email?token={new_verify}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            api,
            "POST",
            "/api/auth/forgot-password",
            None,
            json!({"email":new_email})
        )
        .await
        .0,
        StatusCode::OK
    );
    let reset = extract_link(adapter, &new_email, "/reset-password?token=");
    let before:Value=sqlx::query_scalar("SELECT jsonb_build_object('hash',password_hash,'epoch',auth_version) FROM users WHERE id=$1").bind(id).fetch_one(pool).await.unwrap();
    sqlx::raw_sql("CREATE FUNCTION fixture_reset_audit_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.context->>'operation'='password_reset' THEN RAISE EXCEPTION 'owned reset audit rollback fixture'; END IF; RETURN NEW; END $$; CREATE TRIGGER fixture_reset_audit_failure BEFORE INSERT ON crm_operational_logs FOR EACH ROW EXECUTE FUNCTION fixture_reset_audit_failure();").execute(pool).await.unwrap();
    let failed=request(api,"POST","/api/auth/reset-password",None,json!({"email":new_email,"token":reset,"password":NEW_PASSWORD,"password_confirmation":NEW_PASSWORD})).await;
    sqlx::raw_sql("DROP TRIGGER fixture_reset_audit_failure ON crm_operational_logs; DROP FUNCTION fixture_reset_audit_failure();").execute(pool).await.unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    let after:Value=sqlx::query_scalar("SELECT jsonb_build_object('hash',password_hash,'epoch',auth_version) FROM users WHERE id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(before, after);
    let hash = utils::hash_password_async(NEW_PASSWORD).await.unwrap();
    let (a, b) = tokio::join!(
        account_links::reset(pool, &reset, &new_email, hash.clone()),
        account_links::reset(pool, &reset, &new_email, hash)
    );
    assert_eq!(
        usize::from(a.is_ok()) + usize::from(b.is_ok()),
        1,
        "A reset capability consumes once under real concurrency"
    );
    let error = if let Err(error) = a {
        error
    } else {
        b.unwrap_err()
    };
    assert_eq!(error.status, StatusCode::BAD_REQUEST.as_str());
    assert_eq!(request(api,"POST","/api/auth/reset-password",None,json!({"email":new_email,"token":reset,"password":NEW_PASSWORD,"password_confirmation":NEW_PASSWORD})).await.0,StatusCode::BAD_REQUEST);
    assert_eq!(
        login(api, &new_email, NEW_PASSWORD, None).await.0,
        StatusCode::OK
    );
    let logs:Vec<Value>=sqlx::query_scalar("SELECT jsonb_build_object('user_id',user_id,'recipient',to_email,'model',model,'status',status,'message',provider_message_id) FROM email_logs WHERE user_id=$1 ORDER BY id").bind(id).fetch_all(pool).await.unwrap();
    assert!(logs.len() >= 5);
    for log in &logs {
        assert_eq!(log["user_id"], id);
        assert!(!log["model"].to_string().contains(&reset));
        assert!(!log["model"].to_string().contains(&new_verify));
    }
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
async fn mfa_rate(api: &Router, pool: &PgPool) {
    let email = format!("mfa-rate-{}@example.test", Uuid::new_v4());
    let id = account(pool, &email).await;
    let signed = login(api, &email, PASSWORD, None).await;
    let setup = request(
        api,
        "POST",
        "/api/auth/me/mfa/enable",
        Some(token(&signed.1)),
        json!({"password":PASSWORD}),
    )
    .await;
    assert_eq!(setup.0, StatusCode::OK, "{setup:?}");
    let key = setup.1["secret"].as_str().unwrap();
    let enabled = request(
        api,
        "POST",
        "/api/auth/me/mfa/verify",
        Some(token(&setup.1["auth"])),
        json!({"code":current_code(key)}),
    )
    .await;
    assert_eq!(enabled.0, StatusCode::OK, "{enabled:?}");
    for _ in 0..5 {
        assert_eq!(
            login(api, &email, PASSWORD, Some((&invalid_code(key), false)))
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
    }
    let blocked = login(
        api,
        &email,
        PASSWORD,
        Some((setup.1["backup_codes"][0].as_str().unwrap(), true)),
    )
    .await;
    assert_eq!(
        blocked.0,
        StatusCode::TOO_MANY_REQUESTS,
        "A correct recovery code cannot bypass the persisted failure limit: {blocked:?}"
    );
    assert_eq!(snapshot(pool, id).await["secret"]["backup_codes_used"], 0);
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

async fn privilege_invariants(api: &Router, pool: &PgPool) {
    async fn signed(pool: &PgPool, id: i64) -> String {
        utils::create_jwt_versioned(
            id,
            &std::env::var("JWT_SECRET").unwrap(),
            1,
            epoch(pool, id).await,
        )
        .unwrap()
    }
    let one:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,email_verified_at) VALUES('First isolated super','isolated-super-one@example.test','not-a-login-password','SuPeRaDmIn',true,NOW() AT TIME ZONE 'UTC') RETURNING id").fetch_one(pool).await.unwrap();
    let two:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,email_verified_at) VALUES('Second isolated super','isolated-super-two@example.test','not-a-login-password','super-admin',true,NOW() AT TIME ZONE 'UTC') RETURNING id").fetch_one(pool).await.unwrap();
    let configured_hash = utils::hash_password_async(PASSWORD).await.unwrap();
    let configured:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,is_active,email_verified_at) VALUES('Configured verified actor','configured-auth-proof@example.test',$1,'admin',true,NULL) RETURNING id").bind(configured_hash).fetch_one(pool).await.unwrap();
    let before_verified = signed(pool, configured).await;
    let rejected=request(api,"POST","/api/admin/member-management",Some(&before_verified),json!({"request_id":Uuid::new_v4(),"name":"Must not create before ownership","email":"not-yet-configured@example.test","password":PASSWORD})).await;
    assert_eq!(
        rejected.0,
        StatusCode::FORBIDDEN,
        "An unverified configured admin is not SuperAdmin"
    );
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .unwrap();
    let (raw, _, _) = account_links::verification_on(&mut tx, configured)
        .await
        .unwrap();
    tx.commit().await.unwrap();
    let verified = request(
        api,
        "GET",
        &format!("/api/auth/verify-email?token={raw}"),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(verified.0, StatusCode::OK, "{verified:?}");
    assert_eq!(
        request(
            api,
            "GET",
            "/api/user/profile",
            Some(&before_verified),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED,
        "The old unverified admin JWT cannot survive actual privilege elevation"
    );
    let properly_authenticated =
        login(api, "configured-auth-proof@example.test", PASSWORD, None).await;
    assert_eq!(
        properly_authenticated.0,
        StatusCode::OK,
        "{properly_authenticated:?}"
    );
    let configured_token = token(&properly_authenticated.1).to_owned();

    let before = snapshot(pool, configured).await;
    let self_email=request(api,"PUT","/api/user/profile",Some(&configured_token),json!({"expected_version":2,"email":"configured-self-loss@example.test","current_password":PASSWORD})).await;
    assert_eq!(self_email.0, StatusCode::FORBIDDEN, "{self_email:?}");
    let self_admin_email = request(
        api,
        "PUT",
        &format!("/api/admin/member-management/{configured}"),
        Some(&configured_token),
        json!({"expected_version":2,"email":"configured-self-loss@example.test"}),
    )
    .await;
    assert_eq!(
        self_admin_email.0,
        StatusCode::FORBIDDEN,
        "{self_admin_email:?}"
    );
    assert_eq!(
        before,
        snapshot(pool, configured).await,
        "Configured-only self authority loss requires a separate super administrator"
    );
    let one_token = signed(pool, one).await;
    let two_token = signed(pool, two).await;
    let own = request(
        api,
        "PUT",
        &format!("/api/admin/member-management/{one}"),
        Some(&one_token),
        json!({"expected_version":1,"role":"user"}),
    )
    .await;
    assert_eq!(
        own.0,
        StatusCode::BAD_REQUEST,
        "Self-demotion is not a second-party operation: {own:?}"
    );
    let two_path = format!("/api/admin/member-management/{two}");
    let one_path = format!("/api/admin/member-management/{one}");
    let (a, b) = tokio::join!(
        request(
            api,
            "PUT",
            &two_path,
            Some(&one_token),
            json!({"expected_version":1,"role":"user"})
        ),
        request(
            api,
            "PUT",
            &one_path,
            Some(&two_token),
            json!({"expected_version":1,"role":"user"})
        )
    );
    let mut statuses = [a.0.as_u16(), b.0.as_u16()];
    statuses.sort_unstable();
    assert_eq!(
        statuses,
        [200, 401],
        "Concurrent actors are revalidated under the write lock: {a:?} {b:?}"
    );
    let last:i64=sqlx::query_scalar("SELECT id FROM users WHERE LOWER(role) IN('super_admin','super-admin','superadmin') AND is_active IS DISTINCT FROM false").fetch_one(pool).await.unwrap();
    let denied = request(
        api,
        "PUT",
        &format!("/api/admin/member-management/{last}"),
        Some(&configured_token),
        json!({"expected_version":1,"is_active":false}),
    )
    .await;
    assert_eq!(
        denied.0,
        StatusCode::CONFLICT,
        "Even configured SuperAdmin authority cannot remove the last canonical super: {denied:?}"
    );
    let mail = format!("race-{}@example.test", Uuid::new_v4());
    let make = |email: String| json!({"request_id":Uuid::new_v4(),"name":"Email identity race","email":email,"password":PASSWORD,"role":"user","send_welcome_email":false});
    let (a, b) = tokio::join!(
        request(
            api,
            "POST",
            "/api/admin/member-management",
            Some(&configured_token),
            make(mail.clone())
        ),
        request(
            api,
            "POST",
            "/api/admin/member-management",
            Some(&configured_token),
            make(mail.to_uppercase())
        )
    );
    let mut statuses = [a.0.as_u16(), b.0.as_u16()];
    statuses.sort_unstable();
    assert_eq!(
        statuses,
        [200, 409],
        "Actual concurrent case variants cannot create ambiguous identities: {a:?} {b:?}"
    );
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM users WHERE LOWER(email)=$1")
        .bind(mail)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(count, 1);
}

async fn mfa_races(api: &Router, pool: &PgPool) {
    let email = format!("mfa-races-{}@example.test", Uuid::new_v4());
    let id = account(pool, &email).await;
    let signed = login(api, &email, PASSWORD, None).await;
    let setup = request(
        api,
        "POST",
        "/api/auth/me/mfa/enable",
        Some(token(&signed.1)),
        json!({"password":PASSWORD}),
    )
    .await;
    assert_eq!(setup.0, StatusCode::OK, "{setup:?}");
    let pending = token(&setup.1["auth"]);
    let code = current_code(setup.1["secret"].as_str().unwrap());
    let (a, b) = tokio::join!(
        request(
            api,
            "POST",
            "/api/auth/me/mfa/enable",
            Some(pending),
            json!({"password":PASSWORD})
        ),
        request(
            api,
            "POST",
            "/api/auth/me/mfa/verify",
            Some(pending),
            json!({"code":code})
        )
    );
    let mut statuses = [a.0.as_u16(), b.0.as_u16()];
    statuses.sort_unstable();
    assert_eq!(
        statuses,
        [200, 401],
        "Exact secret/epoch locking closes setup-enable races: {a:?} {b:?}"
    );
    let coherent:(bool,bool)=sqlx::query_as("SELECT u.mfa_enabled IS TRUE,s.totp_verified_at IS NOT NULL FROM users u JOIN user_mfa_secrets s ON s.user_id=u.id WHERE u.id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(coherent.0, coherent.1);
    let enabled = if b.0 == StatusCode::OK {
        b.1
    } else {
        let current = request(
            api,
            "POST",
            "/api/auth/me/mfa/verify",
            Some(token(&a.1["auth"])),
            json!({"code":current_code(a.1["secret"].as_str().unwrap())}),
        )
        .await;
        assert_eq!(current.0, StatusCode::OK, "{current:?}");
        current.1
    };
    sqlx::query("UPDATE user_mfa_secrets SET backup_codes=NULL WHERE user_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            "/api/user/profile",
            Some(token(&enabled["auth"])),
            Value::Null
        )
        .await
        .0,
        StatusCode::UNAUTHORIZED
    );
    let unavailable = login(api, &email, PASSWORD, Some(("ABCD-1234", true))).await;
    assert_eq!(
        unavailable.0,
        StatusCode::SERVICE_UNAVAILABLE,
        "Nullable/corrupt recovery data is never defaulted into a successful challenge: {unavailable:?}"
    );
    assert!(unavailable.1.get("access_token").is_none());
    sqlx::query("UPDATE user_mfa_secrets SET totp_verified_at=NULL WHERE user_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let mismatch = login(api, &email, PASSWORD, None).await;
    assert_eq!(
        mismatch.0,
        StatusCode::SERVICE_UNAVAILABLE,
        "Recorded enabled/verified disagreement is unavailable, not a no-MFA login: {mismatch:?}"
    );
    assert!(mismatch.1.get("access_token").is_none());
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}

static DELAYED_GATES: std::sync::LazyLock<(tokio::sync::Notify, tokio::sync::Notify)> =
    std::sync::LazyLock::new(|| (tokio::sync::Notify::new(), tokio::sync::Notify::new()));
async fn delayed_effect(
    axum::extract::State(state): axum::extract::State<AppState>,
    _user: revolution_api::models::User,
    axum::Extension(verified): axum::Extension<
        revolution_api::services::account_authority::VerifiedAuthority,
    >,
) -> Result<axum::Json<Value>, (StatusCode, &'static str)> {
    let mut tx = state.db.pool.begin().await.unwrap();
    // Deliberately initialize a transaction clock before waiting. Admission must use actual DB
    // wall time, not a NOW() value frozen before a long lock wait or observation.
    sqlx::query("SELECT NOW()").execute(&mut *tx).await.unwrap();
    DELAYED_GATES.0.notify_one();
    DELAYED_GATES.1.notified().await;
    sqlx::query("LOCK TABLE users IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *tx)
        .await
        .unwrap();
    sqlx::query("SELECT id FROM users WHERE id=$1 FOR NO KEY UPDATE")
        .bind(verified.subject)
        .execute(&mut *tx)
        .await
        .unwrap();
    revolution_api::services::account_authority::verified_on(&mut tx, &verified).await?;
    sqlx::query("INSERT INTO user_activity_log(user_id,action,details,created_at) VALUES($1,'fixture_delayed_authority','{}',NOW())").bind(verified.subject).execute(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    Ok(axum::Json(json!({"admitted":true})))
}
async fn delayed_authority(state: &AppState, pool: &PgPool) {
    let email = format!("delayed-authority-{}@example.test", Uuid::new_v4());
    let id = account(pool, &email).await;
    let signed =
        utils::create_jwt_versioned(id, &state.config.jwt_secret, 1, epoch(pool, id).await)
            .unwrap();
    let api = Router::new()
        .route(
            "/fixture/delayed-effect",
            axum::routing::post(delayed_effect),
        )
        .nest("/api", routes::api_router())
        .with_state(state.clone());
    let app = api.clone();
    let jwt = signed.clone();
    let first = tokio::spawn(async move {
        request(
            &app,
            "POST",
            "/fixture/delayed-effect",
            Some(&jwt),
            Value::Null,
        )
        .await
    });
    tokio::time::timeout(
        std::time::Duration::from_secs(3),
        DELAYED_GATES.0.notified(),
    )
    .await
    .unwrap();
    DELAYED_GATES.1.notify_one();
    assert_eq!(
        first.await.unwrap().0,
        StatusCode::OK,
        "Live captured authority admits the control effect"
    );
    let app = api.clone();
    let jwt = signed.clone();
    let pending = tokio::spawn(async move {
        request(
            &app,
            "POST",
            "/fixture/delayed-effect",
            Some(&jwt),
            Value::Null,
        )
        .await
    });
    tokio::time::timeout(
        std::time::Duration::from_secs(3),
        DELAYED_GATES.0.notified(),
    )
    .await
    .unwrap();
    let logout = request(&api, "POST", "/api/auth/logout", Some(&signed), json!({})).await;
    assert_eq!(logout.0, StatusCode::OK, "{logout:?}");
    DELAYED_GATES.1.notify_one();
    assert_eq!(
        pending.await.unwrap().0,
        StatusCode::UNAUTHORIZED,
        "Single-device logout cancels a delayed uncommitted admission"
    );
    let now = chrono::Utc::now().timestamp();
    let expiring=jsonwebtoken::encode(&jsonwebtoken::Header::default(),&json!({"sub":id,"iat":now,"exp":now+3,"token_type":"access","tv":epoch(pool,id).await,"av":1,"mfa":false,"jti":Uuid::new_v4()}),&jsonwebtoken::EncodingKey::from_secret(state.config.jwt_secret.as_bytes())).unwrap();
    let app = api.clone();
    let pending = tokio::spawn(async move {
        request(
            &app,
            "POST",
            "/fixture/delayed-effect",
            Some(&expiring),
            Value::Null,
        )
        .await
    });
    tokio::time::timeout(
        std::time::Duration::from_secs(3),
        DELAYED_GATES.0.notified(),
    )
    .await
    .unwrap();
    tokio::time::sleep(std::time::Duration::from_secs(4)).await;
    DELAYED_GATES.1.notify_one();
    assert_eq!(
        pending.await.unwrap().0,
        StatusCode::UNAUTHORIZED,
        "A transaction clock frozen before waiting must not extend bearer validity"
    );
    let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM user_activity_log WHERE user_id=$1 AND action='fixture_delayed_authority'").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(count, 1, "Denied delayed requests make no effect");
    sqlx::query("DELETE FROM users WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
}
