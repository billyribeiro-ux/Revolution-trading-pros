//! Operator content becomes authoritative without replacing untouched hosted aliases.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::json;
use sqlx::PgPool;

pub async fn proof(api: &Router, pool: &PgPool, admin: &str, adapter: &Adapter) {
    let alias = "legacy-hosted-policy-fixture";
    let id:i64=sqlx::query_scalar("INSERT INTO email_templates(name,slug,subject,html_content,variables,is_active) VALUES('Existing hosted fixture',$1,'Local {{name}}','<p>Original local {{name}}</p>','[\"name\"]',true) RETURNING id").bind(alias).fetch_one(pool).await.unwrap();
    let state = super::state(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let email = &state.services.email;
    let model = json!({"name":"<Policy & fixture>","reset_link":"https://example.test/reset?token=private-fixture"});
    let before = adapter.requests.lock().unwrap().len();
    email
        .send_transactional(pool, "policy-hosted@example.test", alias, model.clone())
        .await
        .unwrap();
    assert_eq!(
        adapter.requests.lock().unwrap()[before]["TemplateAlias"],
        alias
    );
    let path = format!("/api/admin/email/templates/{id}");
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            Some(admin),
            json!({"name":"Renamed hosted display name"})
        )
        .await
        .0,
        StatusCode::OK
    );
    email
        .send_transactional(pool, "policy-name-only@example.test", alias, model.clone())
        .await
        .unwrap();
    assert_eq!(
        adapter.requests.lock().unwrap()[before + 1]["TemplateAlias"],
        alias,
        "Display-name edits preserve hosted delivery"
    );
    let saved=request(api,"PUT",&path,Some(admin),json!({"subject":"Saved {{name}}","body_html":"<p>Saved {{name}}</p>","body_text":"Saved text {{name}}"})).await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    email
        .send_transactional(pool, "policy-saved@example.test", alias, model.clone())
        .await
        .unwrap();
    {
        let calls = adapter.requests.lock().unwrap();
        let saved = &calls[before + 2];
        assert!(saved["TemplateAlias"].is_null());
        assert_eq!(saved["Subject"], "Saved <Policy & fixture>");
        assert_eq!(
            saved["HtmlBody"],
            "<p>Saved &lt;Policy &amp; fixture&gt;</p>"
        );
        assert_eq!(saved["TextBody"], "Saved text <Policy & fixture>");
    }
    let cold = super::state(revolution_api::db::Database { pool: pool.clone() }, None).await;
    cold.services
        .email
        .send_transactional(pool, "policy-cold@example.test", alias, model.clone())
        .await
        .unwrap();
    assert_eq!(
        adapter.requests.lock().unwrap().last().unwrap()["HtmlBody"],
        "<p>Saved &lt;Policy &amp; fixture&gt;</p>"
    );
    let logged:serde_json::Value=sqlx::query_scalar("SELECT to_jsonb(e) FROM email_logs e WHERE to_email='policy-cold@example.test' ORDER BY id DESC LIMIT 1").fetch_one(pool).await.unwrap();
    assert_eq!(logged["status"], "sent");
    assert_eq!(logged["model"]["reset_link"], "[redacted]");
    assert_eq!(
        request(api, "PUT", &path, Some(admin), json!({"is_active":false}))
            .await
            .0,
        StatusCode::OK
    );
    let calls = adapter.requests.lock().unwrap().len();
    assert!(
        email
            .send_transactional(pool, "policy-inactive@example.test", alias, model.clone())
            .await
            .is_err()
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), calls);
    assert_eq!(
        request(api, "PUT", &path, Some(admin), json!({"is_active":true}))
            .await
            .0,
        StatusCode::OK
    );
    let renamed = "renamed-policy-fixture";
    assert_eq!(
        request(api, "PUT", &path, Some(admin), json!({"slug":renamed}))
            .await
            .0,
        StatusCode::OK
    );
    assert!(
        email
            .send_transactional(pool, "policy-old-alias@example.test", alias, model.clone())
            .await
            .is_err()
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), calls);
    email
        .send_transactional(pool, "policy-renamed@example.test", renamed, model.clone())
        .await
        .unwrap();
    assert_eq!(
        adapter.requests.lock().unwrap().last().unwrap()["HtmlBody"],
        "<p>Saved &lt;Policy &amp; fixture&gt;</p>"
    );
    assert_eq!(
        request(api, "DELETE", &path, Some(admin), json!(null))
            .await
            .0,
        StatusCode::OK
    );
    let calls = adapter.requests.lock().unwrap().len();
    assert!(
        email
            .send_transactional(pool, "policy-deleted@example.test", renamed, model.clone())
            .await
            .is_err()
    );
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        calls,
        "Deletion must not revive hosted content"
    );
    let cold = super::state(revolution_api::db::Database { pool: pool.clone() }, None).await;
    assert!(
        cold.services
            .email
            .send_transactional(
                pool,
                "policy-deleted-cold@example.test",
                renamed,
                model.clone()
            )
            .await
            .is_err()
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), calls);
    let payload = json!({"name":"Explicit policy recreation","slug":renamed,"subject":"Restored {{name}}","body_html":"<p>Restored {{name}}</p>"});
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO template_policy_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let rollback = request(
        api,
        "POST",
        "/api/admin/email/templates",
        Some(admin),
        payload.clone(),
    )
    .await;
    sqlx::query("ALTER TABLE template_policy_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(rollback.0, StatusCode::SERVICE_UNAVAILABLE);
    assert!(
        sqlx::query_scalar::<_, bool>(
            "SELECT EXISTS(SELECT 1 FROM email_template_revocations WHERE alias=$1)"
        )
        .bind(renamed)
        .fetch_one(pool)
        .await
        .unwrap()
    );
    assert!(
        email
            .send_transactional(pool, "policy-rollback@example.test", renamed, model.clone())
            .await
            .is_err()
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), calls);
    let recreated = request(
        api,
        "POST",
        "/api/admin/email/templates",
        Some(admin),
        payload,
    )
    .await;
    assert_eq!(recreated.0, StatusCode::OK, "{recreated:?}");
    email
        .send_transactional(
            pool,
            "policy-recreated@example.test",
            renamed,
            model.clone(),
        )
        .await
        .unwrap();
    assert_eq!(
        adapter.requests.lock().unwrap().last().unwrap()["HtmlBody"],
        "<p>Restored &lt;Policy &amp; fixture&gt;</p>"
    );
    let calls = adapter.requests.lock().unwrap().len();
    sqlx::query("ALTER TABLE email_template_revocations RENAME TO template_policy_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let unavailable = email
        .send_transactional(
            pool,
            "policy-storage@example.test",
            "otherwise-legacy-hosted-fixture",
            model,
        )
        .await;
    sqlx::query("ALTER TABLE template_policy_unavailable RENAME TO email_template_revocations")
        .execute(pool)
        .await
        .unwrap();
    assert!(unavailable.is_err());
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        calls,
        "Unavailable policy never falls back to hosted content"
    );
    let status:String=sqlx::query_scalar("SELECT status FROM email_logs WHERE to_email='policy-storage@example.test' ORDER BY id DESC LIMIT 1").fetch_one(pool).await.unwrap();
    assert_eq!(status, "unavailable");
}
