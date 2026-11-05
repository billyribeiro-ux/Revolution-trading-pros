//! QA samples exercise real provider calls, durable replay and production isolation.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/email/campaigns";
async fn create(api: &Router, token: &str, subject: &str) -> i64 {
    let response=request(api,"POST",ROOT,Some(token),json!({"name":"Native sample fixture","subject":subject,"html_content":"<p>Saved content for {{email}} {{user.name}}</p><a href='{{unsubscribe_url}}'>Leave</a>","plain_text_content":"Saved alternative {{email}}","from_name":"Actual QA sender","from_email":"qa-sender@example.test","reply_to":"qa-reply@example.test","ab_test_config":{"enabled":true,"subject_b":"Saved B for {{email}}","split_percentage":50}})).await;
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    response.1["data"]["id"].as_i64().unwrap()
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let id = create(api, &tokens[0], "Saved A for {{email}}").await;
    let path = format!("{ROOT}/{id}");
    let endpoint = format!("{path}/test");
    let history = format!("{path}/tests");
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, url, body) in [
            ("POST", &endpoint, json!({"email":"qa@example.test"})),
            ("GET", &history, Value::Null),
        ] {
            assert_eq!(request(api, method, url, token, body).await.0, status);
        }
    }
    let before = adapter.requests.lock().unwrap().len();
    for body in [
        json!({}),
        json!({"emails":[]}),
        json!({"email":"qa@example.test","emails":["qa@example.test"]}),
        json!({"emails":["QA@example.test","qa@example.test"]}),
        json!({"emails":["1@example.test","2@example.test","3@example.test","4@example.test","5@example.test","6@example.test"]}),
        json!({"email":"invalid"}),
        json!({"email":"qa@example.test\r\nBcc: other@example.test"}),
        json!({"email":"qa@example.test","variant":"c"}),
        json!({"email":"qa@example.test","expected_version":0}),
        json!({"email":"qa@example.test","request_id":uuid::Uuid::nil()}),
    ] {
        let response = request(api, "POST", &endpoint, admin, body).await;
        assert_eq!(response.0, StatusCode::BAD_REQUEST, "{response:?}");
    }
    for search in ["%00".into(), "a".repeat(256)] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{ROOT}?search={search}"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    assert_eq!(
        request(api, "GET", &history, admin, Value::Null).await.1["data"]["requests"],
        json!([])
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/9223372036854775807/test"),
            admin,
            json!({"email":"qa@example.test"})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    // No provider call or partial intent may survive a failed atomic audit.
    let rollback_id = uuid::Uuid::new_v4();
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO native_test_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "POST",
        &endpoint,
        admin,
        json!({"email":"qa@example.test","request_id":rollback_id}),
    )
    .await;
    sqlx::query("ALTER TABLE native_test_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    let staged: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM email_campaign_test_requests WHERE request_id=$1")
            .bind(rollback_id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(staged, 0);
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    let stats_before = request(api, "GET", &format!("{ROOT}/stats"), admin, Value::Null)
        .await
        .1["data"]
        .clone();
    let request_id = uuid::Uuid::new_v4();
    let body = json!({"emails":[" SAMPLE2@example.test ","sample1@example.test"],"request_id":request_id,"variant":"b","expected_version":1});
    let accepted = request(api, "POST", &endpoint, admin, body.clone()).await;
    assert_eq!(accepted.0, StatusCode::OK, "{accepted:?}");
    assert_eq!(
        accepted.1["data"]["submissions"].as_array().unwrap().len(),
        2
    );
    for receipt in accepted.1["data"]["submissions"].as_array().unwrap() {
        assert_eq!(receipt["status"], "accepted");
        assert_eq!(receipt["provider"], "postmark");
        assert!(!receipt["provider_message_id"].as_str().unwrap().is_empty());
        assert!(!receipt["accepted_at"].is_null());
    }
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 2);
    for payload in &adapter.requests.lock().unwrap()[before..] {
        assert!(
            payload["Subject"]
                .as_str()
                .unwrap()
                .starts_with("[Test] Saved B for sample")
        );
        assert_eq!(
            payload["From"],
            "\"Actual QA sender\" <qa-sender@example.test>"
        );
        assert_eq!(payload["ReplyTo"], "qa-reply@example.test");
        assert!(
            payload["HtmlBody"]
                .as_str()
                .unwrap()
                .contains("Saved content for sample")
        );
        assert!(
            payload["TextBody"]
                .as_str()
                .unwrap()
                .contains("Saved alternative sample")
        );
        assert!(
            payload["HtmlBody"]
                .as_str()
                .unwrap()
                .contains("preview&#x3D;campaign-test")
                || payload["HtmlBody"]
                    .as_str()
                    .unwrap()
                    .contains("preview=campaign-test")
        );
        assert!(!payload["HtmlBody"].as_str().unwrap().contains("token="));
    }
    let replay = request(api, "POST", &endpoint, admin, body.clone()).await;
    assert_eq!(replay, accepted);
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 2);
    let stored = request(
        api,
        "GET",
        &format!("{history}?request_id={request_id}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(stored.1["data"]["requests"], json!([accepted.1["data"]]));
    for patch in [
        json!({"emails":["other@example.test"],"request_id":request_id,"variant":"b"}),
        json!({"emails":["sample1@example.test","sample2@example.test"],"request_id":request_id,"variant":"a"}),
    ] {
        assert_eq!(
            request(api, "POST", &endpoint, admin, patch).await.0,
            StatusCode::CONFLICT
        );
    }
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            admin,
            json!({"subject":"Edited saved subject"})
        )
        .await
        .0,
        StatusCode::OK
    );
    // Replay retains its actual earlier creative version after source changes.
    assert_eq!(
        request(api, "POST", &endpoint, admin, body.clone()).await,
        accepted
    );
    assert_eq!(
        request(
            api,
            "POST",
            &endpoint,
            admin,
            json!({"email":"stale@example.test","expected_version":1})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let unchanged = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(unchanged.1["data"]["status"], "draft");
    assert_eq!(unchanged.1["data"]["sent_count"], 0);
    assert_eq!(unchanged.1["data"]["total_recipients"], 0);
    let stats_after = request(api, "GET", &format!("{ROOT}/stats"), admin, Value::Null)
        .await
        .1["data"]
        .clone();
    assert_eq!(
        stats_before, stats_after,
        "QA receipts must not change production campaign metrics"
    );
    let linked: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM email_campaigns WHERE id=$1 AND delivery_campaign_id IS NOT NULL",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(linked, 0);
    // Missing required variables fail before any sample from a batch is sent.
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            admin,
            json!({"html_content":"{{unknown_required_variable}}"})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            api,
            "POST",
            &endpoint,
            admin,
            json!({"emails":["one@example.test","two@example.test"]})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 2);
    for (subject, status) in [
        ("fixture-rejected", "rejected"),
        ("fixture-unknown", "unknown"),
    ] {
        let native = create(api, &tokens[0], subject).await;
        let url = format!("{ROOT}/{native}/test");
        let key = uuid::Uuid::new_v4();
        let body = json!({"email":"failure@example.test","request_id":key});
        let response = request(api, "POST", &url, admin, body.clone()).await;
        assert_eq!(response.0, StatusCode::BAD_GATEWAY, "{response:?}");
        assert_eq!(response.1["success"], false);
        assert_eq!(response.1["data"]["submissions"][0]["status"], status);
        let count = adapter.requests.lock().unwrap().len();
        assert_eq!(request(api, "POST", &url, admin, body).await, response);
        assert_eq!(
            adapter.requests.lock().unwrap().len(),
            count,
            "No automatic retry of rejected or unknown test intents"
        );
    }
    inflight(api, pool, tokens, adapter, false).await;
    inflight(api, pool, tokens, adapter, true).await;
    let no_b = create(api, &tokens[0], "No B").await;
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{ROOT}/{no_b}"),
            admin,
            json!({"ab_test_config":null})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/{no_b}/test"),
            admin,
            json!({"email":"qa@example.test","variant":"b"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    let single = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    let native = create(&single, &tokens[0], "One connection test").await;
    let result=tokio::time::timeout(std::time::Duration::from_secs(5),request(&single,"POST",&format!("{ROOT}/{native}/test"),admin,json!({"emails":["single1@example.test","single2@example.test","single3@example.test","single4@example.test","single5@example.test"]}))).await.unwrap();
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    assert_eq!(result.1["data"]["submissions"].as_array().unwrap().len(), 5);
    one.close().await;
    assert_eq!(
        request(api, "DELETE", &path, admin, Value::Null).await.0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "POST", &endpoint, admin, body).await,
        accepted,
        "Historical QA receipts survive native draft deletion without resubmission"
    );
}
async fn inflight(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    lose_receipt: bool,
) {
    let admin = Some(tokens[0].as_str());
    let id = create(api, &tokens[0], "fixture-inflight").await;
    let url = format!("{ROOT}/{id}/test");
    let key = uuid::Uuid::new_v4();
    let body = json!({"email":"held@example.test","request_id":key});
    let before = adapter.requests.lock().unwrap().len();
    let during = async {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            adapter.inflight_started.notified(),
        )
        .await
        .unwrap();
        let replay = request(api, "POST", &url, admin, body.clone()).await;
        assert_eq!(replay.0, StatusCode::ACCEPTED, "{replay:?}");
        assert_eq!(replay.1["data"]["submissions"][0]["status"], "processing");
        assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
        if lose_receipt {
            sqlx::query("DELETE FROM email_logs WHERE template_type='native_campaign_test' AND model->>'request_id'=$1").bind(key.to_string()).execute(pool).await.unwrap();
        }
        adapter.inflight_release.notify_one();
    };
    let (submitted, ()) = tokio::join!(request(api, "POST", &url, admin, body.clone()), during);
    assert_eq!(
        submitted.0,
        if lose_receipt {
            StatusCode::SERVICE_UNAVAILABLE
        } else {
            StatusCode::OK
        },
        "{submitted:?}"
    );
    if lose_receipt {
        assert_eq!(
            request(api, "POST", &url, admin, body).await.0,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before + 1,
        "A missing acceptance receipt must never cause another provider call"
    );
}
