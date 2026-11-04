//! Actual API/queue SMTP proof against an owned literal-loopback relay.
use super::{Adapter as PostmarkAdapter, app, request};
use axum::{Router, http::StatusCode};
use base64::{Engine as _, engine::general_purpose::STANDARD};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::PgPool;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicUsize, Ordering},
};
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tower::ServiceExt;

#[derive(Clone, Default)]
struct Relay {
    mode: Arc<AtomicUsize>,
    messages: Arc<Mutex<Vec<String>>>,
}
fn bodies(raw: &str) -> Vec<String> {
    raw.split("Content-Transfer-Encoding: base64\r\n\r\n")
        .skip(1)
        .map(|part| {
            let encoded = part
                .split("\r\n")
                .take_while(|line| !line.is_empty() && !line.starts_with("--"))
                .collect::<String>();
            String::from_utf8(STANDARD.decode(encoded).unwrap()).unwrap()
        })
        .collect()
}
async fn serve(socket: tokio::net::TcpStream, relay: Relay) {
    let (read, mut write) = socket.into_split();
    let mut read = BufReader::new(read);
    write
        .write_all(b"220 fixture.example.test ESMTP\r\n")
        .await
        .unwrap();
    let mut authenticated = false;
    let mut data = false;
    let mut content = String::new();
    loop {
        let mut line = String::new();
        let size =
            tokio::time::timeout(std::time::Duration::from_secs(5), read.read_line(&mut line))
                .await
                .unwrap()
                .unwrap();
        if size == 0 {
            break;
        }
        if data {
            if line == ".\r\n" {
                relay.messages.lock().unwrap().push(content);
                match relay.mode.load(Ordering::SeqCst) {
                    0 => write
                        .write_all(b"250 2.0.0 accepted fixture\r\n")
                        .await
                        .unwrap(),
                    1 => write
                        .write_all(b"550 5.7.0 rejected fixture\r\n")
                        .await
                        .unwrap(),
                    _ => write.shutdown().await.unwrap(),
                }
                break;
            }
            assert!(content.len() + line.len() < 8_000_000);
            content.push_str(&line);
            continue;
        }
        let reply = if line.starts_with("EHLO ") {
            "250-fixture.example.test\r\n250-AUTH PLAIN\r\n250 SMTPUTF8\r\n"
        } else if let Some(encoded) = line.trim().strip_prefix("AUTH PLAIN ") {
            authenticated =
                STANDARD.decode(encoded).unwrap() == b"\0fixture-user\0fixture-password";
            if authenticated {
                "235 2.7.0 Accepted\r\n"
            } else {
                "535 5.7.8 Rejected\r\n"
            }
        } else if line.trim() == "NOOP" || line.starts_with("MAIL FROM:") {
            assert!(authenticated);
            "250 2.0.0 OK\r\n"
        } else if line.starts_with("RCPT TO:") {
            assert!(authenticated);
            let address = line.split('<').nth(1).unwrap().split('>').next().unwrap();
            assert!(address.ends_with(".test"), "Fixture recipients only");
            "250 2.0.0 OK\r\n"
        } else if line.trim() == "DATA" {
            assert!(authenticated);
            data = true;
            "354 Continue\r\n"
        } else if line.trim() == "QUIT" {
            let _ = write.write_all(b"221 Bye\r\n").await;
            break;
        } else {
            panic!("Unexpected SMTP command")
        };
        if write.write_all(reply.as_bytes()).await.is_err() {
            break;
        }
    }
}
pub(super) async fn proof(
    api: &Router,
    pool: &PgPool,
    superadmin: &str,
    admin: &str,
    postmark: &PostmarkAdapter,
) {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:19026")
        .await
        .unwrap();
    let relay = Relay::default();
    let adapter = relay.clone();
    let server = tokio::spawn(async move {
        loop {
            let (socket, _) = listener.accept().await.unwrap();
            tokio::spawn(serve(socket, adapter.clone()));
        }
    });
    let previous = postmark.requests.lock().unwrap().len();
    // Earlier retry/future fixtures have already made their assertions. Defer
    // them so these execution proofs claim only the explicitly queued work.
    sqlx::query("UPDATE jobs SET available_at=NOW()+INTERVAL '1 day' WHERE status='pending'")
        .execute(pool)
        .await
        .unwrap();
    let fields = json!({"provider":"smtp","host":"127.0.0.1","port":19026,"username":"fixture-user","password":"fixture-password","encryption":"null","from_address":"smtp-runtime@example.test","from_name":"Runtime SMTP fixture"});
    let saved = request(
        api,
        "POST",
        "/api/admin/email/settings",
        Some(superadmin),
        fields.clone(),
    )
    .await;
    assert_eq!(saved.0, StatusCode::OK, "{saved:?}");
    let db = revolution_api::db::Database { pool: pool.clone() };
    let blocked=revolution_api::queue::enqueue(pool,"smtp-proof","send_email",json!({"to":"smtp-blocked@example.test","subject":"Pending configuration fixture","body":"<p>Fixture</p>"})).await.unwrap();
    assert_eq!(
        revolution_api::queue::worker::process_next_job(&db)
            .await
            .unwrap(),
        Some(blocked)
    );
    let pending: String = sqlx::query_scalar("SELECT status FROM jobs WHERE id=$1")
        .bind(blocked)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(pending, "pending");
    assert!(relay.messages.lock().unwrap().is_empty());
    assert_eq!(
        postmark.requests.lock().unwrap().len(),
        previous,
        "Unverified SMTP must not fall back to environment credentials"
    );
    let pending_state = super::state(db.clone(), None).await;
    assert!(
        revolution_api::services::campaign_mailer::CampaignMailer::from_state(&pending_state)
            .await
            .is_err()
    );
    let checked = request(
        api,
        "POST",
        "/api/admin/email/settings/test",
        Some(superadmin),
        fields,
    )
    .await;
    assert_eq!(checked.1["success"], true, "{checked:?}");
    sqlx::query("UPDATE jobs SET available_at=NOW() WHERE id=$1")
        .bind(blocked)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        revolution_api::queue::worker::process_next_job(&db)
            .await
            .unwrap(),
        Some(blocked)
    );
    let completed: (String, i32) = sqlx::query_as("SELECT status,attempts FROM jobs WHERE id=$1")
        .bind(blocked)
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(completed, ("completed".into(), 2));
    let receipt:(String,String,String)=sqlx::query_as("SELECT status,provider,provider_message_id FROM email_logs WHERE email='smtp-blocked@example.test' ORDER BY id DESC LIMIT 1").fetch_one(pool).await.unwrap();
    assert_eq!((&receipt.0[..], &receipt.1[..]), ("sent", "smtp"));
    assert!(
        relay.messages.lock().unwrap()[0].contains(&format!("Message-ID: <{}>\r\n", receipt.2))
    );
    let template=request(api,"POST","/api/admin/email/templates",Some(admin),json!({"name":"SMTP runtime fixture","subject":"SMTP template receipt","body":"<p>Hello {{admin_name}} &amp; fixture</p>","variables":["admin_name"]})).await;
    assert_eq!(template.0, StatusCode::OK, "{template:?}");
    let id = template.1["data"]["id"].as_i64().unwrap();
    let send = request(
        api,
        "POST",
        &format!("/api/admin/email/templates/{id}/test"),
        Some(admin),
        json!({"email":"smtp-template@example.test"}),
    )
    .await;
    assert_eq!(send.0, StatusCode::OK, "{send:?}");
    let raw = relay.messages.lock().unwrap().last().unwrap().clone();
    assert!(
        bodies(&raw)
            .iter()
            .any(|body| body.contains("<p>Hello ") && body.contains("&amp; fixture</p>"))
    );
    for (mode, outcome) in [(1, "rejected"), (2, "unknown")] {
        relay.mode.store(mode, Ordering::SeqCst);
        let address = format!("smtp-template-outcome-{mode}@example.test");
        let before = relay.messages.lock().unwrap().len();
        let sent = request(
            api,
            "POST",
            &format!("/api/admin/email/templates/{id}/test"),
            Some(admin),
            json!({"email":address}),
        )
        .await;
        assert_eq!(sent.0, StatusCode::BAD_GATEWAY, "{sent:?}");
        assert_eq!(sent.1["submission_status"], outcome);
        assert_eq!(relay.messages.lock().unwrap().len(), before + 1);
        let status: String = sqlx::query_scalar(
            "SELECT status FROM email_logs WHERE to_email=$1 ORDER BY id DESC LIMIT 1",
        )
        .bind(address)
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(
            status,
            if mode == 1 {
                "failed"
            } else {
                "delivery_unknown"
            }
        );
    }
    relay.mode.store(0, Ordering::SeqCst);
    let subscribed = request(
        api,
        "POST",
        "/api/newsletter/subscribe",
        None,
        json!({"email":"smtp-subscriber@example.test","name":"SMTP fixture","gdpr_consent":true}),
    )
    .await;
    assert!(subscribed.0.is_success(), "{subscribed:?}");
    let raw = relay.messages.lock().unwrap().last().unwrap().clone();
    let parts = bodies(&raw);
    let link = parts
        .iter()
        .find(|body| !body.contains("<main>"))
        .unwrap()
        .split_whitespace()
        .find(|part| part.starts_with("http://"))
        .unwrap()
        .trim_end_matches('.');
    let url = reqwest::Url::parse(link).unwrap();
    let token = url
        .query_pairs()
        .find(|(key, _)| key == "token")
        .unwrap()
        .1
        .into_owned();
    let logged:Value=sqlx::query_scalar("SELECT model FROM email_logs WHERE email='smtp-subscriber@example.test' AND template_alias='newsletter-confirmation' ORDER BY id DESC LIMIT 1").fetch_one(pool).await.unwrap();
    assert_eq!(logged["confirm_url"], "[redacted]");
    assert!(!logged.to_string().contains(&token));
    let confirmed = request(
        api,
        "GET",
        &format!("/api/newsletter/confirm?token={token}"),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(confirmed.0, StatusCode::OK, "{confirmed:?}");
    let consent:(String,bool)=sqlx::query_as("SELECT status,gdpr_consent FROM newsletter_subscribers WHERE email='smtp-subscriber@example.test'").fetch_one(pool).await.unwrap();
    assert_eq!(consent, ("confirmed".into(), true));
    for (mode, expected) in [(1, "failed"), (2, "delivery_unknown")] {
        relay.mode.store(mode, Ordering::SeqCst);
        let address = format!("smtp-outcome-{mode}@example.test");
        let job = revolution_api::queue::enqueue(
            pool,
            "smtp-proof",
            "send_email",
            json!({"to":address,"subject":"SMTP outcome fixture","body":"<p>Fixture</p>"}),
        )
        .await
        .unwrap();
        assert_eq!(
            revolution_api::queue::worker::process_next_job(&db)
                .await
                .unwrap(),
            Some(job)
        );
        let state: (String, i32) = sqlx::query_as("SELECT status,attempts FROM jobs WHERE id=$1")
            .bind(job)
            .fetch_one(pool)
            .await
            .unwrap();
        assert_eq!(state, ("failed".into(), 1));
        let log:(String,Option<String>,Option<chrono::DateTime<chrono::Utc>>)=sqlx::query_as("SELECT status,provider_message_id,sent_at FROM email_logs WHERE email=$1 ORDER BY id DESC LIMIT 1").bind(address).fetch_one(pool).await.unwrap();
        assert_eq!(log.0, expected);
        assert!(log.1.is_none() && log.2.is_none());
        assert_eq!(
            revolution_api::queue::worker::process_next_job(&db)
                .await
                .unwrap(),
            None,
            "Rejected/unknown work must not resend automatically"
        );
    }
    relay.mode.store(0, Ordering::SeqCst);
    let cold = app(db.clone(), None).await;
    let status = request(
        &cold,
        "GET",
        "/api/admin/email/status",
        Some(admin),
        Value::Null,
    )
    .await;
    assert_eq!(status.0, StatusCode::OK, "{status:?}");
    assert_eq!(status.1["provider"], "smtp");
    assert_eq!(status.1["email_service_enabled"], true);
    assert_eq!(status.1["from_email"], "smtp-runtime@example.test");
    let native = request(
        &cold,
        "GET",
        "/api/admin/connections?environment=runtime",
        Some(admin),
        Value::Null,
    )
    .await;
    assert_eq!(native.0, StatusCode::OK, "{native:?}");
    let smtp = native.1["connections"]
        .as_array()
        .unwrap()
        .iter()
        .find(|value| value["key"] == "fluent_smtp")
        .unwrap();
    assert_eq!(smtp["is_connected"], true);
    assert!(smtp["connection"]["last_verified_at"].is_string());

    campaign_proof(api, &db, admin, &relay).await;
    // Selecting SMTP must work when the Postmark service has no credentials,
    // including an unknown submission that must never be reported as unavailable.
    let no_postmark = app(
        db.clone(),
        Some(revolution_api::services::email::EmailService::new(
            "",
            "smtp-runtime@example.test",
            "http://127.0.0.1:5274",
        )),
    )
    .await;
    let selected = request(
        &no_postmark,
        "GET",
        "/api/admin/email/status",
        Some(admin),
        Value::Null,
    )
    .await;
    assert_eq!(selected.1["postmark_token_set"], false);
    assert_eq!(selected.1["provider"], "smtp");
    assert_eq!(selected.1["email_service_enabled"], true);
    relay.mode.store(2, Ordering::SeqCst);
    let unknown = request(
        &no_postmark,
        "POST",
        "/api/admin/email/subscribers",
        Some(admin),
        json!({"email":"smtp-no-postmark-unknown@example.test","first_name":"SMTP fixture"}),
    )
    .await;
    assert_eq!(unknown.0, StatusCode::CREATED, "{unknown:?}");
    assert_eq!(
        unknown.1["confirmation"]["status"], "unknown",
        "{unknown:?}"
    );
    let subscriber = unknown.1["subscriber"]["id"].as_str().unwrap();
    let messages = relay.messages.lock().unwrap().len();
    let repeated = request(
        &no_postmark,
        "POST",
        &format!("/api/admin/email/subscribers/{subscriber}/resubscribe"),
        Some(admin),
        Value::Null,
    )
    .await;
    assert_eq!(repeated.0, StatusCode::TOO_MANY_REQUESTS, "{repeated:?}");
    assert_eq!(relay.messages.lock().unwrap().len(), messages);
    relay.mode.store(0, Ordering::SeqCst);
    let before = relay.messages.lock().unwrap().len();
    sqlx::query("ALTER TABLE settings RENAME TO admin_move_smtp_runtime_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let job=revolution_api::queue::enqueue(pool,"smtp-proof","send_email",json!({"to":"smtp-storage-unavailable@example.test","subject":"Storage outage fixture","body":"<p>Fixture</p>"})).await.unwrap();
    let claimed = revolution_api::queue::worker::process_next_job(&db).await;
    sqlx::query("ALTER TABLE admin_move_smtp_runtime_unavailable RENAME TO settings")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(claimed.unwrap(), Some(job));
    assert_eq!(relay.messages.lock().unwrap().len(), before);
    assert_eq!(
        postmark.requests.lock().unwrap().len(),
        previous,
        "Storage failure must never reactivate environment credentials"
    );
    server.abort();
}

async fn tracking(
    api: &Router,
    method: &str,
    path: &str,
) -> (StatusCode, axum::http::HeaderMap, Vec<u8>) {
    let response = api
        .clone()
        .oneshot(
            axum::http::Request::builder()
                .method(method)
                .uri(path)
                .extension(axum::extract::ConnectInfo(std::net::SocketAddr::from((
                    [127, 0, 0, 1],
                    45001,
                ))))
                .body(axum::body::Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (status, headers, bytes)
}

async fn campaign_proof(
    api: &Router,
    db: &revolution_api::db::Database,
    admin: &str,
    relay: &Relay,
) {
    use revolution_api::services::{
        campaign_delivery, campaign_mailer::CampaignMailer, email::EmailService,
    };
    let pool = &db.pool;
    let state = super::state(
        db.clone(),
        Some(EmailService::new(
            "",
            "unused@example.test",
            "http://127.0.0.1:5274",
        )),
    )
    .await;
    let mailer = CampaignMailer::from_state(&state).await.unwrap();
    assert_eq!(mailer.provider(), "smtp");
    assert_eq!(mailer.sender().email, "smtp-runtime@example.test");
    // Prior campaigns have completed their assertions; isolate worker claims.
    sqlx::query("UPDATE crm_campaigns SET status='paused',pause_reason='manual' WHERE status IN ('sending','scheduled')").execute(pool).await.unwrap();
    for (mode, expected) in [(0, "accepted"), (1, "failed"), (2, "unknown")] {
        relay.mode.store(mode, Ordering::SeqCst);
        let tag = format!("smtp-campaign-{mode}");
        let email = format!("{tag}@example.test");
        sqlx::query("INSERT INTO newsletter_subscribers(email,status,confirmed_at,tags,source) VALUES($1,'confirmed',NOW(),$2,'smtp-fixture')")
            .bind(&email).bind(json!([tag])).execute(pool).await.unwrap();
        let queued = request(api, "POST", "/api/newsletter/bulk-send", Some(admin), json!({"subject":format!("SMTP campaign {mode}"),"html_content":"<!doctype html><html><head><style>.fixture{color:red}</style></head><body><a href='https://target.example.test/offers?one=1&amp;two=2#details'>Offer</a></body></html>","text_content":"Plain alternative https://target.example.test/plain","segment":tag,"rate_limit":100})).await;
        assert_eq!(queued.0, StatusCode::ACCEPTED, "{queued:?}");
        let campaign: i64 = queued.1["job"]["campaign_id"]
            .as_str()
            .unwrap()
            .parse()
            .unwrap();
        let before = relay.messages.lock().unwrap().len();
        assert!(
            campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
        assert_eq!(relay.messages.lock().unwrap().len(), before + 1);
        let receipt: Value = sqlx::query_scalar(
            "SELECT to_jsonb(d) FROM crm_campaign_deliveries d WHERE campaign_id=$1",
        )
        .bind(campaign)
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(receipt["status"], expected);
        assert_eq!(receipt["provider"], "smtp");
        assert_eq!(receipt["provider_stream"], "smtp");
        assert!(receipt["delivered_at"].is_null());
        let raw = relay.messages.lock().unwrap()[before].clone();
        assert!(raw.contains("List-Unsubscribe-Post: List-Unsubscribe=One-Click"));
        let decoded = bodies(&raw);
        let html = decoded.iter().find(|body| body.contains("<html>")).unwrap();
        assert!(html.contains("<style>.fixture{color:red}</style>"));
        assert!(html.contains("/tracking/open/"));
        assert!(decoded.iter().any(|body| {
            body.contains("Plain alternative http://127.0.0.1:5274/api/newsletter/tracking/click/")
        }));
        let token = receipt["tracking_token"].as_str().unwrap();
        let open = format!("/api/newsletter/tracking/open/{token}");
        let head = tracking(api, "HEAD", &open).await;
        assert_eq!(head.0, StatusCode::OK);
        assert!(head.2.is_empty());
        assert!(
            sqlx::query_scalar::<_, bool>(
                "SELECT opened_at IS NULL FROM crm_campaign_deliveries WHERE campaign_id=$1"
            )
            .bind(campaign)
            .fetch_one(pool)
            .await
            .unwrap()
        );
        let cold = app(db.clone(), None).await;
        for _ in 0..2 {
            let observed = tracking(&cold, "GET", &open).await;
            assert_eq!(observed.0, StatusCode::OK);
            assert_eq!(observed.1["content-type"], "image/gif");
            assert!(
                observed.1["cache-control"]
                    .to_str()
                    .unwrap()
                    .contains("no-store")
            );
            assert!(observed.2.starts_with(b"GIF89a"));
        }
        let count: i64 = sqlx::query_scalar(
            "SELECT COUNT(*) FROM crm_campaign_events WHERE delivery_id=$1 AND record_type='Open'",
        )
        .bind(uuid::Uuid::parse_str(receipt["id"].as_str().unwrap()).unwrap())
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(count, if mode == 1 { 0 } else { 1 });
        let (link, target): (uuid::Uuid, String) = sqlx::query_as("SELECT id,target FROM crm_campaign_tracking_links WHERE delivery_id=$1 AND target LIKE '%/offers%' LIMIT 1").bind(uuid::Uuid::parse_str(receipt["id"].as_str().unwrap()).unwrap()).fetch_one(pool).await.unwrap();
        assert_eq!(
            target,
            "https://target.example.test/offers?one=1&two=2#details"
        );
        let click = format!("/api/newsletter/tracking/click/{link}");
        assert_eq!(tracking(&cold, "HEAD", &click).await.0, StatusCode::FOUND);
        assert!(
            sqlx::query_scalar::<_, bool>(
                "SELECT clicked_at IS NULL FROM crm_campaign_deliveries WHERE campaign_id=$1"
            )
            .bind(campaign)
            .fetch_one(pool)
            .await
            .unwrap()
        );
        for _ in 0..2 {
            let observed = tracking(&cold, "GET", &click).await;
            assert_eq!(observed.0, StatusCode::FOUND);
            assert_eq!(observed.1["location"], target);
            assert_eq!(observed.1["referrer-policy"], "no-referrer");
        }
        let current: Value =
            sqlx::query_scalar("SELECT to_jsonb(c) FROM crm_campaigns c WHERE id=$1")
                .bind(campaign)
                .fetch_one(pool)
                .await
                .unwrap();
        assert_eq!(current["opens"], if mode == 1 { 0 } else { 1 });
        assert_eq!(current["clicks"], if mode == 1 { 0 } else { 1 });
        assert_eq!(current["emails_sent"], if mode == 0 { 1 } else { 0 });
        let persisted: (String, Option<String>, bool) = sqlx::query_as("SELECT status,provider_message_id,delivered_at IS NULL FROM crm_campaign_deliveries WHERE campaign_id=$1").bind(campaign).fetch_one(pool).await.unwrap();
        assert_eq!(persisted.0, expected);
        assert!(persisted.2);
        if mode == 0 {
            sqlx::query(
                "ALTER TABLE crm_campaign_events RENAME TO smtp_tracking_events_unavailable",
            )
            .execute(pool)
            .await
            .unwrap();
            let unavailable = tracking(&cold, "GET", &open).await;
            sqlx::query(
                "ALTER TABLE smtp_tracking_events_unavailable RENAME TO crm_campaign_events",
            )
            .execute(pool)
            .await
            .unwrap();
            assert_eq!(unavailable.0, StatusCode::SERVICE_UNAVAILABLE);

            assert!(raw.contains(&format!(
                "Message-ID: <{}>\r\n",
                receipt["provider_message_id"].as_str().unwrap()
            )));
            assert!(
                campaign_delivery::unsubscribe(
                    pool,
                    uuid::Uuid::parse_str(receipt["unsubscribe_token"].as_str().unwrap()).unwrap()
                )
                .await
                .unwrap()
            );
            let consent: String =
                sqlx::query_scalar("SELECT status FROM newsletter_subscribers WHERE email=$1")
                    .bind(&email)
                    .fetch_one(pool)
                    .await
                    .unwrap();
            assert_eq!(consent, "unsubscribed");
        } else {
            assert!(receipt["provider_message_id"].is_null());
            if mode == 2 {
                let wrong_provider = request(api, "POST", "/api/webhooks/postmark/campaigns", Some("only-a-disposable-webhook-fixture-secret"), json!({"RecordType":"Delivery","MessageStream":"fixture-broadcast","MessageID":"unrelated-postmark-fixture","Recipient":email,"DeliveredAt":chrono::Utc::now().to_rfc3339(),"Metadata":{"delivery_id":receipt["id"]}})).await;
                assert_eq!(wrong_provider.0, StatusCode::OK);
                let status: String = sqlx::query_scalar(
                    "SELECT status FROM crm_campaign_deliveries WHERE campaign_id=$1",
                )
                .bind(campaign)
                .fetch_one(pool)
                .await
                .unwrap();
                assert_eq!(
                    status, "unknown",
                    "A different provider cannot reconcile SMTP acceptance"
                );
                assert_eq!(current["pause_reason"], "uncertain_submission");
            }
        }
        assert!(
            !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        );
        assert_eq!(relay.messages.lock().unwrap().len(), before + 1);
    }
    let unknown = format!("/api/newsletter/tracking/open/{}", uuid::Uuid::new_v4());
    assert_eq!(
        tracking(api, "GET", &unknown).await.0,
        StatusCode::NOT_FOUND
    );
    relay.mode.store(0, Ordering::SeqCst);
}
