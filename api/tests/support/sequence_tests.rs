//! Explicit sequence samples retain real receipts without production enrollments.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;
const ROOT: &str = "/api/admin/crm/sequences";

async fn create(api: &Router, token: &str, subject: &str, plain: bool) -> (i64, Uuid, Uuid) {
    let created=request(api,"POST",ROOT,Some(token),json!({"title":"Sequence QA fixture","design_template":if plain{"plain_text"}else{"raw_html"},"settings":{"mailer_settings":{"is_custom":true,"from_name":"Sequence parent sender","from_email":"sequence-parent-qa@example.test","reply_to_email":"sequence-parent-reply@example.test"}},"emails":[{"title":"Saved selected email","subject":subject,"email_body":if plain{"Saved plain {{email}} {{contact.first_name}}"}else{"<p>Saved {{email}} {{contact.first_name}}</p><a href='{{unsubscribe_url}}'>Leave</a>"},"text_body":"Saved alternative {{email}} {{contact.first_name}}","settings":{"is_custom":true,"from_name":"Sequence QA mail","from_email":"sequence-mail-qa@example.test","reply_to_name":"QA reply","reply_to_email":"sequence-mail-reply@example.test"}},{"title":"Other email","subject":"Other saved email","email_body":"<p>Other saved creative</p>"}]})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    (
        created.1["id"].as_str().unwrap().parse().unwrap(),
        serde_json::from_value(created.1["emails"][0]["id"].clone()).unwrap(),
        serde_json::from_value(created.1["emails"][1]["id"].clone()).unwrap(),
    )
}
fn body(mail: Uuid, request: Uuid, email: &str, version: i64) -> Value {
    json!({"email":email,"email_id":mail,"request_id":request,"expected_version":version})
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let (id, mail, other) = create(
        api,
        &tokens[0],
        "Saved QA for {{contact.first_name}}",
        false,
    )
    .await;
    let path = format!("{ROOT}/{id}");
    let endpoint = format!("{path}/test");
    let history = format!("{path}/tests");
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, url, payload) in [
            (
                "POST",
                &endpoint,
                body(mail, Uuid::new_v4(), "sequence-qa@example.test", 1),
            ),
            ("GET", &history, Value::Null),
        ] {
            assert_eq!(request(api, method, url, token, payload).await.0, status);
        }
    }
    let before = adapter.requests.lock().unwrap().len();
    for payload in [
        json!({}),
        json!({"email":"sequence-qa@example.test","email_id":mail}),
        json!({"email":"sequence-qa@example.test","email_id":mail,"request_id":Uuid::new_v4(),"expected_version":1,"emails":["other@example.test"]}),
    ] {
        let response = request(api, "POST", &endpoint, admin, payload).await;
        assert_eq!(response.0, StatusCode::UNPROCESSABLE_ENTITY, "{response:?}");
    }
    for payload in [
        body(mail, Uuid::nil(), "sequence-qa@example.test", 1),
        body(Uuid::nil(), Uuid::new_v4(), "sequence-qa@example.test", 1),
        body(mail, Uuid::new_v4(), "invalid", 1),
        body(
            mail,
            Uuid::new_v4(),
            "qa@example.test\r\nBcc: other@example.test",
            1,
        ),
        body(mail, Uuid::new_v4(), "sequence-qa@example.test", 0),
    ] {
        let response = request(api, "POST", &endpoint, admin, payload).await;
        assert_eq!(response.0, StatusCode::BAD_REQUEST, "{response:?}");
    }
    assert_eq!(
        request(
            api,
            "POST",
            &endpoint,
            admin,
            body(
                Uuid::new_v4(),
                Uuid::new_v4(),
                "sequence-qa@example.test",
                1
            )
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    sqlx::query("INSERT INTO contacts(email,first_name) VALUES('sequence-qa-ambiguous@example.test','One'),('SEQUENCE-QA-AMBIGUOUS@example.test','Other')")
        .execute(pool).await.unwrap();
    let ambiguous = request(
        api,
        "POST",
        &endpoint,
        admin,
        body(
            mail,
            Uuid::new_v4(),
            "sequence-qa-ambiguous@example.test",
            1,
        ),
    )
    .await;
    assert_eq!(
        ambiguous.0,
        StatusCode::BAD_REQUEST,
        "Ambiguous contacts must not provide an arbitrary sample identity: {ambiguous:?}"
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    assert_eq!(
        request(api, "GET", &history, admin, Value::Null).await.1["data"]["requests"],
        json!([])
    );
    let rollback = Uuid::new_v4();
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO sequence_test_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "POST",
        &endpoint,
        admin,
        body(mail, rollback, "sequence-qa@example.test", 1),
    )
    .await;
    sqlx::query("ALTER TABLE sequence_test_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE, "{failed:?}");
    let staged: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_test_requests WHERE request_id=$1")
            .bind(rollback)
            .fetch_one(pool)
            .await
            .unwrap();
    let logs:i64=sqlx::query_scalar("SELECT COUNT(*) FROM email_logs WHERE template_type='sequence_test' AND model->>'request_id'=$1").bind(rollback.to_string()).fetch_one(pool).await.unwrap();
    assert_eq!(
        (staged, logs),
        (0, 0),
        "Intent, receipt and audit roll back together"
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    sqlx::query("INSERT INTO contacts(email,first_name,last_name,company,source) VALUES('sequence-qa@example.test','<Actual & QA>','Fixture','Actual QA company','sequence-tests')").execute(pool).await.unwrap();
    let stats_before = request(api, "GET", &format!("{path}/stats"), admin, Value::Null).await;
    assert_eq!(stats_before.0, StatusCode::OK);
    let key = Uuid::new_v4();
    let payload = body(mail, key, " SEQUENCE-QA@example.test ", 1);
    let accepted = request(api, "POST", &endpoint, admin, payload.clone()).await;
    assert_eq!(accepted.0, StatusCode::OK, "{accepted:?}");
    assert_eq!(accepted.1["success"], true);
    assert_eq!(accepted.1["data"]["metrics_scope"], "sequence_test");
    assert_eq!(accepted.1["data"]["sequence_id"], id.to_string());
    assert_eq!(accepted.1["data"]["email_id"], mail.to_string());
    let receipt = &accepted.1["data"]["submissions"][0];
    assert_eq!(receipt["status"], "accepted");
    assert_eq!(receipt["recipient"], "sequence-qa@example.test");
    assert_eq!(receipt["provider"], "postmark");
    assert!(!receipt["provider_message_id"].as_str().unwrap().is_empty());
    assert!(receipt["accepted_at"].is_string());
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    let actual = adapter.requests.lock().unwrap()[before].clone();
    assert_eq!(actual["Subject"], "[Test] Saved QA for <Actual & QA>");
    assert_eq!(
        actual["From"],
        "\"Sequence QA mail\" <sequence-mail-qa@example.test>"
    );
    assert_eq!(
        actual["ReplyTo"],
        "\"QA reply\" <sequence-mail-reply@example.test>"
    );
    assert!(
        actual["HtmlBody"]
            .as_str()
            .unwrap()
            .contains("&lt;Actual &amp; QA&gt;")
    );
    assert!(
        actual["TextBody"]
            .as_str()
            .unwrap()
            .contains("Saved alternative sequence-qa@example.test <Actual & QA>")
    );
    assert!(
        actual["HtmlBody"]
            .as_str()
            .unwrap()
            .contains("preview&#x3D;sequence-test")
            || actual["HtmlBody"]
                .as_str()
                .unwrap()
                .contains("preview=sequence-test")
    );
    assert!(!actual["HtmlBody"].as_str().unwrap().contains("token="));
    assert_eq!(actual["Metadata"]["test_request_id"], key.to_string());
    assert!(actual["Metadata"].get("delivery_id").is_none());
    let production:(i64,i64)=sqlx::query_as("SELECT (SELECT COUNT(*) FROM crm_sequence_trackers WHERE sequence_id=$1),(SELECT COUNT(*) FROM crm_sequence_engines WHERE sequence_id=$1)").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(production, (0, 0));
    assert_eq!(
        request(api, "GET", &format!("{path}/stats"), admin, Value::Null).await,
        stats_before,
        "QA never changes production sequence metrics"
    );
    assert_eq!(
        request(api, "POST", &endpoint, admin, payload.clone()).await,
        accepted
    );
    let stored = request(
        api,
        "GET",
        &format!("{history}?request_id={key}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(stored.0, StatusCode::OK, "{stored:?}");
    assert_eq!(stored.1["data"]["requests"], json!([accepted.1["data"]]));
    for changed in [
        body(other, key, "sequence-qa@example.test", 1),
        body(mail, key, "other@example.test", 1),
        body(mail, key, "sequence-qa@example.test", 2),
    ] {
        assert_eq!(
            request(api, "POST", &endpoint, admin, changed).await.0,
            StatusCode::CONFLICT
        );
    }
    let actor:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES('sequence-other-qa-admin@example.test','Other QA administrator','not-a-login-password','admin',NOW()) RETURNING id").fetch_one(pool).await.unwrap();
    let actor_token =
        revolution_api::utils::create_jwt(actor, &std::env::var("JWT_SECRET").unwrap(), 1).unwrap();
    assert_eq!(
        request(api, "POST", &endpoint, Some(&actor_token), payload.clone())
            .await
            .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            admin,
            json!({"title":"Edited after actual QA"})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "POST", &endpoint, admin, payload.clone()).await,
        accepted
    );
    assert_eq!(
        request(
            api,
            "POST",
            &endpoint,
            admin,
            body(mail, Uuid::new_v4(), "stale@example.test", 1)
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let frozen:Value=sqlx::query_scalar("SELECT r.definition FROM crm_sequence_test_requests t JOIN crm_sequence_revisions r ON r.id=t.revision_id WHERE t.request_id=$1").bind(key).fetch_one(pool).await.unwrap();
    assert_eq!(frozen["title"], "Sequence QA fixture");
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    assert_eq!(
        request(api, "DELETE", &path, admin, Value::Null).await.0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "POST", &endpoint, admin, payload).await,
        accepted,
        "Archived source never causes a replay to submit again"
    );
    assert_eq!(
        request(api, "GET", &history, admin, Value::Null).await.0,
        StatusCode::OK
    );
    for (subject, outcome) in [
        ("fixture-rejected", "rejected"),
        ("fixture-unknown", "unknown"),
    ] {
        let (id, mail, _) = create(api, &tokens[0], subject, false).await;
        let url = format!("{ROOT}/{id}/test");
        let payload = body(mail, Uuid::new_v4(), "sequence-failure@example.test", 1);
        let response = request(api, "POST", &url, admin, payload.clone()).await;
        assert_eq!(response.0, StatusCode::BAD_GATEWAY, "{response:?}");
        assert_eq!(response.1["success"], false);
        assert_eq!(response.1["data"]["submissions"][0]["status"], outcome);
        let count = adapter.requests.lock().unwrap().len();
        assert_eq!(request(api, "POST", &url, admin, payload).await, response);
        assert_eq!(
            adapter.requests.lock().unwrap().len(),
            count,
            "Rejected/unknown samples never automatically resubmit"
        );
    }
    let (plain, mail, _) = create(api, &tokens[0], "Plain saved QA", true).await;
    let plain_response = request(
        api,
        "POST",
        &format!("{ROOT}/{plain}/test"),
        admin,
        body(mail, Uuid::new_v4(), "plain-sequence@example.test", 1),
    )
    .await;
    assert_eq!(plain_response.0, StatusCode::OK, "{plain_response:?}");
    let actual = adapter.requests.lock().unwrap().last().unwrap().clone();
    assert!(actual.get("HtmlBody").is_none());
    assert!(
        actual["TextBody"]
            .as_str()
            .unwrap()
            .contains("Saved plain plain-sequence@example.test")
    );
    inflight(api, pool, tokens, adapter, false).await;
    inflight(api, pool, tokens, adapter, true).await;
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    let single = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    let (id, mail, _) = create(&single, &tokens[0], "One connection sequence sample", false).await;
    let result = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        request(
            &single,
            "POST",
            &format!("{ROOT}/{id}/test"),
            admin,
            body(mail, Uuid::new_v4(), "one-sequence@example.test", 1),
        ),
    )
    .await
    .unwrap();
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    one.close().await;
}
async fn inflight(
    api: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
    lose_receipt: bool,
) {
    let admin = Some(tokens[0].as_str());
    let (id, mail, _) = create(api, &tokens[0], "fixture-inflight", false).await;
    let endpoint = format!("{ROOT}/{id}/test");
    let key = Uuid::new_v4();
    let payload = body(mail, key, "held-sequence@example.test", 1);
    let before = adapter.requests.lock().unwrap().len();
    let during = async {
        tokio::time::timeout(
            std::time::Duration::from_secs(5),
            adapter.inflight_started.notified(),
        )
        .await
        .unwrap();
        let replay = request(api, "POST", &endpoint, admin, payload.clone()).await;
        assert_eq!(replay.0, StatusCode::ACCEPTED, "{replay:?}");
        assert_eq!(replay.1["data"]["submissions"][0]["status"], "processing");
        assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
        if lose_receipt {
            sqlx::query("DELETE FROM email_logs WHERE template_type='sequence_test' AND model->>'request_id'=$1").bind(key.to_string()).execute(pool).await.unwrap();
        }
        adapter.inflight_release.notify_one();
    };
    let (submitted, ()) = tokio::join!(
        request(api, "POST", &endpoint, admin, payload.clone()),
        during
    );
    assert_eq!(
        submitted.0,
        if lose_receipt {
            StatusCode::SERVICE_UNAVAILABLE
        } else {
            StatusCode::OK
        },
        "{submitted:?}"
    );
    if lose_receipt {
        assert_eq!(
            request(api, "POST", &endpoint, admin, payload).await.0,
            StatusCode::SERVICE_UNAVAILABLE
        );
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{ROOT}/{id}/tests?request_id={key}"),
                admin,
                Value::Null
            )
            .await
            .0,
            StatusCode::SERVICE_UNAVAILABLE
        );
    }
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before + 1,
        "An incomplete acceptance receipt must never cause another provider call"
    );
}
