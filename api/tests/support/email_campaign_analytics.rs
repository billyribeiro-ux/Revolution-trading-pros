//! Real native receipt cohorts, actual provider observations and UTC windows.
use super::request;
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], id: i64) {
    let root = "/api/admin/email/analytics";
    let per = format!("/api/admin/email/campaigns/{id}/analytics");
    let admin = Some(tokens[0].as_str());
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for path in [root, &per] {
            assert_eq!(
                request(api, "GET", path, token, Value::Null).await.0,
                status
            );
        }
    }
    for period in ["1d", "all", "999999999d"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{root}?period={period}"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    for (period, days) in [("7d", 7), ("30d", 30), ("90d", 90), ("365d", 365)] {
        let response = request(
            api,
            "GET",
            &format!("{root}?period={period}"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(response.0, StatusCode::OK, "{response:?}");
        let data = &response.1["analytics"];
        assert_eq!(data["period"], period);
        assert_eq!(data["metrics_scope"], "native_campaign_outbox");
        let parse = |key: &str| {
            chrono::DateTime::parse_from_rfc3339(data["window"][key].as_str().unwrap()).unwrap()
        };
        assert_eq!(
            (parse("end") - parse("start")).num_seconds(),
            days * 86400,
            "UTC observation windows must not drift across daylight-saving transitions"
        );
        assert_eq!(
            (parse("start") - parse("previous_start")).num_seconds(),
            days * 86400
        );
        assert_eq!(data["window"]["timezone"], "UTC");
        assert_eq!(
            data["emails_sent"], 2,
            "Historical native counters and separate CRM campaigns are not native outbox acceptances"
        );
        assert_eq!(data["total_opens"], 2);
        assert_eq!(data["total_clicks"], 1);
        assert_eq!(
            data["average_open_rate"], 50.0,
            "Recipient rate uses unique accepted recipients, not repeated open events"
        );
        assert_eq!(data["average_click_rate"], 50.0);
        assert_eq!(data["comparison"]["emails_sent_change_pct"], Value::Null);
        assert_eq!(data["comparison"]["open_rate_change_points"], Value::Null);
        assert_eq!(
            data["device_breakdown"],
            json!({"desktop":0,"mobile":2,"tablet":0,"unknown":0})
        );
        assert_eq!(data["client_breakdown"]["Fixture Mail"], 2);
        assert_eq!(data["unlisted_client_observations"], 0);
        let trend = data["engagement_trend"].as_array().unwrap();
        assert_eq!(
            trend
                .iter()
                .map(|row| row["opens"].as_i64().unwrap())
                .sum::<i64>(),
            2
        );
        assert_eq!(
            trend
                .iter()
                .map(|row| row["clicks"].as_i64().unwrap())
                .sum::<i64>(),
            1
        );
        assert_eq!(data["top_campaigns"][0]["id"], id);
        assert_eq!(data["top_campaigns"][0]["sent"], 2);
    }
    let response = request(api, "GET", &per, admin, Value::Null).await;
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    let data = &response.1["data"];
    assert_eq!(data["campaign"]["id"], id);
    assert_eq!(data["campaign"]["sent_count"], 2);
    assert_eq!(data["campaign"]["opened_count"], 1);
    assert_eq!(data["campaign"]["open_rate"], 50.0);
    let hourly = data["hourly_stats"].as_array().unwrap();
    for (kind, count) in [("opened", 2), ("clicked", 1)] {
        assert_eq!(
            hourly
                .iter()
                .filter(|row| row["event_type"] == kind)
                .map(|row| row["count"].as_i64().unwrap())
                .sum::<i64>(),
            count
        );
    }
    assert_eq!(
        data["top_links"][0]["link_url"],
        "https://offer.example.test/recorded?a=1&b=2"
    );
    assert_eq!(data["geo_stats"][0]["country"], "United States");
    assert_eq!(
        request(
            api,
            "GET",
            "/api/admin/email/campaigns/9223372036854775807/analytics",
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    // Simulate one genuine acknowledgment belonging to the preceding cohort.
    // Keep its provider identity/events intact, and restore its actual fixture clock.
    let(receipt,accepted):(uuid::Uuid,chrono::DateTime<chrono::Utc>)=sqlx::query_as("SELECT d.id,d.accepted_at FROM crm_campaign_deliveries d JOIN email_campaigns c ON c.delivery_campaign_id=d.campaign_id WHERE c.id=$1 AND d.opened_at IS NOT NULL LIMIT 1").bind(id).fetch_one(pool).await.unwrap();
    sqlx::query(
        "UPDATE crm_campaign_deliveries SET accepted_at=NOW()-INTERVAL '35 days' WHERE id=$1",
    )
    .bind(receipt)
    .execute(pool)
    .await
    .unwrap();
    let previous = request(api, "GET", root, admin, Value::Null).await;
    sqlx::query("UPDATE crm_campaign_deliveries SET accepted_at=$2 WHERE id=$1")
        .bind(receipt)
        .bind(accepted)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(previous.0, StatusCode::OK, "{previous:?}");
    let data = &previous.1["analytics"];
    assert_eq!(data["emails_sent"], 1);
    assert_eq!(data["average_open_rate"], 0.0);
    assert_eq!(
        data["total_opens"], 2,
        "Current event observations can refer to a prior acceptance cohort"
    );
    assert_eq!(data["comparison"]["emails_sent_change_pct"], 0.0);
    assert_eq!(data["comparison"]["open_rate_change_points"], -100.0);
    sqlx::query("ALTER TABLE crm_campaign_events RENAME TO native_analytics_events_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let unavailable = request(api, "GET", root, admin, Value::Null).await;
    sqlx::query("ALTER TABLE native_analytics_events_unavailable RENAME TO crm_campaign_events")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(unavailable.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        request(api, "GET", root, admin, Value::Null).await.0,
        StatusCode::OK
    );
}
