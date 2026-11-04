//! Disposable database proof for the admin completion work.
//! Run explicitly with ADMIN_VERIFY_DATABASE_URL pointing to an EMPTY database
//! whose name begins admin_move_verify. No dotenv files are loaded by this test.
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use revolution_api::{
    AppState,
    cache::{CacheInvalidator, CacheService},
    config::Config,
    db::Database,
    routes,
    services::Services,
};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

async fn app(db: Database) -> Router {
    let mut config = Config::from_env().expect("isolated test configuration");
    config.database_url = std::env::var("ADMIN_VERIFY_DATABASE_URL").unwrap();
    config.redis_url = "redis://127.0.0.1:1".into();
    config.meilisearch_host = "http://127.0.0.1:1".into();
    config.r2_endpoint = "http://127.0.0.1:1".into();
    config.r2_public_url = "http://127.0.0.1:1/media".into();
    let services = Services::new(&config).await.unwrap();
    let cache = CacheService::new(None, None);
    let state = AppState {
        db,
        services,
        config,
        event_broadcaster: routes::realtime::EventBroadcaster::new(),
        ws_manager: routes::websocket::WsConnectionManager::new(),
        cache: cache.clone(),
        cache_invalidator: CacheInvalidator::new(cache),
    };
    Router::new()
        .nest("/api", routes::api_router())
        .with_state(state)
}

async fn request(
    app: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Value,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| json!({"text": String::from_utf8_lossy(&bytes)}));
    (status, value)
}

#[tokio::test]
#[ignore = "requires an explicitly provisioned empty disposable database"]
async fn fresh_install_crm_persistence_media_states_and_migration_failure() {
    let url =
        std::env::var("ADMIN_VERIFY_DATABASE_URL").expect("ADMIN_VERIFY_DATABASE_URL is required");
    let parsed = reqwest::Url::parse(&url).unwrap();
    assert_eq!(parsed.host_str(), Some("127.0.0.1"), "local test DB only");
    assert!(
        parsed.path().starts_with("/admin_move_verify"),
        "disposable DB name required"
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='public'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 0, "fresh-install proof requires an empty database");
    let db = Database { pool: pool.clone() };
    db.migrate()
        .await
        .expect("first boot initializes baseline and forwards");
    db.migrate().await.expect("second boot is idempotent");
    let settings_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cms_site_settings")
        .fetch_one(&pool)
        .await
        .unwrap();
    let stages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_pipeline_stages")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(settings_count, 1);
    assert_eq!(stages, 6);
    let hash = revolution_api::utils::hash_password("OnlyForDisposableTest123!").unwrap();
    let mut tokens = Vec::new();
    for role in ["admin", "user"] {
        let id: i64 = sqlx::query_scalar("INSERT INTO users (email, name, password_hash, role, email_verified_at) VALUES ($1, 'Test', $2, $3, NOW()) RETURNING id")
            .bind(format!("{role}@admin-completion.test")).bind(&hash).bind(role).fetch_one(&pool).await.unwrap();
        tokens.push(
            revolution_api::utils::create_jwt(id, &std::env::var("JWT_SECRET").unwrap(), 1)
                .unwrap(),
        );
    }
    let api = app(db.clone()).await;
    let path = "/api/admin/crm/settings/double-optin";
    assert_eq!(
        request(&api, "GET", path, None, Value::Null).await.0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[1]),
            json!({"enabled": true})
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(&api, "GET", path, Some(&tokens[0]), Value::Null)
            .await
            .1["enabled"],
        false
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"enabled": true, "email_subject": "Fixture subject"})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"redirect_url": "javascript:alert(1)"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"enabled": "yes"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"unknown": true})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let ep = "/api/admin/crm/settings/email-preferences";
    assert_eq!(
        request(
            &api,
            "POST",
            ep,
            Some(&tokens[0]),
            json!({"title": "Fixture preferences", "show_tags": true})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "PUT",
            "/api/admin/crm/settings",
            Some(&tokens[0]),
            json!({"settings": {"fixture": 42}})
        )
        .await
        .0,
        StatusCode::OK
    );
    let batch = json!({"session_id":"fixture-session", "visitor_id":"fixture-visitor", "user_id":"999999", "events":[
        {"event_type":"page_view", "timestamp":chrono::Utc::now().timestamp_millis(), "page_url":"https://example.test/pricing?email=private@example.test#token"},
        {"event_type":"click", "timestamp":chrono::Utc::now().timestamp_millis(), "page_url":"https://example.test/pricing", "element_selector":"#subscribe", "event_value":"private form value", "coordinates":{"x":12,"y":24}},
        {"event_type":"scroll_depth", "timestamp":chrono::Utc::now().timestamp_millis(), "page_url":"https://example.test/pricing", "event_value":75}
    ]});
    let (status, ingested) =
        request(&api, "POST", "/api/behavior/events", None, batch.clone()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(ingested["persisted"], 3);
    let retry = request(&api, "POST", "/api/behavior/events", None, batch.clone())
        .await
        .1;
    assert_eq!(
        retry["persisted"], 0,
        "retries must not double-count events"
    );
    let mut invalid = batch.clone();
    invalid["events"][1]["event_type"] = json!("unknown-event");
    assert_eq!(
        request(&api, "POST", "/api/behavior/events", None, invalid)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let summary_path = "/api/analytics/behavior?period=7d";
    assert_eq!(
        request(&api, "GET", summary_path, None, Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(&api, "GET", summary_path, Some(&tokens[1]), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    let (status, summary) = request(&api, "GET", summary_path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(summary["total_sessions"], 1);
    assert_eq!(summary["avg_scroll_depth"], 75.0);
    assert_eq!(summary["top_clicks"][0]["element"], "#subscribe");
    let saved: Vec<(String, Option<f64>)> =
        sqlx::query_as("SELECT page_url,event_value FROM behavior_events ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(saved.len(), 3, "invalid batches must not partially persist");
    assert!(
        saved.iter().all(|(path, _)| path == "/pricing"),
        "strip sensitive query/fragment data"
    );
    assert!(saved[1].1.is_none(), "never store form input values");
    drop(api);
    // Rebuild services/router and reopen the DB pool, proving state does not
    // depend on process-local memory or the old connection pool.
    pool.close().await;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let db = Database { pool: pool.clone() };
    let api = app(db.clone()).await;
    let (status, doi) = request(&api, "GET", path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(doi["enabled"], true);
    assert_eq!(doi["email_subject"], "Fixture subject");
    assert_eq!(
        request(&api, "GET", ep, Some(&tokens[0]), Value::Null)
            .await
            .1["title"],
        "Fixture preferences"
    );
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/admin/crm/settings",
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .1["fixture"],
        42
    );
    let id: i64 = sqlx::query_scalar("INSERT INTO media (filename, path, size) VALUES ('fixture.txt', 'fixtures/test.txt', 12) RETURNING id")
        .fetch_one(&pool).await.unwrap();
    let detail = format!("/api/admin/media/{id}");
    assert_eq!(
        request(&api, "GET", &detail, Some(&tokens[0]), Value::Null)
            .await
            .1["data"]["scan_status"],
        "pending"
    );
    let (status, result) = request(
        &api,
        "POST",
        &format!("/api/admin/media/scan/{id}"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["success"], false);
    assert_eq!(result["data"]["scan_result"]["status"], "unavailable");
    assert_eq!(result["data"]["scan_result"]["is_clean"], false);
    let stored = request(&api, "GET", &detail, Some(&tokens[0]), Value::Null)
        .await
        .1;
    assert_eq!(stored["data"]["scan_status"], "unavailable");
    assert!(stored["data"]["scan_result"]["scanned_at"].is_null());
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/analytics/behavior?period=7d",
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .1["total_sessions"],
        1
    );
    for role in [
        "admin",
        "administrator",
        "developer",
        "super-admin",
        "super_admin",
        "superadmin",
    ] {
        sqlx::query("UPDATE users SET role=$1 WHERE email='admin@admin-completion.test'")
            .bind(role)
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(
            request(&api, "GET", path, Some(&tokens[0]), Value::Null)
                .await
                .0,
            StatusCode::OK,
            "admin role {role}"
        );
    }
    sqlx::query("UPDATE users SET role='editor' WHERE email='admin@admin-completion.test'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(&api, "GET", path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    sqlx::query("UPDATE users SET role='admin' WHERE email='admin@admin-completion.test'")
        .execute(&pool)
        .await
        .unwrap();
    // A corrupt migration checksum must fail readiness, never be ignored.
    let checksum: Vec<u8> =
        sqlx::query_scalar("SELECT checksum FROM _sqlx_migrations WHERE version=74")
            .fetch_one(&pool)
            .await
            .unwrap();
    sqlx::query("UPDATE _sqlx_migrations SET checksum = '\\x00'::bytea WHERE version=74")
        .execute(&pool)
        .await
        .unwrap();
    let failure = db
        .migrate()
        .await
        .expect_err("required migration mismatch must abort");
    assert!(failure.to_string().contains("startup aborted"));
    sqlx::query("UPDATE _sqlx_migrations SET checksum=$1 WHERE version=74")
        .bind(checksum)
        .execute(&pool)
        .await
        .unwrap();
    db.migrate().await.unwrap();
}
