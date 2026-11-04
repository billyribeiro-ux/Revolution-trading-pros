use super::request;
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;
pub(super) async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    if std::env::var("ADMIN_VERIFY_PG_STAT_STATEMENTS").as_deref() == Ok("1") {
        sqlx::query("CREATE EXTENSION pg_stat_statements")
            .execute(pool)
            .await
            .unwrap();
        sqlx::query("SELECT 1").execute(pool).await.unwrap();
    }
    for (token, expected) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, path) in [
            ("GET", "/api/performance/dashboard"),
            ("GET", "/api/performance/core-web-vitals"),
            ("GET", "/api/performance/recommendations"),
            ("POST", "/api/performance/warm-caches"),
        ] {
            assert_eq!(
                request(api, method, path, token, json!({})).await.0,
                expected
            );
        }
    }
    let url = "http://127.0.0.1:5274/performance-fixture";
    let now = chrono::Utc::now().timestamp_millis();
    for (id, value) in [("a", 1000.), ("b", 2000.), ("c", 3000.), ("d", 5000.)] {
        let input = json!({"metric":"LCP","value":value,"id":format!("performance-{id}"),"timestamp":now,"url":format!("{url}?sensitive-query=must-not-be-stored#fragment"),"userAgent":"Fixture desktop","rating":"good"});
        let (status, receipt) = request(
            api,
            "POST",
            "/api/analytics/web-vitals",
            None,
            input.clone(),
        )
        .await;
        assert_eq!(status, StatusCode::OK, "{receipt}");
        assert_eq!(receipt["persisted"], 1);
        let (_, receipt) = request(api, "POST", "/api/analytics/web-vitals", None, input).await;
        assert_eq!(receipt["persisted"], 0);
    }
    let stored: Vec<String> = sqlx::query_scalar("SELECT page_url FROM performance_samples")
        .fetch_all(pool)
        .await
        .unwrap();
    assert_eq!(stored.len(), 4);
    assert!(stored.iter().all(|value| value == url));
    let id = json!({"name":"CLS","value":0,"id":"performance-cls","timestamp":now,"url":url,"userAgent":"Fixture Mobile"});
    assert_eq!(
        request(api, "POST", "/api/analytics/performance", None, id)
            .await
            .0,
        StatusCode::OK
    );
    let legacy = api
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri("/api/analytics/performance")
                .header("content-type", "text/plain")
                .body(Body::from(
                    json!({"name":"custom.render","value":42,"id":"legacy-text-plain"}).to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(legacy.status(), StatusCode::OK);
    assert_eq!(
        request(
            api,
            "POST",
            "/api/analytics/web-vitals",
            None,
            json!({"metric":"LCP","value":1,"url":"x".repeat(17000)})
        )
        .await
        .0,
        StatusCode::PAYLOAD_TOO_LARGE
    );
    for invalid in [
        json!({"metric":"LCP","value":-1}),
        json!({"metric":"bad\u{0}","value":1}),
        json!({"metric":"LCP","value":1,"id":""}),
        json!({"metric":"LCP","value":1,"timestamp":now+3600000}),
        json!({"metric":"LCP","value":1,"timestamp":now-91*86400000}),
        json!({"metric":"LCP","value":1,"url":"file:///tmp/private"}),
        json!({"metric":"LCP","value":1,"url":"https://user:password@example.test"}),
        json!({"metric":"LCP","value":1,"unknown":true}),
    ] {
        assert_eq!(
            request(api, "POST", "/api/analytics/web-vitals", None, invalid)
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
    }
    // Newer values replace an observation; stale replays cannot overwrite it.
    let newer =
        json!({"metric":"INP","value":400,"id":"performance-updated","timestamp":now,"url":url});
    assert_eq!(
        request(
            api,
            "POST",
            "/api/analytics/web-vitals",
            None,
            newer.clone()
        )
        .await
        .0,
        StatusCode::OK
    );
    let mut updated = newer.clone();
    updated["value"] = json!(600);
    updated["timestamp"] = json!(now + 1);
    request(api, "POST", "/api/analytics/web-vitals", None, updated).await;
    let (_, receipt) = request(api, "POST", "/api/analytics/web-vitals", None, newer).await;
    assert_eq!(receipt["persisted"], 0);
    let collecting = tokio::spawn(revolution_api::routes::performance::refresh_host_measurements());
    let response = tokio::time::timeout(
        std::time::Duration::from_secs(2),
        request(
            api,
            "GET",
            "/api/performance/dashboard",
            Some(&tokens[0]),
            json!({}),
        ),
    )
    .await
    .expect("Dashboard must not wait on host OS probes");
    assert_eq!(response.0, StatusCode::OK);
    collecting.await.unwrap().unwrap();
    let (status, report) = request(
        api,
        "GET",
        "/api/performance/dashboard?days=7",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{report}");
    let vitals = &report["data"]["core_web_vitals"];
    assert_eq!(vitals["lcp"], 3500.0);
    assert_eq!(
        vitals["lcp_rating"], "needs_improvement",
        "Ignore untrusted client rating"
    );
    assert_eq!(vitals["sample_counts"]["LCP"], 4);
    assert_eq!(vitals["cls"], 0.0);
    assert_eq!(vitals["inp"], 600.0);
    assert!(vitals["fid"].is_null());
    assert!(vitals["overall_score"].is_null());
    let server = &report["data"]["server"];
    for key in ["cpu_usage", "memory_usage"] {
        let value = server[key]
            .as_f64()
            .expect("Actual supported host measurement");
        assert!((0.0..=100.0).contains(&value));
    }
    assert!(
        server["uptime"]
            .as_str()
            .is_some_and(|value| value.ends_with('s'))
    );
    let database = &report["data"]["database"];
    if std::env::var("ADMIN_VERIFY_PG_STAT_STATEMENTS").as_deref() == Ok("1") {
        assert_eq!(database["query_statistics_status"], "available");
        assert!(database["query_count"].as_i64().unwrap() > 0);
        assert!(database["query_statistics_requirement"].is_null());
    } else {
        assert_eq!(database["query_statistics_status"], "unavailable");
        assert!(database["query_count"].is_null());
        assert!(
            database["query_statistics_requirement"]
                .as_str()
                .unwrap()
                .contains("pg_stat_statements")
        );
    }
    let (_, mobile) = request(
        api,
        "GET",
        "/api/performance/core-web-vitals?device=mobile",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert!(mobile["data"]["lcp"].is_null());
    assert_eq!(mobile["data"]["cls"], 0.0);
    let (_, empty) = request(
        api,
        "GET",
        "/api/performance/core-web-vitals?url=http%3A%2F%2F127.0.0.1%3A5274%2Fother-page",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(empty["data"]["sample_counts"]["LCP"], 0);
    assert!(empty["data"]["lcp"].is_null());
    for query in ["days=0", "days=91", "device=bad", "url=not-a-url"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/performance/core-web-vitals?{query}"),
                Some(&tokens[0]),
                json!({})
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    let (_, advice) = request(
        api,
        "GET",
        "/api/performance/recommendations",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    let advice = advice["data"]["recommendations"].as_array().unwrap();
    assert!(advice.iter().any(|item| item["title"] == "Investigate INP"));
    assert!(!advice.iter().any(|item| item["title"] == "Investigate FID"));
    let room:i64=sqlx::query_scalar("INSERT INTO trading_rooms(name,slug) VALUES('Performance fixture','performance-fixture') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO room_stats_cache(room_id,room_slug,win_rate) VALUES($1,'performance-fixture',57.5)").bind(room).execute(pool).await.unwrap();
    let trade:i64=sqlx::query_scalar("INSERT INTO room_trades(room_id,room_slug,ticker,quantity,entry_price,entry_date) VALUES($1,'performance-fixture','FIX',1,20,CURRENT_DATE) RETURNING id").bind(room).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES('Performance plan','performance-fixture-plan',12.34,'monthly')").execute(pool).await.unwrap();
    let (status, warmed) = request(
        api,
        "POST",
        "/api/performance/warm-caches",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{warmed}");
    assert_eq!(warmed["data"]["success"], true, "{warmed}");
    assert!(warmed["data"]["total"].as_i64().unwrap() > 0);
    assert_eq!(warmed["data"]["configs"], 0);
    let (status, second) = request(
        api,
        "POST",
        "/api/performance/warm-caches",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(second["data"]["success"], true);
    let (_, after) = request(
        api,
        "GET",
        "/api/performance/dashboard",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert!(after["data"]["cache"]["keys_count"].as_i64().unwrap() > 0);
    assert!(
        after["data"]["cache"]["memory_used"]
            .as_str()
            .unwrap()
            .ends_with(" bytes")
    );
    let rate = after["data"]["cache"]["hit_rate"].as_f64().unwrap();
    assert!(
        rate > 0.0 && rate <= 100.0,
        "Hit rate is already a percentage: {rate}"
    );
    let (_, stats) = request(
        api,
        "GET",
        "/api/room-content/rooms/performance-fixture/stats",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(
        stats["data"]["win_rate"], 0.0,
        "The open trade trigger recomputes actual statistics"
    );
    let (_, trades) = request(
        api,
        "GET",
        "/api/room-content/rooms/performance-fixture/trades",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(trades["data"][0]["ticker"], "FIX");
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("/api/admin/room-content/trades/{trade}"),
            Some(&tokens[0]),
            json!({"ticker":"NEW"})
        )
        .await
        .0,
        StatusCode::OK
    );
    let (_, trades) = request(
        api,
        "GET",
        "/api/room-content/rooms/performance-fixture/trades",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(
        trades["data"][0]["ticker"], "NEW",
        "Warmed room cache must invalidate after a real mutation"
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("/api/admin/room-content/trades/{trade}/close"),
            Some(&tokens[0]),
            json!({"exit_price":21})
        )
        .await
        .0,
        StatusCode::OK
    );
    let (_, stats) = request(
        api,
        "GET",
        "/api/room-content/rooms/performance-fixture/stats",
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(
        stats["data"]["win_rate"], 100.0,
        "Closing a profitable trade invalidates warmed statistics"
    );
    let (_, plan) = request(
        api,
        "GET",
        "/api/subscriptions/plans/performance-fixture-plan",
        None,
        json!({}),
    )
    .await;
    assert_eq!(plan["price_cents"], 1234);
    // Do not acknowledge writes when the persistence dependency is absent.
    sqlx::query(
        "ALTER TABLE performance_samples RENAME TO performance_samples_fixture_unavailable",
    )
    .execute(pool)
    .await
    .unwrap();
    let unavailable = request(
        api,
        "POST",
        "/api/analytics/web-vitals",
        None,
        json!({"metric":"LCP","value":1}),
    )
    .await;
    sqlx::query(
        "ALTER TABLE performance_samples_fixture_unavailable RENAME TO performance_samples",
    )
    .execute(pool)
    .await
    .unwrap();
    assert_eq!(unavailable.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_ne!(unavailable.1["success"], true);
}
