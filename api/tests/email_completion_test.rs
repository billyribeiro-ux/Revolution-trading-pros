//! Real API/DB/provider-adapter proof. Never run with live email credentials.
#[path = "support/authorization_parity.rs"]
mod authorization_parity_proof;
#[path = "support/connections.rs"]
mod connections_proof;
#[path = "support/crm_webhooks.rs"]
mod crm_webhook_proof;
#[path = "support/email_subscribers.rs"]
mod email_subscriber_proof;
#[path = "support/performance.rs"]
mod performance_proof;
#[path = "support/smtp.rs"]
mod smtp_proof;
use axum::{
    Json, Router,
    body::Body,
    extract::{ConnectInfo, State},
    http::{HeaderMap, Request, StatusCode},
    routing::post,
};
use base64::Engine;
use hmac::{Hmac, KeyInit, Mac};
use http_body_util::BodyExt;
use revolution_api::{
    AppState,
    cache::CacheInvalidator,
    config::Config,
    db::Database,
    routes,
    services::{Services, email::EmailService},
};
use serde_json::{Value, json};
use std::{
    net::SocketAddr,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
};
use tower::ServiceExt;

#[derive(Clone, Default)]
struct Adapter {
    requests: Arc<Mutex<Vec<Value>>>,
    rate_limits: Arc<AtomicUsize>,
}
async fn provider(
    State(adapter): State<Adapter>,
    headers: HeaderMap,
    Json(payload): Json<Value>,
) -> (StatusCode, Json<Value>) {
    assert_eq!(
        headers["x-postmark-server-token"],
        "controlled-adapter-token"
    );
    assert!(
        payload["To"].as_str().unwrap().ends_with(".test"),
        "Fixture recipients only"
    );
    adapter.requests.lock().unwrap().push(payload.clone());
    let scenario = payload["TemplateAlias"]
        .as_str()
        .or_else(|| payload["Subject"].as_str())
        .unwrap_or("");
    let scenario = match payload["To"].as_str() {
        Some("subscriber-rejected@example.test") => "fixture-rejected",
        Some("subscriber-unknown@example.test") => "fixture-unknown",
        _ => scenario,
    };
    match scenario {
        "fixture-rejected" => (
            StatusCode::OK,
            Json(json!({"ErrorCode":10,"MessageID":"not-accepted"})),
        ),
        "fixture-unknown" => (StatusCode::OK, Json(json!({"ErrorCode":0}))),
        "fixture-server-error" => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({"ErrorCode":500})),
        ),
        "fixture-rate-limit" if adapter.rate_limits.fetch_add(1, Ordering::SeqCst) == 0 => (
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({"ErrorCode":429})),
        ),
        _ => (
            StatusCode::OK,
            Json(json!({"ErrorCode":0,"MessageID":uuid::Uuid::new_v4().to_string()})),
        ),
    }
}
// Test-only probe exercises the real extractor; no such endpoint is mounted by production.
async fn superadmin_fixture(
    _user: revolution_api::middleware::admin::SuperAdminUser,
) -> StatusCode {
    StatusCode::NO_CONTENT
}
// Test-only digest inspection; never mounted by production and never returns a secret.
async fn credential_fixture(
    State(state): State<AppState>,
    _user: revolution_api::middleware::admin::SuperAdminUser,
    axum::extract::Query(query): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Json<Value> {
    use sha2::Digest;
    let credentials = state
        .services
        .credentials
        .resolve(
            state.db.pool(),
            "stripe",
            query.get("environment").map(String::as_str).unwrap_or(""),
        )
        .await;
    Json(
        json!({"source":credentials.source,"environment":credentials.environment,
        "secret_digest":credentials.credentials.get("secret_key").map(|value|hex::encode(sha2::Sha256::digest(value.as_bytes()))),
        "webhook_digest":credentials.webhook_secret.as_ref().map(|value|hex::encode(sha2::Sha256::digest(value.as_bytes())))}),
    )
}
async fn app(db: Database, email: Option<EmailService>) -> Router {
    let mut config = Config::from_env().unwrap();
    config.redis_url = "redis://127.0.0.1:1".into();
    config.meilisearch_host = "http://127.0.0.1:1".into();
    config.r2_endpoint = "http://127.0.0.1:1".into();
    config.superadmin_emails = vec!["configured-auth-proof@example.test".into()];
    config.developer_emails = config.superadmin_emails.clone();
    config.stripe_secret_key = "sk_test_environment_fixture".into();
    config.stripe_publishable_key = "pk_test_environment_fixture".into();
    config.stripe_webhook_secret = "whsec_environment_fixture".into();
    let mut services = Services::new(&config).await.unwrap();
    if let Some(email) = email {
        services.email = email;
    }
    let cache = services.cache.clone();
    let state = AppState {
        db,
        services,
        config,
        event_broadcaster: routes::realtime::EventBroadcaster::new(),
        ws_manager: routes::websocket::WsConnectionManager::new(),
        cache: cache.clone(),
        cache_invalidator: CacheInvalidator::new(cache),
    };
    Router::new()
        .route(
            "/fixture/superadmin",
            axum::routing::get(superadmin_fixture),
        )
        .route(
            "/fixture/credential-source",
            axum::routing::get(credential_fixture),
        )
        .nest("/api", routes::api_router())
        .with_state(state)
}
async fn request(
    app: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Value,
) -> (StatusCode, Value) {
    let mut request = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json")
        .extension(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 45001))));
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(request.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({"text":String::from_utf8_lossy(&bytes)})),
    )
}

async fn newsletter_bulk_proof(
    api: &Router,
    pool: &sqlx::PgPool,
    tokens: &[String],
    adapter: &Adapter,
) {
    use revolution_api::services::{campaign_delivery, campaign_mailer::CampaignMailer};
    let input = json!({"subject":"fixture-newsletter-bulk","html_content":"<p>Durable newsletter HTML</p>","text_content":"Durable newsletter text","segment":"legacy-fixture","rate_limit":1});
    for (token, expected) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(
                api,
                "POST",
                "/api/newsletter/bulk-send",
                token,
                input.clone()
            )
            .await
            .0,
            expected
        );
    }
    for (email, status, confirmed, unsubscribed) in [
        ("bulk-a@example.test", "confirmed", true, false),
        ("bulk-b@example.test", "confirmed", true, false),
        ("bulk-pending@example.test", "pending", false, false),
        ("bulk-unsubscribed@example.test", "unsubscribed", true, true),
        ("bulk-inconsistent@example.test", "confirmed", true, true),
        ("bulk-no-proof@example.test", "confirmed", false, false),
        ("Bulk-Duplicate@example.test", "confirmed", true, false),
        ("bulk-duplicate@example.test", "unsubscribed", true, true),
    ] {
        sqlx::query("INSERT INTO newsletter_subscribers(email,name,status,confirmed_at,unsubscribed_at,tags,updated_at) VALUES($1,'Bulk fixture',$2,CASE WHEN $3 THEN NOW() ELSE NULL END,CASE WHEN $4 THEN NOW() ELSE NULL END,'[\"legacy-fixture\"]',CASE WHEN $4 THEN NOW()+INTERVAL '1 second' ELSE NOW() END)")
            .bind(email).bind(status).bind(confirmed).bind(unsubscribed).execute(pool).await.unwrap();
    }
    let before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaigns")
        .fetch_one(pool)
        .await
        .unwrap();
    for patch in [
        json!({"rate_limit":0}),
        json!({"rate_limit":501}),
        json!({"subject":"Bad\r\nHeader"}),
        json!({"segment":"no-such-fixture"}),
        json!({"template_id":-1}),
        json!({"html_content":""}),
    ] {
        let mut invalid = input.clone();
        for (key, value) in patch.as_object().unwrap() {
            invalid[key] = value.clone();
        }
        let rejected = request(
            api,
            "POST",
            "/api/newsletter/bulk-send",
            Some(&tokens[0]),
            invalid,
        )
        .await;
        assert_eq!(rejected.0, StatusCode::BAD_REQUEST, "{rejected:?}");
    }
    let after: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaigns")
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(
        before, after,
        "Rejected queue requests must roll back their rows"
    );
    let queued = request(
        api,
        "POST",
        "/api/newsletter/bulk-send",
        Some(&tokens[0]),
        input,
    )
    .await;
    assert_eq!(queued.0, StatusCode::ACCEPTED, "{queued:?}");
    assert_eq!(queued.1["job"]["recipient_count"], 2);
    assert_eq!(queued.1["job"]["eligibility"], "confirmed_only");
    let id = queued.1["job"]["campaign_id"]
        .as_str()
        .unwrap()
        .parse::<i64>()
        .unwrap();
    let rows: Vec<(String, Option<i64>)> = sqlx::query_as(
        "SELECT email,contact_id FROM crm_campaign_deliveries WHERE campaign_id=$1 ORDER BY email",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .unwrap();
    assert_eq!(
        rows,
        vec![
            ("bulk-a@example.test".into(), None),
            ("bulk-b@example.test".into(), None)
        ]
    );
    sqlx::query("UPDATE crm_campaigns SET status='paused',pause_reason='manual' WHERE id<>$1 AND status IN ('sending','scheduled')").bind(id).execute(pool).await.unwrap();
    let path = format!("/api/admin/crm/campaigns/{id}");
    let preview = request(
        api,
        "GET",
        &format!("{path}/recipients"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(preview.0, StatusCode::OK, "{preview:?}");
    assert_eq!(preview.1["audience"]["source"], "newsletter");
    assert_eq!(preview.1["audience"]["eligible_recipients"], 2);
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/pause"),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    let edited = request(
        api,
        "PUT",
        &path,
        Some(&tokens[0]),
        json!({"email_subject":"fixture-newsletter-edited","text_body":"Edited newsletter text"}),
    )
    .await;
    assert_eq!(edited.0, StatusCode::OK, "{edited:?}");
    assert_eq!(edited.1["text_body"], "Edited newsletter text");
    let resumed = request(
        api,
        "POST",
        &format!("{path}/resume"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(resumed.0, StatusCode::OK, "{resumed:?}");
    let mailer = CampaignMailer::from_env("development").unwrap();
    let start = adapter.requests.lock().unwrap().len();
    let (one, two) = tokio::join!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274"),
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
    );
    assert_eq!(
        usize::from(one.unwrap()) + usize::from(two.unwrap()),
        1,
        "Concurrent workers share the durable rate reservation"
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), start + 1);
    assert!(
        !campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap(),
        "A second provider attempt must wait for its pacing slot"
    );
    {
        let requests = adapter.requests.lock().unwrap();
        let payload = requests.last().unwrap();
        assert!(
            payload["HtmlBody"]
                .as_str()
                .unwrap()
                .contains("Durable newsletter HTML")
        );
        assert!(
            payload["TextBody"]
                .as_str()
                .unwrap()
                .contains("Edited newsletter text")
        );
        assert!(
            payload["HtmlBody"]
                .as_str()
                .unwrap()
                .contains("Unsubscribe")
        );
        assert!(
            payload["TextBody"]
                .as_str()
                .unwrap()
                .contains("Unsubscribe:")
        );
    }
    sqlx::query("UPDATE crm_campaigns SET next_delivery_at=NOW()-INTERVAL '1 second' WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    assert!(
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap()
    );
    let counts: (String, i32) =
        sqlx::query_as("SELECT status,emails_sent FROM crm_campaigns WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(counts, ("sent".into(), 2));
    assert_eq!(adapter.requests.lock().unwrap().len(), start + 2);
    let duplicate = request(
        api,
        "POST",
        &format!("{path}/duplicate"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(duplicate.0, StatusCode::OK, "{duplicate:?}");
    assert_eq!(duplicate.1["text_body"], "Edited newsletter text");
    assert_eq!(
        duplicate.1["subscriber_settings"]["newsletter_segment"],
        "legacy-fixture"
    );
    let duplicate_id = duplicate.1["id"].as_str().unwrap();
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/admin/crm/campaigns/{duplicate_id}/recipients"),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .1["audience"]["eligible_recipients"],
        2
    );
}

async fn crm_logs_proof(api: &Router, pool: &sqlx::PgPool, tokens: &[String]) {
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(api, "GET", "/api/admin/crm/logs", token, Value::Null)
                .await
                .0,
            status
        );
        assert_eq!(
            request(api, "POST", "/api/admin/crm/logs/clear", token, json!({}))
                .await
                .0,
            status
        );
    }
    let logs = request(
        api,
        "GET",
        "/api/admin/crm/logs?category=email&search=Newsletter%20broadcast%20queued&per_page=1",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(logs.0, StatusCode::OK, "{logs:?}");
    assert_eq!(logs.1["meta"]["total"], 1);
    let id = logs.1["data"][0]["id"].as_str().unwrap();
    let detail = request(
        api,
        "GET",
        &format!("/api/admin/crm/logs/{id}"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(detail.0, StatusCode::OK);
    assert_eq!(detail.1["message"], "Newsletter broadcast queued");
    assert_eq!(detail.1["context"]["recipient_count"], 2);
    assert!(detail.1["context"].get("unsubscribe_token").is_none());
    let paged = request(
        api,
        "GET",
        "/api/admin/crm/logs?category=email&per_page=1&page=2",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(paged.0, StatusCode::OK, "{paged:?}");
    assert_eq!(paged.1["data"].as_array().unwrap().len(), 1);
    assert_eq!(paged.1["meta"]["current_page"], 2);
    assert_ne!(paged.1["data"][0]["id"], logs.1["data"][0]["id"]);
    let stats = request(
        api,
        "GET",
        "/api/admin/crm/logs/stats",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(stats.0, StatusCode::OK);
    assert!(stats.1["by_category"]["email"].as_i64().unwrap() > 2);
    assert!(stats.1["by_level"]["info"].as_i64().unwrap() > 0);
    assert!(stats.1["by_level"]["warning"].as_i64().unwrap() > 0);
    assert!(stats.1["by_level"]["error"].as_i64().unwrap() > 0);
    for query in [
        "level=invalid",
        "category=payments",
        "date_from=bad",
        "per_page=0",
        "page=-1",
    ] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/admin/crm/logs?{query}"),
                Some(&tokens[0]),
                Value::Null
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    let audit:i64=sqlx::query_scalar("INSERT INTO audit_logs(action,entity_type,status) VALUES('fixture','crm-log-clear','success') RETURNING id").fetch_one(pool).await.unwrap();
    let cleared = request(
        api,
        "POST",
        "/api/admin/crm/logs/clear",
        Some(&tokens[0]),
        json!({"level":"info","category":"email","before_date":"2100-01-01"}),
    )
    .await;
    assert_eq!(cleared.0, StatusCode::OK, "{cleared:?}");
    assert!(cleared.1["deleted_count"].as_i64().unwrap() > 0);
    let security_exists: bool =
        sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM audit_logs WHERE id=$1)")
            .bind(audit)
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(
        security_exists,
        "Operational clear must never remove security audit records"
    );
    let remaining = request(
        api,
        "GET",
        "/api/admin/crm/logs?level=warning&category=email",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert!(
        remaining.1["meta"]["total"].as_i64().unwrap() > 0,
        "Other levels must survive a filtered clear"
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("/api/admin/crm/logs/{id}"),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
}

async fn csv_request(api: &Router, path: &str, token: &str, contents: &str) -> (StatusCode, Value) {
    let body = format!(
        "--fixture-csv-boundary\r\nContent-Disposition: form-data; name=\"file\"; filename=\"fixture.csv\"\r\nContent-Type: text/csv\r\n\r\n{contents}\r\n--fixture-csv-boundary--\r\n"
    );
    let response = api
        .clone()
        .oneshot(
            Request::builder()
                .method("POST")
                .uri(path)
                .header("authorization", format!("Bearer {token}"))
                .header(
                    "content-type",
                    "multipart/form-data; boundary=fixture-csv-boundary",
                )
                .extension(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 45001))))
                .body(Body::from(body))
                .unwrap(),
        )
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (status, serde_json::from_slice(&bytes).unwrap())
}

async fn cms_datasource_proof(api: &Router, pool: &sqlx::PgPool, tokens: &[String]) {
    let root = "/api/cms/datasources";
    assert_eq!(
        request(api, "GET", root, None, Value::Null).await.0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(api, "GET", root, Some(&tokens[1]), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    let created = request(
        api,
        "POST",
        root,
        Some(&tokens[0]),
        json!({"name":"Datasource proof","slug":"datasource-proof"}),
    )
    .await;
    assert_eq!(created.0, StatusCode::CREATED, "{created:?}");
    let id = created.1["id"].as_str().unwrap();
    for query in ["limit=0", "limit=201", "offset=-1"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{root}?{query}"),
                Some(&tokens[0]),
                Value::Null
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    for query in [
        "limit=0",
        "limit=1001",
        "offset=-1",
        "dimension=%00",
        "search=%00",
    ] {
        let rejected = request(
            api,
            "GET",
            &format!("{root}/{id}/entries?{query}"),
            Some(&tokens[0]),
            Value::Null,
        )
        .await;
        assert_eq!(rejected.0, StatusCode::BAD_REQUEST, "{query}: {rejected:?}");
    }
    let listed = request(
        api,
        "GET",
        &format!("{root}?search=Datasource%20proof&limit=1&offset=0"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(listed.0, StatusCode::OK, "{listed:?}");
    assert_eq!(listed.1["data"][0]["id"], id);
    let entry = request(
        api,
        "POST",
        &format!("{root}/{id}/entries"),
        Some(&tokens[0]),
        json!({"name":"Fixture value","value":"fixture-one","dimension":"default"}),
    )
    .await;
    assert_eq!(entry.0, StatusCode::CREATED, "{entry:?}");
    let entry_id = entry.1["id"].as_str().unwrap();
    let other = request(
        api,
        "POST",
        root,
        Some(&tokens[0]),
        json!({"name":"Other datasource"}),
    )
    .await;
    assert_eq!(other.0, StatusCode::CREATED, "{other:?}");
    let other_id = other.1["id"].as_str().unwrap();
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{root}/{other_id}/entries/{entry_id}"),
            Some(&tokens[0]),
            json!({"name":"Wrong parent"})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{root}/{other_id}/entries/{entry_id}"),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let csv_path = format!("{root}/{id}/import");
    let csv =
        "value,name,dimension\nfixture-one,Updated existing,default\nsecond,Second value,default\n";
    let imported = csv_request(api, &csv_path, &tokens[0], csv).await;
    assert_eq!(imported.0, StatusCode::OK, "{imported:?}");
    assert_eq!(imported.1["created"], 1);
    assert_eq!(imported.1["updated"], 1);
    let repeated = csv_request(api, &csv_path, &tokens[0], csv).await;
    assert_eq!(repeated.1["created"], 0);
    assert_eq!(repeated.1["updated"], 2);
    assert_eq!(
        csv_request(api, &csv_path, &tokens[0], "wrong,value\nx,y\n")
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let partial = csv_request(
        api,
        &csv_path,
        &tokens[0],
        &format!(
            "name,value\n{},invalid-name\nThird value,third\n",
            "x".repeat(256)
        ),
    )
    .await;
    assert_eq!(partial.0, StatusCode::OK, "{partial:?}");
    assert_eq!(partial.1["errors"], 1);
    assert_eq!(partial.1["created"], 1);
    assert_eq!(partial.1["row_errors"][0]["row"], 2);
    let rows:Vec<(uuid::Uuid,String)>=sqlx::query_as("SELECT id,value FROM cms_datasource_entries WHERE datasource_id=$1 ORDER BY sort_order,name,id").bind(id.parse::<uuid::Uuid>().unwrap()).fetch_all(pool).await.unwrap();
    assert_eq!(rows.len(), 3);
    let third = rows.iter().find(|row| row.1 == "third").unwrap().0;
    let second = rows.iter().find(|row| row.1 == "second").unwrap().0;
    let reordered = request(
        api,
        "PUT",
        &format!("{root}/{id}/entries/reorder"),
        Some(&tokens[0]),
        json!({"entry_ids":[third,entry_id]}),
    )
    .await;
    assert_eq!(reordered.0, StatusCode::OK, "{reordered:?}");
    let actual: Vec<uuid::Uuid> = sqlx::query_scalar(
        "SELECT id FROM cms_datasource_entries WHERE datasource_id=$1 ORDER BY sort_order,name,id",
    )
    .bind(id.parse::<uuid::Uuid>().unwrap())
    .fetch_all(pool)
    .await
    .unwrap();
    assert_eq!(
        actual,
        vec![third, second, entry_id.parse::<uuid::Uuid>().unwrap()],
        "A subset reorder preserves other entries' slots"
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{root}/{id}/entries/reorder"),
            Some(&tokens[0]),
            json!({"entry_ids":[third,third]})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    for input in [
        json!({"name":""}),
        json!({"value":"x".repeat(501)}),
        json!({"metadata":{"bad":"\u{0}"}}),
    ] {
        assert_eq!(
            request(
                api,
                "PUT",
                &format!("{root}/{id}/entries/{entry_id}"),
                Some(&tokens[0]),
                input
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    let duplicate = request(
        api,
        "POST",
        &format!("{root}/{id}/entries"),
        Some(&tokens[0]),
        json!({"name":"Duplicate","value":"fixture-one"}),
    )
    .await;
    assert_eq!(duplicate.0, StatusCode::CONFLICT, "{duplicate:?}");
    let updated = request(
        api,
        "PUT",
        &format!("{root}/{id}"),
        Some(&tokens[0]),
        json!({"description":"Persisted fixture description","is_locked":true}),
    )
    .await;
    assert_eq!(updated.0, StatusCode::OK, "{updated:?}");
    let editor:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) SELECT 'editor@datasource-completion.test','Fixture editor',password_hash,'editor',NOW() FROM users WHERE email='admin@email-completion.test' RETURNING id").fetch_one(pool).await.unwrap();
    let editor_token =
        revolution_api::utils::create_jwt(editor, &std::env::var("JWT_SECRET").unwrap(), 1)
            .unwrap();
    for (method, path, body) in [
        (
            "POST",
            format!("{root}/{id}/entries"),
            json!({"name":"Blocked","value":"blocked"}),
        ),
        (
            "POST",
            format!("{root}/{id}/entries/bulk"),
            json!({"entries":[{"name":"Blocked","value":"blocked"}]}),
        ),
        (
            "PUT",
            format!("{root}/{id}/entries/{entry_id}"),
            json!({"name":"Blocked"}),
        ),
        (
            "DELETE",
            format!("{root}/{id}/entries/{entry_id}"),
            Value::Null,
        ),
        (
            "PUT",
            format!("{root}/{id}/entries/reorder"),
            json!({"entry_ids":[entry_id]}),
        ),
    ] {
        assert_eq!(
            request(api, method, &path, Some(&editor_token), body)
                .await
                .0,
            StatusCode::FORBIDDEN
        );
    }
    assert_eq!(
        csv_request(api, &csv_path, &editor_token, csv).await.0,
        StatusCode::FORBIDDEN
    );
    let actor: i64 =
        sqlx::query_scalar("SELECT id FROM users WHERE email='admin@email-completion.test'")
            .fetch_one(pool)
            .await
            .unwrap();
    for role in ["super-admin", "superadmin", "SUPER_ADMIN", "developer"] {
        sqlx::query("UPDATE users SET role=$2 WHERE id=$1")
            .bind(actor)
            .bind(role)
            .execute(pool)
            .await
            .unwrap();
        let saved = request(
            api,
            "PUT",
            &format!("{root}/{id}"),
            Some(&tokens[0]),
            json!({"description":format!("Alias {role}")}),
        )
        .await;
        assert_eq!(saved.0, StatusCode::OK, "{role}: {saved:?}");
    }
    sqlx::query("UPDATE users SET role='admin' WHERE id=$1")
        .bind(actor)
        .execute(pool)
        .await
        .unwrap();
    let copy = request(
        api,
        "POST",
        &format!("{root}/{id}/duplicate"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(copy.0, StatusCode::CREATED, "{copy:?}");
    assert_eq!(copy.1["entry_count"], 3);
    let copy_id = copy.1["id"].as_str().unwrap();
    assert_ne!(copy_id, id);
    let copied = request(
        api,
        "GET",
        &format!("{root}/{copy_id}/entries"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(copied.1["meta"]["total"], 3);
    assert!(
        copied.1["data"]
            .as_array()
            .unwrap()
            .iter()
            .all(|row| row["id"] != entry_id)
    );
    let long = request(
        api,
        "POST",
        root,
        Some(&tokens[0]),
        json!({"name":"x".repeat(100)}),
    )
    .await;
    assert_eq!(long.0, StatusCode::CREATED, "{long:?}");
    let long_copy = request(
        api,
        "POST",
        &format!("{root}/{}/duplicate", long.1["id"].as_str().unwrap()),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(long_copy.0, StatusCode::CREATED, "{long_copy:?}");
    assert_eq!(long_copy.1["name"].as_str().unwrap().chars().count(), 100);
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{root}/{id}"),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{root}/{id}/entries/{entry_id}"),
            Some(&tokens[0]),
            json!({"name":"After delete"})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let recreated = request(
        api,
        "POST",
        root,
        Some(&tokens[0]),
        json!({"name":"Datasource proof","slug":"datasource-proof"}),
    )
    .await;
    assert_eq!(recreated.0, StatusCode::CREATED, "{recreated:?}");
    assert_ne!(
        recreated.1["slug"], "datasource-proof",
        "Soft-deleted unique slugs stay reserved"
    );
}

async fn campaign_delivery_proof(
    api: &Router,
    pool: &sqlx::PgPool,
    tokens: &[String],
    adapter: &Adapter,
) {
    use revolution_api::services::{
        campaign_delivery,
        campaign_mailer::{CampaignMailer, Identity},
    };
    assert_eq!(
        std::env::var("POSTMARK_WEBHOOK_TOKEN").as_deref(),
        Ok("only-a-disposable-webhook-fixture-secret")
    );
    assert_eq!(
        std::env::var("POSTMARK_BROADCAST_STREAM").as_deref(),
        Ok("fixture-broadcast")
    );
    let mailer = CampaignMailer::new(
        "development",
        "controlled-adapter-token",
        Identity {
            name: "Fixture sender".into(),
            email: "sender@example.test".into(),
        },
        "fixture-broadcast",
        Some("http://127.0.0.1:19020/email"),
    )
    .unwrap();
    sqlx::query("INSERT INTO contacts(email,first_name,source) VALUES('campaign-delivery@example.test','Campaign','email-completion')").execute(pool).await.unwrap();
    sqlx::query("INSERT INTO newsletter_subscribers(email,status,confirmed_at,source) VALUES('campaign-delivery@example.test','confirmed',NOW(),'email-completion')").execute(pool).await.unwrap();
    let audience = json!({"advanced_filters":[{"field":"email","operator":"equals","value":"campaign-delivery@example.test"}]});
    for (subject, expected) in [
        ("fixture-campaign-accepted", "accepted"),
        ("fixture-rejected", "failed"),
        ("fixture-unknown", "unknown"),
        ("fixture-server-error", "unknown"),
        ("fixture-rate-limit", "queued"),
        ("fixture-lease-expired", "unknown"),
        ("fixture-suppressed", "skipped"),
    ] {
        let created=request(api,"POST","/api/admin/crm/campaigns",Some(&tokens[0]),json!({"title":subject,"email_subject":subject,"email_body":"<p>Frozen fixture body</p>","subscriber_settings":audience})).await;
        assert_eq!(created.0, StatusCode::OK, "{created:?}");
        let id = created.1["id"].as_str().unwrap();
        let path = format!("/api/admin/crm/campaigns/{id}");
        assert_eq!(
            request(api, "POST", &format!("{path}/send"), None, Value::Null)
                .await
                .0,
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            request(
                api,
                "POST",
                &format!("{path}/send"),
                Some(&tokens[1]),
                Value::Null
            )
            .await
            .0,
            StatusCode::FORBIDDEN
        );
        let due = (chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339();
        let scheduled = request(
            api,
            "POST",
            &format!("{path}/schedule"),
            Some(&tokens[0]),
            json!({"scheduled_at":due}),
        )
        .await;
        assert_eq!(scheduled.0, StatusCode::OK, "{scheduled:?}");
        assert_eq!(scheduled.1["status"], "scheduled");
        let before = adapter.requests.lock().unwrap().len();
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
                &format!("{path}/pause"),
                Some(&tokens[0]),
                Value::Null
            )
            .await
            .0,
            StatusCode::OK
        );
        let edited = request(
            api,
            "PUT",
            &path,
            Some(&tokens[0]),
            json!({"email_body":"<p>Updated frozen body</p>"}),
        )
        .await;
        assert_eq!(edited.0, StatusCode::OK, "{edited:?}");
        // Resume preserves a future schedule, then the fixture advances only its
        // own durable delivery deadline to exercise the due worker immediately.
        assert_eq!(
            request(
                api,
                "POST",
                &format!("{path}/resume"),
                Some(&tokens[0]),
                Value::Null
            )
            .await
            .0,
            StatusCode::OK
        );
        let campaign_id = id.parse::<i64>().unwrap();
        sqlx::query("UPDATE crm_campaign_deliveries SET due_at=NOW() WHERE campaign_id=$1")
            .bind(campaign_id)
            .execute(pool)
            .await
            .unwrap();
        if subject == "fixture-rate-limit" {
            adapter.rate_limits.store(0, Ordering::SeqCst);
        }
        if subject == "fixture-lease-expired" {
            sqlx::query("UPDATE crm_campaign_deliveries SET status='processing',lease_token=gen_random_uuid(),lease_expires_at=NOW()-INTERVAL '1 minute',attempt_count=1,attempts_remaining=2 WHERE campaign_id=$1").bind(campaign_id).execute(pool).await.unwrap();
        }
        if subject == "fixture-suppressed" {
            sqlx::query("UPDATE newsletter_subscribers SET status='unsubscribed',unsubscribed_at=NOW(),updated_at=NOW() WHERE email='campaign-delivery@example.test'").execute(pool).await.unwrap();
        }
        if subject == "fixture-lease-expired" {
            assert_eq!(campaign_delivery::recover_expired(pool).await.unwrap(), 1);
        }
        let processed = if expected == "accepted" {
            let (first, second) = tokio::join!(
                campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274"),
                campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            );
            let first = first.unwrap();
            let second = second.unwrap();
            assert_eq!(
                u8::from(first) + u8::from(second),
                1,
                "Concurrent workers must claim one email exactly once"
            );
            first || second
        } else {
            campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                .await
                .unwrap()
        };
        assert_eq!(
            processed,
            !matches!(subject, "fixture-lease-expired" | "fixture-suppressed")
        );
        let stored: Value = sqlx::query_scalar(
            "SELECT to_jsonb(d) FROM crm_campaign_deliveries d WHERE campaign_id=$1",
        )
        .bind(campaign_id)
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(stored["status"], expected, "{stored}");
        if expected == "queued" {
            assert_eq!(stored["attempt_count"], 1);
            sqlx::query("UPDATE crm_campaign_deliveries SET due_at=NOW() WHERE campaign_id=$1")
                .bind(campaign_id)
                .execute(pool)
                .await
                .unwrap();
            assert!(
                campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
                    .await
                    .unwrap()
            );
            let accepted: (String, i32) = sqlx::query_as(
                "SELECT status,attempt_count FROM crm_campaign_deliveries WHERE campaign_id=$1",
            )
            .bind(campaign_id)
            .fetch_one(pool)
            .await
            .unwrap();
            assert_eq!(accepted, ("accepted".into(), 2));
        }
        if expected == "unknown" {
            let before = adapter.requests.lock().unwrap().len();
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
                    &format!("{path}/resume"),
                    Some(&tokens[0]),
                    Value::Null
                )
                .await
                .0,
                StatusCode::CONFLICT
            );
        }
        if expected == "unknown" {
            let event = json!({"RecordType":"Delivery","MessageStream":"fixture-broadcast","MessageID":uuid::Uuid::new_v4().to_string(),"Recipient":"campaign-delivery@example.test","DeliveredAt":chrono::Utc::now().to_rfc3339(),"Metadata":{"delivery_id":stored["id"]}});
            let endpoint = "/api/webhooks/postmark/campaigns";
            assert_eq!(
                request(api, "POST", endpoint, None, event.clone()).await.0,
                StatusCode::UNAUTHORIZED
            );
            let recorded = request(
                api,
                "POST",
                endpoint,
                Some("only-a-disposable-webhook-fixture-secret"),
                event.clone(),
            )
            .await;
            assert_eq!(recorded.0, StatusCode::OK, "{recorded:?}");
            assert_eq!(recorded.1["status"], "recorded");
            assert_eq!(
                request(
                    api,
                    "POST",
                    endpoint,
                    Some("only-a-disposable-webhook-fixture-secret"),
                    event.clone()
                )
                .await
                .1["status"],
                "duplicate"
            );
            for record in ["Open", "Click"] {
                let mut tracking = event.clone();
                tracking["RecordType"] = json!(record);
                tracking["ReceivedAt"] = json!(chrono::Utc::now().to_rfc3339());
                let received = request(
                    api,
                    "POST",
                    endpoint,
                    Some("only-a-disposable-webhook-fixture-secret"),
                    tracking.clone(),
                )
                .await;
                assert_eq!(received.0, StatusCode::OK, "{received:?}");
                assert_eq!(
                    request(
                        api,
                        "POST",
                        endpoint,
                        Some("only-a-disposable-webhook-fixture-secret"),
                        tracking
                    )
                    .await
                    .1["status"],
                    "duplicate"
                );
            }
            let reconciled = request(api, "GET", &path, Some(&tokens[0]), Value::Null).await;
            assert_eq!(reconciled.1["status"], "sent", "{reconciled:?}");
            assert_eq!(reconciled.1["delivery_stats"]["unknown"], 0);
            assert_eq!(reconciled.1["delivery_stats"]["accepted"], 1);
            assert_eq!(reconciled.1["delivery_stats"]["delivered"], 1);
            assert_eq!(reconciled.1["open_count"], 1);
            assert_eq!(reconciled.1["click_count"], 1);
            let receipts = request(
                api,
                "GET",
                &format!("{path}/deliveries?per_page=1"),
                Some(&tokens[0]),
                Value::Null,
            )
            .await;
            assert_eq!(receipts.0, StatusCode::OK, "{receipts:?}");
            assert_eq!(receipts.1["meta"]["total"], 1);
            assert_eq!(receipts.1["data"][0]["status"], "accepted");
            assert!(receipts.1["data"][0].get("unsubscribe_token").is_none());
        }
        if expected == "accepted" {
            let payload = adapter.requests.lock().unwrap().last().unwrap().clone();
            assert!(
                payload["HtmlBody"]
                    .as_str()
                    .unwrap()
                    .contains("<p>Updated frozen body</p>")
            );
            assert_eq!(payload["MessageStream"], "fixture-broadcast");
            assert_eq!(stored["provider_stream"], "fixture-broadcast");
            assert!(
                stored["sender_identity"]
                    .as_str()
                    .unwrap()
                    .contains("sender@example.test")
            );
            let callback = payload["Headers"][0]["Value"]
                .as_str()
                .unwrap()
                .trim_start_matches('<')
                .trim_end_matches('>');
            let url = reqwest::Url::parse(callback).unwrap();
            let token = url
                .query_pairs()
                .find(|(key, _)| key == "token")
                .unwrap()
                .1
                .to_string();
            let unsubscribed = request(
                api,
                "GET",
                &format!("/api/newsletter/unsubscribe?token={token}"),
                None,
                Value::Null,
            )
            .await;
            assert_eq!(unsubscribed.0, StatusCode::OK, "{unsubscribed:?}");
            assert_eq!(unsubscribed.1["status"], "unsubscribed");
            let form_response = api
                .clone()
                .oneshot(
                    Request::builder()
                        .method("POST")
                        .uri(format!("/api/newsletter/unsubscribe?token={token}"))
                        .header("content-type", "application/x-www-form-urlencoded")
                        .body(Body::from("List-Unsubscribe=One-Click"))
                        .unwrap(),
                )
                .await
                .unwrap();
            assert_eq!(form_response.status(), StatusCode::OK);
            assert!(sqlx::query_scalar::<_,bool>("SELECT unsubscribed_at IS NOT NULL FROM crm_campaign_deliveries WHERE campaign_id=$1").bind(campaign_id).fetch_one(pool).await.unwrap());
        }
        sqlx::query("UPDATE newsletter_subscribers SET status='confirmed',unsubscribed_at=NULL,confirmed_at=NOW(),updated_at=NOW() WHERE email='campaign-delivery@example.test'").execute(pool).await.unwrap();
    }
    let endpoint = "/api/webhooks/postmark/campaigns";
    for (changed, suppressed, expected) in [
        (
            chrono::Utc::now() - chrono::Duration::days(1),
            true,
            "confirmed",
        ),
        (chrono::Utc::now(), true, "unsubscribed"),
        (
            chrono::Utc::now() + chrono::Duration::seconds(1),
            false,
            "unsubscribed",
        ),
    ] {
        let event = json!({"RecordType":"SubscriptionChange","MessageStream":"fixture-broadcast","Recipient":"campaign-delivery@example.test","ChangedAt":changed.to_rfc3339(),"SuppressSending":suppressed,"SuppressionReason":"ManualSuppression"});
        let received = request(
            api,
            "POST",
            endpoint,
            Some("only-a-disposable-webhook-fixture-secret"),
            event,
        )
        .await;
        assert_eq!(received.0, StatusCode::OK, "{received:?}");
        let status:String=sqlx::query_scalar("SELECT status FROM newsletter_subscribers WHERE email='campaign-delivery@example.test'").fetch_one(pool).await.unwrap();
        assert_eq!(
            status, expected,
            "Late suppression cannot override a newer confirmation; provider reactivation cannot invent consent"
        );
    }
}

#[tokio::test]
#[ignore = "requires an empty disposable DB and explicit loopback provider fixture environment"]
async fn transactional_receipts_newsletter_and_email_jobs_use_real_adapter_acknowledgment() {
    let url = std::env::var("ADMIN_VERIFY_DATABASE_URL").expect("explicit disposable DB required");
    let parsed = reqwest::Url::parse(&url).unwrap();
    assert_eq!(parsed.host_str(), Some("127.0.0.1"));
    assert!(parsed.path().starts_with("/admin_move_verify"));
    assert_eq!(std::env::var("ENVIRONMENT").as_deref(), Ok("development"));
    assert_eq!(
        std::env::var("POSTMARK_TOKEN").as_deref(),
        Ok("controlled-adapter-token"),
        "Never use real credentials in this proof"
    );
    assert_eq!(
        std::env::var("POSTMARK_FROM_EMAIL").as_deref(),
        Ok("sender@example.test")
    );
    let endpoint = reqwest::Url::parse(
        &std::env::var("POSTMARK_TEST_ENDPOINT")
            .expect("loopback adapter URL required before any sender construction"),
    )
    .unwrap();
    assert_eq!(endpoint.scheme(), "http");
    assert_eq!(endpoint.host_str(), Some("127.0.0.1"));
    assert_eq!(endpoint.path(), "/email");
    assert!(endpoint.query().is_none());
    assert!(endpoint.fragment().is_none());
    assert!(endpoint.username().is_empty());
    let address = SocketAddr::from((
        [127, 0, 0, 1],
        endpoint
            .port()
            .expect("explicit task adapter port required"),
    ));
    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("Refuse to use an adapter owned by another process");
    let adapter = Adapter::default();
    let provider_app = Router::new()
        .route("/email", post(provider))
        .route("/email/withTemplate", post(provider))
        .with_state(adapter.clone());
    let server = tokio::spawn(async move {
        axum::serve(listener, provider_app).await.unwrap();
    });
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let tables: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='public'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(tables, 0, "Fresh DB required");
    let db = Database { pool: pool.clone() };
    db.migrate().await.unwrap();
    let sender = EmailService::from_env("http://127.0.0.1:5274");
    assert!(sender.is_enabled());
    for (alias, status, success) in [
        ("fixture-accepted", "sent", true),
        ("fixture-rejected", "failed", false),
        ("fixture-unknown", "delivery_unknown", false),
    ] {
        let result = sender
            .send_transactional(
                &pool,
                "transactional@example.test",
                alias,
                json!({"fixture":true}),
            )
            .await;
        assert_eq!(result.is_ok(), success, "{result:?}");
        let row: Value = sqlx::query_scalar(
            "SELECT to_jsonb(e) FROM email_logs e WHERE template_alias=$1 ORDER BY id DESC LIMIT 1",
        )
        .bind(alias)
        .fetch_one(&pool)
        .await
        .unwrap();
        assert_eq!(row["status"], status);
        assert_eq!(row["sent_at"].is_string(), success);
        assert_eq!(row["provider_message_id"].is_string(), success);
    }
    let long_alias = "a".repeat(75);
    sender
        .send_transactional(&pool, "long-alias@example.test", &long_alias, json!({}))
        .await
        .unwrap();
    let stored_alias: String = sqlx::query_scalar(
        "SELECT template_alias FROM email_logs WHERE to_email='long-alias@example.test'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(stored_alias, long_alias);
    let disabled = EmailService::new("", "sender@example.test", "http://127.0.0.1:5274");
    let before = adapter.requests.lock().unwrap().len();
    assert!(
        disabled
            .send_transactional(
                &pool,
                "unavailable@example.test",
                "fixture-unavailable",
                json!({})
            )
            .await
            .is_err()
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    let unavailable: Value = sqlx::query_scalar(
        "SELECT to_jsonb(e) FROM email_logs e WHERE template_alias='fixture-unavailable'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(unavailable["status"], "unavailable");
    assert!(unavailable["sent_at"].is_null());

    let hash = revolution_api::utils::hash_password("OnlyForDisposableTest123!").unwrap();
    let mut tokens = Vec::new();
    for role in ["admin", "user"] {
        let id:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES($1,'Fixture',$2,$3,NOW()) RETURNING id").bind(format!("{role}@email-completion.test")).bind(&hash).bind(role).fetch_one(&pool).await.unwrap();
        tokens.push(
            revolution_api::utils::create_jwt(id, &std::env::var("JWT_SECRET").unwrap(), 1)
                .unwrap(),
        );
    }
    let disabled_app = app(db.clone(), Some(disabled)).await;
    let pending = request(
        &disabled_app,
        "POST",
        "/api/newsletter/subscribe",
        None,
        json!({"email":"pending-email-proof@example.test","name":"Fixture","gdpr_consent":true}),
    )
    .await;
    assert_eq!(pending.0, StatusCode::SERVICE_UNAVAILABLE, "{pending:?}");
    assert_eq!(pending.1["status"], "pending");
    assert_eq!(pending.1["delivery_status"], "unavailable");
    let pending_status: String = sqlx::query_scalar(
        "SELECT status FROM newsletter_subscribers WHERE email='pending-email-proof@example.test'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(pending_status, "pending");
    let api = app(db.clone(), None).await;
    let submitted = request(
        &api,
        "POST",
        "/api/newsletter/subscribe",
        None,
        json!({"email":"newsletter-email-proof@example.test","name":"Fixture","gdpr_consent":true}),
    )
    .await;
    assert_eq!(submitted.0, StatusCode::OK, "{submitted:?}");
    let confirmation = {
        let payloads = adapter.requests.lock().unwrap();
        payloads
            .iter()
            .rev()
            .find(|payload| payload["TemplateAlias"] == "newsletter-confirmation")
            .unwrap()["TemplateModel"]["confirm_url"]
            .as_str()
            .unwrap()
            .to_owned()
    };
    let confirmation = reqwest::Url::parse(&confirmation).unwrap();
    let confirm_token = confirmation
        .query_pairs()
        .find(|(key, _)| key == "token")
        .unwrap()
        .1
        .into_owned();
    let confirmed = request(
        &api,
        "GET",
        &format!(
            "/api/newsletter/confirm?token={}",
            urlencoding::encode(&confirm_token)
        ),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(confirmed.0, StatusCode::OK, "{confirmed:?}");
    let subscriber:(i64,String)=sqlx::query_as("SELECT id,status FROM newsletter_subscribers WHERE email='newsletter-email-proof@example.test'").fetch_one(&pool).await.unwrap();
    assert_eq!(subscriber.1, "confirmed");
    let wrong_action = request(
        &api,
        "GET",
        &format!(
            "/api/newsletter/unsubscribe?token={}",
            urlencoding::encode(&confirm_token)
        ),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(wrong_action.0, StatusCode::BAD_REQUEST);
    let email = "newsletter-email-proof@example.test";
    let payload = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(format!("{}:{email}", subscriber.0));
    let mut mac =
        Hmac::<sha2::Sha256>::new_from_slice(std::env::var("JWT_SECRET").unwrap().as_bytes())
            .unwrap();
    mac.update(format!("{}:{email}:unsubscribe", subscriber.0).as_bytes());
    let unsubscribe = format!(
        "{payload}:{}",
        base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(mac.finalize().into_bytes())
    );
    let unsubscribed = request(
        &api,
        "GET",
        &format!(
            "/api/newsletter/unsubscribe?token={}",
            urlencoding::encode(&unsubscribe)
        ),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(unsubscribed.0, StatusCode::OK, "{unsubscribed:?}");
    let unsubscribed: (String, bool) = sqlx::query_as(
        "SELECT status,unsubscribed_at IS NOT NULL FROM newsletter_subscribers WHERE id=$1",
    )
    .bind(subscriber.0)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(unsubscribed, ("unsubscribed".into(), true));
    assert_eq!(
        request(
            &api,
            "POST",
            "/api/newsletter/subscribe",
            None,
            json!({"email":"newsletter-email-proof@example.test","gdpr_consent":true})
        )
        .await
        .0,
        StatusCode::TOO_MANY_REQUESTS
    );
    // Simulate the cooldown elapsing, retaining the actual configured 60-second limit.
    sqlx::query("UPDATE newsletter_subscribers SET metadata=metadata-'_confirmation' WHERE id=$1")
        .bind(subscriber.0)
        .execute(&pool)
        .await
        .unwrap();
    let requested_again = request(
        &api,
        "POST",
        "/api/newsletter/subscribe",
        None,
        json!({"email":"newsletter-email-proof@example.test","name":"Fixture","gdpr_consent":true}),
    )
    .await;
    assert_eq!(requested_again.0, StatusCode::OK, "{requested_again:?}");
    let pending_again: String =
        sqlx::query_scalar("SELECT status FROM newsletter_subscribers WHERE id=$1")
            .bind(subscriber.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(pending_again, "pending");
    let reverified = request(
        &api,
        "GET",
        &format!(
            "/api/newsletter/confirm?token={}",
            urlencoding::encode(&confirm_token)
        ),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(reverified.0, StatusCode::OK, "{reverified:?}");
    let active:(String,bool,chrono::NaiveDateTime)=sqlx::query_as("SELECT status,unsubscribed_at IS NULL,confirmed_at FROM newsletter_subscribers WHERE id=$1").bind(subscriber.0).fetch_one(&pool).await.unwrap();
    assert_eq!(active.0, "confirmed");
    assert!(active.1);
    assert_eq!(
        request(
            &api,
            "GET",
            &format!(
                "/api/newsletter/confirm?token={}",
                urlencoding::encode(&confirm_token)
            ),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    let replayed: chrono::NaiveDateTime =
        sqlx::query_scalar("SELECT confirmed_at FROM newsletter_subscribers WHERE id=$1")
            .bind(subscriber.0)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(active.2, replayed);

    for (subject, expected, terminal) in [
        ("fixture-rejected", "failed", true),
        ("fixture-unknown", "failed", true),
        ("fixture-server-error", "failed", true),
        ("fixture-rate-limit", "pending", false),
    ] {
        let id=revolution_api::queue::enqueue(&pool,"email-proof","send_email",json!({"to":"queue-email-proof@example.test","subject":subject,"body":"<p>Fixture</p>"})).await.unwrap();
        assert_eq!(
            revolution_api::queue::worker::process_next_job(&db)
                .await
                .unwrap(),
            Some(id)
        );
        let state: (String, i32) = sqlx::query_as("SELECT status,attempts FROM jobs WHERE id=$1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(state, (expected.into(), 1));
        if !terminal {
            sqlx::query("UPDATE jobs SET available_at=NOW() WHERE id=$1")
                .bind(id)
                .execute(&pool)
                .await
                .unwrap();
            assert_eq!(
                revolution_api::queue::worker::process_next_job(&db)
                    .await
                    .unwrap(),
                Some(id)
            );
            let state: (String, i32) =
                sqlx::query_as("SELECT status,attempts FROM jobs WHERE id=$1")
                    .bind(id)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            assert_eq!(state, ("completed".into(), 2));
        }
    }
    assert_eq!(
        request(&api, "GET", "/api/admin/email/logs", None, Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/admin/email/logs",
            Some(&tokens[1]),
            Value::Null
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let status = request(
        &api,
        "GET",
        "/api/admin/email/status",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status.0, StatusCode::OK, "{status:?}");
    assert_eq!(status.1["postmark_token_set"], true);
    assert_eq!(status.1["email_service_enabled"], true);
    campaign_delivery_proof(&api, &pool, &tokens, &adapter).await;
    newsletter_bulk_proof(&api, &pool, &tokens, &adapter).await;
    crm_logs_proof(&api, &pool, &tokens).await;
    cms_datasource_proof(&api, &pool, &tokens).await;
    crm_webhook_proof::proof(&api, &pool, &tokens).await;
    performance_proof::proof(&api, &pool, &tokens).await;
    email_subscriber_proof::proof(&api, &disabled_app, &pool, &tokens, &adapter).await;
    authorization_parity_proof::proof(&api, &pool).await;
    connections_proof::proof(&api, &pool, &tokens).await;
    smtp_proof::proof(&api, &pool, &tokens).await;
    server.abort();
}
