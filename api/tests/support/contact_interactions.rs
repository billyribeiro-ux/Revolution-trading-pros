//! Explicit contact actions persist real records and provider acknowledgments.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;
const ROOT: &str = "/api/admin/crm/contacts";
async fn create(api: &Router, token: &str, suffix: &str) -> i64 {
    let row=request(api,"POST",ROOT,Some(token),json!({"email":format!("contact-interaction-{suffix}@example.test"),"first_name":"<Actual & Personal>","source":"contact-interactions","custom_fields":{"retained":"original"},"lifecycle_stage":"mql"})).await;
    assert_eq!(row.0, StatusCode::OK, "{row:?}");
    let at = chrono::DateTime::parse_from_rfc3339(row.1["created_at"].as_str().unwrap()).unwrap();
    assert!(
        (chrono::Utc::now().timestamp() - at.timestamp()).abs() < 10,
        "New contact timestamps use actual UTC wall time even when PostgreSQL defaults to New York"
    );
    row.1["id"].as_i64().unwrap()
}
fn message(key: Uuid, email: &str, subject: &str) -> Value {
    json!({"request_id":key,"expected_email":email,"subject":subject,"body_html":"<p>Saved personal {{contact.first_name}} {{email}}</p>","text_body":"Personal alternative {{name}}"})
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let id = create(api, &tokens[0], "primary").await;
    let path = format!("{ROOT}/{id}");
    let owner: i64 =
        sqlx::query_scalar("SELECT id FROM users WHERE email='admin@email-completion.test'")
            .fetch_one(pool)
            .await
            .unwrap();
    let selected = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"owner_id":owner.to_string(),"lifecycle_stage":"sql"}),
    )
    .await;
    assert_eq!(selected.0, StatusCode::OK, "{selected:?}");
    let at =
        chrono::DateTime::parse_from_rfc3339(selected.1["updated_at"].as_str().unwrap()).unwrap();
    assert!(
        (chrono::Utc::now().timestamp() - at.timestamp()).abs() < 10,
        "Updated contact wall time must not inherit the database timezone"
    );
    assert_eq!(selected.1["owner_id"], owner);
    assert_eq!(selected.1["lifecycle_stage"], "sql");
    assert!(selected.1["user_id"].is_null());
    assert_eq!(selected.1["custom_fields"]["retained"], "original");
    let edited = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"job_title":"Recorded analyst"}),
    )
    .await;
    assert_eq!(edited.1["lifecycle_stage"], "sql");
    assert_eq!(edited.1["custom_fields"]["retained"], "original");
    assert!(edited.1["last_activity_at"].is_string());
    let filtered=request(api,"GET",&format!("{ROOT}?search=contact-interaction-primary%40example.test&lifecycle_stage=sql&owner_id={owner}&sort_by=last_activity_at&sort_order=desc"),admin,Value::Null).await;
    assert_eq!(filtered.0, StatusCode::OK, "{filtered:?}");
    assert_eq!(filtered.1["meta"]["total"], 1);
    assert_eq!(filtered.1["data"][0]["id"], id);
    let inactive:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,is_active) VALUES('contact-inactive-owner@example.test','Inactive owner fixture','not-a-login-password','user',false) RETURNING id").fetch_one(pool).await.unwrap();
    for payload in [
        json!({"owner_id":inactive}),
        json!({"owner_id":"invalid"}),
        json!({"lifecycle_stage":"invented"}),
    ] {
        assert_eq!(
            request(api, "PUT", &path, admin, payload).await.0,
            StatusCode::BAD_REQUEST
        );
    }
    let cleared = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"owner_id":null,"lifecycle_stage":null}),
    )
    .await;
    assert_eq!(cleared.0, StatusCode::OK, "{cleared:?}");
    assert!(cleared.1["owner_id"].is_null() && cleared.1["lifecycle_stage"].is_null());
    assert_eq!(cleared.1["custom_fields"]["retained"], "original");
    let unassigned = request(
        api,
        "GET",
        &format!("{ROOT}?search=contact-interaction-primary%40example.test&owner_id=unassigned"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(unassigned.1["meta"]["total"], 1);
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}?unknown_filter=ignored"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    lead_contract(api, pool, tokens).await;
    let tag = request(
        api,
        "POST",
        "/api/admin/crm/contact-tags",
        admin,
        json!({"title":"Contact interaction tag","color":"#345678"}),
    )
    .await;
    assert_eq!(tag.0, StatusCode::OK, "{tag:?}");
    let tag = tag.1["id"].as_i64().unwrap();
    let second_tag = request(
        api,
        "POST",
        "/api/admin/crm/contact-tags",
        admin,
        json!({"title":"Contact interaction second tag"}),
    )
    .await;
    assert_eq!(second_tag.0, StatusCode::OK, "{second_tag:?}");
    let second_tag = second_tag.1["id"].as_i64().unwrap();
    let second_tag_key = second_tag.to_string();
    let list = request(
        api,
        "POST",
        "/api/admin/crm/lists",
        admin,
        json!({"title":"Contact interaction list"}),
    )
    .await;
    assert_eq!(list.0, StatusCode::OK, "{list:?}");
    let list = list.1["id"].as_i64().unwrap();
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, url, payload) in [
            ("GET", format!("{path}/notes"), Value::Null),
            ("POST", format!("{path}/notes"), json!({"content":"Denied"})),
            ("GET", format!("{path}/emails"), Value::Null),
            (
                "POST",
                format!("{path}/send-email"),
                message(
                    Uuid::new_v4(),
                    "contact-interaction-primary@example.test",
                    "Denied",
                ),
            ),
            ("GET", format!("{path}/tags"), Value::Null),
            ("POST", format!("{path}/tags"), json!({"tag_id":tag})),
            ("DELETE", format!("{path}/tags/{tag}"), Value::Null),
            ("GET", format!("{path}/lists"), Value::Null),
            ("POST", format!("{path}/lists"), json!({"list_id":list})),
            ("DELETE", format!("{path}/lists/{list}"), Value::Null),
        ] {
            assert_eq!(request(api, method, &url, token, payload).await.0, status);
        }
    }
    for endpoint in ["notes", "emails", "tags", "lists"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{ROOT}/9223372036854775807/{endpoint}"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::NOT_FOUND
        );
    }
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/notes"),
            admin,
            json!({"content":" "})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let note = request(
        api,
        "POST",
        &format!("{path}/notes"),
        admin,
        json!({"content":"  Actual persisted personal note  "}),
    )
    .await;
    assert_eq!(note.0, StatusCode::OK, "{note:?}");
    assert_eq!(note.1["data"]["content"], "Actual persisted personal note");
    assert_eq!(note.1["data"]["created_by"]["id"], owner.to_string());
    let notes = request(
        api,
        "GET",
        &format!("{path}/notes?per_page=1"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(notes.1["meta"]["total"], 1);
    assert_eq!(notes.1["data"][0], note.1["data"]);
    sqlx::query("UPDATE contacts SET tags='[\"Unregistered actual contact tag\"]' WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let legacy_tags = request(api, "GET", &format!("{path}/tags"), admin, Value::Null).await;
    assert_eq!(legacy_tags.0, StatusCode::OK, "{legacy_tags:?}");
    assert!(legacy_tags.1["data"][0]["id"].is_null());
    assert_eq!(
        legacy_tags.1["data"][0]["definition_status"],
        "unregistered"
    );
    for count in [1, 0] {
        let ack = request(
            api,
            "POST",
            &format!("{path}/tags"),
            admin,
            json!({"tag_id":tag.to_string()}),
        )
        .await;
        assert_eq!(ack.0, StatusCode::OK, "{ack:?}");
        assert_eq!(ack.1["contact_id"], id.to_string());
        assert_eq!(ack.1["tag_id"], tag.to_string());
        assert_eq!(ack.1["applied_count"], count);
    }
    for count in [1, 0] {
        let ack = request(
            api,
            "POST",
            &format!("{path}/lists"),
            admin,
            json!({"list_id":list.to_string()}),
        )
        .await;
        assert_eq!(ack.0, StatusCode::OK, "{ack:?}");
        assert_eq!(ack.1["added_count"], count);
    }
    let memberships = request(api, "GET", &format!("{path}/lists"), admin, Value::Null).await;
    assert_eq!(memberships.1["meta"]["total"], 1);
    assert!(memberships.1["data"][0]["subscribed_at"].is_string());
    let other = create(api, &tokens[0], "other").await;
    let bulk = request(
        api,
        "POST",
        "/api/admin/crm/contact-tags/bulk-apply",
        admin,
        json!({"tag_ids":[tag,second_tag],"contact_ids":[id,other]}),
    )
    .await;
    assert_eq!(bulk.0, StatusCode::OK, "{bulk:?}");
    assert_eq!(bulk.1["applied_count"], 3);
    let members = request(
        api,
        "GET",
        &format!("/api/admin/crm/contact-tags/{tag}/contacts?per_page=1&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(members.0, StatusCode::OK, "{members:?}");
    assert_eq!(members.1["meta"]["total"], 2);
    assert_eq!(members.1["data"].as_array().unwrap().len(), 1);
    let removed = request(
        api,
        "POST",
        &format!("/api/admin/crm/contact-tags/{second_tag}/remove"),
        admin,
        json!({"contact_ids":[id,other]}),
    )
    .await;
    assert_eq!(removed.1["removed_count"], 2);
    let missing = request(
        api,
        "POST",
        "/api/admin/crm/contact-tags/bulk-apply",
        admin,
        json!({"tag_ids":[second_tag],"contact_ids":[id,9223372036854775807_i64]}),
    )
    .await;
    assert_eq!(missing.0, StatusCode::NOT_FOUND);
    let unchanged = request(api, "GET", &format!("{path}/tags"), admin, Value::Null).await;
    assert!(!unchanged.1["data"].as_array().unwrap().iter().any(|row| {
        row["id"]
            .as_str()
            .is_some_and(|value| value == second_tag_key)
    }));
    let before = adapter.requests.lock().unwrap().len();
    let rollback = Uuid::new_v4();
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO contact_interaction_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed_note = request(
        api,
        "POST",
        &format!("{path}/notes"),
        admin,
        json!({"content":"Must roll back note"}),
    )
    .await;
    let failed_tag = request(
        api,
        "POST",
        &format!("{path}/tags"),
        admin,
        json!({"tag_id":second_tag}),
    )
    .await;
    let failed_list = request(
        api,
        "DELETE",
        &format!("{path}/lists/{list}"),
        admin,
        Value::Null,
    )
    .await;
    let failed_mail = request(
        api,
        "POST",
        &format!("{path}/send-email"),
        admin,
        message(
            rollback,
            "contact-interaction-primary@example.test",
            "Must not submit",
        ),
    )
    .await;
    sqlx::query("ALTER TABLE contact_interaction_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    for result in [failed_note, failed_tag, failed_list, failed_mail] {
        assert_eq!(result.0, StatusCode::SERVICE_UNAVAILABLE, "{result:?}");
    }
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    let intents: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_contact_mail_requests WHERE request_id=$1")
            .bind(rollback)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(intents, 0);
    let receipts:i64=sqlx::query_scalar("SELECT COUNT(*) FROM email_logs WHERE template_type='crm_contact_mail' AND model->>'request_id'=$1").bind(rollback.to_string()).fetch_one(pool).await.unwrap();
    assert_eq!(
        receipts, 0,
        "Failed audit cannot leave a staged email receipt"
    );
    assert_eq!(
        request(api, "GET", &format!("{path}/notes"), admin, Value::Null)
            .await
            .1["meta"]["total"],
        1
    );
    assert_eq!(
        request(api, "GET", &format!("{path}/lists"), admin, Value::Null)
            .await
            .1["meta"]["total"],
        1
    );
    assert_eq!(
        request(api, "GET", &format!("{path}/tags"), admin, Value::Null).await,
        unchanged
    );
    rename_overlap(api, pool, tokens, other, second_tag).await;
    personal(api, pool, tokens, adapter, id, other).await;
    for count in [1, 0] {
        let ack = request(
            api,
            "DELETE",
            &format!("{ROOT}/{other}/tags/{tag}"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(ack.0, StatusCode::OK, "{ack:?}");
        assert_eq!(ack.1["removed_count"], count);
    }
    for count in [1, 0] {
        let ack = request(
            api,
            "DELETE",
            &format!("{path}/lists/{list}"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(ack.0, StatusCode::OK, "{ack:?}");
        assert_eq!(ack.1["removed_count"], count);
    }
}
async fn lead_contract(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = Some(tokens[0].as_str());
    let created=request(api,"POST","/api/admin/crm/leads",admin,json!({"email":"contact-interaction-lead@example.test","lifecycle_stage":"lead","custom_fields":{"preserved":true}})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let at =
        chrono::DateTime::parse_from_rfc3339(created.1["created_at"].as_str().unwrap()).unwrap();
    assert!(
        (chrono::Utc::now().timestamp() - at.timestamp()).abs() < 10,
        "Lead creation uses the same explicit UTC writer"
    );
    let id = created.1["id"].as_str().unwrap();
    assert_eq!(created.1["lifecycle_stage"], "lead");
    assert!(created.1["lead_score"].is_null() && created.1["is_hot"].is_null());
    let updated = request(
        api,
        "PUT",
        &format!("/api/admin/crm/leads/{id}"),
        admin,
        json!({"lifecycle_stage":"sql","job_title":"Actual lead title"}),
    )
    .await;
    assert_eq!(updated.0, StatusCode::OK, "{updated:?}");
    assert_eq!(updated.1["lifecycle_stage"], "sql");
    assert_eq!(updated.1["custom_fields"]["preserved"], true);
    let filtered = request(
        api,
        "GET",
        "/api/admin/crm/leads?search=contact-interaction-lead%40example.test&lifecycle_stage=sql",
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(filtered.0, StatusCode::OK, "{filtered:?}");
    assert_eq!(filtered.1["meta"]["total"], 1);
    assert_eq!(filtered.1["data"][0]["id"], id);
    let stats = request(api, "GET", "/api/admin/crm/leads/stats", admin, Value::Null).await;
    assert_eq!(stats.0, StatusCode::OK, "{stats:?}");
    assert!(stats.1["total_value"].is_null());
    assert!(stats.1["unknown_score_leads"].as_i64().unwrap() >= 1);
    let raw:i64=sqlx::query_scalar("SELECT COUNT(*) FROM contacts WHERE email='contact-interaction-lead@example.test' AND custom_fields->>'lifecycle_stage'='sql'").fetch_one(pool).await.unwrap();
    assert_eq!(raw, 1);
}
async fn rename_overlap(api: &Router, pool: &PgPool, tokens: &[String], contact: i64, tag: i64) {
    let admin = Some(tokens[0].as_str());
    let mut hold = pool.begin().await.unwrap();
    sqlx::query("LOCK TABLE contacts IN SHARE ROW EXCLUSIVE MODE")
        .execute(&mut *hold)
        .await
        .unwrap();
    let release = async {
        tokio::time::timeout(std::time::Duration::from_secs(5),async{
            loop{let waiting:i64=sqlx::query_scalar("SELECT COUNT(*) FROM pg_stat_activity WHERE datname=current_database() AND wait_event_type='Lock' AND query LIKE 'LOCK TABLE contacts IN SHARE ROW EXCLUSIVE MODE%'").fetch_one(pool).await.unwrap();if waiting>=2{break;}tokio::time::sleep(std::time::Duration::from_millis(20)).await;}
        }).await.unwrap();
        hold.commit().await.unwrap();
    };
    let contact_tags_path = format!("{ROOT}/{contact}/tags");
    let tag_path = format!("/api/admin/crm/contact-tags/{tag}");
    let (apply, rename, ()) = tokio::join!(
        request(
            api,
            "POST",
            &contact_tags_path,
            admin,
            json!({"tag_id":tag})
        ),
        request(
            api,
            "PUT",
            &tag_path,
            admin,
            json!({"title":"Contact interaction renamed concurrently"})
        ),
        release
    );
    assert_eq!(apply.0, StatusCode::OK, "{apply:?}");
    assert_eq!(rename.0, StatusCode::OK, "{rename:?}");
    let saved: Value = sqlx::query_scalar("SELECT tags FROM contacts WHERE id=$1")
        .bind(contact)
        .fetch_one(pool)
        .await
        .unwrap();
    let tags = saved.as_array().unwrap();
    assert!(tags.contains(&json!("Contact interaction renamed concurrently")));
    assert!(!tags.contains(&json!("Contact interaction second tag")));
}
async fn personal(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    id: i64,
    other: i64,
) {
    let admin = Some(tokens[0].as_str());
    let path = format!("{ROOT}/{id}");
    let endpoint = format!("{path}/send-email");
    let email = "contact-interaction-primary@example.test";
    for value in [
        json!({}),
        json!({"request_id":Uuid::new_v4(),"expected_email":email,"subject":"Missing body"}),
        json!({"request_id":Uuid::new_v4(),"expected_email":email,"subject":"Unknown field","body_html":"<p>Body</p>","from_email":"spoof@example.test"}),
    ] {
        assert_eq!(
            request(api, "POST", &endpoint, admin, value).await.0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }
    for value in [
        message(Uuid::nil(), email, "Invalid ID"),
        message(Uuid::new_v4(), "invalid", "Invalid email"),
        message(Uuid::new_v4(), email, "Header\r\nBcc: other@example.test"),
    ] {
        assert_eq!(
            request(api, "POST", &endpoint, admin, value).await.0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        request(
            api,
            "POST",
            &endpoint,
            admin,
            message(Uuid::new_v4(), "other@example.test", "Stale recipient")
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let template:i64=sqlx::query_scalar("INSERT INTO email_templates(name,slug,subject,html_content,is_active) VALUES('Contact personal origin','contact-personal-origin','Origin saved subject','<p>Origin saved template</p>',true) RETURNING id").fetch_one(pool).await.unwrap();
    let key = Uuid::new_v4();
    let mut payload = message(key, email, " Personal {{contact.first_name}} ");
    payload["template_id"] = json!(template.to_string());
    let before = adapter.requests.lock().unwrap().len();
    let accepted = request(api, "POST", &endpoint, admin, payload.clone()).await;
    assert_eq!(accepted.0, StatusCode::OK, "{accepted:?}");
    assert_eq!(accepted.1["success"], true);
    assert_eq!(
        accepted.1["data"]["source_subject"],
        "Personal {{contact.first_name}}"
    );
    assert_eq!(accepted.1["data"]["template_id"], template.to_string());
    assert_eq!(
        accepted.1["data"]["source_hash"].as_str().unwrap().len(),
        64
    );
    let receipt = &accepted.1["data"]["submissions"][0];
    assert_eq!(receipt["status"], "accepted");
    assert_eq!(receipt["recipient"], email);
    assert_eq!(receipt["subject"], "Personal <Actual & Personal>");
    assert!(
        receipt["accepted_at"].is_string()
            && !receipt["provider_message_id"].as_str().unwrap().is_empty()
    );
    let actual = { adapter.requests.lock().unwrap()[before].clone() };
    assert_eq!(actual["Subject"], "Personal <Actual & Personal>");
    assert!(
        actual["HtmlBody"]
            .as_str()
            .unwrap()
            .contains("&lt;Actual &amp; Personal&gt;")
    );
    assert!(
        actual["TextBody"]
            .as_str()
            .unwrap()
            .contains("Personal alternative <Actual & Personal>")
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    let outbox: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaign_deliveries WHERE contact_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        outbox, 0,
        "A personal message cannot create a campaign audience"
    );
    assert_eq!(
        request(api, "POST", &endpoint, admin, payload.clone()).await,
        accepted
    );
    let history = request(
        api,
        "GET",
        &format!("{path}/emails?request_id={key}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(history.0, StatusCode::OK, "{history:?}");
    assert_eq!(history.1["meta"]["total"], 1);
    assert_eq!(history.1["data"][0]["request_id"], key.to_string());
    assert_eq!(history.1["data"][0]["status"], "accepted");
    assert!(history.1["data"][0]["opened_at"].is_null());
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/{other}/emails"),
            admin,
            Value::Null
        )
        .await
        .1["meta"]["total"],
        0
    );
    let mut changed = payload.clone();
    changed["body_html"] = json!("Changed intent");
    assert_eq!(
        request(api, "POST", &endpoint, admin, changed).await.0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            admin,
            json!({"first_name":"After accepted source edit"})
        )
        .await
        .0,
        StatusCode::OK
    );
    sqlx::query("DELETE FROM email_templates WHERE id=$1")
        .bind(template)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(api, "POST", &endpoint, admin, payload.clone()).await,
        accepted,
        "Source/template edits cannot resubmit or rewrite a frozen intent"
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    let snapshot: Value = sqlx::query_scalar(
        "SELECT content_snapshot FROM crm_contact_mail_requests WHERE request_id=$1",
    )
    .bind(key)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(snapshot["subject"], "Personal <Actual & Personal>");
    assert!(
        snapshot["html"]
            .as_str()
            .unwrap()
            .contains("&lt;Actual &amp; Personal&gt;")
    );
    let second = request(
        api,
        "POST",
        &endpoint,
        admin,
        message(Uuid::new_v4(), email, "Explicit new contact message"),
    )
    .await;
    assert_eq!(second.0, StatusCode::OK, "{second:?}");
    let first = request(
        api,
        "GET",
        &format!("{path}/emails?per_page=1&page=1"),
        admin,
        Value::Null,
    )
    .await;
    let next = request(
        api,
        "GET",
        &format!("{path}/emails?per_page=1&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(first.1["meta"]["total"], 2);
    assert_ne!(first.1["data"][0]["id"], next.1["data"][0]["id"]);
    for (subject, status) in [
        ("fixture-rejected", "rejected"),
        ("fixture-unknown", "unknown"),
    ] {
        let person = create(api, &tokens[0], status).await;
        let url = format!("{ROOT}/{person}/send-email");
        let body = message(
            Uuid::new_v4(),
            &format!("contact-interaction-{status}@example.test"),
            subject,
        );
        let result = request(api, "POST", &url, admin, body.clone()).await;
        assert_eq!(result.0, StatusCode::BAD_GATEWAY, "{result:?}");
        assert_eq!(result.1["data"]["submissions"][0]["status"], status);
        let count = adapter.requests.lock().unwrap().len();
        assert_eq!(request(api, "POST", &url, admin, body).await, result);
        assert_eq!(adapter.requests.lock().unwrap().len(), count);
    }
    held(api, pool, tokens, adapter, false).await;
    held(api, pool, tokens, adapter, true).await;
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    let single = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    let person = create(&single, &tokens[0], "single").await;
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        request(
            &single,
            "POST",
            &format!("{ROOT}/{person}/send-email"),
            admin,
            message(
                Uuid::new_v4(),
                "contact-interaction-single@example.test",
                "One connection personal message",
            ),
        ),
    )
    .await
    .unwrap();
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    one.close().await;
}
async fn held(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter, lose: bool) {
    let admin = Some(tokens[0].as_str());
    let suffix = if lose { "held-missing" } else { "held" };
    let id = create(api, &tokens[0], suffix).await;
    let endpoint = format!("{ROOT}/{id}/send-email");
    let key = Uuid::new_v4();
    let body = message(
        key,
        &format!("contact-interaction-{suffix}@example.test"),
        "fixture-inflight",
    );
    let before = adapter.requests.lock().unwrap().len();
    let during = async {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            adapter.inflight_started.notified(),
        )
        .await
        .unwrap();
        let replay = request(api, "POST", &endpoint, admin, body.clone()).await;
        assert_eq!(replay.0, StatusCode::ACCEPTED, "{replay:?}");
        assert_eq!(replay.1["data"]["submissions"][0]["status"], "processing");
        if lose {
            sqlx::query("DELETE FROM email_logs WHERE template_type='crm_contact_mail' AND model->>'request_id'=$1").bind(key.to_string()).execute(pool).await.unwrap();
        } else {
            assert_eq!(
                request(
                    api,
                    "PUT",
                    &format!("{ROOT}/{id}"),
                    admin,
                    json!({"email":"contact-interaction-changed@example.test"})
                )
                .await
                .0,
                StatusCode::OK
            );
        }
        adapter.inflight_release.notify_one();
    };
    let (result, ()) = tokio::join!(request(api, "POST", &endpoint, admin, body.clone()), during);
    assert_eq!(
        result.0,
        if lose {
            StatusCode::SERVICE_UNAVAILABLE
        } else {
            StatusCode::OK
        },
        "{result:?}"
    );
    assert_eq!(
        request(api, "POST", &endpoint, admin, body.clone()).await.0,
        result.0
    );
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before + 1,
        "In-flight edits or missing receipts cannot cause a second provider submission"
    );
    if lose {
        let history = request(
            api,
            "GET",
            &format!("{ROOT}/{id}/emails?request_id={key}"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(history.1["data"][0]["status"], "unknown");
        assert!(history.1["data"][0]["provider_message_id"].is_null());
    } else {
        let deleted = request(api, "DELETE", &format!("{ROOT}/{id}"), admin, Value::Null).await;
        assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
        assert_eq!(deleted.1["deleted"], true);
        assert_eq!(deleted.1["id"], id.to_string());
        assert_eq!(
            request(api, "POST", &endpoint, admin, body).await,
            result,
            "Accepted receipt survives source deletion without resubmission"
        );
        let history = request(
            api,
            "GET",
            &format!("{ROOT}/{id}/emails?request_id={key}"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(history.0, StatusCode::OK, "{history:?}");
        assert_eq!(history.1["data"][0]["status"], "accepted");
        assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    }
}
