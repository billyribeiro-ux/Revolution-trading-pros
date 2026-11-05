//! A segment match never replaces recorded marketing consent.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use revolution_api::services::{campaign_delivery, campaign_mailer::CampaignMailer};
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let admin = Some(tokens[0].as_str());
    let tag:i64=sqlx::query_scalar("INSERT INTO member_tags(name,slug) VALUES('Campaign member cohort','campaign-member-cohort') RETURNING id").fetch_one(pool).await.unwrap();
    let mut users = Vec::new();
    for suffix in [
        "accepted",
        "pending",
        "withdrawn",
        "latest-withdrawn",
        "deleted",
        "later-withdrawn",
        "later-deleted",
    ] {
        let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,email_verified_at) VALUES($1,$2,'not-a-login-password',NOW()) RETURNING id").bind(suffix).bind(format!("member-campaign-{suffix}@example.test")).fetch_one(pool).await.unwrap();
        users.push(id);
        sqlx::query("INSERT INTO user_member_tags(user_id,tag_id) VALUES($1,$2)")
            .bind(id)
            .bind(tag)
            .execute(pool)
            .await
            .unwrap();
        let status = if suffix == "pending" {
            "pending"
        } else if suffix == "withdrawn" {
            "unsubscribed"
        } else {
            "confirmed"
        };
        sqlx::query("INSERT INTO newsletter_subscribers(email,name,status,confirmed_at,unsubscribed_at,updated_at) VALUES($1,$2,$3,CASE WHEN $3='pending' THEN NULL ELSE NOW() END,CASE WHEN $3='unsubscribed' THEN NOW() ELSE NULL END,NOW()-INTERVAL '1 day')").bind(format!("member-campaign-{suffix}@example.test")).bind(suffix).bind(status).execute(pool).await.unwrap();
    }
    sqlx::query("INSERT INTO newsletter_subscribers(email,name,status,confirmed_at,unsubscribed_at,updated_at) VALUES('MEMBER-CAMPAIGN-LATEST-WITHDRAWN@EXAMPLE.TEST','Latest withdrawal','unsubscribed',NOW(),NOW(),NOW())").execute(pool).await.unwrap();
    sqlx::query("UPDATE users SET deleted_at=NOW() WHERE id=$1")
        .bind(users[4])
        .execute(pool)
        .await
        .unwrap();
    let definition = json!({"name":"Campaign dynamic member cohort","rules":[{"field":"tag","operator":"has","value":tag}]});
    let segment = request(
        api,
        "POST",
        "/api/admin/members/segments",
        admin,
        definition,
    )
    .await;
    assert_eq!(segment.0, StatusCode::OK, "{segment:?}");
    let segment_id = segment.1["id"].as_i64().unwrap();
    let audience =
        json!({"members":true,"member_segment":segment_id,"eligibility":"confirmed_only"});
    let create = |settings: Value| json!({"title":"Member audience fixture","email_subject":"Member audience fixture","email_body":"<p>Only confirmed members</p>","subscriber_settings":settings});
    for invalid in [
        json!({"member_segment":segment_id}),
        json!({"members":true,"member_segment":0}),
        json!({"members":true,"newsletter_segment":"all"}),
        json!({"members":true,"dynamic_segment":"1"}),
        json!({"members":true,"subscribers":[{"list":"1"}]}),
        json!({"members":true,"eligibility":"all"}),
    ] {
        assert_eq!(
            request(
                api,
                "POST",
                "/api/admin/crm/campaigns",
                admin,
                create(invalid)
            )
            .await
            .0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        request(
            api,
            "POST",
            "/api/admin/crm/campaigns",
            admin,
            create(json!({"members":true,"member_segment":999999}))
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let campaign = request(
        api,
        "POST",
        "/api/admin/crm/campaigns",
        admin,
        create(audience.clone()),
    )
    .await;
    assert_eq!(campaign.0, StatusCode::OK, "{campaign:?}");
    let id = campaign.1["id"].as_str().unwrap().parse::<i64>().unwrap();
    let path = format!("/api/admin/crm/campaigns/{id}");
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(
                api,
                "GET",
                &format!("{path}/recipients"),
                token,
                Value::Null
            )
            .await
            .0,
            status
        );
        assert_eq!(
            request(api, "POST", &format!("{path}/send"), token, Value::Null)
                .await
                .0,
            status
        );
    }
    let preview = request(
        api,
        "GET",
        &format!("{path}/recipients?per_page=1&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(preview.0, StatusCode::OK, "{preview:?}");
    assert_eq!(preview.1["audience"]["source"], "members");
    assert_eq!(preview.1["audience"]["matching_contacts"], 6);
    assert_eq!(preview.1["audience"]["eligible_recipients"], 3);
    assert_eq!(preview.1["data"].as_array().unwrap().len(), 1);
    let static_id:i64=sqlx::query_scalar("INSERT INTO member_segments(name,slug,is_dynamic,is_active,filter_criteria) VALUES('Static campaign cohort','static-campaign-cohort',false,true,'[]') RETURNING id").fetch_one(pool).await.unwrap();
    for user in [users[0], users[1], users[4]] {
        sqlx::query("INSERT INTO user_member_segments(user_id,segment_id) VALUES($1,$2)")
            .bind(user)
            .bind(static_id)
            .execute(pool)
            .await
            .unwrap();
    }
    let static_campaign = request(
        api,
        "POST",
        "/api/admin/crm/campaigns",
        admin,
        create(json!({"members":true,"member_segment":static_id})),
    )
    .await;
    assert_eq!(static_campaign.0, StatusCode::OK, "{static_campaign:?}");
    let static_id = static_campaign.1["id"].as_str().unwrap();
    let preview = request(
        api,
        "GET",
        &format!("/api/admin/crm/campaigns/{static_id}/recipients"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(preview.0, StatusCode::OK, "{preview:?}");
    assert_eq!(preview.1["meta"]["total"], 1);
    assert_eq!(
        preview.1["data"][0]["email"],
        "member-campaign-accepted@example.test"
    );
    let due = chrono::Utc::now() + chrono::Duration::days(1);
    let scheduled = request(
        api,
        "POST",
        &format!("{path}/schedule"),
        admin,
        json!({"scheduled_at":due}),
    )
    .await;
    assert_eq!(scheduled.0, StatusCode::OK, "{scheduled:?}");
    assert_eq!(scheduled.1["status"], "scheduled");
    let frozen: Value =
        sqlx::query_scalar("SELECT audience_snapshot FROM crm_campaigns WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(frozen["eligible_at_submission"], 3);
    assert_eq!(frozen["criteria"]["segment"]["id"], segment_id);
    assert_eq!(frozen["criteria"]["segment"]["authoring_version"], 1);
    assert_eq!(frozen["criteria"]["segment"]["conditions"][0]["value"], tag);
    let links:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_campaign_deliveries WHERE campaign_id=$1 AND contact_id IS NOT NULL").bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(
        links, 0,
        "Member IDs must never masquerade as contact foreign keys"
    );
    let member_ids:Vec<i64>=sqlx::query_scalar("SELECT (contact_snapshot->>'member_id')::bigint FROM crm_campaign_deliveries WHERE campaign_id=$1 ORDER BY (contact_snapshot->>'member_id')::bigint").bind(id).fetch_all(pool).await.unwrap();
    assert_eq!(member_ids, vec![users[0], users[5], users[6]]);
    // Changing live rules or confirming new recipients cannot rewrite a submitted queue.
    sqlx::query("DELETE FROM user_member_tags WHERE user_id=$1 AND tag_id=$2")
        .bind(users[0])
        .bind(tag)
        .execute(pool)
        .await
        .unwrap();
    let edited = request(
        api,
        "PUT",
        &format!("/api/admin/members/segments/{segment_id}"),
        admin,
        json!({"rules":[{"field":"total_spent","operator":"gt","value":999999}]}),
    )
    .await;
    assert_eq!(edited.0, StatusCode::OK, "{edited:?}");
    sqlx::query("UPDATE newsletter_subscribers SET status='confirmed',confirmed_at=NOW(),updated_at=NOW() WHERE email='member-campaign-pending@example.test'").execute(pool).await.unwrap();
    sqlx::query("UPDATE newsletter_subscribers SET status='unsubscribed',unsubscribed_at=NOW(),updated_at=NOW() WHERE email='member-campaign-later-withdrawn@example.test'").execute(pool).await.unwrap();
    sqlx::query("UPDATE users SET deleted_at=NOW() WHERE id=$1")
        .bind(users[6])
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE crm_campaigns SET scheduled_at=NOW() WHERE id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE crm_campaign_deliveries SET due_at=NOW() WHERE campaign_id=$1")
        .bind(id)
        .execute(pool)
        .await
        .unwrap();
    let before = adapter.requests.lock().unwrap().len();
    let mailer = CampaignMailer::from_env("development").unwrap();
    for _ in 0..3 {
        campaign_delivery::process_one(pool, &mailer, "http://127.0.0.1:5274")
            .await
            .unwrap();
    }
    let states: Vec<(String, String)> = sqlx::query_as(
        "SELECT email,status FROM crm_campaign_deliveries WHERE campaign_id=$1 ORDER BY email",
    )
    .bind(id)
    .fetch_all(pool)
    .await
    .unwrap();
    assert_eq!(
        states,
        vec![
            (
                "member-campaign-accepted@example.test".into(),
                "accepted".into()
            ),
            (
                "member-campaign-later-deleted@example.test".into(),
                "skipped".into()
            ),
            (
                "member-campaign-later-withdrawn@example.test".into(),
                "skipped".into()
            )
        ]
    );
    {
        let requests = adapter.requests.lock().unwrap();
        assert_eq!(requests.len(), before + 1);
        assert_eq!(
            requests.last().unwrap()["To"],
            "member-campaign-accepted@example.test"
        );
    }
    let snapshot: Value =
        sqlx::query_scalar("SELECT audience_snapshot FROM crm_campaigns WHERE id=$1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(snapshot, frozen);
    sqlx::query("UPDATE member_segments SET is_active=false WHERE id=$1")
        .bind(segment_id)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "POST",
            "/api/admin/crm/campaigns",
            admin,
            create(audience)
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
}
