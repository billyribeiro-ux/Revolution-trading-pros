//! Frozen sequence cohorts -> actual shared-worker receipts, without live providers.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use revolution_api::{
    routes::crm::stage_sequence_delivery as stage,
    services::{campaign_delivery, campaign_mailer::CampaignMailer},
};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/crm/sequences";
pub(super) async fn contact(pool: &PgPool, suffix: &str) -> i64 {
    let email = format!("sequence-worker-{suffix}@example.test");
    let id:i64=sqlx::query_scalar("INSERT INTO contacts(email,first_name,last_name,tags,source) VALUES($1,'<Actual & First>','Fixture','[\"Included\"]','sequence-worker') RETURNING id").bind(&email).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO newsletter_subscribers(email,status,confirmed_at) VALUES($1,'confirmed',NOW())").bind(email).execute(pool).await.unwrap();
    id
}
pub(super) async fn sequence(
    api: &Router,
    token: &str,
    title: &str,
    subject: &str,
) -> (i64, String, String) {
    let created=request(api,"POST",ROOT,Some(token),json!({"title":title,"settings":{"mailer_settings":{"is_custom":true,"from_name":"Sequence sender","from_email":"sequence-sender@example.test","reply_to_email":"sequence-reply@example.test"}},"emails":[{"title":"First","subject":subject,"email_body":"<p>{{contact.first_name}}</p><a href='{{unsubscribe_url}}'>Leave</a>","text_body":"Actual alternative {{email}}","status":"active","delay_value":0,"delay_unit":"minutes"},{"title":"Second","subject":"Original frozen second","email_body":"<p>Original frozen second body</p>","status":"active","delay_value":1,"delay_unit":"minutes"}]})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["id"].as_str().unwrap().parse::<i64>().unwrap();
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/{id}/activate"),
            Some(token),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    (
        id,
        created.1["emails"][0]["id"].as_str().unwrap().into(),
        created.1["emails"][1]["id"].as_str().unwrap().into(),
    )
}
pub(super) async fn enroll(api: &Router, token: &str, id: i64, contact: i64) {
    let result = request(
        api,
        "POST",
        &format!("{ROOT}/{id}/subscribe"),
        Some(token),
        json!({"contact_ids":[contact.to_string()]}),
    )
    .await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    assert_eq!(result.1["subscribed_count"], 1);
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let token = &tokens[0];
    let admin = Some(token.as_str());
    let mailer = CampaignMailer::from_env("development").unwrap();
    let person = contact(pool, "frozen").await;
    let (id, a, b) = sequence(
        api,
        token,
        "Sequence frozen worker",
        "First {{contact.first_name}}",
    )
    .await;
    let path = format!("{ROOT}/{id}");
    enroll(api, token, id, person).await;
    let crm_before = request(
        api,
        "GET",
        "/api/admin/crm/campaigns/stats",
        admin,
        Value::Null,
    )
    .await;
    let before = adapter.requests.lock().unwrap().len();
    let (one, two) = tokio::join!(stage(pool), stage(pool));
    assert_eq!(
        [one.unwrap(), two.unwrap()]
            .into_iter()
            .filter(|value| *value)
            .count(),
        1,
        "Concurrent schedulers stage a logical step once"
    );
    let engine: i64 =
        sqlx::query_scalar("SELECT campaign_id FROM crm_sequence_engines WHERE sequence_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    let queued: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaign_deliveries WHERE campaign_id=$1")
            .bind(engine)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(queued, 1);
    for (method, suffix, body) in [
        ("PUT", "", json!({"title":"Must not edit an engine"})),
        ("DELETE", "", Value::Null),
        ("POST", "/duplicate", Value::Null),
        ("POST", "/send", Value::Null),
        ("POST", "/pause", Value::Null),
    ] {
        assert_eq!(
            request(
                api,
                method,
                &format!("/api/admin/crm/campaigns/{engine}{suffix}"),
                admin,
                body
            )
            .await
            .0,
            StatusCode::CONFLICT
        );
    }
    let (one, two) = tokio::join!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274"),
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
    );
    assert_eq!(
        [one.unwrap(), two.unwrap()]
            .into_iter()
            .filter(|value| *value)
            .count(),
        1
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    let payload = adapter.requests.lock().unwrap().last().unwrap().clone();
    assert_eq!(payload["Subject"], "First <Actual & First>");
    assert_eq!(
        payload["From"],
        "\"Sequence sender\" <sequence-sender@example.test>"
    );
    assert_eq!(payload["ReplyTo"], "sequence-reply@example.test");
    assert!(
        payload["HtmlBody"]
            .as_str()
            .unwrap()
            .contains("&lt;Actual &amp; First&gt;")
    );
    assert!(
        payload["TextBody"]
            .as_str()
            .unwrap()
            .contains("Actual alternative sequence-worker-frozen@example.test")
    );
    assert_eq!(
        request(api, "POST", &format!("{path}/pause"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert!(!stage(pool).await.unwrap());
    let edited=request(api,"PUT",&path,admin,json!({"emails":[{"id":a},{"id":b,"email_subject":"Changed current second","email_body":"<p>Changed current source</p>"}]})).await;
    assert_eq!(edited.0, StatusCode::OK, "{edited:?}");
    assert_eq!(
        request(api, "POST", &format!("{path}/activate"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert!(
        stage(pool).await.unwrap(),
        "Acceptance advances the frozen cohort only after resume"
    );
    assert!(
        !stage(pool).await.unwrap(),
        "The next email must not be staged before its actual due time"
    );
    let interval:f64=sqlx::query_scalar("SELECT EXTRACT(EPOCH FROM(t.next_execution_at-d.accepted_at))::float8 FROM crm_sequence_trackers t JOIN crm_campaign_deliveries d ON d.sequence_tracker_id=t.id AND d.sequence_mail_id=t.last_sequence_mail_id WHERE t.sequence_id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(interval, 60.0);
    // Advance only this disposable fixture's clock after proving its due boundary.
    sqlx::query("UPDATE crm_sequence_trackers SET next_execution_at=NOW()-INTERVAL '1 second' WHERE sequence_id=$1").bind(id).execute(pool).await.unwrap();
    assert!(stage(pool).await.unwrap());
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert!(stage(pool).await.unwrap());
    assert_eq!(
        adapter.requests.lock().unwrap().last().unwrap()["Subject"],
        "Original frozen second"
    );
    assert!(
        adapter.requests.lock().unwrap().last().unwrap()["HtmlBody"]
            .as_str()
            .unwrap()
            .contains("Original frozen second body")
    );
    let stats = request(api, "GET", &format!("{path}/stats"), admin, Value::Null).await;
    assert_eq!(stats.0, StatusCode::OK, "{stats:?}");
    assert_eq!(stats.1["total_sent"], 2);
    assert_eq!(stats.1["completed"], 1);
    assert_eq!(stats.1["open_rate"], 0.0);
    assert!(stats.1["revenue"]["amount"].is_null());
    assert_eq!(
        request(
            api,
            "GET",
            "/api/admin/crm/campaigns/stats",
            admin,
            Value::Null
        )
        .await,
        crm_before,
        "Internal sequence receipts must not become CRM broadcast totals"
    );
    for (subject, expected) in [
        ("fixture-rejected", "failed"),
        ("fixture-unknown", "paused"),
    ] {
        let person = contact(pool, expected).await;
        let (id, _, _) = sequence(api, token, &format!("Sequence {expected}"), subject).await;
        enroll(api, token, id, person).await;
        let before = adapter.requests.lock().unwrap().len();
        assert!(stage(pool).await.unwrap());
        assert!(
            campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
        assert!(stage(pool).await.unwrap());
        let state: String =
            sqlx::query_scalar("SELECT status FROM crm_sequence_trackers WHERE sequence_id=$1")
                .bind(id)
                .fetch_one(pool)
                .await
                .unwrap();
        assert_eq!(state, expected);
        assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
        let receipt:String=sqlx::query_scalar("SELECT d.status FROM crm_campaign_deliveries d JOIN crm_sequence_trackers t ON t.id=d.sequence_tracker_id WHERE t.sequence_id=$1").bind(id).fetch_one(pool).await.unwrap();
        assert_eq!(
            receipt,
            if expected == "failed" {
                "failed"
            } else {
                "unknown"
            }
        );
        assert!(!stage(pool).await.unwrap());
        assert!(
            !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
        if expected == "paused" {
            assert_eq!(
                request(
                    api,
                    "POST",
                    &format!("{ROOT}/{id}/activate"),
                    admin,
                    Value::Null
                )
                .await
                .0,
                StatusCode::CONFLICT
            );
        }
        assert_eq!(
            adapter.requests.lock().unwrap().len(),
            before + 1,
            "No automatic retry after a terminal rejection or unknown outcome"
        );
    }
    for suffix in ["withdrawn", "changed", "deleted"] {
        let person = contact(pool, suffix).await;
        let (id, _, _) = sequence(
            api,
            token,
            &format!("Sequence {suffix}"),
            "Eligibility fixture",
        )
        .await;
        enroll(api, token, id, person).await;
        assert!(stage(pool).await.unwrap());
        match suffix {
            "withdrawn" => {
                sqlx::query("UPDATE newsletter_subscribers SET status='unsubscribed',unsubscribed_at=NOW(),updated_at=NOW() WHERE email=$1").bind(format!("sequence-worker-{suffix}@example.test")).execute(pool).await.unwrap();
            }
            "changed" => {
                sqlx::query("UPDATE contacts SET email='sequence-worker-changed-new@example.test' WHERE id=$1").bind(person).execute(pool).await.unwrap();
            }
            _ => {
                sqlx::query("DELETE FROM contacts WHERE id=$1")
                    .bind(person)
                    .execute(pool)
                    .await
                    .unwrap();
            }
        }
        let before = adapter.requests.lock().unwrap().len();
        assert!(
            !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
        assert!(stage(pool).await.unwrap());
        assert_eq!(adapter.requests.lock().unwrap().len(), before);
        let state: String =
            sqlx::query_scalar("SELECT status FROM crm_sequence_trackers WHERE sequence_id=$1")
                .bind(id)
                .fetch_one(pool)
                .await
                .unwrap();
        assert_eq!(state, "cancelled");
    }
    let person = contact(pool, "inflight").await;
    let (id, _, _) = sequence(api, token, "Sequence in-flight", "fixture-inflight").await;
    enroll(api, token, id, person).await;
    assert!(stage(pool).await.unwrap());
    let before = adapter.requests.lock().unwrap().len();
    let during = async {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            adapter.inflight_started.notified(),
        )
        .await
        .unwrap();
        assert_eq!(
            request(
                api,
                "POST",
                &format!("{ROOT}/{id}/pause"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::OK
        );
        assert_eq!(
            request(
                api,
                "POST",
                &format!("{ROOT}/{id}/activate"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::CONFLICT
        );
        assert_eq!(
            request(
                api,
                "POST",
                &format!("{ROOT}/{id}/unsubscribe"),
                admin,
                json!({"contact_ids":[person]})
            )
            .await
            .1["unsubscribed_count"],
            1
        );
        adapter.inflight_release.notify_one();
    };
    let (receipt, ()) = tokio::join!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274"),
        during
    );
    assert!(receipt.unwrap());
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    assert!(!stage(pool).await.unwrap());
    let accepted:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaign_deliveries d JOIN crm_sequence_trackers t ON t.id=d.sequence_tracker_id WHERE t.sequence_id=$1 AND d.accepted_at IS NOT NULL").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(
        accepted, 1,
        "An actual ACK remains recorded after cancellation"
    );
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    let single = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    let person = contact(pool, "single").await;
    let (id, _, _) = sequence(
        &single,
        token,
        "Sequence single pool",
        "One connection source",
    )
    .await;
    enroll(&single, token, id, person).await;
    assert!(
        tokio::time::timeout(std::time::Duration::from_secs(5), stage(&one))
            .await
            .unwrap()
            .unwrap()
    );
    assert!(
        campaign_delivery::process_one(&one, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert!(stage(&one).await.unwrap());
    assert_eq!(
        request(
            &single,
            "POST",
            &format!("{ROOT}/{id}/pause"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    one.close().await;
}
