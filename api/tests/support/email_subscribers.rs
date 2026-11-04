use super::{Adapter, request};
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;

pub(super) async fn proof(
    api: &Router,
    disabled: &Router,
    pool: &PgPool,
    tokens: &[String],
    adapter: &Adapter,
) {
    let root = "/api/admin/email/subscribers";
    let token = Some(tokens[0].as_str());
    for (auth, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, path, body) in [
            ("GET", root.to_string(), Value::Null),
            ("GET", format!("{root}/stats"), Value::Null),
            ("GET", format!("{root}/export"), Value::Null),
            (
                "POST",
                root.to_string(),
                json!({"email":"unauthorized-subscriber@example.test"}),
            ),
            ("POST", format!("{root}/bulk-delete"), json!({"ids":["1"]})),
        ] {
            assert_eq!(request(api, method, &path, auth, body).await.0, status);
        }
    }
    for query in ["page=0", "per_page=-1", "status=fake"] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("/api/newsletter/subscribers?{query}"),
                token,
                Value::Null
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(request(api,"POST","/api/newsletter/subscribe",None,json!({"email":"subscriber-bounds@example.test","gdpr_consent":true,"name":"a".repeat(256)})).await.0,StatusCode::BAD_REQUEST);
    let before = adapter.requests.lock().unwrap().len();
    let saved=request(disabled,"POST",root,token,json!({"email":"Subscriber-Offline@example.test","first_name":"Offline","tags":["fixture","fixture"]})).await;
    assert_eq!(saved.0, StatusCode::CREATED, "{saved:?}");
    assert_eq!(saved.1["subscriber"]["status"], "pending");
    assert_eq!(saved.1["confirmation"]["status"], "unavailable");
    assert_eq!(adapter.requests.lock().unwrap().len(), before);
    let offline = saved.1["subscriber"]["id"].as_str().unwrap();
    let consent: bool =
        sqlx::query_scalar("SELECT gdpr_consent FROM newsletter_subscribers WHERE id=$1")
            .bind(offline.parse::<i64>().unwrap())
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(
        !consent,
        "Admin actions cannot fabricate the recipient's opt-in"
    );
    assert!(saved.1["subscriber"]["subscribed_at"].is_null());
    assert!(saved.1["subscriber"]["email_score"].is_null());
    assert_eq!(
        request(
            api,
            "POST",
            root,
            token,
            json!({"email":"subscriber-offline@EXAMPLE.test"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    for body in [
        json!({"email":"invalid"}),
        json!({"email":"bad-subscriber@example.test","status":"subscribed"}),
        json!({"email":"bad-subscriber@example.test","tags":[""]}),
        json!({"email":"bad-subscriber@example.test","custom_fields":[]} ),
    ] {
        assert!(
            request(api, "POST", root, token, body)
                .await
                .0
                .is_client_error()
        );
    }
    for query in ["page=0", "per_page=-1", "status=fake"] {
        assert!(
            request(api, "GET", &format!("{root}?{query}"), token, Value::Null)
                .await
                .0
                .is_client_error()
        );
    }
    // Simulate elapsed time on the fixture rather than sleeping or reducing the actual cooldown.
    sqlx::query("UPDATE newsletter_subscribers SET metadata=metadata-'_confirmation' WHERE id=$1")
        .bind(offline.parse::<i64>().unwrap())
        .execute(pool)
        .await
        .unwrap();
    let resend_path = format!("{root}/{offline}/resubscribe");
    let (first, second) = tokio::join!(
        request(api, "POST", &resend_path, token, Value::Null),
        request(api, "POST", &resend_path, token, Value::Null)
    );
    assert!(
        (first.0 == StatusCode::OK && second.0 == StatusCode::TOO_MANY_REQUESTS)
            || (second.0 == StatusCode::OK && first.0 == StatusCode::TOO_MANY_REQUESTS),
        "{first:?} {second:?}"
    );
    assert_eq!(adapter.requests.lock().unwrap().len(), before + 1);
    let link = {
        adapter
            .requests
            .lock()
            .unwrap()
            .iter()
            .rev()
            .find(|p| p["To"] == "subscriber-offline@example.test")
            .unwrap()["TemplateModel"]["confirm_url"]
            .as_str()
            .unwrap()
            .to_string()
    };
    assert!(!format!("{saved:?}").contains("token="));
    let url = reqwest::Url::parse(&link).unwrap();
    let confirmed = request(
        api,
        "GET",
        &format!("/api/newsletter/confirm?{}", url.query().unwrap()),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(confirmed.0, StatusCode::OK, "{confirmed:?}");
    let read = request(api, "GET", &format!("{root}/{offline}"), token, Value::Null).await;
    assert_eq!(read.1["subscriber"]["status"], "subscribed");
    let consent: (bool, Option<String>) = sqlx::query_as(
        "SELECT gdpr_consent,consent_source FROM newsletter_subscribers WHERE id=$1",
    )
    .bind(offline.parse::<i64>().unwrap())
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(consent, (true, Some("email_confirmation".into())));
    let changed=request(api,"PUT",&format!("{root}/{offline}"),token,json!({"first_name":"=Formula","last_name":"Fixture","custom_fields":{"tier":"gold","nested":{"verified":false}}})).await;
    assert_eq!(changed.0, StatusCode::OK, "{changed:?}");
    assert_eq!(changed.1["subscriber"]["custom_fields"]["tier"], "gold");
    assert!(
        request(
            api,
            "PUT",
            &format!("{root}/{offline}"),
            token,
            json!({"email":"new-owner@example.test"})
        )
        .await
        .0
        .is_client_error()
    );
    for (method, tags) in [
        ("POST", json!(["vip", "fixture"])),
        ("DELETE", json!(["fixture"])),
    ] {
        assert_eq!(
            request(
                api,
                method,
                &format!("{root}/{offline}/tags"),
                token,
                json!({"tags":tags})
            )
            .await
            .0,
            StatusCode::OK
        );
    }
    let list = request(
        api,
        "GET",
        &format!("{root}?search=Formula&tags=vip&status=subscribed&per_page=1"),
        token,
        Value::Null,
    )
    .await;
    assert_eq!(list.0, StatusCode::OK, "{list:?}");
    assert_eq!(list.1["total"], 1);
    assert_eq!(list.1["subscribers"][0]["id"], offline);
    let csv = api
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!("{root}/export?search=Formula&tags=vip"))
                .header("Authorization", format!("Bearer {}", tokens[0]))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(csv.status(), StatusCode::OK);
    assert!(
        csv.headers()["content-type"]
            .to_str()
            .unwrap()
            .starts_with("text/csv")
    );
    let csv =
        String::from_utf8(csv.into_body().collect().await.unwrap().to_bytes().to_vec()).unwrap();
    assert!(csv.contains("\"'=Formula\""), "{csv}");
    assert!(csv.contains("subscriber-offline@example.test"));
    assert!(!csv.contains("pending-email-proof"));
    assert!(!csv.contains("token="));
    // Case variants: newest suppression wins, stale IDs cannot mutate, deletion removes
    // every variant rather than exposing an older confirmed audience record.
    let old:i64=sqlx::query_scalar("INSERT INTO newsletter_subscribers(email,status,gdpr_consent,confirmed_at,updated_at) VALUES('Subscriber-Duplicate@example.test','confirmed',true,NOW()-INTERVAL '2 days',NOW()-INTERVAL '1 day') RETURNING id").fetch_one(pool).await.unwrap();
    let newer:i64=sqlx::query_scalar("INSERT INTO newsletter_subscribers(email,status,unsubscribed_at,updated_at) VALUES('subscriber-duplicate@example.test','unsubscribed',NOW(),NOW()) RETURNING id").fetch_one(pool).await.unwrap();
    let list = request(
        api,
        "GET",
        &format!("{root}?search=subscriber-duplicate"),
        token,
        Value::Null,
    )
    .await;
    assert_eq!(list.1["total"], 1);
    assert_eq!(list.1["subscribers"][0]["status"], "unsubscribed");
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{root}/{old}"),
            token,
            json!({"first_name":"Stale"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{root}/bulk-delete"),
            token,
            json!({"ids":[newer.to_string(),"999999999"]})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM newsletter_subscribers WHERE LOWER(email)='subscriber-duplicate@example.test'").fetch_one(pool).await.unwrap();
    assert_eq!(count, 2, "Bulk deletion must be atomic");
    let legacy_list = request(
        api,
        "GET",
        "/api/newsletter/subscribers?search=subscriber-duplicate",
        token,
        Value::Null,
    )
    .await;
    assert_eq!(legacy_list.0, StatusCode::OK, "{legacy_list:?}");
    assert_eq!(legacy_list.1["meta"]["total"], 1);
    assert_eq!(legacy_list.1["data"][0]["id"], newer);
    let deleted = request(
        api,
        "POST",
        &format!("{root}/bulk-delete"),
        token,
        json!({"ids":[newer.to_string()]}),
    )
    .await;
    assert_eq!(deleted.1["deleted"], 1);
    assert_eq!(deleted.1["deleted_records"], 2);
    assert_eq!(
        request(api, "GET", &format!("{root}/{old}"), token, Value::Null)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    sqlx::query("INSERT INTO newsletter_subscribers(email,status,tags) SELECT 'subscriber-pages-'||n||'@example.test','pending','[\"page-tag\"]'::jsonb FROM generate_series(1,9) n").execute(pool).await.unwrap();
    let first_page = request(
        api,
        "GET",
        &format!("{root}?search=subscriber-pages-&tags=page-tag&per_page=5&page=1"),
        token,
        Value::Null,
    )
    .await;
    let next_page = request(
        api,
        "GET",
        &format!("{root}?search=subscriber-pages-&tags=page-tag&per_page=5&page=2"),
        token,
        Value::Null,
    )
    .await;
    assert_eq!(first_page.1["total"], 9);
    assert_eq!(first_page.1["subscribers"].as_array().unwrap().len(), 5);
    assert_eq!(next_page.1["subscribers"].as_array().unwrap().len(), 4);
    let first_ids: Vec<_> = first_page.1["subscribers"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s["id"].clone())
        .collect();
    assert!(
        next_page.1["subscribers"]
            .as_array()
            .unwrap()
            .iter()
            .all(|s| !first_ids.contains(&s["id"]))
    );
    // Real provider webhook facts supply activity and blocking status; no engagement score is invented.
    let now = chrono::Utc::now();
    for (kind, details) in [
        ("Open", json!({})),
        ("Click", json!({})),
        ("SpamComplaint", json!({})),
    ] {
        sqlx::query("INSERT INTO crm_campaign_events(fingerprint,recipient_email,record_type,occurred_at,details) VALUES($1,'subscriber-offline@example.test',$2,$3,$4)").bind(format!("subscriber-proof-{kind}")).bind(kind).bind(now).bind(details).execute(pool).await.unwrap();
    }
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{root}/{offline}/unsubscribe"),
            token,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    let read = request(api, "GET", &format!("{root}/{offline}"), token, Value::Null).await;
    assert_eq!(read.1["subscriber"]["status"], "complained");
    assert!(read.1["subscriber"]["last_opened_at"].is_string());
    assert!(read.1["subscriber"]["last_clicked_at"].is_string());
    assert!(read.1["subscriber"]["email_score"].is_null());
    let stats = request(api, "GET", &format!("{root}/stats"), token, Value::Null).await;
    assert_eq!(stats.0, StatusCode::OK);
    assert!(stats.1["complained"].as_i64().unwrap() >= 1);
    for (email, outcome) in [
        ("subscriber-rejected@example.test", "failed"),
        ("subscriber-unknown@example.test", "unknown"),
    ] {
        let result = request(api, "POST", root, token, json!({"email":email})).await;
        assert_eq!(result.0, StatusCode::CREATED, "{result:?}");
        assert_eq!(result.1["confirmation"]["status"], outcome);
        assert_eq!(result.1["subscriber"]["status"], "pending");
        assert!(result.1["subscriber"]["subscribed_at"].is_null());
    }
    // Expired in-flight claims are uncertain; reads never misreport them as accepted.
    sqlx::query("UPDATE newsletter_subscribers SET metadata=jsonb_set(metadata,'{_confirmation}',jsonb_build_object('status','processing','lease_until',(NOW()-INTERVAL '1 minute')::text)) WHERE id=$1").bind(offline.parse::<i64>().unwrap()).execute(pool).await.unwrap();
    let read = request(api, "GET", &format!("{root}/{offline}"), token, Value::Null).await;
    assert_eq!(read.1["subscriber"]["confirmation"]["status"], "unknown");
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{root}/{offline}"),
            token,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    // CSV export traverses the full filtered snapshot, independent of list pagination.
    sqlx::query("INSERT INTO newsletter_subscribers(email,status) SELECT 'subscriber-export-'||n||'@example.test','pending' FROM generate_series(1,501) n").execute(pool).await.unwrap();
    let csv = api
        .clone()
        .oneshot(
            Request::builder()
                .uri(format!(
                    "{root}/export?search=subscriber-export-&per_page=1"
                ))
                .header("Authorization", format!("Bearer {}", tokens[0]))
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(csv.status(), StatusCode::OK);
    let csv =
        String::from_utf8(csv.into_body().collect().await.unwrap().to_bytes().to_vec()).unwrap();
    assert_eq!(csv.lines().count(), 502);
    assert!(csv.contains("subscriber-export-501@example.test"));
    sqlx::query("ALTER TABLE newsletter_subscribers RENAME TO subscriber_export_offline_fixture")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(api, "GET", &format!("{root}/export"), token, Value::Null).await;
    sqlx::query("ALTER TABLE subscriber_export_offline_fixture RENAME TO newsletter_subscribers")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        failed.0,
        StatusCode::INTERNAL_SERVER_ERROR,
        "Unavailable storage cannot produce a successful empty CSV"
    );
    let legacy_old:i64=sqlx::query_scalar("INSERT INTO newsletter_subscribers(email,status,updated_at) VALUES('Legacy-Delete-Subscriber@example.test','confirmed',NOW()-INTERVAL '1 day') RETURNING id").fetch_one(pool).await.unwrap();
    let legacy_new:i64=sqlx::query_scalar("INSERT INTO newsletter_subscribers(email,status,updated_at) VALUES('legacy-delete-subscriber@example.test','unsubscribed',NOW()) RETURNING id").fetch_one(pool).await.unwrap();
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("/api/newsletter/subscribers/{legacy_old}"),
            token,
            Value::Null
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("/api/newsletter/subscribers/{legacy_new}"),
            token,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    let count:i64=sqlx::query_scalar("SELECT COUNT(*) FROM newsletter_subscribers WHERE LOWER(email)='legacy-delete-subscriber@example.test'").fetch_one(pool).await.unwrap();
    assert_eq!(count, 0);
}
