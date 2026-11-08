//! Real-router ingest and bounded retention proof on an empty local database.
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

#[tokio::test]
#[ignore = "requires an explicitly provisioned empty disposable database"]
async fn ingested_observations_are_minimized_and_expire() {
    let url = std::env::var("PERFORMANCE_VERIFY_DATABASE_URL")
        .expect("PERFORMANCE_VERIFY_DATABASE_URL required");
    let parsed = reqwest::Url::parse(&url).unwrap();
    assert_eq!(parsed.host_str(), Some("127.0.0.1"));
    assert!(parsed.path().starts_with("/performance_verify"));
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let table_count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='public'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(table_count, 0, "the disposable database must be empty");
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
    let response = api
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/analytics/web-vitals")
                .header("content-type", "application/json")
                .body(Body::from(
                    json!({
                        "metric": "LCP", "value": 1234, "id": "performance-current",
                        "url": "https://example.test/page?private=secret#fragment"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), StatusCode::OK);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let receipt: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(receipt["persisted"], 1);
    let stored_url: String = sqlx::query_scalar(
        "SELECT page_url FROM performance_samples WHERE measurement_id='performance-current'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(stored_url, "https://example.test/page");

    sqlx::query("INSERT INTO performance_samples(measurement_id,metric,value,device,observed_at) VALUES('performance-expired','LCP',1000,'desktop',NOW()-INTERVAL '91 days')")
        .execute(&pool).await.unwrap();
    assert_eq!(routes::performance::prune_expired(&pool).await.unwrap(), 1);
    assert_eq!(routes::performance::prune_expired(&pool).await.unwrap(), 0);
    let remaining: Vec<String> =
        sqlx::query_scalar("SELECT measurement_id FROM performance_samples")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(remaining, vec!["performance-current"]);
}
