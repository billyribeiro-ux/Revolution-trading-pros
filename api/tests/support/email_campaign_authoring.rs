//! Native campaign identities, independent drafts, actual source and atomic audit.
use super::request;
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = Some(tokens[0].as_str());
    let root = "/api/admin/email/campaigns";
    let segment: i64 =
        sqlx::query_scalar("SELECT id FROM member_segments WHERE slug='active-subscribers'")
            .fetch_one(pool)
            .await
            .unwrap();
    let first:i64=sqlx::query_scalar("INSERT INTO email_templates(name,slug,subject,html_content,text_content,is_active) VALUES('Campaign authoring one','campaign-authoring-one','Template subject','<p>First campaign source</p>','First alternative',true) RETURNING id").fetch_one(pool).await.unwrap();
    let second:i64=sqlx::query_scalar("INSERT INTO email_templates(name,slug,subject,html_content,is_active) VALUES('Campaign authoring two','campaign-authoring-two','Other subject','<p>Second campaign source</p>',true) RETURNING id").fetch_one(pool).await.unwrap();
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, path, body) in [
            ("GET", root.to_string(), Value::Null),
            ("POST", root.to_string(), json!({"name":"Rejected"})),
            ("GET", format!("{root}/stats"), Value::Null),
        ] {
            assert_eq!(request(api, method, &path, token, body).await.0, status);
        }
    }
    for bad in [
        json!({"name":""}),
        json!({"name":"Bad","subject":"Header\r\ninjection"}),
        json!({"name":"Bad","from_email":"not-an-address"}),
        json!({"name":"Bad","segment_id":0}),
        json!({"name":"Bad","status":"sent"}),
        json!({"name":"Bad","ab_test_config":{"enabled":true,"subject_b":"B","split_percentage":100}}),
        json!({"name":"Bad","template_id":999999}),
    ] {
        let status = if bad.get("status").is_some() {
            StatusCode::UNPROCESSABLE_ENTITY
        } else {
            StatusCode::BAD_REQUEST
        };
        assert_eq!(request(api, "POST", root, admin, bad).await.0, status);
    }
    let planned = chrono::Utc::now() + chrono::Duration::days(1);
    let created=request(api,"POST",root,admin,json!({"name":"Native authoring fixture","subject":"Actual campaign subject","template_id":first,"segment_id":segment,"from_name":"Fixture sender","from_email":"sender@example.test","reply_to":"reply@example.test","scheduled_at":planned,"tags":["Fixture"],"ab_test_config":{"enabled":true,"subject_b":"Other campaign subject","split_percentage":40}})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["data"]["id"].as_i64().unwrap();
    let path = format!("{root}/{id}");
    assert_eq!(
        created.1["data"]["status"], "draft",
        "Saving a proposed time does not invent a submitted audience"
    );
    assert_eq!(
        created.1["data"]["html_content"],
        "<p>First campaign source</p>"
    );
    assert_eq!(created.1["data"]["plain_text_content"], "First alternative");
    let created_at =
        chrono::DateTime::parse_from_rfc3339(created.1["data"]["created_at"].as_str().unwrap())
            .unwrap();
    assert!(
        (chrono::Utc::now().timestamp() - created_at.timestamp()).abs() < 10,
        "Naive legacy storage must not shift new authoring timestamps by the database timezone"
    );
    assert_eq!(created.1["data"]["sent_count"], 0);
    assert_eq!(created.1["data"]["metrics_scope"], "recorded_outbox");
    let actor: i64 =
        sqlx::query_scalar("SELECT id FROM users WHERE email='admin@email-completion.test'")
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(created.1["data"]["created_by"], actor);
    assert_eq!(created.1["data"]["creator"]["id"], actor);
    let queue: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM email_campaigns WHERE id=$1 AND delivery_campaign_id IS NOT NULL",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(queue, 0);
    let reread = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(reread.1, created.1);
    let page = request(
        api,
        "GET",
        &format!("{root}?search=Native%20authoring&per_page=1&page=1&type=broadcast"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(page.0, StatusCode::OK, "{page:?}");
    assert_eq!(page.1["meta"]["total"], 1);
    assert_eq!(page.1["data"][0]["id"], id);
    for query in [
        "?per_page=101",
        "?page=0",
        "?status=unknown",
        "?type=unknown",
        "?from_date=2026-11-06T00:00:00Z&to_date=2026-11-05T00:00:00Z",
    ] {
        assert_eq!(
            request(api, "GET", &format!("{root}{query}"), admin, Value::Null)
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
    }
    let edit=request(api,"PUT",&path,admin,json!({"name":"Native authoring edited","template_id":second,"segment_id":null,"reply_to":null,"scheduled_at":null})).await;
    assert_eq!(edit.0, StatusCode::OK, "{edit:?}");
    assert_eq!(
        edit.1["data"]["html_content"],
        "<p>Second campaign source</p>"
    );
    assert_eq!(edit.1["data"]["plain_text_content"], Value::Null);
    assert_eq!(edit.1["data"]["segment_id"], Value::Null);
    assert_eq!(edit.1["data"]["reply_to"], Value::Null);
    assert_eq!(edit.1["data"]["planned_at"], Value::Null);
    // Later template edits cannot silently change authored campaign content.
    sqlx::query(
        "UPDATE email_templates SET html_content='<p>Later template revision</p>' WHERE id=$1",
    )
    .bind(second)
    .execute(pool)
    .await
    .unwrap();
    let preview = request(api, "GET", &format!("{path}/preview"), admin, Value::Null).await;
    assert_eq!(preview.0, StatusCode::OK, "{preview:?}");
    assert_eq!(preview.1["data"]["html"], "<p>Second campaign source</p>");
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO email_campaign_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(api, "PUT", &path, admin, json!({"name":"Must roll back"})).await;
    sqlx::query("ALTER TABLE email_campaign_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await.1["data"]["name"],
        "Native authoring edited"
    );
    let cloned = request(
        api,
        "POST",
        &format!("{path}/duplicate"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(cloned.0, StatusCode::OK, "{cloned:?}");
    let clone = cloned.1["data"]["id"].as_i64().unwrap();
    assert_ne!(clone, id);
    assert_eq!(cloned.1["data"]["status"], "draft");
    assert_eq!(
        cloned.1["data"]["html_content"],
        "<p>Second campaign source</p>"
    );
    assert_eq!(cloned.1["data"]["planned_at"], Value::Null);
    sqlx::query("UPDATE email_campaigns SET sent_count=999,open_count=777 WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let stats = request(api, "GET", &format!("{root}/stats"), admin, Value::Null).await;
    assert_eq!(stats.0, StatusCode::OK, "{stats:?}");
    assert_eq!(
        stats.1["data"]["total_sent"], 0,
        "Legacy aggregates are preserved but never relabeled as verified outbox receipts"
    );
    assert_eq!(stats.1["data"]["avg_open_rate"], Value::Null);
    let preserved = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(preserved.1["data"]["historical_counts"]["sent"], 999);
    assert_eq!(preserved.1["data"]["sent_count"], 0);
    sqlx::query("UPDATE email_campaigns SET status='sent' WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(api, "PUT", &path, admin, json!({"subject":"Cannot edit"}))
            .await
            .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{root}/{clone}"),
            Some(tokens[1].as_str()),
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{root}/{clone}"),
            admin,
            Value::Null
        )
        .await
        .1["deleted_id"],
        clone
    );
    assert_eq!(
        request(api, "GET", &format!("{root}/{clone}"), admin, Value::Null)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    // An independent connection pool reads the persisted authoring record.
    let cold = PgPool::connect(&std::env::var("ADMIN_VERIFY_DATABASE_URL").unwrap())
        .await
        .unwrap();
    let values: (String, Value) =
        sqlx::query_as("SELECT name,authoring_config FROM email_campaigns WHERE id=$1")
            .bind(id)
            .fetch_one(&cold)
            .await
            .unwrap();
    assert_eq!(values.0, "Native authoring edited");
    assert_eq!(values.1["ab_test_config"]["split_percentage"], 40);
    cold.close().await;
}
