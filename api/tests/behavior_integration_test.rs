//! Isolated Postgres proof for consented behavior ingestion and admin reads.
//! Run with BEHAVIOR_VERIFY_DATABASE_URL pointing to an empty local database
//! whose name begins behavior_verify. No production database is accepted.
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

async fn request(
    api: &Router,
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
    let response = api
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
#[ignore = "requires an explicitly provisioned empty disposable database"]
async fn behavior_events_heatmaps_sessions_and_retention() {
    let url = std::env::var("BEHAVIOR_VERIFY_DATABASE_URL")
        .expect("BEHAVIOR_VERIFY_DATABASE_URL required");
    let parsed = reqwest::Url::parse(&url).unwrap();
    assert_eq!(parsed.host_str(), Some("127.0.0.1"));
    assert!(parsed.path().starts_with("/behavior_verify"));
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
    assert_eq!(count, 0, "database must be empty");
    let db = Database { pool: pool.clone() };
    db.migrate().await.unwrap();

    let mut config = Config::from_env().unwrap();
    config.database_url = url;
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
    let api = Router::new()
        .nest("/api", routes::api_router())
        .with_state(state);
    let hash = revolution_api::utils::hash_password("DisposableOnly123!").unwrap();
    let admin_id: i64 = sqlx::query_scalar("INSERT INTO users (email,name,password_hash,role,email_verified_at) VALUES ('admin@behavior.test','Test',$1,'admin',NOW()) RETURNING id").bind(&hash).fetch_one(&pool).await.unwrap();
    let user_id: i64 = sqlx::query_scalar("INSERT INTO users (email,name,password_hash,role,email_verified_at) VALUES ('user@behavior.test','Test',$1,'user',NOW()) RETURNING id").bind(&hash).fetch_one(&pool).await.unwrap();
    let secret = std::env::var("JWT_SECRET").unwrap();
    let admin = revolution_api::utils::create_jwt(admin_id, &secret, 1).unwrap();
    let user = revolution_api::utils::create_jwt(user_id, &secret, 1).unwrap();
    let now = chrono::Utc::now().timestamp_millis();
    let batch = json!({"session_id":"test-session","visitor_id":"anonymous-visitor","events":[
        {"event_type":"page_view","timestamp":now,"page_url":"https://example.test/pricing?email=secret@example.test#token"},
        {"event_type":"click","timestamp":now+1,"page_url":"https://example.test/pricing","element_selector":"#private-id","coordinates":{"x":24,"y":35},"coordinate_space":"viewport_percent","event_metadata":{"value":"secret"}},
        {"event_type":"scroll_depth","timestamp":now+2,"page_url":"https://example.test/pricing","event_value":75}
    ]});
    let (status, receipt) =
        request(&api, "POST", "/api/behavior/events", None, batch.clone()).await;
    assert_eq!(status, StatusCode::OK, "{receipt}");
    assert_eq!(receipt["persisted"], 3);
    assert_eq!(
        request(&api, "POST", "/api/behavior/events", None, batch.clone())
            .await
            .1["persisted"],
        0
    );
    let stored: Vec<(String, Option<String>)> =
        sqlx::query_as("SELECT page_url,element_selector FROM behavior_events ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(stored.len(), 3);
    assert!(
        stored
            .iter()
            .all(|row| row.0 == "/pricing" && row.1.is_none())
    );
    let private = json!({"session_id":"private-test","visitor_id":"anonymous","events":[{"event_type":"page_view","timestamp":now,"page_url":"https://example.test/account/private"}]});
    assert_eq!(
        request(&api, "POST", "/api/behavior/events", None, private)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let invalid = json!({"session_id":"invalid-test","visitor_id":"anonymous","events":[{"event_type":"click","timestamp":now,"page_url":"https://example.test/pricing","coordinates":{"x":101,"y":20},"coordinate_space":"viewport_percent"}]});
    assert_eq!(
        request(&api, "POST", "/api/behavior/events", None, invalid)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );

    for path in [
        "/api/analytics/behavior/heatmaps?period=7d",
        "/api/analytics/behavior/sessions?period=7d",
        "/api/analytics/behavior/sessions/test-session",
    ] {
        assert_eq!(
            request(&api, "GET", path, None, Value::Null).await.0,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            request(&api, "GET", path, Some(&user), Value::Null).await.0,
            StatusCode::FORBIDDEN
        );
    }
    let (status, pages) = request(
        &api,
        "GET",
        "/api/analytics/behavior/heatmaps?period=7d",
        Some(&admin),
        Value::Null,
    )
    .await;
    let (summary_status, summary) = request(
        &api,
        "GET",
        "/api/analytics/behavior?period=7d",
        Some(&admin),
        Value::Null,
    )
    .await;
    assert_eq!(summary_status, StatusCode::OK, "{summary}");
    assert_eq!(summary["click_events"], 1);
    assert_eq!(summary["avg_scroll_depth"], 75.0);
    assert_eq!(summary["top_scrolls"][0]["views"], 1);
    assert_eq!(status, StatusCode::OK, "{pages}");
    assert_eq!(pages["pages"][0]["clicks"], 1);
    let (status, heatmap) = request(
        &api,
        "GET",
        "/api/analytics/behavior/heatmaps?period=7d&page_url=%2Fpricing",
        Some(&admin),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{heatmap}");
    assert_eq!(heatmap["points"][0]["count"], 1);
    assert_eq!(heatmap["scroll_depth"][2]["sessions"], 1);
    let (status, sessions) = request(
        &api,
        "GET",
        "/api/analytics/behavior/sessions?period=7d",
        Some(&admin),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{sessions}");
    assert_eq!(sessions["pagination"]["total"], 1);
    let (status, timeline) = request(
        &api,
        "GET",
        "/api/analytics/behavior/sessions/test-session",
        Some(&admin),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{timeline}");
    assert_eq!(timeline["events"].as_array().map(Vec::len), Some(3));
    assert_eq!(timeline["events"][0]["page_url"], "/pricing");

    sqlx::query("INSERT INTO behavior_events (event_hash,session_id,visitor_id,event_type,page_url,happened_at,received_at) VALUES ('expired-test','expired','anonymous','page_view','/pricing',NOW()-INTERVAL '91 days',NOW()-INTERVAL '91 days')").execute(&pool).await.unwrap();
    assert_eq!(routes::behavior::prune_expired(&pool).await.unwrap(), 1);
    let remaining: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM behavior_events")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(remaining, 3);
    pool.close().await;
}
