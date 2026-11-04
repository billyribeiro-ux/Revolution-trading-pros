//! Controlled read-only provider adapter; never contacts a live integration.
use super::request;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode, Uri},
    routing::any,
};
use serde_json::{Value, json};
use sha2::Digest;
use sqlx::PgPool;
fn digest(value: &str) -> String {
    hex::encode(sha2::Sha256::digest(value.as_bytes()))
}
use std::sync::{Arc, Mutex};
#[derive(Clone, Default)]
struct Adapter {
    requests: Arc<Mutex<Vec<String>>>,
    pause: Arc<std::sync::atomic::AtomicBool>,
    started: Arc<tokio::sync::Notify>,
    release: Arc<tokio::sync::Notify>,
}
async fn provider(
    State(adapter): State<Adapter>,
    headers: HeaderMap,
    uri: Uri,
) -> (StatusCode, Json<Value>) {
    adapter.requests.lock().unwrap().push(uri.path().into());
    let credential = headers
        .get("authorization")
        .or_else(|| headers.get("accesskey"))
        .and_then(|value| value.to_str().ok())
        .unwrap_or("");
    if credential.contains("fixture-live-wait")
        && adapter.pause.load(std::sync::atomic::Ordering::Acquire)
    {
        adapter.started.notify_one();
        adapter.release.notified().await;
    }
    if credential.contains("fixture-reject") {
        return (
            StatusCode::UNAUTHORIZED,
            Json(json!({"error":"fixture rejection"})),
        );
    }
    if credential.contains("fixture-unavailable") {
        return (StatusCode::SERVICE_UNAVAILABLE, Json(json!({})));
    }
    if credential.contains("fixture-malformed") {
        return (StatusCode::OK, Json(json!({"unrelated":true})));
    }
    if credential.contains("fixture-invalid-large") {
        return (
            StatusCode::OK,
            Json(json!({"object":"list","data":"invalid","padding":"a".repeat(65537)})),
        );
    }
    if credential.contains("fixture-bunny-large") {
        return (StatusCode::OK,Json(Value::Array((0..2000).map(|id|json!({"ObjectName":format!("fixture-{id}.png"),"IsDirectory":false,"metadata":"x".repeat(100)})).collect())));
    }
    if credential.contains("fixture-large") {
        return (
            StatusCode::OK,
            Json(
                json!({"object":"list","data":(0..2000).map(|id|json!({"id":format!("fixture-model-{id}"),"object":"model","metadata":"x".repeat(100)})).collect::<Vec<_>>()}),
            ),
        );
    }
    tokio::time::sleep(std::time::Duration::from_millis(4)).await;
    let payload = match uri.path() {
        "/stripe/v1/balance" => {
            json!({"object":"balance","available":[],"livemode":credential.contains("fixture-live")})
        }
        "/sendgrid/v3/scopes" => {
            json!({"scopes":if credential.contains("fixture-no-scope"){vec!["mail.read"]}else{vec!["mail.send"]}})
        }
        "/openai/v1/models" => json!({"object":"list","data":[]}),
        "/bunny_cdn/fixture-zone/" => json!([]),
        "/bunny_stream/library/123/videos" => {
            assert_eq!(uri.query(), Some("page=1&itemsPerPage=10"));
            json!({"items":[{"guid":"fixture-video","videoLibraryId":if credential.contains("fixture-wrong-library"){999}else{123}}],"totalItems":1,"currentPage":1,"itemsPerPage":10})
        }
        "/meilisearch/version" => json!({"pkgVersion":"fixture-1"}),
        "/google_analytics/v1beta/properties/123/dataStreams/456" => {
            json!({"webStreamData":{"measurementId":"G-FIXTURE"}})
        }
        "/fixture-bucket" | "/fixture-bucket/" => {
            assert!(
                credential.starts_with("AWS4-HMAC-SHA256 "),
                "Actual SDK signing is required"
            );
            assert!(credential.contains("Credential=fixture-access/"));
            json!({})
        }
        _ => {
            return (
                StatusCode::NOT_FOUND,
                Json(json!({"error":"unexpected adapter path"})),
            );
        }
    };
    (StatusCode::OK, Json(payload))
}
pub(super) async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    assert_eq!(
        std::env::var("CONNECTION_TEST_BASE_URL").as_deref(),
        Ok("http://127.0.0.1:19021"),
        "Only the explicit disposable adapter may be used"
    );
    assert_eq!(std::env::var("ENVIRONMENT").as_deref(), Ok("development"));
    let adapter = Adapter::default();
    let listener = tokio::net::TcpListener::bind("127.0.0.1:19021")
        .await
        .unwrap();
    let app = Router::new()
        .fallback(any(provider))
        .with_state(adapter.clone());
    let server = tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    let id:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES('connection-super@example.test','Connection proof','not-a-login-password','super_admin',NOW()) RETURNING id").fetch_one(pool).await.unwrap();
    let token =
        revolution_api::utils::create_jwt(id, &std::env::var("JWT_SECRET").unwrap(), 1).unwrap();
    for credential in [None, Some(&tokens[0]), Some(&tokens[1])] {
        let result = request(
            api,
            "POST",
            "/api/admin/connections/sendgrid/connect",
            credential.map(String::as_str),
            json!({"credentials":{"api_key":"fixture-good","from_email":"sender@example.test"}}),
        )
        .await;
        assert_eq!(
            result.0,
            if credential.is_none() {
                StatusCode::UNAUTHORIZED
            } else {
                StatusCode::FORBIDDEN
            },
            "{result:?}"
        );
    }

    for payload in [
        json!({"credentials":{"api_key":"fixture-good","from_email":"sender@example.test","unknown_secret":"fixture"}}),
        json!({"credentials":{"api_key":"fixture-good","from_email":"not-an-address"}}),
        json!({"credentials":{"api_key":"fixture-good","from_email":"sender@example.test"},"settings":[]}),
    ] {
        let before = adapter.requests.lock().unwrap().len();
        let invalid = request(
            api,
            "POST",
            "/api/admin/connections/sendgrid/connect",
            Some(&token),
            payload,
        )
        .await;
        assert_eq!(invalid.0, StatusCode::BAD_REQUEST, "{invalid:?}");
        assert_eq!(adapter.requests.lock().unwrap().len(), before);
    }
    let definitions = request(
        api,
        "GET",
        "/api/admin/connections?category=Payment",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(definitions.0, StatusCode::OK, "{definitions:?}");
    assert_eq!(definitions.1["connections"].as_array().unwrap().len(), 1);
    assert_eq!(definitions.1["connections"][0]["is_connected"], false);
    for (key, credentials) in [
        (
            "stripe",
            json!({"secret_key":"fixture-live","publishable_key":"fixture-public","webhook_secret":"whsec-fixture-live"}),
        ),
        (
            "sendgrid",
            json!({"api_key":"fixture-good","from_email":"sender@example.test"}),
        ),
        ("openai", json!({"api_key":"fixture-good"})),
        (
            "bunny_cdn",
            json!({"api_key":"fixture-good","storage_zone":"fixture-zone"}),
        ),
        (
            "meilisearch",
            json!({"api_key":"fixture-good","host":"http://127.0.0.1:19021"}),
        ),
        (
            "google_analytics",
            json!({"measurement_id":"G-FIXTURE","access_token":"fixture-good","property_id":"123","stream_id":"456"}),
        ),
        (
            "cloudflare_r2",
            json!({"account_id":"00000000000000000000000000000000","access_key_id":"fixture-access","secret_access_key":"fixture-secret","bucket_name":"fixture-bucket"}),
        ),
    ] {
        let result = request(
            api,
            "POST",
            "/api/admin/connections",
            Some(&token),
            json!({"key":key,"credentials":credentials,"environment":"production"}),
        )
        .await;
        assert_eq!(result.0, StatusCode::OK, "{result:?}");
        assert_eq!(
            result.1["verified"],
            true,
            "{key}: {result:?}; actual adapter paths: {:?}",
            adapter.requests.lock().unwrap()
        );
        assert!(
            result.1["verification"]["latency_ms"].as_i64().unwrap() >= 4,
            "Measured adapter delay"
        );
        let stored:(String,String)=sqlx::query_as("SELECT status,credentials_encrypted FROM service_connections WHERE service_key=$1 AND environment='production'").bind(key).fetch_one(pool).await.unwrap();
        assert_eq!(stored.0, "connected");
        assert!(stored.1.starts_with("v1:"));
        assert!(!stored.1.contains("fixture-good"));
    }

    let runtime = request(
        api,
        "GET",
        "/fixture/credential-source?environment=production",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(runtime.1["source"], "db");
    assert_eq!(runtime.1["secret_digest"], digest("fixture-live"));
    assert_eq!(
        runtime.1["webhook_digest"],
        digest("whsec-fixture-live"),
        "Encrypted webhook secret must reach the actual runtime resolver"
    );
    let development = request(
        api,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(development.1["environment"], "sandbox");
    assert_eq!(development.1["source"], "env");
    assert_eq!(
        development.1["secret_digest"],
        digest("sk_test_environment_fixture")
    );
    let rotate=request(api,"POST","/api/admin/connections/stripe/connect",Some(&token),json!({"credentials":{"secret_key":"fixture-live-rotated","publishable_key":"fixture-public"},"environment":"production"})).await;
    assert_eq!(rotate.1["verified"], true, "{rotate:?}");
    let rotated = request(
        api,
        "GET",
        "/fixture/credential-source?environment=production",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(
        rotated.1["secret_digest"],
        digest("fixture-live-rotated"),
        "Rotation invalidates the actual API state's cached keys"
    );
    for (credential, expected) in [
        ("fixture-reject", "rejected"),
        ("fixture-unavailable", "unavailable"),
        ("fixture-malformed", "rejected"),
        ("fixture-invalid-large", "unavailable"),
        ("fixture-no-scope", "rejected"),
    ] {
        let key = if credential == "fixture-invalid-large" {
            "openai"
        } else {
            "sendgrid"
        };
        let result = request(
            api,
            "POST",
            &format!("/api/admin/connections/{key}/test"),
            Some(&token),
            json!({"credentials":{"api_key":credential}}),
        )
        .await;
        assert_eq!(result.0, StatusCode::OK, "{result:?}");
        assert_eq!(result.1["success"], false, "{result:?}");
        assert_eq!(result.1["status"], expected);
        assert_eq!(result.1["persisted"], false);
    }

    for (key, credentials) in [
        ("openai", json!({"api_key":"fixture-large"})),
        (
            "bunny_cdn",
            json!({"api_key":"fixture-bunny-large","storage_zone":"fixture-zone","storage_hostname":"ny.storage.bunnycdn.com"}),
        ),
        (
            "bunny_cdn",
            json!({"api_key":"fixture-good","storage_zone":"fixture-zone","library_id":"123","stream_api_key":"fixture-good","storage_hostname":"syd.storage.bunnycdn.com"}),
        ),
    ] {
        let verified = request(
            api,
            "POST",
            &format!("/api/admin/connections/{key}/test"),
            Some(&token),
            json!({"credentials":credentials}),
        )
        .await;
        assert_eq!(verified.0, StatusCode::OK, "{verified:?}");
        assert_eq!(
            verified.1["success"], true,
            "Valid large/regional/configured-library response must pass: {verified:?}"
        );
    }
    for credentials in [
        json!({"api_key":"fixture-good","storage_zone":"fixture-zone","library_id":"123"}),
        json!({"api_key":"fixture-good","storage_zone":"fixture-zone","stream_api_key":"fixture-good"}),
        json!({"api_key":"fixture-good","storage_zone":"fixture-zone","storage_hostname":"127.0.0.1"}),
        json!({"api_key":"fixture-good","storage_zone":"fixture-zone","storage_hostname":"storage.bunnycdn.com.evil.example.test"}),
    ] {
        let before = adapter.requests.lock().unwrap().len();
        let invalid = request(
            api,
            "POST",
            "/api/admin/connections/bunny_cdn/test",
            Some(&token),
            json!({"credentials":credentials}),
        )
        .await;
        assert_eq!(invalid.1["success"], false, "{invalid:?}");
        assert_eq!(
            adapter.requests.lock().unwrap().len(),
            before,
            "Invalid settings must be rejected before outbound HTTP"
        );
    }
    for stream_key in ["fixture-reject", "fixture-wrong-library"] {
        let denied=request(api,"POST","/api/admin/connections/bunny_cdn/test",Some(&token),json!({"credentials":{"api_key":"fixture-good","storage_zone":"fixture-zone","library_id":"123","stream_api_key":stream_key}})).await;
        assert_eq!(
            denied.1["success"], false,
            "Storage success cannot fabricate Stream success: {denied:?}"
        );
    }
    let valid = request(
        api,
        "POST",
        "/api/admin/connections/sendgrid/test",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(valid.1["success"], true, "{valid:?}");
    assert_eq!(valid.1["persisted"], true);
    let flat = request(
        api,
        "POST",
        "/api/admin/connections/sendgrid/test",
        Some(&token),
        json!({"api_key":"fixture-good"}),
    )
    .await;
    assert_eq!(flat.1["success"], true, "Legacy flat payload: {flat:?}");
    let mismatch = request(
        api,
        "POST",
        "/api/admin/connections/stripe/test",
        Some(&token),
        json!({"credentials":{"secret_key":"fixture-good"},"environment":"production"}),
    )
    .await;
    assert_eq!(
        mismatch.1["success"], false,
        "Sandbox identity must not verify production"
    );
    let sandbox=request(api,"POST","/api/admin/connections/stripe/connect",Some(&token),json!({"credentials":{"secret_key":"fixture-good","publishable_key":"fixture-public"},"environment":"sandbox"})).await;
    assert_eq!(sandbox.1["verified"], true, "{sandbox:?}");
    let sandbox_runtime = request(
        api,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(sandbox_runtime.1["source"], "db");
    assert_eq!(sandbox_runtime.1["secret_digest"], digest("fixture-good"));

    let runtime_view = request(
        api,
        "GET",
        "/api/admin/connections?environment=runtime",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(runtime_view.0, StatusCode::OK, "{runtime_view:?}");
    assert_eq!(runtime_view.1["environment"], "sandbox");
    assert_eq!(runtime_view.1["runtime_environment"], "sandbox");
    let runtime_stripe = runtime_view.1["connections"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["key"] == "stripe")
        .unwrap();
    assert_eq!(runtime_stripe["connection"]["environment"], "sandbox");
    assert_eq!(runtime_stripe["is_connected"], true);
    let production_view = request(
        api,
        "GET",
        "/api/admin/connections/stripe?environment=production",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(production_view.0, StatusCode::OK, "{production_view:?}");
    assert_eq!(
        production_view.1["data"]["connection"]["environment"],
        "production"
    );
    assert_eq!(production_view.1["data"]["is_connected"], true);

    let disconnected = request(
        api,
        "POST",
        "/api/admin/connections/stripe/disconnect?environment=sandbox",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(disconnected.0, StatusCode::OK, "{disconnected:?}");
    let revoked_runtime = request(
        api,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(revoked_runtime.1["source"], "blocked");
    assert!(
        revoked_runtime.1["secret_digest"].is_null(),
        "Persisted disconnect must not reactivate valid environment keys"
    );

    let production:String=sqlx::query_scalar("SELECT status FROM service_connections WHERE service_key='stripe' AND environment='production'").fetch_one(pool).await.unwrap();
    assert_eq!(production, "connected");
    let sandbox_deleted = request(
        api,
        "DELETE",
        "/api/admin/connections/stripe?environment=sandbox",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(sandbox_deleted.0, StatusCode::OK, "{sandbox_deleted:?}");
    let fresh = super::app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let deleted_runtime = request(
        &fresh,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(deleted_runtime.1["source"], "blocked");
    assert!(
        deleted_runtime.1["secret_digest"].is_null(),
        "Deletion must remain revoked after recreating API state"
    );
    let failed_reconnect=request(api,"POST","/api/admin/connections/stripe/connect",Some(&token),json!({"credentials":{"secret_key":"fixture-reject","publishable_key":"fixture-public"},"environment":"sandbox"})).await;
    assert_eq!(failed_reconnect.1["verified"], false);
    let blocked = request(
        api,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(blocked.1["source"], "blocked");
    let reconnect=request(api,"POST","/api/admin/connections/stripe/connect",Some(&token),json!({"credentials":{"secret_key":"fixture-good","publishable_key":"fixture-public"},"environment":"sandbox"})).await;
    assert_eq!(reconnect.1["verified"], true);
    let restored_runtime = request(
        api,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(restored_runtime.1["source"], "db");
    assert_eq!(restored_runtime.1["secret_digest"], digest("fixture-good"));

    let failed_save=request(api,"POST","/api/admin/connections/sendgrid/connect",Some(&token),json!({"credentials":{"api_key":"fixture-reject","from_email":"sender@example.test"},"environment":"production"})).await;
    assert_eq!(failed_save.0, StatusCode::OK, "{failed_save:?}");
    assert_eq!(failed_save.1["verified"], false);
    assert_eq!(failed_save.1["connection"]["status"], "error");
    let audit_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM admin_audit_logs WHERE action='connection.verification'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert!(audit_count >= 9);
    let audits:String=sqlx::query_scalar("SELECT COALESCE(string_agg(new_value,''),'') FROM admin_audit_logs WHERE action LIKE 'connection.%'").fetch_one(pool).await.unwrap();
    assert!(!audits.contains("fixture-reject"));
    assert!(!audits.contains("fixture-secret"));
    assert!(
        adapter.requests.lock().unwrap().len() >= 16,
        "Actual provider requests required"
    );
    let restored=request(api,"POST","/api/admin/connections/sendgrid/connect",Some(&token),json!({"credentials":{"api_key":"fixture-good","from_email":"sender@example.test"},"environment":"production"})).await;
    assert_eq!(restored.1["verified"], true, "{restored:?}");
    let connection_id:i64=sqlx::query_scalar("SELECT id FROM service_connections WHERE service_key='sendgrid' AND environment='production'").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO integration_webhooks(connection_id,name,url,events) VALUES($1,'Atomic revoke fixture','https://fixture.example.test/receiver','[]')").bind(connection_id).execute(pool).await.unwrap();
    sqlx::raw_sql("CREATE FUNCTION admin_move_connection_audit_failure() RETURNS trigger LANGUAGE plpgsql AS $$ BEGIN IF NEW.action IN ('connection.disconnected','connection.deleted') THEN RAISE EXCEPTION 'controlled audit outage'; END IF; RETURN NEW; END $$; CREATE TRIGGER admin_move_connection_audit_failure BEFORE INSERT ON admin_audit_logs FOR EACH ROW EXECUTE FUNCTION admin_move_connection_audit_failure();").execute(pool).await.unwrap();
    for (method, path) in [
        ("POST", "/api/admin/connections/sendgrid/disconnect"),
        ("DELETE", "/api/admin/connections/sendgrid"),
    ] {
        let failure = request(api, method, path, Some(&token), Value::Null).await;
        assert_eq!(failure.0, StatusCode::INTERNAL_SERVER_ERROR, "{failure:?}");
        let state: (String, bool) = sqlx::query_as(
            "SELECT status,credentials_encrypted IS NOT NULL FROM service_connections WHERE id=$1",
        )
        .bind(connection_id)
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(
            state,
            ("connected".into(), true),
            "Failed audit must roll back credential revocation"
        );
        let active: bool =
            sqlx::query_scalar("SELECT is_active FROM integration_webhooks WHERE connection_id=$1")
                .bind(connection_id)
                .fetch_one(pool)
                .await
                .unwrap();
        assert!(active, "Dependent webhook mutation must roll back too");
        let barriers: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM service_connection_revocations WHERE service_key='sendgrid'",
        )
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(
            barriers, 0,
            "Audit failure must roll back revocation barriers too"
        );
    }
    sqlx::raw_sql("DROP TRIGGER admin_move_connection_audit_failure ON admin_audit_logs; DROP FUNCTION admin_move_connection_audit_failure();").execute(pool).await.unwrap();
    let revoked = request(
        api,
        "POST",
        "/api/admin/connections/sendgrid/disconnect",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(revoked.0, StatusCode::OK, "{revoked:?}");
    let active: bool =
        sqlx::query_scalar("SELECT is_active FROM integration_webhooks WHERE connection_id=$1")
            .bind(connection_id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(!active);
    let deleted = request(
        api,
        "DELETE",
        "/api/admin/connections/sendgrid",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
    let hooks: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM integration_webhooks WHERE connection_id=$1")
            .bind(connection_id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(hooks, 0);

    for disconnect in [false, true] {
        let waiting=request(api,"POST","/api/admin/connections/stripe/connect",Some(&token),json!({"credentials":{"secret_key":"fixture-live-wait","publishable_key":"fixture-public"},"environment":"production"})).await;
        assert_eq!(waiting.1["verified"], true, "{waiting:?}");
        adapter
            .pause
            .store(true, std::sync::atomic::Ordering::Release);
        let api_clone = api.clone();
        let token_clone = token.clone();
        let stale = tokio::spawn(async move {
            request(
                &api_clone,
                "POST",
                "/api/admin/connections/stripe/test",
                Some(&token_clone),
                Value::Null,
            )
            .await
        });
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            adapter.started.notified(),
        )
        .await
        .unwrap();
        let change = if disconnect {
            request(
                api,
                "POST",
                "/api/admin/connections/stripe/disconnect",
                Some(&token),
                Value::Null,
            )
            .await
        } else {
            request(api,"POST","/api/admin/connections/stripe/connect",Some(&token),json!({"credentials":{"secret_key":"fixture-live-after-wait","publishable_key":"fixture-public"},"environment":"production"})).await
        };
        assert_eq!(change.0, StatusCode::OK, "{change:?}");
        adapter
            .pause
            .store(false, std::sync::atomic::Ordering::Release);
        adapter.release.notify_one();
        let stale = stale.await.unwrap();
        assert_eq!(
            stale.0,
            StatusCode::CONFLICT,
            "A delayed verification must not overwrite replacement/revocation: {stale:?}"
        );
        let runtime = request(
            api,
            "GET",
            "/fixture/credential-source?environment=production",
            Some(&token),
            Value::Null,
        )
        .await;
        if disconnect {
            assert_eq!(runtime.1["source"], "blocked");
            assert!(runtime.1["secret_digest"].is_null());
        } else {
            assert_eq!(
                runtime.1["secret_digest"],
                digest("fixture-live-after-wait")
            );
        }
    }
    // Reproduce legacy state in this disposable DB, then use entirely new API state.
    sqlx::query("DELETE FROM service_connections WHERE service_key='stripe'")
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM service_connection_revocations WHERE service_key='stripe'")
        .execute(pool)
        .await
        .unwrap();
    let config = revolution_api::config::Config::from_env().unwrap();
    let encrypted = revolution_api::utils::crypto::encrypt_map(
        &config.credentials_encryption_key,
        &std::collections::HashMap::from([("secret_key".into(), "legacy-unscoped-fixture".into())]),
    );
    sqlx::query("INSERT INTO service_connections(service_key,name,category,environment,credentials_encrypted,status,health_score,health_status,last_verified_at) VALUES('stripe','Legacy fixture','Payment',NULL,$1,'connected',100,'healthy',NOW())").bind(encrypted).execute(pool).await.unwrap();
    let fresh = super::app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    for scope in ["production", "development"] {
        let legacy = request(
            &fresh,
            "GET",
            &format!("/fixture/credential-source?environment={scope}"),
            Some(&token),
            Value::Null,
        )
        .await;
        assert_eq!(legacy.1["source"], "blocked");
        assert!(legacy.1["secret_digest"].is_null());
    }
    let legacy_view = request(
        &fresh,
        "GET",
        "/api/admin/connections/stripe",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(legacy_view.0, StatusCode::OK, "{legacy_view:?}");
    assert_eq!(legacy_view.1["data"]["status"], "pending");
    assert_eq!(legacy_view.1["data"]["is_connected"], false);
    assert!(legacy_view.1["data"]["connection"]["last_verified_at"].is_null());
    let legacy_delete = request(
        &fresh,
        "DELETE",
        "/api/admin/connections/stripe",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(legacy_delete.0, StatusCode::OK, "{legacy_delete:?}");
    let scopes: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM service_connection_revocations WHERE service_key='stripe'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        scopes, 2,
        "Legacy unscoped removal blocks both environments"
    );
    let fresh = super::app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let deleted = request(
        &fresh,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(deleted.1["source"], "blocked");
    assert!(deleted.1["secret_digest"].is_null());
    sqlx::query(
        "ALTER TABLE service_connection_revocations RENAME TO admin_move_revocations_unavailable",
    )
    .execute(pool)
    .await
    .unwrap();
    let fresh = super::app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let outage = request(
        &fresh,
        "GET",
        "/fixture/credential-source?environment=development",
        Some(&token),
        Value::Null,
    )
    .await;
    sqlx::query(
        "ALTER TABLE admin_move_revocations_unavailable RENAME TO service_connection_revocations",
    )
    .execute(pool)
    .await
    .unwrap();
    assert_eq!(outage.1["source"], "unavailable");
    assert!(
        outage.1["secret_digest"].is_null(),
        "Storage failure cannot fall back to a valid environment key"
    );
    server.abort();
}
