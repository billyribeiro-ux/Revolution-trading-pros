//! Actual source transitions, frozen trigger revisions and consented delivery.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use revolution_api::{
    routes::crm::{process_sequence_trigger as process, stage_sequence_delivery as stage},
    services::{campaign_delivery, campaign_mailer::CampaignMailer},
};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/crm/sequences";
async fn sequence(api: &Router, token: &str, kind: &str, config: Value) -> i64 {
    let result=request(api,"POST",ROOT,Some(token),json!({"title":format!("Actual {kind} trigger"),"trigger_type":kind,"trigger_config":config,"emails":[{"title":"Captured trigger email","subject":"Actual trigger fixture","email_body":"<p>Captured original trigger body</p>","status":"active","delay_value":0,"delay_unit":"minutes"}]})).await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    let id = result.1["id"].as_str().unwrap().parse().unwrap();
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
    id
}
async fn pause(api: &Router, token: &str, id: i64) {
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/{id}/pause"),
            Some(token),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
}
async fn custom(api: &Router, token: &str, contact: i64, name: &str, key: uuid::Uuid) -> Value {
    let event = request(
        api,
        "POST",
        &format!("{ROOT}/events"),
        Some(token),
        json!({"event_id":key,"contact_id":contact,"event_name":name,"properties":{"actual":true}}),
    )
    .await;
    assert_eq!(event.0, StatusCode::OK, "{event:?}");
    event.1
}
async fn drain(pool: &PgPool) {
    for _ in 0..30 {
        if !process(pool).await.unwrap() {
            return;
        }
    }
    panic!("Trigger proof did not drain bounded work");
}
async fn confirmed(pool: &PgPool, email: &str) -> i64 {
    let id: i64 = sqlx::query_scalar(
        "INSERT INTO contacts(email,first_name,tags) VALUES($1,'Actual fixture','[]') RETURNING id",
    )
    .bind(email)
    .fetch_one(pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO newsletter_subscribers(email,status,confirmed_at) VALUES($1,'confirmed',NOW())").bind(email).execute(pool).await.unwrap();
    id
}
pub(super) async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let token = &tokens[0];
    let admin = Some(token.as_str());
    for bad in [
        json!({"tag":42}),
        json!({"unknown":true}),
        json!({"product_id":-1}),
    ] {
        assert_eq!(request(api,"POST",ROOT,admin,json!({"title":"Invalid trigger config","trigger_type":"tag_added","trigger_config":bad})).await.0,StatusCode::BAD_REQUEST);
    }
    let person = confirmed(pool, "sequence-custom-trigger@example.test").await;
    let id = sequence(
        api,
        token,
        "custom_event",
        json!({"event_name":"proof.real"}),
    )
    .await;
    for (credential, expected) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(request(api,"POST",&format!("{ROOT}/events"),credential,json!({"event_id":uuid::Uuid::new_v4(),"contact_id":person,"event_name":"proof.real"})).await.0,expected);
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{ROOT}/{id}/triggers"),
                credential,
                Value::Null
            )
            .await
            .0,
            expected
        );
    }
    let legacy = sequence(api, token, "manual", json!({})).await;
    pause(api, token, legacy).await;
    sqlx::query("UPDATE crm_sequences SET trigger_type='historical-unsupported',authoring_version=authoring_version+1 WHERE id=$1").bind(legacy).execute(pool).await.unwrap();
    let review = request(api, "GET", &format!("{ROOT}/{legacy}"), admin, Value::Null).await;
    assert_eq!(review.0, StatusCode::OK);
    assert_eq!(review.1["sequence"]["trigger_type"], "legacy");
    assert_eq!(
        review.1["sequence"]["historical_trigger_type"],
        "historical-unsupported"
    );
    assert!(review.1["sequence"]["trigger_error"].is_string());
    assert_eq!(
        review.1["sequence"]["emails_count"], 1,
        "A legacy trigger does not erase valid email facts"
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/{legacy}/activate"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{ROOT}/{legacy}"),
            admin,
            json!({"trigger_type":"manual","trigger_config":{}})
        )
        .await
        .0,
        StatusCode::OK
    );
    let preserved:String=sqlx::query_scalar("SELECT definition->>'trigger_type' FROM crm_sequence_revisions WHERE sequence_id=$1 AND authoring_version=2").bind(legacy).fetch_one(pool).await.unwrap();
    assert_eq!(
        preserved, "historical-unsupported",
        "Explicit repair preserves original trigger revision"
    );
    let ignored = custom(api, token, person, "proof.other", uuid::Uuid::new_v4()).await;
    assert_eq!(ignored["matched_sequences"], 0);
    let key = uuid::Uuid::new_v4();
    let event = custom(api, token, person, "proof.real", key).await;
    assert_eq!(event["matched_sequences"], 1);
    assert_eq!(custom(api, token, person, "proof.real", key).await, event);
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/events"),
            admin,
            json!({"event_id":key,"contact_id":person,"event_name":"different"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let frozen: uuid::Uuid = sqlx::query_scalar(
        "SELECT revision_id FROM crm_sequence_trigger_matches WHERE sequence_id=$1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    pause(api, token, id).await;
    assert!(
        !process(pool).await.unwrap(),
        "Operator pause holds captured trigger work"
    );
    let during_pause = custom(api, token, person, "proof.real", uuid::Uuid::new_v4()).await;
    assert_eq!(
        during_pause["matched_sequences"], 0,
        "Paused sequences do not collect new events"
    );
    assert_eq!(request(api,"PUT",&format!("{ROOT}/{id}"),admin,json!({"emails":[{"title":"Edited after capture","subject":"Edited trigger fixture","email_body":"<p>New creative must not replace the captured revision</p>","status":"active","delay_value":0,"delay_unit":"minutes"}]})).await.0,StatusCode::OK);
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
        StatusCode::OK
    );
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO trigger_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    assert!(process(pool).await.is_err());
    sqlx::query("ALTER TABLE trigger_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_trackers WHERE sequence_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        count, 0,
        "Audit failure rolls back tracker and contact effects"
    );
    sqlx::query("UPDATE crm_sequence_trigger_matches SET available_at=NOW() WHERE sequence_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let (one, two) = tokio::join!(process(pool), process(pool));
    assert_eq!(
        [one.unwrap(), two.unwrap()]
            .into_iter()
            .filter(|v| *v)
            .count(),
        1
    );
    let tracker: (uuid::Uuid, String) = sqlx::query_as(
        "SELECT revision_id,contact_email FROM crm_sequence_trackers WHERE sequence_id=$1",
    )
    .bind(id)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        tracker,
        (frozen, "sequence-custom-trigger@example.test".into())
    );
    let before = adapter.requests.lock().unwrap().len();
    assert!(stage(pool).await.unwrap());
    let mailer = CampaignMailer::from_env("development").unwrap();
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert!(stage(pool).await.unwrap());
    {
        let actual = adapter.requests.lock().unwrap();
        assert_eq!(actual.len(), before + 1);
        assert!(
            actual.last().unwrap()["HtmlBody"]
                .as_str()
                .unwrap()
                .contains("Captured original trigger body")
        );
    }
    custom(api, token, person, "proof.real", uuid::Uuid::new_v4()).await;
    drain(pool).await;
    let statuses: Vec<String> = sqlx::query_scalar(
        "SELECT status FROM crm_sequence_trigger_matches WHERE sequence_id=$1 ORDER BY id",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .unwrap();
    assert_eq!(statuses, vec!["enrolled", "already_enrolled"]);
    assert!(!stage(pool).await.unwrap());
    pause(api, token, id).await;

    // No backfill: only actual transitions after activation can enroll.
    let signup = sequence(api, token, "signup", json!({})).await;
    let email = "sequence-signup-trigger@example.test";
    let member:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES($1,'Actual signup','not-a-login-password','user',NOW()) RETURNING id").bind(email).fetch_one(pool).await.unwrap();
    drain(pool).await;
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_trackers WHERE sequence_id=$1")
            .bind(signup)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(
        count, 0,
        "Account registration without newsletter consent does not enroll"
    );
    sqlx::query("INSERT INTO newsletter_subscribers(email,status,confirmed_at) VALUES($1,'confirmed',NOW())").bind(email).execute(pool).await.unwrap();
    drain(pool).await;
    let identity: (i64, String) = sqlx::query_as(
        "SELECT contact_id,contact_email FROM crm_sequence_trackers WHERE sequence_id=$1",
    )
    .bind(signup)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(identity.1, email);
    let actual_contact: String = sqlx::query_scalar("SELECT email FROM contacts WHERE id=$1")
        .bind(identity.0)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(actual_contact, email);
    pause(api, token, signup).await;

    let tagged = confirmed(pool, "sequence-tag-trigger@example.test").await;
    let tag_sequence = sequence(api, token, "tag_added", json!({"tag":"Actual tag"})).await;
    sqlx::query("UPDATE contacts SET tags='[\"Actual tag\"]' WHERE id=$1")
        .bind(tagged)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE contacts SET tags='[\"Actual tag\"]' WHERE id=$1")
        .bind(tagged)
        .execute(pool)
        .await
        .unwrap();
    // Hold the real source contact while the actual enrollment worker acquires
    // its sequence lock. The tag transition must still complete its FK checks.
    let mut writer = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM contacts WHERE id=$1 FOR UPDATE")
        .bind(tagged)
        .fetch_one(&mut *writer)
        .await
        .unwrap();
    let worker_pool = pool.clone();
    let worker = tokio::spawn(async move { process(&worker_pool).await });
    let mut waiting = false;
    for _ in 0..100 {
        waiting=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM pg_stat_activity WHERE datname=current_database() AND wait_event_type='Lock' AND query LIKE 'SELECT id,LOWER(email) AS email FROM contacts WHERE id=%')")
            .fetch_one(pool).await.unwrap();
        if waiting {
            break;
        }
        tokio::time::sleep(std::time::Duration::from_millis(20)).await;
    }
    assert!(waiting, "Actual worker must reach the held contact row");
    sqlx::query("UPDATE contacts SET tags='[]' WHERE id=$1")
        .bind(tagged)
        .execute(&mut *writer)
        .await
        .unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(3),
        sqlx::query("UPDATE contacts SET tags='[\"Actual tag\"]' WHERE id=$1")
            .bind(tagged)
            .execute(&mut *writer),
    )
    .await
    .expect("Tag event FK and sequence worker must not deadlock")
    .unwrap();
    writer.commit().await.unwrap();
    assert!(worker.await.unwrap().unwrap());
    drain(pool).await;
    let matches: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM crm_sequence_trigger_matches WHERE sequence_id=$1",
    )
    .bind(tag_sequence)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        matches, 2,
        "Only the original addition and actual re-addition emit events"
    );
    pause(api, token, tag_sequence).await;

    let purchase = sequence(api, token, "purchase", json!({})).await;
    let order:i64=sqlx::query_scalar("INSERT INTO orders(user_id,order_number,status,total,billing_email) VALUES($1,'sequence-trigger-order','pending',12.34,$2) RETURNING id").bind(member).bind(email).fetch_one(pool).await.unwrap();
    assert!(
        !process(pool).await.unwrap(),
        "Pending orders do not trigger purchase enrollment"
    );
    sqlx::query("UPDATE orders SET status='completed',completed_at=NOW() WHERE id=$1")
        .bind(order)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE orders SET status='completed' WHERE id=$1")
        .bind(order)
        .execute(pool)
        .await
        .unwrap();
    drain(pool).await;
    let matches: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM crm_sequence_trigger_matches WHERE sequence_id=$1",
    )
    .bind(purchase)
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        matches, 1,
        "Repeated completed-order writes do not emit another purchase"
    );
    // Refunding the source before enrollment invalidates the pending purchase.
    let refunded:i64=sqlx::query_scalar("INSERT INTO orders(user_id,order_number,status,total,billing_email) VALUES($1,'sequence-trigger-refunded','completed',9.99,$2) RETURNING id").bind(member).bind(email).fetch_one(pool).await.unwrap();
    sqlx::query("UPDATE orders SET status='refunded',refunded_at=NOW() WHERE id=$1")
        .bind(refunded)
        .execute(pool)
        .await
        .unwrap();
    drain(pool).await;
    let refunded_state:String=sqlx::query_scalar("SELECT m.status FROM crm_sequence_trigger_matches m JOIN crm_trigger_events e ON e.id=m.event_id WHERE m.sequence_id=$1 AND e.source_key=$2")
        .bind(purchase).bind(format!("purchase:order:{refunded}")).fetch_one(pool).await.unwrap();
    assert_eq!(refunded_state, "skipped");
    pause(api, token, purchase).await;

    let withdraw = confirmed(pool, "sequence-trigger-withdrawn@example.test").await;
    let stopped = sequence(
        api,
        token,
        "custom_event",
        json!({"event_name":"proof.withdraw"}),
    )
    .await;
    custom(api, token, withdraw, "proof.withdraw", uuid::Uuid::new_v4()).await;
    sqlx::query("UPDATE newsletter_subscribers SET status='unsubscribed',unsubscribed_at=NOW() WHERE email='sequence-trigger-withdrawn@example.test'").execute(pool).await.unwrap();
    drain(pool).await;
    let state: String =
        sqlx::query_scalar("SELECT status FROM crm_sequence_trigger_matches WHERE sequence_id=$1")
            .bind(stopped)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(state, "skipped");
    pause(api, token, stopped).await;

    let history = request(
        api,
        "GET",
        &format!("{ROOT}/{id}/triggers?per_page=1&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(history.0, StatusCode::OK, "{history:?}");
    assert_eq!(history.1["meta"]["total"], 2);
    assert_eq!(history.1["data"].as_array().unwrap().len(), 1);
    assert_eq!(history.1["metrics_scope"], "sequence_trigger");
    // A fresh single-connection pool exercises acquisition order, not a mock.
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("ADMIN_VERIFY_DATABASE_URL").unwrap())
        .await
        .unwrap();
    assert!(!process(&one).await.unwrap());
    one.close().await;
}
