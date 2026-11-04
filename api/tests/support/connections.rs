//! Controlled read-only provider adapter; never contacts a live integration.
use super::request;
use axum::{
    Json, Router,
    extract::State,
    http::{HeaderMap, StatusCode, Uri},
    routing::any,
};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::sync::{Arc, Mutex};
#[derive(Clone, Default)]
struct Adapter {
    requests: Arc<Mutex<Vec<String>>>,
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
    if credential.contains("fixture-large") {
        return (
            StatusCode::OK,
            Json(json!({"object":"list","data":[],"padding":"a".repeat(65537)})),
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
            json!({"secret_key":"fixture-live","publishable_key":"fixture-public"}),
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
    for (credential, expected) in [
        ("fixture-reject", "rejected"),
        ("fixture-unavailable", "unavailable"),
        ("fixture-malformed", "rejected"),
        ("fixture-large", "unavailable"),
        ("fixture-no-scope", "rejected"),
    ] {
        let key = if credential == "fixture-large" {
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
    let disconnected = request(
        api,
        "POST",
        "/api/admin/connections/stripe/disconnect?environment=sandbox",
        Some(&token),
        Value::Null,
    )
    .await;
    assert_eq!(disconnected.0, StatusCode::OK, "{disconnected:?}");
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
    server.abort();
}
