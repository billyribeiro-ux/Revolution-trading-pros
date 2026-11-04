//! Counts only actual provider acceptances linked to physical template identity.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let base = "/api/admin/email/templates";
    let admin = Some(tokens[0].as_str());
    let alias = "catalog-identity-fixture";
    let state = super::state(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let email = &state.services.email;
    let model = json!({"name":"Catalog fixture"});
    email
        .send_transactional(pool, "catalog-before@example.test", alias, model.clone())
        .await
        .unwrap();
    let payload = json!({"name":"Catalog identity fixture","slug":alias,"subject":"Catalog {{name}}","body_html":"<p>Catalog {{name}}</p>","body_text":"Catalog text {{name}}","category":"catalog-fixture","description":"Durable description","thumbnail":"/fixture.png","design_json":{"blocks":[{"text":"Saved"}]},"tags":["fixture","catalog"],"is_public":true,"variables":["name"]});
    let created = request(api, "POST", base, admin, payload.clone()).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["data"]["id"].as_i64().unwrap();
    let path = format!("{base}/{id}");
    assert_eq!(
        created.1["data"]["usage_count"], 0,
        "A reused alias must not inherit historical receipts"
    );
    for key in [
        "description",
        "thumbnail",
        "design_json",
        "tags",
        "is_public",
    ] {
        assert_eq!(created.1["data"][key], payload[key]);
    }
    let cold = super::app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    let reloaded = request(&cold, "GET", &path, admin, Value::Null).await;
    assert_eq!(reloaded.0, StatusCode::OK);
    assert_eq!(reloaded.1["data"]["design_json"], payload["design_json"]);
    for (field, value) in [
        ("tags", json!([1])),
        ("is_public", json!("true")),
        ("thumbnail", json!("javascript:alert(1)")),
        ("description", json!("bad\0text")),
        ("design_json", json!({"text":"x".repeat(1_000_001)})),
    ] {
        let invalid = request(api, "PUT", &path, admin, json!({field:value})).await;
        assert_eq!(invalid.0, StatusCode::BAD_REQUEST, "{field}: {invalid:?}");
    }
    assert_eq!(
        request(api, "PUT", &path, admin, json!({"usage_count":999}))
            .await
            .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    let selected = request(
        api,
        "GET",
        &format!("{base}?category=CATALOG-FIXTURE&per_page=1"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(selected.0, StatusCode::OK, "{selected:?}");
    assert_eq!(selected.1["meta"]["total"], 1);
    assert_eq!(selected.1["data"][0]["id"], id);
    email
        .send_transactional(pool, "catalog-app@example.test", alias, model.clone())
        .await
        .unwrap();
    let sent = request(
        api,
        "POST",
        &format!("{path}/test"),
        admin,
        json!({"email":"catalog-admin-test@example.test","data":model}),
    )
    .await;
    assert_eq!(sent.0, StatusCode::OK, "{sent:?}");
    let counted = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(counted.1["data"]["usage_count"], 2);
    let linked:(i64,String)=sqlx::query_as("SELECT email_template_id,subject FROM email_logs WHERE to_email='catalog-app@example.test'").fetch_one(pool).await.unwrap();
    assert_eq!(linked, (id, "Catalog Catalog fixture".into()));
    let duplicate_path = format!("{path}/duplicate");
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(api, "POST", &duplicate_path, token, Value::Null)
                .await
                .0,
            status
        );
    }
    let before = adapter.requests.lock().unwrap().len();
    let copy = request(api, "POST", &duplicate_path, admin, Value::Null).await;
    assert_eq!(copy.0, StatusCode::OK, "{copy:?}");
    let copy_id = copy.1["data"]["id"].as_i64().unwrap();
    let copy_alias = copy.1["data"]["slug"].as_str().unwrap();
    assert_ne!(copy_id, id);
    assert_ne!(copy_alias, alias);
    assert_eq!(copy.1["data"]["usage_count"], 0);
    assert_eq!(copy.1["data"]["design_json"], payload["design_json"]);
    assert_eq!(copy.1["data"]["body_html"], payload["body_html"]);
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before,
        "Duplication must never deliver mail"
    );
    email
        .send_transactional(pool, "catalog-copy@example.test", copy_alias, model.clone())
        .await
        .unwrap();
    assert_eq!(
        request(api, "GET", &format!("{base}/{copy_id}"), admin, Value::Null)
            .await
            .1["data"]["usage_count"],
        1
    );
    let cleared=request(api,"PUT",&path,admin,json!({"description":null,"thumbnail":null,"design_json":null,"tags":null,"is_public":null})).await;
    assert_eq!(cleared.0, StatusCode::OK, "{cleared:?}");
    assert!(cleared.1["data"]["description"].is_null());
    assert!(cleared.1["data"]["design_json"].is_null());
    assert_eq!(cleared.1["data"]["tags"], json!([]));
    assert_eq!(cleared.1["data"]["is_public"], false);
    let renamed = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"slug":"catalog-renamed-fixture"}),
    )
    .await;
    assert_eq!(renamed.0, StatusCode::OK, "{renamed:?}");
    assert_eq!(renamed.1["data"]["usage_count"], 2);
    let recreated = request(api, "POST", base, admin, payload).await;
    assert_eq!(recreated.0, StatusCode::OK, "{recreated:?}");
    let new_id = recreated.1["data"]["id"].as_i64().unwrap();
    assert_ne!(new_id, id);
    assert_eq!(recreated.1["data"]["usage_count"], 0);
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO catalog_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let rollback = request(api, "POST", &duplicate_path, admin, Value::Null).await;
    sqlx::query("ALTER TABLE catalog_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(rollback.0, StatusCode::SERVICE_UNAVAILABLE);
    let copies: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM email_templates WHERE name='Catalog identity fixture (Copy)'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(copies, 1, "Failed audit must roll back the duplicate");
    let deleted = request(api, "DELETE", &path, admin, Value::Null).await;
    assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
    assert_eq!(deleted.1["deleted"], true);
    let old: Option<i64> = sqlx::query_scalar(
        "SELECT email_template_id FROM email_logs WHERE to_email='catalog-app@example.test'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(old, None);
    assert_eq!(
        request(api, "GET", &format!("{base}/{new_id}"), admin, Value::Null)
            .await
            .1["data"]["usage_count"],
        0
    );
}
