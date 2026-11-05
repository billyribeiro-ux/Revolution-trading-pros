//! Contact history and configured scores use recorded facts and atomic writes.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/crm/contacts";
async fn settings(api: &Router, token: &str, value: Value) {
    let result = request(
        api,
        "PUT",
        "/api/admin/crm/settings",
        Some(token),
        json!({"settings":{"scoring_settings":value}}),
    )
    .await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
}
fn configured(max: i32) -> Value {
    json!({"enabled":true,"max_score":max,"rules":[
        {"id":"verified-tag","points":80,"conditions":[{"field":"tags","operator":"contains","value":"Contact policy VIP"}]},
        {"id":"customer","points":40,"conditions":[{"field":"status","operator":"equals","value":"customer"}]},
        {"id":"unqualified","points":-200,"conditions":[{"field":"status","operator":"equals","value":"unqualified"}]},
        {"id":"not-matching","points":5,"conditions":[{"field":"email","operator":"equals","value":"no-contact-policy-match@example.test"}]}
    ]})
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let saved: Option<Value> =
        sqlx::query_scalar("SELECT value FROM settings WHERE key='crm.settings'")
            .fetch_optional(pool)
            .await
            .unwrap();
    settings(api, &tokens[0], json!({"enabled":true,"max_score":100})).await;
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, path) in [
            ("GET", format!("{ROOT}/1/timeline")),
            ("POST", format!("{ROOT}/1/recalculate-score")),
        ] {
            assert_eq!(
                request(api, method, &path, token, Value::Null).await.0,
                status
            );
        }
    }
    for (method, url) in [
        ("GET", format!("{ROOT}/9223372036854775807/timeline")),
        (
            "POST",
            format!("{ROOT}/9223372036854775807/recalculate-score"),
        ),
    ] {
        assert_eq!(
            request(api, method, &url, admin, Value::Null).await.0,
            StatusCode::NOT_FOUND
        );
    }
    let created=request(api,"POST",ROOT,admin,json!({"email":"crm-contact-observed@example.test","first_name":"Observed","last_name":"Contact","status":"customer","source":"contact-observation-proof","tags":["Contact policy VIP"]})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["id"].as_i64().unwrap();
    let path = format!("{ROOT}/{id}");
    let score_path = format!("{path}/recalculate-score");
    assert!(created.1["score"].is_null());
    assert!(created.1["lead_score"].is_null());
    assert_eq!(created.1["scoring"]["status"], "unconfigured");
    let actor: i64 =
        sqlx::query_scalar("SELECT id FROM users WHERE email='admin@email-completion.test'")
            .fetch_one(pool)
            .await
            .unwrap();
    let event: (Option<i64>, Value) = sqlx::query_as(
        "SELECT created_by,metadata FROM crm_contact_events WHERE contact_id=$1 AND kind='created'",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(event.0, Some(actor));
    let timeline = request(api, "GET", &format!("{path}/timeline"), admin, Value::Null).await;
    assert_eq!(timeline.0, StatusCode::OK, "{timeline:?}");
    assert_eq!(timeline.1["data"].as_array().unwrap().len(), 1);
    assert_eq!(timeline.1["data"][0]["created_by"]["id"], actor.to_string());
    assert_eq!(
        request(api, "GET", &format!("{path}/timeline"), admin, Value::Null).await,
        timeline,
        "Repeated reads retain recorded timestamps"
    );
    for query in ["page=0", "per_page=0", "per_page=101", "page=100001"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{path}/timeline?{query}"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::UNPROCESSABLE_ENTITY
        );
    }
    let legacy:i64=sqlx::query_scalar("INSERT INTO contacts(email,score,created_at,updated_at) VALUES('crm-contact-legacy@example.test',88,'2019-01-02 03:04:05','2019-01-02 03:04:05') RETURNING id").fetch_one(pool).await.unwrap();
    // Emulate a pre-journal record; retain its actual stored creation timestamp.
    sqlx::query("DELETE FROM crm_contact_events WHERE contact_id=$1")
        .bind(legacy)
        .execute(pool)
        .await
        .unwrap();
    let legacy_path = format!("{ROOT}/{legacy}");
    let legacy_row = request(api, "GET", &legacy_path, admin, Value::Null).await;
    assert_eq!(legacy_row.0, StatusCode::OK, "{legacy_row:?}");
    assert_eq!(legacy_row.1["recorded_score"], 88);
    assert_eq!(legacy_row.1["historical_score"], 88);
    assert!(legacy_row.1["score"].is_null());
    let legacy_timeline = request(
        api,
        "GET",
        &format!("{legacy_path}/timeline"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(
        legacy_timeline.1["data"][0]["id"],
        format!("legacy-created-{legacy}")
    );
    assert!(
        legacy_timeline.1["data"][0]["occurred_at"]
            .as_str()
            .unwrap()
            .starts_with("2019-01-02T03:04:05")
    );
    let unknown:i64=sqlx::query_scalar("INSERT INTO contacts(email,status,created_at,updated_at,score) VALUES('crm-contact-null@example.test',NULL,NULL,NULL,NULL) RETURNING id").fetch_one(pool).await.unwrap();
    let unknown_row = request(api, "GET", &format!("{ROOT}/{unknown}"), admin, Value::Null).await;
    assert_eq!(unknown_row.0, StatusCode::OK, "{unknown_row:?}");
    assert!(
        unknown_row.1["status"].is_null()
            && unknown_row.1["created_at"].is_null()
            && unknown_row.1["score"].is_null()
    );
    assert!(unknown_row.1["facts"]["orders"].is_null());
    assert!(unknown_row.1["facts"]["unmeasured"]["engagement_score"].is_null());
    settings(api, &tokens[0], configured(100)).await;
    let scored = request(api, "POST", &score_path, admin, Value::Null).await;
    assert_eq!(scored.0, StatusCode::OK, "{scored:?}");
    assert_eq!(scored.1["score"], 100);
    assert_eq!(scored.1["scoring"]["status"], "calculated");
    assert_eq!(
        scored.1["scoring"]["contributions"][0]["applied_points"],
        80
    );
    assert_eq!(
        scored.1["scoring"]["contributions"][1]["applied_points"],
        40
    );
    assert_eq!(scored.1["scoring"]["contributions"][3]["applied_points"], 0);
    assert!(scored.1["scoring"]["evaluated_at"].is_string());
    let reloaded = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(
        reloaded.1["score"], 100,
        "Calculated assessment survives its own update and reload"
    );
    let raw: (i32, Value) =
        sqlx::query_as("SELECT score,score_assessment FROM contacts WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(raw.0, 100);
    assert_eq!(raw.1["raw_total"], 120);
    assert_eq!(raw.1["status"], "calculated");
    // Raw numeric predicates retain their established stored-field contract.
    let raw_match: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM contacts WHERE id=$1 AND score>=75")
            .bind(legacy)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(raw_match, 1);
    settings(api, &tokens[0], configured(60)).await;
    let stale = request(api, "GET", &path, admin, Value::Null).await;
    assert!(stale.1["score"].is_null());
    assert_eq!(stale.1["recorded_score"], 100);
    assert_eq!(stale.1["scoring"]["status"], "stale");
    assert_eq!(
        request(api, "POST", &score_path, admin, Value::Null)
            .await
            .1["score"],
        60
    );
    let changed = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"status":"unqualified","tags":[]}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert!(changed.1["score"].is_null());
    assert_eq!(changed.1["scoring"]["status"], "stale");
    let negative = request(api, "POST", &score_path, admin, Value::Null).await;
    assert_eq!(negative.0, StatusCode::OK, "{negative:?}");
    assert_eq!(
        negative.1["score"], 0,
        "Explicit negative points clamp at zero"
    );
    for policy in [
        json!({"enabled":false,"max_score":100}),
        json!({"enabled":true,"max_score":100}),
    ] {
        settings(api, &tokens[0], policy).await;
        let unknown = request(
            api,
            "POST",
            &format!("{legacy_path}/recalculate-score"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(unknown.0, StatusCode::OK, "{unknown:?}");
        assert!(unknown.1["score"].is_null());
        assert_eq!(unknown.1["recorded_score"], 88);
        assert_eq!(unknown.1["historical_score"], 88);
    }
    for field in ["score", "lead_score", "status); DROP TABLE contacts;--"] {
        settings(api,&tokens[0],json!({"enabled":true,"max_score":100,"rules":[{"id":"invalid","points":30,"conditions":[{"field":field,"operator":"equals","value":50}]}]})).await;
        assert_eq!(
            request(api, "POST", &score_path, admin, Value::Null)
                .await
                .0,
            if matches!(field, "score" | "lead_score") {
                StatusCode::UNPROCESSABLE_ENTITY
            } else {
                StatusCode::BAD_REQUEST
            }
        );
        let row = request(api, "GET", &path, admin, Value::Null).await;
        assert_eq!(row.1["scoring"]["status"], "invalid_policy");
        assert!(row.1["score"].is_null());
    }
    settings(api, &tokens[0], configured(100)).await;
    let before = request(api, "GET", &path, admin, Value::Null).await;
    let events_before: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_contact_events WHERE contact_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO contact_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let score_failed = request(api, "POST", &score_path, admin, Value::Null).await;
    let update_failed = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"first_name":"Must roll back"}),
    )
    .await;
    let delete_failed = request(api, "DELETE", &path, admin, Value::Null).await;
    let create_failed = request(
        api,
        "POST",
        ROOT,
        admin,
        json!({"email":"crm-contact-audit-rollback@example.test"}),
    )
    .await;
    sqlx::query("ALTER TABLE contact_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    for response in [score_failed, update_failed, delete_failed, create_failed] {
        assert_eq!(response.0, StatusCode::SERVICE_UNAVAILABLE, "{response:?}");
    }
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await,
        before,
        "Failed audits preserve contact, assessment and actor journal"
    );
    let events_after: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_contact_events WHERE contact_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(events_after, events_before);
    let leaked: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM contacts WHERE email='crm-contact-audit-rollback@example.test'",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(leaked, 0);
    let unknown_actor: Option<i64> = sqlx::query_scalar(
        "SELECT created_by FROM crm_contact_events WHERE contact_id=$1 ORDER BY id LIMIT 1",
    )
    .bind(unknown)
    .fetch_one(pool)
    .await
    .unwrap();
    assert!(
        unknown_actor.is_none(),
        "Direct/import/worker writes cannot invent an administrator actor"
    );
    let member:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role) VALUES('crm-contact-member-link@example.test','Linked fixture','not-a-login-password','user') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("UPDATE contacts SET user_id=$2 WHERE id=$1")
        .bind(id)
        .bind(member)
        .execute(pool)
        .await
        .unwrap();
    let duplicate:i64=sqlx::query_scalar("INSERT INTO contacts(email,source) VALUES('crm-contact-observed@example.test','same-address-unlinked') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO orders(user_id,order_number,status,total,currency,created_at,completed_at) VALUES($1,'crm-contact-order-fixture','completed',12,'USD','2024-02-03 01:02:03','2024-02-03 02:03:04')").bind(member).execute(pool).await.unwrap();
    let (stage, pipeline): (i64, i64) =
        sqlx::query_as("SELECT id,pipeline_id FROM crm_pipeline_stages ORDER BY id LIMIT 1")
            .fetch_one(pool)
            .await
            .unwrap();
    let deal:i64=sqlx::query_scalar("INSERT INTO crm_deals(name,contact_id,pipeline_id,stage_id,status) VALUES('Contact observed deal',$1,$2,$3,'open') RETURNING id").bind(id).bind(pipeline).bind(stage).fetch_one(pool).await.unwrap();
    let note = request(
        api,
        "POST",
        &format!("/api/admin/crm/deals/{deal}/notes"),
        admin,
        json!({"content":"Actual persisted contact-linked note"}),
    )
    .await;
    assert_eq!(note.0, StatusCode::OK, "{note:?}");
    sqlx::query("INSERT INTO crm_deal_stage_history(deal_id,to_stage_id,changed_by,reason) VALUES($1,$2,$3,'Actual stage history fixture')").bind(deal).bind(stage).bind(actor).execute(pool).await.unwrap();
    sqlx::query("INSERT INTO crm_deal_activities(deal_id,activity_type,title) VALUES($1,'stage_changed','Duplicate stage activity must be excluded')").bind(deal).execute(pool).await.unwrap();
    let measured = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(measured.1["facts"]["orders"]["completed"], 1);
    assert_eq!(measured.1["facts"]["deals"]["total"], 1);
    assert_eq!(measured.1["facts"]["deals"]["notes"], 1);
    assert!(measured.1["facts"]["email"]["human_engagement"].is_null());
    let duplicate_row = request(
        api,
        "GET",
        &format!("{ROOT}/{duplicate}"),
        admin,
        Value::Null,
    )
    .await;
    assert!(
        duplicate_row.1["facts"]["orders"].is_null(),
        "Another same-address contact cannot borrow the explicit member link"
    );
    observed_email(api, pool, tokens, adapter, id).await;
    let measured = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(measured.1["facts"]["email"]["accepted"], 1);
    assert_eq!(measured.1["facts"]["email"]["open_signals"], 1);
    assert_eq!(measured.1["facts"]["email"]["click_signals"], 1);
    assert!(measured.1["facts"]["email"]["human_engagement"].is_null());
    let full = request(
        api,
        "GET",
        &format!("{path}/timeline?per_page=100"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(full.0, StatusCode::OK, "{full:?}");
    let events = full.1["data"].as_array().unwrap();
    for kind in [
        "note",
        "deal_stage_changed",
        "email_accepted",
        "email_open_signal",
        "email_click_signal",
        "order_completed",
        "score_assessed",
    ] {
        assert!(
            events.iter().any(|event| event["type"] == kind),
            "Missing actual {kind}: {full:?}"
        );
    }
    assert!(
        !events
            .iter()
            .any(|event| event["title"] == "Duplicate stage activity must be excluded")
    );
    let first = request(
        api,
        "GET",
        &format!("{path}/timeline?per_page=2&page=1"),
        admin,
        Value::Null,
    )
    .await;
    let second = request(
        api,
        "GET",
        &format!("{path}/timeline?per_page=2&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(first.1["meta"]["total"], full.1["meta"]["total"]);
    assert!(first.1["data"].as_array().unwrap().iter().all(|one| {
        !second.1["data"]
            .as_array()
            .unwrap()
            .iter()
            .any(|two| one["id"] == two["id"])
    }));
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    let single = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        request(&single, "POST", &score_path, admin, Value::Null),
    )
    .await
    .unwrap();
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    one.close().await;
    assert_eq!(
        request(api, "DELETE", &legacy_path, admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    let deleted: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM crm_contact_events WHERE contact_id=$1 AND kind='deleted'",
    )
    .bind(legacy)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(deleted, 1);
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{legacy_path}/timeline"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    if let Some(saved) = saved {
        sqlx::query("UPDATE settings SET value=$1 WHERE key='crm.settings'")
            .bind(saved)
            .execute(pool)
            .await
            .unwrap();
    } else {
        sqlx::query("DELETE FROM settings WHERE key='crm.settings'")
            .execute(pool)
            .await
            .unwrap();
    }
}
async fn observed_email(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    id: i64,
) {
    use revolution_api::services::{campaign_delivery, campaign_mailer::CampaignMailer};
    let admin = Some(tokens[0].as_str());
    sqlx::query("INSERT INTO newsletter_subscribers(email,status,confirmed_at) VALUES('crm-contact-observed@example.test','confirmed',NOW())").execute(pool).await.unwrap();
    let created=request(api,"POST","/api/admin/crm/campaigns",admin,json!({"title":"Contact actual provider facts","email_subject":"Contact actual provider facts","email_body":"<p>Controlled contact sample</p>","subscriber_settings":{"advanced_filters":[{"field":"source","operator":"equals","value":"contact-observation-proof"}]}})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let campaign = created.1["id"].as_str().unwrap().parse::<i64>().unwrap();
    let sent = request(
        api,
        "POST",
        &format!("/api/admin/crm/campaigns/{campaign}/send"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(sent.0, StatusCode::OK, "{sent:?}");
    let mailer = CampaignMailer::from_env("development").unwrap();
    let before = adapter.requests.lock().unwrap().len();
    for _ in 0..25 {
        if sqlx::query_scalar::<_,bool>("SELECT EXISTS(SELECT 1 FROM crm_campaign_deliveries WHERE campaign_id=$1 AND accepted_at IS NOT NULL)").bind(campaign).fetch_one(pool).await.unwrap(){break;}
        assert!(
            campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
    }
    let(delivery,message,contact):(uuid::Uuid,String,i64)=sqlx::query_as("SELECT id,provider_message_id,contact_id FROM crm_campaign_deliveries WHERE campaign_id=$1 AND accepted_at IS NOT NULL").bind(campaign).fetch_one(pool).await.unwrap();
    assert_eq!(contact, id);
    assert!(
        adapter.requests.lock().unwrap()[before..]
            .iter()
            .any(|payload| payload["Subject"] == "Contact actual provider facts")
    );
    for kind in ["Open", "Click"] {
        let event = json!({"RecordType":kind,"MessageStream":"fixture-broadcast","MessageID":message,"Recipient":"crm-contact-observed@example.test","ReceivedAt":chrono::Utc::now().to_rfc3339(),"Metadata":{"delivery_id":delivery},"OriginalLink":if kind=="Click"{Some("https://offer.example.test/contact-observation")}else{None}});
        let recorded = request(
            api,
            "POST",
            "/api/webhooks/postmark/campaigns",
            Some("only-a-disposable-webhook-fixture-secret"),
            event,
        )
        .await;
        assert_eq!(recorded.0, StatusCode::OK, "{recorded:?}");
        assert_eq!(recorded.1["status"], "recorded");
    }
}
