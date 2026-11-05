//! Actual callback decisions, atomic scheduler recovery and saved-provider pause boundaries.
use super::{
    Adapter, request,
    sequence_delivery_proof::{contact, enroll, sequence},
};
use axum::{Router, http::StatusCode};
use revolution_api::{
    routes::crm::stage_sequence_delivery as stage,
    services::{campaign_delivery, campaign_mailer::CampaignMailer},
};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/crm/sequences";
async fn callback(api: &Router, pool: &PgPool, sequence: i64, mail: uuid::Uuid, kind: &str) {
    let(delivery,email,message):(uuid::Uuid,String,String)=sqlx::query_as("SELECT d.id,d.email,d.provider_message_id FROM crm_campaign_deliveries d JOIN crm_sequence_trackers t ON t.id=d.sequence_tracker_id WHERE t.sequence_id=$1 AND d.sequence_mail_id=$2").bind(sequence).bind(mail).fetch_one(pool).await.unwrap();
    let at = (chrono::Utc::now() - chrono::Duration::milliseconds(1)).to_rfc3339();
    let payload = json!({"RecordType":kind,"MessageStream":"fixture-broadcast","MessageID":message,"Recipient":email,"DeliveredAt":at,"ReceivedAt":at,"Metadata":{"delivery_id":delivery},"OriginalLink":if kind=="Click"{Some("https://offer.example.test/sequence-observation")}else{None}});
    let accepted = request(
        api,
        "POST",
        "/api/webhooks/postmark/campaigns",
        Some("only-a-disposable-webhook-fixture-secret"),
        payload.clone(),
    )
    .await;
    assert_eq!(accepted.0, StatusCode::OK, "{accepted:?}");
    assert_eq!(accepted.1["status"], "recorded");
    assert_eq!(
        request(
            api,
            "POST",
            "/api/webhooks/postmark/campaigns",
            Some("only-a-disposable-webhook-fixture-secret"),
            payload
        )
        .await
        .1["status"],
        "duplicate"
    );
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let token = &tokens[0];
    let admin = Some(token.as_str());
    let mailer = CampaignMailer::from_env("development").unwrap();
    let person = contact(pool, "conditions").await;
    let a = uuid::Uuid::new_v4();
    let b = uuid::Uuid::new_v4();
    let source=request(api,"POST",ROOT,admin,json!({"title":"Sequence actual conditions","emails":[
        {"id":a,"title":"First","subject":"Actual condition first","email_body":"First source","status":"active","delay_value":0,"delay_unit":"minutes","conditions":[{"type":"tag","value":"Included"},{"type":"custom","value":{"field":"email","operator":"contains","value":"sequence-worker-conditions"}}]},
        {"id":b,"title":"Recorded open","subject":"Actual condition second","email_body":"Second source","status":"active","delay_value":0,"delay_unit":"minutes","conditions":[{"type":"opened","previous_email_id":a}]},
        {"title":"No recorded click","subject":"Must skip recorded click","email_body":"Must not send","status":"active","delay_value":0,"delay_unit":"minutes","conditions":[{"type":"not_clicked","previous_email_id":b}]},
        {"title":"Absent tag","subject":"Must skip absent tag","email_body":"Must not send","status":"active","delay_value":0,"delay_unit":"minutes","conditions":[{"type":"tag","value":"Absent actual tag"}]},
        {"title":"Final","subject":"Actual condition final","email_body":"Final source","status":"active","delay_value":0,"delay_unit":"minutes","conditions":[{"type":"not_opened","previous_email_id":b},{"type":"custom","value":{"field":"email","operator":"equals","value":"sequence-worker-conditions@example.test"}}]}
    ]})).await;
    assert_eq!(source.0, StatusCode::OK, "{source:?}");
    let id = source.1["id"].as_str().unwrap().parse::<i64>().unwrap();
    let path = format!("{ROOT}/{id}");
    assert_eq!(
        request(api, "POST", &format!("{path}/activate"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    enroll(api, token, id, person).await;
    let native = request(
        api,
        "GET",
        "/api/admin/email/campaigns/stats",
        admin,
        Value::Null,
    )
    .await;
    let before = adapter.requests.lock().unwrap().len();
    assert!(stage(pool).await.unwrap());
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    callback(api, pool, id, a, "Delivery").await;
    callback(api, pool, id, a, "Open").await;
    assert!(stage(pool).await.unwrap());
    assert!(stage(pool).await.unwrap());
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    callback(api, pool, id, b, "Click").await;
    assert!(stage(pool).await.unwrap());
    assert!(stage(pool).await.unwrap());
    assert!(stage(pool).await.unwrap());
    assert!(stage(pool).await.unwrap());
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert!(stage(pool).await.unwrap());
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before + 3,
        "Recorded observations and actual tag/field facts decide steps; skipped steps never submit"
    );
    let stats = request(api, "GET", &format!("{path}/stats"), admin, Value::Null).await;
    assert_eq!(stats.0, StatusCode::OK, "{stats:?}");
    assert_eq!(stats.1["sequence_id"], id.to_string());
    assert_eq!(stats.1["total_sent"], 3);
    assert_eq!(stats.1["total_opened"], 1);
    assert_eq!(stats.1["total_clicked"], 1);
    assert_eq!(stats.1["total_delivered"], 1);
    assert_eq!(stats.1["completed"], 1);
    assert_eq!(stats.1["active_subscribers"], 0);
    assert_eq!(stats.1["queued"], 0);
    assert_eq!(
        request(
            api,
            "GET",
            "/api/admin/email/campaigns/stats",
            admin,
            Value::Null
        )
        .await,
        native,
        "Sequence observations must not become native broadcast metrics"
    );
    let person = contact(pool, "rollback").await;
    let (id, _, _) = sequence(
        api,
        token,
        "Sequence atomic staging",
        "Atomic staging fixture",
    )
    .await;
    enroll(api, token, id, person).await;
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO sequence_stage_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = stage(pool).await;
    sqlx::query("ALTER TABLE sequence_stage_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert!(failed.is_err());
    assert_eq!(
        request(api, "GET", &format!("{ROOT}/{id}"), admin, Value::Null)
            .await
            .1["sequence"]["last_error"],
        "Sequence scheduling failed; no unconfirmed submission is retried"
    );
    let engines: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_engines WHERE sequence_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(engines, 0);
    let before = adapter.requests.lock().unwrap().len();
    assert!(stage(pool).await.unwrap());
    assert!(
        request(api, "GET", &format!("{ROOT}/{id}"), admin, Value::Null)
            .await
            .1["sequence"]["last_error"]
            .is_null(),
        "Recovered scheduling failure must clear in the same transaction as progress"
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO sequence_advance_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = stage(pool).await;
    sqlx::query("ALTER TABLE sequence_advance_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert!(failed.is_err());
    assert_eq!(
        request(api, "GET", &format!("{ROOT}/{id}"), admin, Value::Null)
            .await
            .1["sequence"]["last_error"],
        "Sequence scheduling failed; no unconfirmed submission is retried"
    );
    assert!(stage(pool).await.unwrap());
    assert!(
        request(api, "GET", &format!("{ROOT}/{id}"), admin, Value::Null)
            .await
            .1["sequence"]["last_error"]
            .is_null(),
        "Recovered receipt advancement must clear its current scheduling error"
    );
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before + 1,
        "Retrying receipt advancement after audit recovery cannot repeat the accepted provider call"
    );
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
    let person = contact(pool, "provider-pause").await;
    let (id, _, _) = sequence(
        api,
        token,
        "Sequence saved provider pause",
        "Provider pause fixture",
    )
    .await;
    enroll(api, token, id, person).await;
    assert!(stage(pool).await.unwrap());
    let engine: i64 =
        sqlx::query_scalar("SELECT campaign_id FROM crm_sequence_engines WHERE sequence_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    // Simulate an already persisted configuration outage to test the actual
    // recovery function; this does not claim a real-provider outage was observed.
    sqlx::query("UPDATE crm_campaigns SET status='paused',pause_reason='provider_unavailable',last_error='Controlled stored outage' WHERE id=$1").bind(engine).execute(pool).await.unwrap();
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
    campaign_delivery::recover_provider_paused(pool)
        .await
        .unwrap();
    let reason: String = sqlx::query_scalar("SELECT pause_reason FROM crm_campaigns WHERE id=$1")
        .bind(engine)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(reason, "sequence_paused");
    assert!(!stage(pool).await.unwrap());
    assert!(
        !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
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
        StatusCode::OK
    );
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert!(stage(pool).await.unwrap());
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
    let invalid=request(api,"POST",ROOT,admin,json!({"title":"Sequence variable validation","emails":[{"title":"Invalid","email_subject":"Bad variable","email_body":"{{unknown_required_value}}","status":"active"}]})).await;
    assert_eq!(invalid.0, StatusCode::OK);
    let invalid = invalid.1["id"].as_str().unwrap();
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/{invalid}/activate"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    retry_and_reconciliation(api, pool, tokens, adapter).await;
}

async fn retry_and_reconciliation(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
) {
    let token = &tokens[0];
    let admin = Some(token.as_str());
    let mailer = CampaignMailer::from_env("development").unwrap();
    adapter
        .rate_limits
        .store(0, std::sync::atomic::Ordering::SeqCst);
    let person = contact(pool, "retry").await;
    let (id, _, _) = sequence(api, token, "Sequence actual retry", "fixture-rate-limit").await;
    enroll(api, token, id, person).await;
    assert!(stage(pool).await.unwrap());
    let before = adapter.requests.lock().unwrap().len();
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    let retry:(String,i32,bool)=sqlx::query_as("SELECT d.status,d.attempt_count,d.due_at>NOW() FROM crm_campaign_deliveries d JOIN crm_sequence_trackers t ON t.id=d.sequence_tracker_id WHERE t.sequence_id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(retry, ("queued".into(), 1, true));
    assert!(!stage(pool).await.unwrap());
    assert!(
        !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before + 1,
        "No early retry after actual429"
    );
    // Move only the disposable delivery's due time after proving backoff.
    sqlx::query("UPDATE crm_campaign_deliveries d SET due_at=NOW() FROM crm_sequence_trackers t WHERE t.id=d.sequence_tracker_id AND t.sequence_id=$1 AND d.status='queued'").bind(id).execute(pool).await.unwrap();
    let (one, two) = tokio::join!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274"),
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
    );
    assert_eq!(u8::from(one.unwrap()) + u8::from(two.unwrap()), 1);
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 2);
    assert!(stage(pool).await.unwrap());
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

    let person = contact(pool, "expired").await;
    let (id, _, _) = sequence(
        api,
        token,
        "Sequence expired lease",
        "Sequence lease fixture",
    )
    .await;
    enroll(api, token, id, person).await;
    assert!(stage(pool).await.unwrap());
    // Persist a controlled expired lease; no provider request is made here.
    sqlx::query("UPDATE crm_campaign_deliveries d SET status='processing',lease_token=gen_random_uuid(),lease_expires_at=NOW()-INTERVAL '1 minute',attempt_count=1,attempts_remaining=2 FROM crm_sequence_trackers t WHERE t.id=d.sequence_tracker_id AND t.sequence_id=$1").bind(id).execute(pool).await.unwrap();
    let before = adapter.requests.lock().unwrap().len();
    assert_eq!(campaign_delivery::recover_expired(pool).await.unwrap(), 1);
    assert!(stage(pool).await.unwrap());
    assert!(
        !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
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
    let delivery:uuid::Uuid=sqlx::query_scalar("SELECT d.id FROM crm_campaign_deliveries d JOIN crm_sequence_trackers t ON t.id=d.sequence_tracker_id WHERE t.sequence_id=$1").bind(id).fetch_one(pool).await.unwrap();
    let event = json!({"RecordType":"Delivery","MessageStream":"fixture-broadcast","MessageID":uuid::Uuid::new_v4().to_string(),"Recipient":"sequence-worker-expired@example.test","DeliveredAt":chrono::Utc::now().to_rfc3339(),"Metadata":{"delivery_id":delivery}});
    let reconciled = request(
        api,
        "POST",
        "/api/webhooks/postmark/campaigns",
        Some("only-a-disposable-webhook-fixture-secret"),
        event,
    )
    .await;
    assert_eq!(reconciled.0, StatusCode::OK, "{reconciled:?}");
    assert_eq!(reconciled.1["status"], "recorded");
    let resumed = request(
        api,
        "POST",
        &format!("{ROOT}/{id}/activate"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(resumed.0, StatusCode::OK, "{resumed:?}");
    assert!(
        stage(pool).await.unwrap(),
        "Reconciled receipt advances the paused enrollment"
    );
    assert!(
        !stage(pool).await.unwrap(),
        "Reconciled acceptance retains the next email's delay"
    );
    assert!(
        !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before,
        "Reconciliation never resubmits the accepted email"
    );
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
}
