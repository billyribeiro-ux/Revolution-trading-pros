//! Sequence authoring/enrollment foundation only; worker execution has a separate proof.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
const ROOT: &str = "/api/admin/crm/sequences";

async fn audience_identity_proof(api: &Router, pool: &PgPool, token: &str) {
    let admin = Some(token);
    let contacts: Vec<i64> = sqlx::query_scalar(
        "INSERT INTO contacts(email,first_name,tags,source) VALUES
         ('sequence-audience-identity@example.test','Matching','[\"Sequence eligible\"]','sequence-identity'),
         ('SEQUENCE-AUDIENCE-IDENTITY@example.test','Outside filter','[]','sequence-identity') RETURNING id",
    )
    .fetch_all(pool)
    .await
    .unwrap();
    let newsletter: i64 = sqlx::query_scalar(
        "INSERT INTO newsletter_subscribers(email,name,status,confirmed_at,tags)
         VALUES('sequence-audience-identity@example.test','Source fixture','confirmed',NOW(),'[\"Sequence source eligible\"]') RETURNING id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    let member: i64 = sqlx::query_scalar(
        "INSERT INTO users(email,name,password_hash,role,email_verified_at)
         VALUES('sequence-audience-identity@example.test','Source fixture','not-a-login-password','user',NOW()) RETURNING id",
    )
    .fetch_one(pool)
    .await
    .unwrap();
    // Independent source tables can share or differ in numeric IDs. Choose an
    // actual CRM contact whose ID differs to exercise address-based mapping.
    let newsletter_contact = *contacts.iter().find(|id| **id != newsletter).unwrap();
    let member_contact = *contacts.iter().find(|id| **id != member).unwrap();
    for (title, audience, selected, denied) in [
        (
            "Sequence contact identity",
            json!({"advanced_filters":[{"field":"tags","operator":"contains","value":"Sequence eligible"}]}),
            contacts[0],
            Some(contacts[1]),
        ),
        (
            "Sequence newsletter identity",
            json!({"newsletter_segment":"Sequence source eligible"}),
            newsletter_contact,
            None,
        ),
        (
            "Sequence member identity",
            json!({"members":true}),
            member_contact,
            None,
        ),
    ] {
        let created = request(
            api,
            "POST",
            ROOT,
            admin,
            json!({"title":title,"subscriber_settings":audience,"emails":[{"title":"Identity email","subject":"Identity fixture","html_content":"<p>Actual selected contact</p>","status":"active","delay_value":0,"delay_unit":"minutes"}]}),
        )
        .await;
        assert_eq!(created.0, StatusCode::OK, "{created:?}");
        let id = created.1["id"].as_str().unwrap().parse::<i64>().unwrap();
        let path = format!("{ROOT}/{id}");
        let activated = request(api, "POST", &format!("{path}/activate"), admin, Value::Null).await;
        assert_eq!(activated.0, StatusCode::OK, "{activated:?}");
        if let Some(denied) = denied {
            let rejected = request(
                api,
                "POST",
                &format!("{path}/subscribe"),
                admin,
                json!({"contact_ids":[denied]}),
            )
            .await;
            assert_eq!(
                rejected.0,
                StatusCode::BAD_REQUEST,
                "A nonmatching contact cannot borrow a same-address contact's tags: {rejected:?}"
            );
            let count: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM crm_sequence_trackers WHERE sequence_id=$1",
            )
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
            assert_eq!(count, 0, "Rejected enrollment leaves no tracker");
        }
        let enrolled = request(
            api,
            "POST",
            &format!("{path}/subscribe"),
            admin,
            json!({"contact_ids":[selected]}),
        )
        .await;
        assert_eq!(enrolled.0, StatusCode::OK, "{enrolled:?}");
        assert_eq!(enrolled.1["subscribed_count"], 1);
        let tracker: (i64, String) = sqlx::query_as(
            "SELECT contact_id,contact_email FROM crm_sequence_trackers WHERE sequence_id=$1",
        )
        .bind(id)
        .fetch_one(pool)
        .await
        .unwrap();
        assert_eq!(
            tracker,
            (selected, "sequence-audience-identity@example.test".into())
        );
        assert_eq!(
            request(api, "POST", &format!("{path}/pause"), admin, Value::Null)
                .await
                .0,
            StatusCode::OK
        );
        assert_eq!(
            request(api, "DELETE", &path, admin, Value::Null).await.0,
            StatusCode::OK
        );
    }
}

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let before = adapter.requests.lock().unwrap().len();
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, path, body) in [
            ("GET", ROOT.to_string(), Value::Null),
            ("POST", ROOT.to_string(), json!({"title":"Denied"})),
            ("PUT", format!("{ROOT}/1"), json!({"status":"active"})),
            (
                "POST",
                format!("{ROOT}/1/subscribe"),
                json!({"contact_ids":[1]}),
            ),
        ] {
            assert_eq!(request(api, method, &path, token, body).await.0, status);
        }
    }
    for body in [
        json!({"title":""}),
        json!({"title":"Bad","slug":"BAD SLUG"}),
        json!({"title":"Bad","status":"active"}),
        json!({"title":"Bad","settings":{"mailer_settings":{"is_custom":true,"from_email":"invalid"}}}),
        json!({"title":"Bad","emails":[{"title":"Invalid","subject":"header\r\ninjection"}]}),
    ] {
        assert_eq!(
            request(api, "POST", ROOT, admin, body).await.0,
            StatusCode::BAD_REQUEST
        );
    }
    let a = uuid::Uuid::new_v4();
    let b = uuid::Uuid::new_v4();
    let created=request(api,"POST",ROOT,admin,json!({"title":"Sequence foundation","description":"Persisted description","emails":[{"id":a,"title":"First","subject":"First {{email}}","html_content":"<p>First source</p>","plain_text_content":"First alternative","status":"active","delay_value":0,"delay_unit":"minutes"},{"id":b,"title":"Second","subject":"Second","html_content":"<p>Second source</p>","status":"active","delay_value":1,"delay_unit":"minutes","conditions":[{"type":"opened","previous_email_id":a}]}]})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["id"].as_str().unwrap().parse::<i64>().unwrap();
    let path = format!("{ROOT}/{id}");
    assert_eq!(created.1["emails_count"], 2);
    assert_eq!(created.1["status"], "draft");
    assert_eq!(created.1["authoring_version"], 1);
    assert!(created.1["open_rate"].is_null());
    assert!(created.1["total_revenue"].is_null());
    let updated = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"description":"Actual revised description","expected_version":1}),
    )
    .await;
    assert_eq!(updated.0, StatusCode::OK, "{updated:?}");
    assert_eq!(updated.1["authoring_version"], 2);
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            admin,
            json!({"description":"Stale","expected_version":1})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let clear = request(
        api,
        "PUT",
        &format!("{path}/emails/{a}"),
        admin,
        json!({"text_body":null}),
    )
    .await;
    assert_eq!(clear.0, StatusCode::OK, "{clear:?}");
    assert!(clear.1["text_body"].is_null());
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/emails/reorder"),
            admin,
            json!({"email_ids":[b,a]})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST,
        "Earlier-mail condition cannot become a forward reference"
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/emails"),
            admin,
            json!({"title":"Bad delay","delay_value":367,"delay_unit":"days"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let activated = request(api, "POST", &format!("{path}/activate"), admin, Value::Null).await;
    assert_eq!(activated.0, StatusCode::OK, "{activated:?}");
    assert_eq!(activated.1["status"], "active");
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{path}/emails/{a}"),
            admin,
            json!({"title":"Must pause"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let contacts:Vec<i64>=sqlx::query_scalar("INSERT INTO contacts(email,first_name,source) VALUES('sequence-foundation-one@example.test','One','sequence-foundation'),('sequence-foundation-two@example.test','Two','sequence-foundation') RETURNING id").fetch_all(pool).await.unwrap();
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/subscribe"),
            admin,
            json!({"contact_ids":contacts})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST,
        "A contact record is not consent"
    );
    sqlx::query("INSERT INTO newsletter_subscribers(email,name,status,confirmed_at) VALUES('sequence-foundation-one@example.test','One','confirmed',NOW()),('sequence-foundation-two@example.test','Two','confirmed',NOW())").execute(pool).await.unwrap();
    let body = json!({"contact_ids":contacts.iter().map(ToString::to_string).collect::<Vec<_>>()});
    let enrolled = request(
        api,
        "POST",
        &format!("{path}/subscribe"),
        admin,
        body.clone(),
    )
    .await;
    assert_eq!(enrolled.0, StatusCode::OK, "{enrolled:?}");
    assert_eq!(enrolled.1["subscribed_count"], 2);
    let replay = request(api, "POST", &format!("{path}/subscribe"), admin, body).await;
    assert_eq!(replay.1["subscribed_count"], 0);
    assert_eq!(replay.1["already_enrolled"], 2);
    let subscribers = request(
        api,
        "GET",
        &format!("{path}/subscribers?per_page=1&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(subscribers.0, StatusCode::OK, "{subscribers:?}");
    assert_eq!(subscribers.1["meta"]["total"], 2);
    assert_eq!(subscribers.1["data"].as_array().unwrap().len(), 1);
    assert_eq!(subscribers.1["data"][0]["emails_sent"], 0);
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{path}/unsubscribe"),
            admin,
            json!({"contact_ids":[contacts[0]],"reason":"Actual fixture removal"})
        )
        .await
        .1["unsubscribed_count"],
        1
    );
    let duplicate = request(
        api,
        "POST",
        &format!("{path}/duplicate"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(duplicate.0, StatusCode::OK, "{duplicate:?}");
    assert_eq!(duplicate.1["status"], "draft");
    assert_eq!(duplicate.1["subscribers_count"], 0);
    let copy_a = duplicate.1["emails"][0]["id"].clone();
    assert_ne!(copy_a, a.to_string());
    assert_eq!(
        duplicate.1["emails"][1]["conditions"][0]["previous_email_id"],
        copy_a
    );
    assert_eq!(
        request(api, "POST", &format!("{path}/pause"), admin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO sequence_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(api, "PUT", &path, admin, json!({"title":"Must roll back"})).await;
    sqlx::query("ALTER TABLE sequence_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await.1["sequence"]["title"],
        "Sequence foundation"
    );
    let raw = json!([{"subject":"Preserved legacy source","body":"Original legacy body"}]);
    let legacy:i64=sqlx::query_scalar("INSERT INTO crm_sequences(name,title,slug,status,steps,emails_sent) VALUES('Legacy foundation','Legacy foundation','legacy-sequence-foundation','draft',$1,999) RETURNING id").bind(&raw).fetch_one(pool).await.unwrap();
    let legacy_path = format!("{ROOT}/{legacy}");
    let legacy_row = request(api, "GET", &legacy_path, admin, Value::Null).await;
    assert_eq!(legacy_row.0, StatusCode::OK);
    assert_eq!(legacy_row.1["sequence"]["historical_steps"], raw);
    assert!(!legacy_row.1["sequence"]["definition_error"].is_null());
    assert!(legacy_row.1["sequence"]["emails_count"].is_null());
    assert_eq!(legacy_row.1["sequence"]["total_sent"], 0);
    assert_eq!(legacy_row.1["sequence"]["historical_counts"]["sent"], 999);
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{legacy_path}/activate"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let repaired=request(api,"PUT",&legacy_path,admin,json!({"emails":[{"title":"Reviewed actual source","subject":"Reviewed subject","email_body":"Original legacy body"}]})).await;
    assert_eq!(repaired.0, StatusCode::OK, "{repaired:?}");
    let preserved:Value=sqlx::query_scalar("SELECT definition->'steps' FROM crm_sequence_revisions WHERE sequence_id=$1 AND authoring_version=1").bind(legacy).fetch_one(pool).await.unwrap();
    assert_eq!(preserved, raw);
    assert_eq!(
        request(api, "DELETE", &path, admin, Value::Null).await.0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await.0,
        StatusCode::NOT_FOUND
    );
    let retained: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM crm_sequence_trackers WHERE sequence_id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(retained, 2);
    audience_identity_proof(api, pool, &tokens[0]).await;
    assert_eq!(
        adapter.requests.lock().unwrap().len(),
        before,
        "Authoring and enrollment must not contact a provider; this proof does not establish worker execution"
    );
}
