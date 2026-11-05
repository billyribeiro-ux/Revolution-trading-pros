//! Native API -> durable shared queue -> actual controlled provider receipts.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use revolution_api::services::{campaign_delivery, campaign_mailer::CampaignMailer};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/email/campaigns";
async fn draft(api: &Router, token: &str, payload: Value) -> i64 {
    let response = request(api, "POST", ROOT, Some(token), payload).await;
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    response.1["data"]["id"].as_i64().unwrap()
}
async fn member(pool: &PgPool, tag: i64, email: &str, first: &str) {
    let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,metadata,email_verified_at) VALUES($1,$2,'not-a-login-password',$3,NOW()) RETURNING id").bind(format!("{first} Fixture")).bind(email).bind(json!({"first_name":first,"last_name":"Fixture"})).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_member_tags(user_id,tag_id) VALUES($1,$2)")
        .bind(id)
        .bind(tag)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("INSERT INTO newsletter_subscribers(email,name,status,confirmed_at) VALUES($1,$2,'confirmed',NOW())").bind(email).bind(first).execute(pool).await.unwrap();
}
async fn segment(api: &Router, token: &str, name: &str, tag: i64) -> i64 {
    let response = request(
        api,
        "POST",
        "/api/admin/members/segments",
        Some(token),
        json!({"name":name,"rules":[{"field":"tag","operator":"has","value":tag}]}),
    )
    .await;
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    response.1["id"].as_i64().unwrap()
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let before_status = adapter.requests.lock().unwrap().len();
    let diagnostics = request(api, "GET", "/api/admin/email/status", admin, Value::Null).await;
    assert_eq!(diagnostics.0, StatusCode::OK, "{diagnostics:?}");
    assert_eq!(diagnostics.1["broadcast_configured"], true);
    assert_eq!(diagnostics.1["broadcast_provider"], "postmark");
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before_status,
        "Reading configuration status never submits mail or probes a recipient"
    );
    let tag:i64=sqlx::query_scalar("INSERT INTO member_tags(name,slug) VALUES('Native delivery members','native-delivery-members') RETURNING id").fetch_one(pool).await.unwrap();
    member(
        pool,
        tag,
        "native-delivery-first@example.test",
        "<Native & first>",
    )
    .await;
    let selected = segment(api, &tokens[0], "Native delivery audience", tag).await;
    let template=request(api,"POST","/api/admin/email/templates",admin,json!({"name":"Native delivery source","slug":"native-delivery-source","subject":"Template subject","body_html":"<p>{{name}} / {{first_name}}</p><a href='{{app_url}}/pricing'>Pricing</a>","body_text":"{{name}} / {{email}}"})).await;
    assert_eq!(template.0, StatusCode::OK, "{template:?}");
    let template_id = template.1["data"]["id"].as_i64().unwrap();
    let id=draft(api,&tokens[0],json!({"name":"Native workflow A/B fixture","subject":"Native A {{first_name}}","template_id":template_id,"segment_id":selected,"ab_test_config":{"enabled":true,"subject_b":"Native B {{first_name}}","split_percentage":50}})).await;
    let path = format!("{ROOT}/{id}");
    let due = chrono::Utc::now() + chrono::Duration::days(1);
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for action in ["send", "schedule", "pause", "resume", "cancel"] {
            assert_eq!(
                request(
                    api,
                    "POST",
                    &format!("{path}/{action}"),
                    token,
                    json!({"scheduled_at":due})
                )
                .await
                .0,
                status
            );
        }
    }
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/schedule"),
            admin,
            json!({"scheduled_at":chrono::Utc::now()-chrono::Duration::days(1)})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let before = adapter.requests.lock().unwrap().len();
    let scheduled = request(
        api,
        "POST",
        &format!("{path}/schedule"),
        admin,
        json!({"scheduled_at":due}),
    )
    .await;
    assert_eq!(scheduled.0, StatusCode::OK, "{scheduled:?}");
    assert_eq!(scheduled.1["data"]["status"], "scheduled");
    assert_eq!(scheduled.1["data"]["total_recipients"], 1);
    assert!(scheduled.1["data"]["open_rate"].is_null());
    let engine: i64 =
        sqlx::query_scalar("SELECT delivery_campaign_id FROM email_campaigns WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT native_email_campaign_id FROM crm_campaigns WHERE id=$1"
        )
        .bind(engine)
        .fetch_one(pool)
        .await
        .unwrap(),
        id
    );
    let crm_list = request(
        api,
        "GET",
        "/api/admin/crm/campaigns?search=Native%20workflow",
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(crm_list.0, StatusCode::OK, "{crm_list:?}");
    assert_eq!(crm_list.1["meta"]["total"], 0);
    for (method, tail, body) in [
        ("PUT", "", json!({"title":"Wrong origin"})),
        ("POST", "/duplicate", Value::Null),
        ("POST", "/send", Value::Null),
        ("POST", "/schedule", json!({"scheduled_at":due})),
        ("POST", "/pause", Value::Null),
        ("POST", "/resume", Value::Null),
        ("DELETE", "", Value::Null),
    ] {
        let response = request(
            api,
            method,
            &format!("/api/admin/crm/campaigns/{engine}{tail}"),
            admin,
            body,
        )
        .await;
        assert_eq!(response.0, StatusCode::CONFLICT, "{response:?}");
    }
    let mailer = CampaignMailer::from_env("development").unwrap();
    assert!(
        !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before,
        "A future submission must not reach a provider"
    );
    assert_eq!(
        request(api, "POST", &format!("{path}/pause"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    let original: (String, String) = sqlx::query_as(
        "SELECT variant,subject_template FROM crm_campaign_deliveries WHERE campaign_id=$1",
    )
    .bind(engine)
    .fetch_one(pool)
    .await
    .unwrap();
    // Choose a deterministic opposite hash bucket only to exercise both real
    // provider subject branches, without relying on a random small cohort.
    let need_a = original.0 == "b";
    let other:String=sqlx::query_scalar("SELECT email FROM (SELECT 'native-delivery-extra-'||n||'@example.test' AS email FROM generate_series(1,1024) n) candidates WHERE (get_byte(decode(md5($1::text||':'||email),'hex'),0)<128)=$2 LIMIT 1").bind(engine.to_string()).bind(need_a).fetch_one(pool).await.unwrap();
    member(pool, tag, &other, "Second").await;
    let resumed = request(api, "POST", &format!("{path}/resume"), admin, Value::Null).await;
    assert_eq!(resumed.0, StatusCode::OK, "{resumed:?}");
    assert_eq!(resumed.1["data"]["status"], "scheduled");
    assert_eq!(resumed.1["data"]["total_recipients"], 2);
    let scheduled_at: chrono::DateTime<chrono::Utc> =
        sqlx::query_scalar("SELECT scheduled_at FROM crm_campaigns WHERE id=$1")
            .bind(engine)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        scheduled_at.timestamp_millis(),
        due.timestamp_millis(),
        "Pause and resume must preserve a future requested time"
    );
    assert!(
        !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    let variants: Vec<String> = sqlx::query_scalar(
        "SELECT variant FROM crm_campaign_deliveries WHERE campaign_id=$1 ORDER BY variant",
    )
    .bind(engine)
    .fetch_all(pool)
    .await
    .unwrap();
    assert_eq!(variants, vec!["a", "b"]);
    let unchanged:(String,String)=sqlx::query_as("SELECT variant,subject_template FROM crm_campaign_deliveries WHERE campaign_id=$1 AND email='native-delivery-first@example.test'").bind(engine).fetch_one(pool).await.unwrap();
    assert_eq!(unchanged, original);
    sqlx::query("UPDATE crm_campaigns SET scheduled_at=NOW() WHERE id=$1")
        .bind(engine)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE crm_campaign_deliveries SET due_at=NOW() WHERE campaign_id=$1")
        .bind(engine)
        .execute(pool)
        .await
        .unwrap();
    // Advance both persisted fixture clocks before exercising another resume.
    sqlx::query("UPDATE email_campaigns SET authoring_config=jsonb_set(authoring_config,'{planned_at}',to_jsonb(NOW())) WHERE id=$1")
        .bind(id).execute(pool).await.unwrap();
    for step in 0..2 {
        assert!(
            campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
        if step == 0 {
            assert_eq!(
                request(api, "POST", &format!("{path}/pause"), admin, Value::Null)
                    .await
                    .0,
                StatusCode::OK
            );
            let rename = request(
                api,
                "PUT",
                &path,
                admin,
                json!({"name":"Native started A/B renamed"}),
            )
            .await;
            assert_eq!(rename.0, StatusCode::OK, "{rename:?}");
            for change in [
                json!({"subject":"Changed after first acceptance"}),
                json!({"template_id":template_id}),
                json!({"ab_test_config":null}),
            ] {
                assert_eq!(
                    request(api, "PUT", &path, admin, change).await.0,
                    StatusCode::CONFLICT
                );
            }
            assert_eq!(
                request(api, "POST", &format!("{path}/resume"), admin, Value::Null)
                    .await
                    .0,
                StatusCode::OK
            );
        }
    }
    let sent = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(sent.0, StatusCode::OK, "{sent:?}");
    assert_eq!(sent.1["data"]["status"], "sent");
    assert_eq!(sent.1["data"]["sent_count"], 2);
    assert_eq!(sent.1["data"]["historical_counts"]["sent"], 0);
    report_proof(api, pool, tokens, id, engine).await;

    let requests = {
        let requests = adapter.requests.lock().unwrap();
        requests[before..].to_vec()
    };
    assert_eq!(requests.len(), 2);
    for payload in &requests {
        let email = payload["To"].as_str().unwrap();
        let stored:(String,String,Option<String>)=sqlx::query_as("SELECT variant,subject_template,rendered_subject FROM crm_campaign_deliveries WHERE campaign_id=$1 AND email=$2").bind(engine).bind(email).fetch_one(pool).await.unwrap();
        let first = if email == "native-delivery-first@example.test" {
            "<Native & first>"
        } else {
            "Second"
        };
        let prefix = if stored.0 == "a" {
            "Native A"
        } else {
            "Native B"
        };
        assert_eq!(payload["Subject"], format!("{prefix} {first}"));
        assert_eq!(stored.2.as_deref(), payload["Subject"].as_str());
        assert_eq!(payload["Metadata"]["variant"], stored.0);
        assert_eq!(payload["Metadata"]["campaign_id"], engine.to_string());
        if email == "native-delivery-first@example.test" {
            let html = payload["HtmlBody"].as_str().unwrap();
            assert!(html.contains("&lt;Native &amp; first&gt;"));
            assert!(!html.contains("<Native & first>"));
            assert!(
                payload["TextBody"]
                    .as_str()
                    .unwrap()
                    .contains("<Native & first>")
            );
        }
    }
    let usage = request(
        api,
        "GET",
        &format!("/api/admin/email/templates/{template_id}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(usage.0, StatusCode::OK, "{usage:?}");
    assert_eq!(
        usage.1["data"]["usage_count"], 2,
        "Each actual accepted native template use is counted once"
    );
    assert_eq!(
        request(api, "POST", &format!("{path}/send"), admin, Value::Null)
            .await
            .0,
        StatusCode::CONFLICT
    );
    // Audit failure rolls back a native identity link and all queued recipients.
    let rollback=draft(api,&tokens[0],json!({"name":"Native atomic rollback","subject":"Rollback","html_content":"<p>Body</p>","segment_id":selected})).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO native_submission_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let rejected = request(
        api,
        "POST",
        &format!("{ROOT}/{rollback}/send"),
        admin,
        Value::Null,
    )
    .await;
    sqlx::query("ALTER TABLE native_submission_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        rejected.0,
        StatusCode::INTERNAL_SERVER_ERROR,
        "{rejected:?}"
    );
    assert!(
        sqlx::query_scalar::<_, Option<i64>>(
            "SELECT delivery_campaign_id FROM email_campaigns WHERE id=$1"
        )
        .bind(rollback)
        .fetch_one(pool)
        .await
        .unwrap()
        .is_none()
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM crm_campaigns WHERE native_email_campaign_id=$1"
        )
        .bind(rollback)
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    let invalid=draft(api,&tokens[0],json!({"name":"Native unsupported variable","subject":"Unsupported","html_content":"<p>{{unknown_required_variable}}</p>","segment_id":selected})).await;
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/{invalid}/send"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    // Pause, change creative before acceptance, then cancel without sending.
    let cancelled=draft(api,&tokens[0],json!({"name":"Native cancellation fixture","subject":"Initial","html_content":"<p>Initial</p>","segment_id":selected})).await;
    let cp = format!("{ROOT}/{cancelled}");
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{cp}/schedule"),
            admin,
            json!({"scheduled_at":due})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "POST", &format!("{cp}/pause"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(request(api,"PUT",&cp,admin,json!({"subject":"Changed before acceptance","html_content":"<p>Changed before acceptance</p>"})).await.0,StatusCode::OK);
    let cancelled_engine: i64 =
        sqlx::query_scalar("SELECT delivery_campaign_id FROM email_campaigns WHERE id=$1")
            .bind(cancelled)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT subject FROM crm_campaigns WHERE id=$1")
            .bind(cancelled_engine)
            .fetch_one(pool)
            .await
            .unwrap(),
        "Changed before acceptance"
    );
    let changed = request(
        api,
        "PUT",
        &cp,
        admin,
        json!({"scheduled_at":due+chrono::Duration::days(1)}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    let cleared = request(api, "PUT", &cp, admin, json!({"scheduled_at":null})).await;
    assert_eq!(cleared.0, StatusCode::OK, "{cleared:?}");
    assert_eq!(cleared.1["data"]["planned_at"], Value::Null);
    assert_eq!(cleared.1["data"]["scheduled_at"], Value::Null);
    assert_eq!(cleared.1["data"]["status"], "paused");
    let resumed = request(api, "POST", &format!("{cp}/resume"), admin, Value::Null).await;
    assert_eq!(resumed.0, StatusCode::OK, "{resumed:?}");
    assert_eq!(resumed.1["data"]["status"], "sending");
    let actual_due =
        chrono::DateTime::parse_from_rfc3339(resumed.1["data"]["scheduled_at"].as_str().unwrap())
            .unwrap();
    assert!(
        (chrono::Utc::now().timestamp() - actual_due.timestamp()).abs() < 5,
        "A cleared paused request must resume now rather than inherit its earlier future due time"
    );
    let stopped = request(api, "POST", &format!("{cp}/cancel"), admin, Value::Null).await;
    assert_eq!(stopped.0, StatusCode::OK, "{stopped:?}");
    assert_eq!(stopped.1["data"]["status"], "cancelled");
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM crm_campaign_deliveries WHERE campaign_id=$1 AND status='queued'"
        )
        .bind(cancelled_engine)
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 2);
    single_connection_proof(pool, tokens, selected, due).await;
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 2);
    failure_proof(api, pool, tokens, adapter, selected).await;
    segment_race_proof(api, pool, tokens, adapter, selected).await;
}

async fn report_proof(api: &Router, pool: &PgPool, tokens: &[String], native: i64, engine: i64) {
    let path = format!("{ROOT}/{native}/report");
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(api, "GET", &path, token, Value::Null).await.0,
            status
        );
    }
    let initial = request(api, "GET", &path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(initial.0, StatusCode::OK, "{initial:?}");
    assert_eq!(initial.1["data"]["metrics"]["sent"], 2);
    assert_eq!(initial.1["data"]["metrics"]["delivered"], 0);
    assert_eq!(
        initial.1["data"]["rates"]["click_to_open_rate"],
        Value::Null
    );
    assert_eq!(initial.1["data"]["engagement_over_time"], json!([]));
    assert_eq!(initial.1["data"]["ab_test_results"]["winner"], Value::Null);
    for key in ["variant_a", "variant_b"] {
        assert_eq!(initial.1["data"]["ab_test_results"][key]["sent"], 1);
    }
    let (delivery,email,message,variant):(uuid::Uuid,String,String,String)=sqlx::query_as("SELECT id,email,provider_message_id,variant FROM crm_campaign_deliveries WHERE campaign_id=$1 ORDER BY email LIMIT 1").bind(engine).fetch_one(pool).await.unwrap();
    let now = chrono::Utc::now();
    for (index, kind) in ["Delivery", "Open", "Open", "Click"].iter().enumerate() {
        let at = (now - chrono::Duration::milliseconds(4 - index as i64)).to_rfc3339();
        let event = json!({"RecordType":kind,"MessageStream":"fixture-broadcast","MessageID":message,"Recipient":email,"DeliveredAt":at,"ReceivedAt":at,"Metadata":{"delivery_id":delivery},"OriginalLink":if *kind=="Click" {Some("https://offer.example.test/recorded?a=1&b=2")} else {None},"Platform":"Mobile","Client":{"Name":"Fixture Mail"},"Geo":{"CountryISOCode":"US","Country":"United States","IP":"192.0.2.50"}});
        let receive = request(
            api,
            "POST",
            "/api/webhooks/postmark/campaigns",
            Some("only-a-disposable-webhook-fixture-secret"),
            event.clone(),
        )
        .await;
        assert_eq!(receive.0, StatusCode::OK, "{receive:?}");
        assert_eq!(receive.1["status"], "recorded");
        assert_eq!(
            request(
                api,
                "POST",
                "/api/webhooks/postmark/campaigns",
                Some("only-a-disposable-webhook-fixture-secret"),
                event
            )
            .await
            .1["status"],
            "duplicate"
        );
    }
    let actual = request(api, "GET", &path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(actual.0, StatusCode::OK, "{actual:?}");
    let result = &actual.1["data"];
    assert_eq!(result["metrics_scope"], "recorded_outbox");
    assert_eq!(result["metrics"]["sent"], 2);
    assert_eq!(result["metrics"]["delivered"], 1);
    assert_eq!(result["metrics"]["opened"], 2);
    assert_eq!(result["metrics"]["unique_opens"], 1);
    assert_eq!(result["metrics"]["clicked"], 1);
    assert_eq!(result["rates"]["open_rate"], 50.0);
    assert_eq!(result["rates"]["click_to_open_rate"], 100.0);
    assert_eq!(result["metrics"]["soft_bounced"], Value::Null);
    assert_eq!(
        result["engagement_over_time"],
        json!([{"hour":0,"opens":2,"clicks":1}])
    );
    assert_eq!(
        result["top_links"],
        json!([{"url":"https://offer.example.test/recorded?a=1&b=2","clicks":1,"unique_clicks":1}])
    );
    assert_eq!(
        result["device_breakdown"],
        json!({"desktop":0,"mobile":2,"tablet":0,"unknown":0})
    );
    assert_eq!(
        result["email_client_breakdown"],
        json!([{"client":"Fixture Mail","count":2,"percentage":100.0}])
    );
    assert_eq!(
        result["geographic_distribution"],
        json!([{"country":"United States","country_code":"US","opens":2,"clicks":1}])
    );
    let key = if variant == "a" {
        "variant_a"
    } else {
        "variant_b"
    };
    assert_eq!(result["ab_test_results"][key]["opens"], 1);
    assert_eq!(result["ab_test_results"][key]["open_rate"], 100.0);
    assert_eq!(result["ab_test_results"]["winner"], Value::Null);
    let leaked:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_campaign_events WHERE delivery_id=$1 AND details::text LIKE '%192.0.2.50%')").bind(delivery).fetch_one(pool).await.unwrap();
    assert!(
        !leaked,
        "Report observations must not retain a raw IP address"
    );
    sqlx::query("UPDATE email_campaigns SET sent_count=99999 WHERE id=$1")
        .bind(native)
        .execute(pool)
        .await
        .unwrap();
    let separate = request(api, "GET", &path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(separate.1["data"]["metrics"]["sent"], 2);
    assert_eq!(separate.1["data"]["historical_counts"]["sent"], 99999);
    super::email_campaign_analytics_proof::proof(api, pool, tokens, native).await;
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/9223372036854775807/report"),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
}

async fn single_connection_proof(
    pool: &PgPool,
    tokens: &[String],
    segment: i64,
    due: chrono::DateTime<chrono::Utc>,
) {
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    let api = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    let id=draft(&api,&tokens[0],json!({"name":"Native single connection fixture","subject":"Single connection","html_content":"<p>Source</p>","segment_id":segment})).await;
    let path = format!("{ROOT}/{id}");
    for (method, suffix, body) in [
        ("POST", "/schedule", json!({"scheduled_at":due})),
        ("POST", "/pause", Value::Null),
        (
            "PUT",
            "",
            json!({"name":"Native single connection edited","scheduled_at":null}),
        ),
        ("POST", "/resume", Value::Null),
        ("GET", "/report", Value::Null),
        ("GET", "/analytics", Value::Null),
        ("POST", "/cancel", Value::Null),
    ] {
        let response = tokio::time::timeout(
            std::time::Duration::from_secs(5),
            request(
                &api,
                method,
                &format!("{path}{suffix}"),
                Some(&tokens[0]),
                body,
            ),
        )
        .await
        .expect(
            "Native workflow must not wait for a second connection while holding its transaction",
        );
        assert_eq!(
            response.0,
            StatusCode::OK,
            "{method} {suffix}: {response:?}"
        );
    }
    one.close().await;
}

async fn failure_proof(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    segment: i64,
) {
    let admin = Some(tokens[0].as_str());
    let mailer = CampaignMailer::from_env("development").unwrap();
    for subject in ["fixture-rejected", "fixture-unknown"] {
        let id=draft(api,&tokens[0],json!({"name":format!("Native {subject}"),"subject":subject,"html_content":"<p>Native failure fixture</p>","segment_id":segment})).await;
        let path = format!("{ROOT}/{id}");
        let before = adapter.requests.lock().unwrap().len();
        assert_eq!(
            request(api, "POST", &format!("{path}/send"), admin, Value::Null)
                .await
                .0,
            StatusCode::OK
        );
        assert!(
            campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
        if subject == "fixture-unknown" {
            let unknown = request(api, "GET", &path, admin, Value::Null).await;
            assert_eq!(unknown.1["data"]["status"], "paused");
            assert_eq!(unknown.1["data"]["sent_count"], 0);
            assert_eq!(unknown.1["data"]["delivery_stats"]["unknown"], 1);
            assert_eq!(unknown.1["data"]["pause_reason"], "uncertain_submission");
            assert!(
                unknown.1["data"]["last_error"]
                    .as_str()
                    .unwrap()
                    .contains("reconciliation")
            );
            assert_eq!(
                request(api, "POST", &format!("{path}/resume"), admin, Value::Null)
                    .await
                    .0,
                StatusCode::CONFLICT
            );
            assert!(
                !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                    .await
                    .unwrap()
            );
            assert_eq!(
                adapter.requests.lock().unwrap().len(),
                before + 1,
                "An unknown native result must not be submitted again"
            );
        } else {
            assert!(
                campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                    .await
                    .unwrap()
            );
            let failed = request(api, "GET", &path, admin, Value::Null).await;
            assert_eq!(failed.1["data"]["status"], "failed");
            assert_eq!(failed.1["data"]["failed_count"], 2);
            assert_eq!(failed.1["data"]["sent_count"], 0);
            assert_eq!(
                request(
                    api,
                    "PUT",
                    &path,
                    admin,
                    json!({"subject":"Native recovered from explicit rejection"})
                )
                .await
                .0,
                StatusCode::OK
            );
            assert_eq!(
                request(api, "POST", &format!("{path}/resume"), admin, Value::Null)
                    .await
                    .0,
                StatusCode::OK
            );
            for _ in 0..2 {
                assert!(
                    campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                        .await
                        .unwrap()
                );
            }
            let recovered = request(api, "GET", &path, admin, Value::Null).await;
            assert_eq!(recovered.1["data"]["status"], "sent");
            assert_eq!(recovered.1["data"]["sent_count"], 2);
            assert_eq!(recovered.1["data"]["failed_count"], 0);
            assert_eq!(adapter.requests.lock().unwrap().len(), before + 4);
        }
    }
    let id=draft(api,&tokens[0],json!({"name":"Native actual in-flight fixture","subject":"fixture-inflight","html_content":"<p>In-flight fixture</p>","segment_id":segment})).await;
    let path = format!("{ROOT}/{id}");
    assert_eq!(
        request(api, "POST", &format!("{path}/send"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    let during = async {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            adapter.inflight_started.notified(),
        )
        .await
        .unwrap();
        assert_eq!(
            request(api, "POST", &format!("{path}/pause"), admin, Value::Null)
                .await
                .0,
            StatusCode::OK
        );
        for (method, suffix, body) in [
            ("POST", "/cancel", Value::Null),
            ("POST", "/resume", Value::Null),
            ("PUT", "", json!({"subject":"Must not edit in flight"})),
        ] {
            assert_eq!(
                request(api, method, &format!("{path}{suffix}"), admin, body)
                    .await
                    .0,
                StatusCode::CONFLICT
            );
        }
        adapter.inflight_release.notify_one();
    };
    let (processed, ()) = tokio::join!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274"),
        during
    );
    assert!(processed.unwrap());
    let paused = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(paused.1["data"]["status"], "paused");
    assert_eq!(
        paused.1["data"]["sent_count"], 1,
        "Pause does not recall an in-flight provider acceptance"
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            admin,
            json!({"subject":"Native in-flight remainder"})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "POST", &format!("{path}/resume"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    let sent = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(sent.1["data"]["status"], "sent");
    assert_eq!(sent.1["data"]["sent_count"], 2);
    // Actual429 backs off only its recipient; elapsed time is advanced in this fixture.
    adapter
        .rate_limits
        .store(0, std::sync::atomic::Ordering::SeqCst);
    let id=draft(api,&tokens[0],json!({"name":"Native safe backoff fixture","subject":"fixture-rate-limit","html_content":"<p>Backoff fixture</p>","segment_id":segment})).await;
    let path = format!("{ROOT}/{id}");
    let before = adapter.requests.lock().unwrap().len();
    assert_eq!(
        request(api, "POST", &format!("{path}/send"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    let retry:(String,i32,bool)=sqlx::query_as("SELECT r.status,r.attempt_count,r.due_at>NOW() FROM crm_campaign_deliveries r JOIN email_campaigns c ON c.delivery_campaign_id=r.campaign_id WHERE c.id=$1 AND r.attempt_count=1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(retry, ("queued".into(), 1, true));
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    sqlx::query("UPDATE crm_campaign_deliveries r SET due_at=NOW() FROM email_campaigns c WHERE c.delivery_campaign_id=r.campaign_id AND c.id=$1 AND r.status='queued'").bind(id).execute(pool).await.unwrap();
    let (one, two) = tokio::join!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274"),
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
    );
    assert_eq!(u8::from(one.unwrap()) + u8::from(two.unwrap()), 1);
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 3);
    let complete = request(api, "GET", &path, admin, Value::Null).await;
    assert_eq!(complete.1["data"]["status"], "sent");
    assert_eq!(complete.1["data"]["sent_count"], 2);
}
async fn segment_race_proof(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    segment: i64,
) {
    let admin = Some(tokens[0].as_str());
    let before = adapter.requests.lock().unwrap().len();
    let prior = request(
        api,
        "GET",
        &format!("/api/admin/members/segments/{segment}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(prior.0, StatusCode::OK);
    let id=draft(api,&tokens[0],json!({"name":"Native segment version race","subject":"No stale audience","html_content":"<p>Race fixture</p>","segment_id":segment})).await;
    let mut held = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM email_campaigns WHERE id=$1 FOR UPDATE")
        .bind(id)
        .fetch_one(&mut *held)
        .await
        .unwrap();
    let pending = tokio::spawn({
        let api = api.clone();
        let token = tokens[0].clone();
        async move {
            request(
                &api,
                "POST",
                &format!("{ROOT}/{id}/send"),
                Some(&token),
                Value::Null,
            )
            .await
        }
    });
    tokio::time::timeout(std::time::Duration::from_secs(5),async {
        loop {
            let waiting:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE datname=current_database() AND wait_event_type='Lock' AND query LIKE '%authoring_config,delivery_campaign_id%FOR UPDATE%')").fetch_one(pool).await.unwrap();
            if waiting {break;}
            tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        }
    }).await.expect("Observe the actual submission row-lock barrier after its audience preflight");
    let changed = request(
        api,
        "PUT",
        &format!("/api/admin/members/segments/{segment}"),
        admin,
        json!({"rules":[{"field":"days_since_signup","operator":"gte","value":100000}]}),
    )
    .await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    held.commit().await.unwrap();
    let outcome = tokio::time::timeout(std::time::Duration::from_secs(5), pending)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(outcome.0, StatusCode::CONFLICT, "{outcome:?}");
    assert_eq!(
        sqlx::query_scalar::<_, Option<i64>>(
            "SELECT delivery_campaign_id FROM email_campaigns WHERE id=$1"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap(),
        None
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM crm_campaigns WHERE native_email_campaign_id=$1"
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("/api/admin/members/segments/{segment}"),
            admin,
            json!({"rules":prior.1["conditions"]})
        )
        .await
        .0,
        StatusCode::OK
    );
}
