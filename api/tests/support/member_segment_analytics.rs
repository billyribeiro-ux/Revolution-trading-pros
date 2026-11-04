//! Real current facts, captured baselines, CSV and recorded provider open signals.
use super::request;
use axum::{Router, http::StatusCode};
use revolution_api::services::member_segmentation as evaluation;
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], id: i64, first_user: i64) {
    let admin = Some(tokens[0].as_str());
    let path = format!("/api/admin/members/segments/{id}");
    let url = format!("{path}/analytics");
    let first = request(api, "GET", &url, admin, Value::Null).await;
    assert_eq!(first.0, StatusCode::OK, "{first:?}");
    assert_eq!(first.1["total_members"], 3);
    assert_eq!(first.1["avg_spending"].as_f64(), Some(210.0));
    assert_eq!(first.1["avg_lifetime_days"].as_f64(), Some(40.0));
    assert_eq!(first.1["conversion_rate"].as_f64(), Some(66.7));
    assert!(first.1["email_open_rate"].is_null());
    assert!(
        first.1["growth_30d"].is_null(),
        "Current mutable facts never fabricate a historical baseline"
    );
    assert_eq!(first.1["top_products"][0]["count"], 1);
    let version = first.1["authoring_version"].as_i64().unwrap();
    // Explicit historical-count fixtures test comparison arithmetic, not provider history.
    sqlx::query("INSERT INTO member_segment_observations(segment_id,authoring_version,observed_date,observed_at,member_count) VALUES($1,$2,CURRENT_DATE-30,NOW()-INTERVAL '30 days 1 minute',2)").bind(id).bind(version).execute(pool).await.unwrap();
    assert_eq!(
        request(api, "GET", &url, admin, Value::Null).await.1["growth_30d"].as_f64(),
        Some(50.0)
    );
    let name_only = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"description":"Name-only comparison preserves its definition"}),
    )
    .await;
    assert_eq!(name_only.0, StatusCode::OK, "{name_only:?}");
    assert_eq!(
        request(api, "GET", &url, admin, Value::Null).await.1["growth_30d"].as_f64(),
        Some(50.0)
    );
    let before = evaluation::load(pool, id, false).await.unwrap();
    let mut conditions = name_only.1["conditions"].as_array().unwrap().clone();
    conditions.push(json!({"field":"days_since_signup","operator":"gt","value":1}));
    assert_eq!(
        request(api, "PUT", &path, admin, json!({"rules":conditions}))
            .await
            .0,
        StatusCode::OK
    );
    let changed = request(api, "GET", &url, admin, Value::Null).await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert!(
        changed.1["growth_30d"].is_null(),
        "A changed audience must not inherit an incompatible baseline"
    );
    assert_eq!(changed.1["authoring_version"], version + 1);
    assert!(
        evaluation::observe(pool, id, &before, 3).await.is_err(),
        "A stale observation cannot be assigned to a newer definition"
    );
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for action in ["analytics", "export"] {
            assert_eq!(
                request(api, "GET", &format!("{path}/{action}"), token, Value::Null)
                    .await
                    .0,
                status
            );
        }
    }
    sqlx::query("UPDATE users SET name='=SUM(1,2)' WHERE id=$1")
        .bind(first_user)
        .execute(pool)
        .await
        .unwrap();
    let exported = request(api, "GET", &format!("{path}/export"), admin, Value::Null).await;
    assert_eq!(exported.0, StatusCode::OK, "{exported:?}");
    let mut reader = csv::Reader::from_reader(exported.1["text"].as_str().unwrap().as_bytes());
    assert_eq!(reader.headers().unwrap().get(4), Some("total_spent"));
    let rows = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(rows[0].get(1), Some("'=SUM(1,2)"));
    assert_eq!(rows[0].get(4), Some("480.00"));
    // Earlier campaign proof actually submitted mail and ingested authenticated open callbacks.
    let email:String=sqlx::query_scalar("SELECT email FROM crm_campaign_deliveries WHERE status='accepted' AND opened_at IS NOT NULL ORDER BY created_at LIMIT 1").fetch_one(pool).await.unwrap();
    let user:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash) VALUES('Recorded open fixture',$1,'not-a-login-password') ON CONFLICT(email) DO UPDATE SET name=users.name RETURNING id").bind(&email).fetch_one(pool).await.unwrap();
    let observed:i64=sqlx::query_scalar("INSERT INTO member_segments(name,slug,is_dynamic) VALUES('Observed open fixture','observed-open-fixture',false) RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_member_segments(user_id,segment_id) VALUES($1,$2)")
        .bind(user)
        .bind(observed)
        .execute(pool)
        .await
        .unwrap();
    let stats = evaluation::analytics(pool, observed).await.unwrap();
    assert_eq!(stats["total_members"], 1);
    assert!(stats["open_signals"].as_i64().unwrap() > 0);
    assert!(stats["email_open_rate"].as_f64().unwrap() > 0.0);
    let rules=evaluation::rules(Some(json!([{"field":"email_engagement","operator":"eq","value":"engaged"},{"field":"tag","operator":"not_has","value":"nonexistent-fixture-tag"}]))).unwrap();
    assert!(evaluation::count(pool, &rules).await.unwrap() >= 1);
    let empty:i64=sqlx::query_scalar("INSERT INTO member_segments(name,slug,is_dynamic) VALUES('Empty static fixture','empty-static-analytics-fixture',false) RETURNING id").fetch_one(pool).await.unwrap();
    let stats = evaluation::analytics(pool, empty).await.unwrap();
    assert_eq!(stats["total_members"], 0);
    assert!(stats["avg_spending"].is_null());
    assert!(stats["conversion_rate"].is_null());
    assert!(stats["email_open_rate"].is_null());
    let system: i64 = sqlx::query_scalar("SELECT id FROM member_segments WHERE slug='trial-users'")
        .fetch_one(pool)
        .await
        .unwrap();
    let worker = tokio::spawn(evaluation::run_observations(pool.clone()));
    tokio::time::timeout(std::time::Duration::from_secs(5), async {
        loop {
            let recorded: bool = sqlx::query_scalar(
                "SELECT EXISTS(SELECT 1 FROM member_segment_observations WHERE segment_id=$1)",
            )
            .bind(system)
            .fetch_one(pool)
            .await
            .unwrap();
            if recorded {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(20)).await;
        }
    })
    .await
    .unwrap();
    worker.abort();
    let _ = worker.await;
    sqlx::query(
        "ALTER TABLE member_segment_observations RENAME TO segment_observations_unavailable",
    )
    .execute(pool)
    .await
    .unwrap();
    let unavailable = request(api, "GET", &url, admin, Value::Null).await;
    sqlx::query(
        "ALTER TABLE segment_observations_unavailable RENAME TO member_segment_observations",
    )
    .execute(pool)
    .await
    .unwrap();
    assert_eq!(unavailable.0, StatusCode::SERVICE_UNAVAILABLE);
}
