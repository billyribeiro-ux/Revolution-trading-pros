//! Saved template content, strict previews and actual provider test receipts.
use super::{Adapter, app, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let payload = json!({"name":"Primary template workflow","slug":"mail-template-workflow","subject":"Hello {{name}}","body_html":"<p>Hello {{name}}</p>","body_text":"Literal {{name}}","email_type":"marketing","variables":[{"key":"name","type":"string"}],"is_active":true});
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(
                api,
                "POST",
                "/api/admin/email/templates",
                token,
                payload.clone()
            )
            .await
            .0,
            status
        );
    }
    let created = request(
        api,
        "POST",
        "/api/admin/email/templates",
        Some(&tokens[0]),
        payload.clone(),
    )
    .await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["data"]["id"].as_i64().unwrap();
    let path = format!("/api/admin/email/templates/{id}");
    assert_eq!(created.1["data"]["body_html"], payload["body_html"]);
    assert_eq!(created.1["data"]["body"], payload["body_html"]);
    assert_eq!(created.1["data"]["body_text"], payload["body_text"]);
    assert_eq!(created.1["data"]["email_type"], "marketing");
    assert_eq!(created.1["data"]["variables"], payload["variables"]);
    assert_eq!(
        request(
            api,
            "POST",
            "/api/admin/email/templates",
            Some(&tokens[0]),
            payload.clone()
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    for (field, value) in [
        ("body_html", json!("")),
        ("subject", json!("Bad\r\nHeader")),
        ("variables", json!({})),
        ("body_html", json!("{{#if broken}")),
    ] {
        let mut invalid = payload.clone();
        invalid[field] = value;
        assert_eq!(
            request(
                api,
                "POST",
                "/api/admin/email/templates",
                Some(&tokens[0]),
                invalid
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    let updated = request(
        api,
        "PUT",
        &path,
        Some(&tokens[0]),
        json!({"name":"Renamed template workflow"}),
    )
    .await;
    assert_eq!(updated.0, StatusCode::OK, "{updated:?}");
    assert_eq!(
        updated.1["data"]["slug"], "mail-template-workflow",
        "Renaming must preserve a delivery alias"
    );
    assert_eq!(updated.1["data"]["body_html"], payload["body_html"]);
    let model = json!({"name":"<Customer & fixture>","reset_link":"https://example.test/reset?token=do-not-log"});
    let preview = request(
        api,
        "POST",
        &format!("{path}/preview"),
        Some(&tokens[0]),
        json!({"data":model}),
    )
    .await;
    assert_eq!(preview.0, StatusCode::OK, "{preview:?}");
    assert_eq!(preview.1["data"]["subject"], "Hello <Customer & fixture>");
    assert_eq!(
        preview.1["data"]["html"],
        "<p>Hello &lt;Customer &amp; fixture&gt;</p>"
    );
    assert_eq!(preview.1["data"]["text"], "Literal <Customer & fixture>");
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/preview"),
            Some(&tokens[0]),
            json!({})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST,
        "Missing required variables must never appear as a successful empty preview"
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/preview"),
            Some(&tokens[0]),
            json!({"data":[]})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let before = adapter.requests.lock().unwrap().len();
    let sent = request(
        api,
        "POST",
        &format!("{path}/test"),
        Some(&tokens[0]),
        json!({"email":"local-template-test@example.test","data":model}),
    )
    .await;
    assert_eq!(sent.0, StatusCode::OK, "{sent:?}");
    assert_eq!(sent.1["submission_status"], "accepted");
    {
        let calls = adapter.requests.lock().unwrap();
        assert_eq!(calls.len(), before + 1);
        assert_eq!(calls[before]["Subject"], preview.1["data"]["subject"]);
        assert_eq!(calls[before]["HtmlBody"], preview.1["data"]["html"]);
        assert_eq!(calls[before]["TextBody"], preview.1["data"]["text"]);
    }
    let logged:Value=sqlx::query_scalar("SELECT to_jsonb(e) FROM email_logs e WHERE to_email='local-template-test@example.test' ORDER BY id DESC LIMIT 1").fetch_one(pool).await.unwrap();
    assert_eq!(logged["provider_message_id"], sent.1["provider_message_id"]);
    assert_eq!(logged["template_alias"], "mail-template-workflow");
    assert_eq!(logged["model"]["reset_link"], "[redacted]");
    let cold = app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let reloaded = request(&cold, "GET", &path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(reloaded.0, StatusCode::OK);
    assert_eq!(reloaded.1["data"]["body_text"], payload["body_text"]);
    let cleared = request(api, "PUT", &path, Some(&tokens[0]), json!({"body_text":""})).await;
    assert_eq!(cleared.0, StatusCode::OK);
    assert!(cleared.1["data"]["body_text"].is_null());
    let long_slug = "template-identity-".to_string() + &"x".repeat(230);
    let mut long = payload.clone();
    long["slug"] = json!(long_slug);
    let long = request(
        api,
        "POST",
        "/api/admin/email/templates",
        Some(&tokens[0]),
        long,
    )
    .await;
    assert_eq!(long.0, StatusCode::OK, "{long:?}");
    let long_id = long.1["data"]["id"].as_i64().unwrap();
    let long_sent=request(api,"POST",&format!("/api/admin/email/templates/{long_id}/test"),Some(&tokens[0]),json!({"email":"long-local-template-test@example.test","data":{"name":"Long alias fixture"}})).await;
    assert_eq!(long_sent.0, StatusCode::OK, "{long_sent:?}");
    let alias:String=sqlx::query_scalar("SELECT template_alias FROM email_logs WHERE to_email='long-local-template-test@example.test' ORDER BY id DESC LIMIT 1").fetch_one(pool).await.unwrap();
    assert_eq!(alias, long_slug);
    let first = request(
        api,
        "GET",
        "/api/admin/email/templates?search=template-workflow&per_page=1&page=1",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(first.0, StatusCode::OK);
    assert_eq!(first.1["meta"]["total"], 1);
    assert_eq!(first.1["data"].as_array().unwrap().len(), 1);
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO template_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let mut rollback = payload.clone();
    rollback["slug"] = json!("template-rollback-fixture");
    let failed = request(
        api,
        "POST",
        "/api/admin/email/templates",
        Some(&tokens[0]),
        rollback,
    )
    .await;
    sqlx::query("ALTER TABLE template_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    assert!(
        !sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM email_templates WHERE slug='template-rollback-fixture')"
        )
        .fetch_one(pool)
        .await
        .unwrap()
    );
    let referenced:i64=sqlx::query_scalar("INSERT INTO email_campaigns(name,subject,template_id,status) VALUES('Template reference fixture','Fixture',$1,'draft') RETURNING id").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(
        request(api, "DELETE", &path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::CONFLICT
    );
    sqlx::query("DELETE FROM email_campaigns WHERE id=$1")
        .bind(referenced)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(api, "DELETE", &path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "GET", &path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    sqlx::query("ALTER TABLE email_templates RENAME TO template_store_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let unavailable = request(
        api,
        "GET",
        "/api/admin/email/templates",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    sqlx::query("ALTER TABLE template_store_unavailable RENAME TO email_templates")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(unavailable.0, StatusCode::SERVICE_UNAVAILABLE);
}
