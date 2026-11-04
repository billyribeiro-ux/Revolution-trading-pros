use super::request;
use async_trait::async_trait;
use axum::{
    Router,
    body::Bytes,
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::post,
};
use revolution_api::services::crm_webhooks::{self, Attempt, Outcome, Transport};
use serde_json::{Value, json};
use sqlx::PgPool;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use uuid::Uuid;

type ReceivedMessages = Arc<Mutex<Vec<(HeaderMap, Vec<u8>)>>>;
#[derive(Clone, Default)]
struct Receiver {
    messages: ReceivedMessages,
    failures: Arc<AtomicUsize>,
    code: Arc<AtomicUsize>,
}
async fn receive(State(receiver): State<Receiver>, headers: HeaderMap, bytes: Bytes) -> StatusCode {
    receiver
        .messages
        .lock()
        .unwrap()
        .push((headers, bytes.to_vec()));
    if receiver
        .failures
        .try_update(Ordering::SeqCst, Ordering::SeqCst, |n| n.checked_sub(1))
        .is_ok()
    {
        StatusCode::TOO_MANY_REQUESTS
    } else {
        StatusCode::from_u16(receiver.code.load(Ordering::SeqCst) as u16).unwrap()
    }
}
// Controlled HTTP adapter only: production always uses HttpTransport with HTTPS,
// checked and pinned DNS, no proxies and no redirects. No runtime bypass exists.
struct Adapter {
    url: String,
}
#[async_trait]
impl Transport for Adapter {
    async fn send(&self, attempt: &Attempt) -> Outcome {
        let client = reqwest::Client::builder()
            .no_proxy()
            .redirect(reqwest::redirect::Policy::none())
            .timeout(std::time::Duration::from_secs(2))
            .build()
            .unwrap();
        let request =
            crm_webhooks::signed_request(&client, reqwest::Url::parse(&self.url).unwrap(), attempt)
                .unwrap();
        crm_webhooks::response_outcome(request.send().await).await
    }
}
async fn row(pool: &PgPool, id: Uuid) -> Value {
    sqlx::query_scalar("SELECT to_jsonb(d) FROM crm_webhook_deliveries d WHERE id=$1")
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap()
}
async fn queued(api: &Router, token: &str, id: &str) -> Uuid {
    let (status, body) = request(
        api,
        "POST",
        &format!("/api/admin/crm/webhooks/{id}/test"),
        Some(token),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::ACCEPTED, "{body}");
    assert!(body["success"].is_null());
    Uuid::parse_str(body["delivery_id"].as_str().unwrap()).unwrap()
}
pub(super) async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let prefix = "/api/admin/crm/webhooks";
    let input = json!({"name":"Webhook fixture","url":"https://receiver.example.test/hook","events":crm_webhooks::EVENTS.iter().map(|(key,_)|*key).collect::<Vec<_>>(),"headers":{"X-Fixture":"signed-value"},"secret":"only-disposable-webhook-secret","is_active":false});
    for (token, expected) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, path, body) in [
            ("GET", prefix.to_string(), json!({})),
            ("GET", format!("{prefix}/events"), json!({})),
            ("POST", prefix.to_string(), input.clone()),
        ] {
            assert_eq!(request(api, method, &path, token, body).await.0, expected);
        }
    }
    let (status, events) = request(
        api,
        "GET",
        &format!("{prefix}/events"),
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(events.as_object().unwrap().len(), 13);
    for patch in [
        json!({"name":""}),
        json!({"url":"http://example.test"}),
        json!({"url":"https://127.0.0.1"}),
        json!({"url":"https://user:password@example.test"}),
        json!({"url":"https://example.test/#fragment"}),
        json!({"events":["unknown"]}),
        json!({"events":["deal_won","deal_won"]}),
        json!({"headers":{"Host":"unsafe"}}),
        json!({"headers":{"X-Webhook-Signature":"unsafe"}}),
        json!({"headers":{"X-Bad":"a\r\nb"}}),
        json!({"secret":"x".repeat(256)}),
    ] {
        let mut invalid = input.clone();
        invalid
            .as_object_mut()
            .unwrap()
            .extend(patch.as_object().unwrap().clone());
        let result = request(api, "POST", prefix, Some(&tokens[0]), invalid).await;
        assert_eq!(result.0, StatusCode::BAD_REQUEST, "{:?}", result);
    }
    let (status, created) = request(api, "POST", prefix, Some(&tokens[0]), input).await;
    assert_eq!(status, StatusCode::CREATED, "{created}");
    let id = created["id"].as_str().unwrap();
    let path = format!("{prefix}/{id}");
    assert_eq!(created["is_active"], false);
    assert_eq!(created["headers"]["X-Fixture"], "signed-value");
    for (token, expected) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, route, body) in [
            ("GET", path.clone(), json!({})),
            ("PUT", path.clone(), json!({"is_active":true})),
            ("DELETE", path.clone(), json!({})),
            ("POST", format!("{path}/test"), json!({})),
            ("GET", format!("{path}/logs"), json!({})),
        ] {
            assert_eq!(request(api, method, &route, token, body).await.0, expected);
        }
    }
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/test"),
            Some(&tokens[0]),
            json!({})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            Some(&tokens[0]),
            json!({"is_active":true})
        )
        .await
        .0,
        StatusCode::OK
    );
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("INSERT INTO contacts(email) VALUES('rollback-webhook@example.test')")
        .execute(&mut *tx)
        .await
        .unwrap();
    tx.rollback().await.unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM crm_webhook_deliveries WHERE webhook_id=$1"
        )
        .bind(id.parse::<i64>().unwrap())
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    let contact:i64=sqlx::query_scalar("INSERT INTO contacts(email,first_name,tags) VALUES('webhook-contact@example.test','Webhook','[]') RETURNING id").fetch_one(pool).await.unwrap();
    let first: Uuid = sqlx::query_scalar(
        "SELECT id FROM crm_webhook_deliveries WHERE webhook_id=$1 AND event='contact_created'",
    )
    .bind(id.parse::<i64>().unwrap())
    .fetch_one(pool)
    .await
    .unwrap();
    let receiver = Receiver::default();
    receiver.code.store(204, Ordering::SeqCst);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let address = listener.local_addr().unwrap();
    let receiver_app = Router::new()
        .route("/hook", post(receive))
        .with_state(receiver.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, receiver_app).await.unwrap();
    });
    let adapter = Adapter {
        url: format!("http://{address}/hook"),
    };
    assert!(
        crm_webhooks::process_one(pool, &adapter, Some(first))
            .await
            .unwrap()
    );
    assert_eq!(row(pool, first).await["status"], "delivered");
    {
        let messages = receiver.messages.lock().unwrap();
        let (headers, bytes) = &messages[0];
        assert_eq!(headers["x-fixture"], "signed-value");
        assert_eq!(headers["x-webhook-delivery-id"], first.to_string());
        assert_eq!(
            headers["x-webhook-signature"],
            format!(
                "sha256={}",
                revolution_api::services::cms_webhooks::generate_signature(
                    std::str::from_utf8(bytes).unwrap(),
                    "only-disposable-webhook-secret"
                )
            )
        );
        let payload: Value = serde_json::from_slice(bytes).unwrap();
        assert_eq!(payload["event"], "contact_created");
        assert_eq!(payload["data"]["id"], contact.to_string());
        assert_eq!(payload["data"]["email"], "webhook-contact@example.test");
    }
    receiver.failures.store(1, Ordering::SeqCst);
    let retry = queued(api, &tokens[0], id).await;
    assert!(
        crm_webhooks::process_one(pool, &adapter, Some(retry))
            .await
            .unwrap()
    );
    assert_eq!(row(pool, retry).await["status"], "queued");
    assert_eq!(row(pool, retry).await["response_code"], 429);
    assert!(
        !crm_webhooks::process_one(pool, &adapter, Some(retry))
            .await
            .unwrap()
    );
    sqlx::query("UPDATE crm_webhook_deliveries SET due_at=NOW()-INTERVAL '1 second' WHERE id=$1")
        .bind(retry)
        .execute(pool)
        .await
        .unwrap();
    assert!(
        crm_webhooks::process_one(pool, &adapter, Some(retry))
            .await
            .unwrap()
    );
    assert_eq!(row(pool, retry).await["status"], "delivered");
    assert_eq!(row(pool, retry).await["attempts"], 2);
    {
        let messages = receiver.messages.lock().unwrap();
        assert_eq!(messages[1].1, messages[2].1, "Retry payload must be stable");
        assert_eq!(
            messages[1].0["x-webhook-delivery-id"],
            messages[2].0["x-webhook-delivery-id"]
        );
    }
    receiver.code.store(400, Ordering::SeqCst);
    let failed = queued(api, &tokens[0], id).await;
    crm_webhooks::process_one(pool, &adapter, Some(failed))
        .await
        .unwrap();
    assert_eq!(row(pool, failed).await["status"], "failed");
    assert_eq!(row(pool, failed).await["attempts"], 1);
    // Claims serialize across workers; the second cannot send an active lease.
    let leased = queued(api, &tokens[0], id).await;
    let (left, right) = tokio::join!(
        crm_webhooks::claim(pool, Some(leased)),
        crm_webhooks::claim(pool, Some(leased))
    );
    assert_eq!(
        usize::from(left.unwrap().is_some()) + usize::from(right.unwrap().is_some()),
        1
    );
    sqlx::query(
        "UPDATE crm_webhook_deliveries SET lease_expires_at=NOW()-INTERVAL '1 second' WHERE id=$1",
    )
    .bind(leased)
    .execute(pool)
    .await
    .unwrap();
    receiver.code.store(204, Ordering::SeqCst);
    crm_webhooks::process_one(pool, &adapter, Some(leased))
        .await
        .unwrap();
    assert_eq!(row(pool, leased).await["status"], "delivered");
    assert_eq!(row(pool, leased).await["attempts"], 2);
    receiver.code.store(500, Ordering::SeqCst);
    let exhausted = queued(api, &tokens[0], id).await;
    for attempt in 1..=3 {
        sqlx::query(
            "UPDATE crm_webhook_deliveries SET due_at=NOW()-INTERVAL '1 second' WHERE id=$1",
        )
        .bind(exhausted)
        .execute(pool)
        .await
        .unwrap();
        assert!(
            crm_webhooks::process_one(pool, &adapter, Some(exhausted))
                .await
                .unwrap()
        );
        assert_eq!(row(pool, exhausted).await["attempts"], attempt);
    }
    assert_eq!(row(pool, exhausted).await["status"], "failed");
    assert!(
        !crm_webhooks::process_one(pool, &adapter, Some(exhausted))
            .await
            .unwrap()
    );
    // A legacy row bypassing authoring validation still cannot reach loopback.
    let blocked = queued(api, &tokens[0], id).await;
    sqlx::query("UPDATE crm_webhooks SET url=$2 WHERE id=$1")
        .bind(id.parse::<i64>().unwrap())
        .bind(format!("https://{address}/hook"))
        .execute(pool)
        .await
        .unwrap();
    let received = receiver.messages.lock().unwrap().len();
    assert!(
        crm_webhooks::process_one(pool, &crm_webhooks::HttpTransport, Some(blocked))
            .await
            .unwrap()
    );
    assert_eq!(row(pool, blocked).await["status"], "failed");
    assert!(row(pool, blocked).await["response_code"].is_null());
    assert_eq!(receiver.messages.lock().unwrap().len(), received);
    sqlx::query("UPDATE crm_webhooks SET url='https://receiver.example.test/hook' WHERE id=$1")
        .bind(id.parse::<i64>().unwrap())
        .execute(pool)
        .await
        .unwrap();
    // Every established CRM event is sourced from an actual committed mutation.
    sqlx::query("UPDATE contacts SET tags='[\"fixture-tag\"]' WHERE id=$1")
        .bind(contact)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE contacts SET tags='[]',status='unsubscribed' WHERE id=$1")
        .bind(contact)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE contacts SET tags='[\"cleared-tag\"]' WHERE id=$1")
        .bind(contact)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE contacts SET tags=NULL WHERE id=$1")
        .bind(contact)
        .execute(pool)
        .await
        .unwrap();
    let initially_tagged:i64=sqlx::query_scalar("INSERT INTO contacts(email,tags) VALUES('initial-tag@example.test','[\"initial-tag\"]') RETURNING id").fetch_one(pool).await.unwrap();
    for (event, tag) in [
        ("tag_applied", "initial-tag"),
        ("tag_removed", "cleared-tag"),
    ] {
        let exists:bool=sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM crm_webhook_deliveries WHERE webhook_id=$1 AND event=$2 AND payload->'data'->>'tag'=$3)").bind(id.parse::<i64>().unwrap()).bind(event).bind(tag).fetch_one(pool).await.unwrap();
        assert!(exists);
    }
    sqlx::query("DELETE FROM contacts WHERE id=$1")
        .bind(initially_tagged)
        .execute(pool)
        .await
        .unwrap();
    let list: i64 = sqlx::query_scalar(
        "INSERT INTO crm_lists(name,title,slug) VALUES('Webhook list','Webhook list','webhook-list') RETURNING id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    sqlx::query("INSERT INTO crm_list_members(list_id,contact_id) VALUES($1,$2)")
        .bind(list)
        .bind(contact)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM crm_list_members WHERE list_id=$1")
        .bind(list)
        .execute(pool)
        .await
        .unwrap();
    let pipeline: i64 = sqlx::query_scalar(
        "INSERT INTO crm_pipelines(name) VALUES('Webhook pipeline') RETURNING id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let stage: i64 = sqlx::query_scalar(
        "INSERT INTO crm_pipeline_stages(pipeline_id,name) VALUES($1,'Webhook stage') RETURNING id",
    )
    .bind(pipeline)
    .fetch_one(pool)
    .await
    .unwrap();
    let deal:i64=sqlx::query_scalar("INSERT INTO crm_deals(name,status,amount,currency,pipeline_id,stage_id) VALUES('Webhook deal','open',12.34,'USD',$1,$2) RETURNING id").bind(pipeline).bind(stage).fetch_one(pool).await.unwrap();
    sqlx::query("UPDATE crm_deals SET status='won' WHERE id=$1")
        .bind(deal)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE crm_deals SET status='lost' WHERE id=$1")
        .bind(deal)
        .execute(pool)
        .await
        .unwrap();
    for event in ["Open", "Click"] {
        sqlx::query("INSERT INTO crm_campaign_events(fingerprint,recipient_email,record_type,occurred_at) VALUES($1,'webhook-contact@example.test',$2,NOW())").bind(format!("webhook-fixture-{event}")).bind(event).execute(pool).await.unwrap();
    }
    sqlx::query("DELETE FROM contacts WHERE id=$1")
        .bind(contact)
        .execute(pool)
        .await
        .unwrap();
    let names: Vec<String> =
        sqlx::query_scalar("SELECT DISTINCT event FROM crm_webhook_deliveries WHERE webhook_id=$1")
            .bind(id.parse::<i64>().unwrap())
            .fetch_all(pool)
            .await
            .unwrap();
    for (event, _) in crm_webhooks::EVENTS {
        assert!(
            names.iter().any(|name| name == event),
            "Missing real event {event}: {names:?}"
        );
    }
    let (status, logs) = request(
        api,
        "GET",
        &format!("{path}/logs?per_page=2&page=2"),
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(logs["data"].as_array().unwrap().len(), 2);
    assert_eq!(logs["meta"]["current_page"], 2);
    assert!(logs["data"][0].get("lease_token").is_none());
    let cancelled = queued(api, &tokens[0], id).await;
    request(
        api,
        "PUT",
        &path,
        Some(&tokens[0]),
        json!({"is_active":false,"secret":"","headers":{}}),
    )
    .await;
    assert_eq!(row(pool, cancelled).await["status"], "cancelled");
    let (_, saved) = request(api, "GET", &path, Some(&tokens[0]), json!({})).await;
    assert!(saved["secret"].is_null());
    assert_eq!(saved["headers"], json!({}));
    for suffix in ["?page=0", "?per_page=101", "?status=bad"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{prefix}{suffix}"),
                Some(&tokens[0]),
                json!({})
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    let (_, list) = request(
        api,
        "GET",
        &format!("{prefix}?search=Webhook%20fixture&status=inactive&per_page=1"),
        Some(&tokens[0]),
        json!({}),
    )
    .await;
    assert_eq!(list["meta"]["total"], 1);
    assert!(list["data"][0].get("secret").is_none());
    assert_eq!(list["stats"]["totalTriggers"], 9);
    assert_eq!(list["stats"]["totalFailures"], 6);
    assert_eq!(
        request(api, "DELETE", &path, Some(&tokens[0]), json!({}))
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "GET", &path, Some(&tokens[0]), json!({}))
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM crm_webhook_deliveries WHERE webhook_id=$1"
        )
        .bind(id.parse::<i64>().unwrap())
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    server.abort();
    let _ = server.await;
}
