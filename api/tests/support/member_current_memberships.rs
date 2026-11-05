//! Current subscription criteria must not lose an older active paid membership.
//! The legacy `trial` writer permits overlap; keep the active/trialing unique guard intact.
use super::request;
use axum::{Router, http::StatusCode};
use revolution_api::services::member_segmentation as evaluation;
use serde_json::{Value, json};
use sqlx::PgPool;

async fn matching(pool: &PgPool, tag: i64, field: &str, operator: &str, value: &str) -> Vec<i64> {
    let rules=evaluation::rules(Some(json!([{"field":"tag","operator":"has","value":tag},{"field":field,"operator":operator,"value":value}]))).unwrap();
    evaluation::query("SELECT m.id", &rules)
        .unwrap()
        .push(" ORDER BY m.id")
        .build_query_scalar()
        .fetch_all(pool)
        .await
        .unwrap()
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let tag:i64=sqlx::query_scalar("INSERT INTO member_tags(name,slug) VALUES('Current membership cohort','current-membership-cohort') RETURNING id").fetch_one(pool).await.unwrap();
    let mut users = vec![];
    for suffix in [
        "active-cancelled",
        "active-trial",
        "expired-trial",
        "current-trial",
        "future-active",
        "none",
    ] {
        let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash) VALUES($1,$2,'not-a-login-password') RETURNING id").bind(suffix).bind(format!("current-membership-{suffix}@example.test")).fetch_one(pool).await.unwrap();
        sqlx::query("INSERT INTO user_member_tags(user_id,tag_id) VALUES($1,$2)")
            .bind(id)
            .bind(tag)
            .execute(pool)
            .await
            .unwrap();
        users.push(id);
    }
    let paid:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES('Paid membership fixture','current-paid-membership-fixture',20,'monthly') RETURNING id").fetch_one(pool).await.unwrap();
    let other:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES('Other membership fixture','current-other-membership-fixture',10,'monthly') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,created_at) SELECT id,$2,'active',NOW()-INTERVAL '30 days',NOW()+INTERVAL '30 days',NOW()-INTERVAL '10 days' FROM unnest($1::bigint[]) id").bind(vec![users[0],users[1]]).bind(paid).execute(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,created_at) VALUES($1,$2,'cancelled',NOW()-INTERVAL '5 days',NOW()-INTERVAL '1 day',NOW()-INTERVAL '1 day')").bind(users[0]).bind(other).execute(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,trial_ends_at,created_at) SELECT id,$2,'trial',NOW()-INTERVAL '1 day',NOW()+INTERVAL '5 days',NOW()-INTERVAL '12 hours' FROM unnest($1::bigint[]) id").bind(vec![users[1],users[3]]).bind(other).execute(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,trial_ends_at) VALUES($1,$2,'trialing',NOW()-INTERVAL '10 days',NOW()-INTERVAL '1 day')").bind(users[2]).bind(other).execute(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at) VALUES($1,$2,'active',NOW()+INTERVAL '1 day',NOW()+INTERVAL '31 days')").bind(users[4]).bind(paid).execute(pool).await.unwrap();
    assert_eq!(
        matching(pool, tag, "subscription_status", "eq", "active").await,
        vec![users[0], users[1]]
    );
    assert_eq!(
        matching(pool, tag, "subscription_status", "neq", "active").await,
        vec![users[2], users[3], users[4], users[5]]
    );
    assert_eq!(
        matching(pool, tag, "subscription_status", "eq", "trialing").await,
        vec![users[1], users[3]]
    );
    assert_eq!(
        matching(pool, tag, "subscription_status", "eq", "trial").await,
        vec![users[1], users[3]]
    );
    assert_eq!(
        matching(pool, tag, "subscription_status", "eq", "cancelled").await,
        vec![users[0]]
    );
    assert_eq!(
        matching(pool, tag, "subscription_status", "eq", "none").await,
        vec![users[5]]
    );
    assert_eq!(
        matching(pool, tag, "status", "eq", "active").await,
        vec![users[0], users[1]]
    );
    assert_eq!(
        matching(pool, tag, "status", "eq", "trial").await,
        vec![users[3]]
    );
    assert_eq!(
        matching(pool, tag, "status", "eq", "churned").await,
        vec![users[2]]
    );
    assert_eq!(
        matching(pool, tag, "status", "eq", "never_subscribed").await,
        vec![users[4], users[5]]
    );
    let paid_summary: (String, i64, Option<String>) = sqlx::query_as(
        "SELECT status,active_subscriptions_count,current_plan FROM admin_member_facts WHERE id=$1",
    )
    .bind(users[1])
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(
        paid_summary,
        ("active".into(), 1, Some("Paid membership fixture".into()))
    );
    for user in [users[2], users[4]] {
        let plan: Option<String> =
            sqlx::query_scalar("SELECT current_plan FROM admin_member_facts WHERE id=$1")
                .bind(user)
                .fetch_one(pool)
                .await
                .unwrap();
        assert!(plan.is_none());
    }
    let admin = Some(tokens[0].as_str());
    let response = request(
        api,
        "GET",
        &format!("/api/admin/members?tag={tag}&status=active"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(response.0, StatusCode::OK, "{response:?}");
    assert_eq!(response.1["pagination"]["total"], 2);
    let active: i64 =
        sqlx::query_scalar("SELECT id FROM member_segments WHERE slug='active-subscribers'")
            .fetch_one(pool)
            .await
            .unwrap();
    let trial: i64 = sqlx::query_scalar("SELECT id FROM member_segments WHERE slug='trial-users'")
        .fetch_one(pool)
        .await
        .unwrap();
    for id in [active, trial] {
        let version: i64 =
            sqlx::query_scalar("SELECT authoring_version FROM member_segments WHERE id=$1")
                .bind(id)
                .fetch_one(pool)
                .await
                .unwrap();
        assert_eq!(
            version, 2,
            "Semantic migration invalidates only the prior status-rule baseline"
        );
    }
    let active = request(
        api,
        "GET",
        &format!("/api/admin/members?segment={active}&per_page=100"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(active.0, StatusCode::OK, "{active:?}");
    let active = active.1["members"].as_array().unwrap();
    for id in [users[0], users[1]] {
        assert!(active.iter().any(|row| row["id"] == id));
    }
    let trial = request(
        api,
        "GET",
        &format!("/api/admin/members?segment={trial}&per_page=100"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(trial.0, StatusCode::OK, "{trial:?}");
    let trial = trial.1["members"].as_array().unwrap();
    assert!(trial.iter().any(|row| row["id"] == users[3]));
    assert!(
        !trial
            .iter()
            .any(|row| [users[1], users[2], users[4]].contains(&row["id"].as_i64().unwrap()))
    );
}
