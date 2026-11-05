//! Actual default-grant and current-fact equivalence across non-UTC PostgreSQL sessions.
use super::request;
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

async fn timezone(pool: &PgPool, zone: &str) -> String {
    let count = pool.options().get_max_connections();
    assert!((1..=5).contains(&count), "Disposable proof pool only");
    let mut held = Vec::new();
    let mut previous = Vec::<String>::new();
    for _ in 0..count {
        let mut connection = pool.acquire().await.unwrap();
        previous.push(
            sqlx::query_scalar("SHOW TIME ZONE")
                .fetch_one(&mut *connection)
                .await
                .unwrap(),
        );
        held.push(connection);
    }
    assert!(
        previous.iter().all(|zone| zone == &previous[0]),
        "Proof requires one known prior session-zone configuration"
    );
    for connection in &mut held {
        let actual: String = sqlx::query_scalar("SELECT set_config('TimeZone',$1,false)")
            .bind(zone)
            .fetch_one(&mut **connection)
            .await
            .unwrap();
        assert_eq!(actual, zone);
    }
    previous[0].clone()
}
async fn get(api: &Router, path: &str, token: &str) -> Value {
    let (status, body) = request(api, "GET", path, Some(token), Value::Null).await;
    assert_eq!(status, StatusCode::OK, "{path}: {body}");
    body
}
async fn snapshot(
    api: &Router,
    pool: &PgPool,
    token: &str,
    prefix: &str,
    plan: i64,
    users: &[i64],
) -> Value {
    let all = get(
        api,
        &format!("/api/admin/members?search={prefix}&per_page=100&sort_by=id&sort_dir=asc"),
        token,
    )
    .await;
    assert_eq!(all["pagination"]["total"], 8);
    let rows = all["members"].as_array().unwrap();
    let statuses = rows
        .iter()
        .map(|row| {
            (
                row["id"].as_i64().unwrap(),
                row["status"].as_str().unwrap().to_owned(),
            )
        })
        .collect::<std::collections::BTreeMap<_, _>>();
    for (index, status) in [
        (0, "active"),
        (1, "churned"),
        (2, "never_subscribed"),
        (3, "trial"),
        (4, "churned"),
        (5, "never_subscribed"),
        (6, "active"),
        (7, "active"),
    ] {
        assert_eq!(statuses[&users[index]], status);
    }
    let active = get(
        api,
        &format!(
            "/api/admin/members?search={prefix}&status=active&per_page=100&sort_by=id&sort_dir=asc"
        ),
        token,
    )
    .await;
    assert_eq!(
        active["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["id"].as_i64().unwrap())
            .collect::<Vec<_>>(),
        vec![users[0], users[6], users[7]]
    );
    let catalog = get(
        api,
        &format!("/api/admin/members/services?search={prefix}&selected_id={plan}"),
        token,
    )
    .await;
    assert_eq!(catalog["selected"]["members_count"], 4);
    let local = get(
        api,
        &format!("/api/admin/members/service/{plan}?search={prefix}&per_page=100"),
        token,
    )
    .await;
    assert_eq!(local["pagination"]["total"], 7);
    for (key, count) in [
        ("active_members", 3),
        ("trial_members", 1),
        ("expired_members", 2),
        ("cancelled_members", 0),
        ("other_members", 1),
    ] {
        assert_eq!(local["stats"][key], count, "{key}: {local}");
    }
    let mut measured = Vec::new();
    for (status, expected) in [("active", 3_i64), ("trial", 2)] {
        let rules = [revolution_api::services::member_segmentation::Condition {
            field: "subscription_status".into(),
            operator: "eq".into(),
            value: json!(status),
        }];
        let mut query =
            revolution_api::services::member_segmentation::query("SELECT COUNT(*)", &rules)
                .unwrap();
        query
            .push(" AND m.id=ANY(")
            .push_bind(users.to_vec())
            .push(")");
        let count: i64 = query.build_query_scalar().fetch_one(pool).await.unwrap();
        assert_eq!(count, expected, "{status}");
        measured.push(count);
    }
    json!({"members":all["members"],"active":active["members"],"catalog_count":catalog["selected"]["members_count"],"local_stats":local["stats"],"local_members":local["members"],"segment_counts":measured})
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let original = timezone(pool, "America/New_York").await;
    let session: String = sqlx::query_scalar("SHOW TIME ZONE")
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(session, "America/New_York");
    let prefix = format!("member-utc-{}", Uuid::new_v4());
    let users:Vec<i64>=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,created_at) SELECT $1||'-'||n,$1||'-'||n||'@example.test','not-a-login-password','user',(NOW() AT TIME ZONE 'UTC') FROM generate_series(1,9)n RETURNING id").bind(&prefix).fetch_all(pool).await.unwrap();
    let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES($1,$1,10,'monthly') RETURNING id").bind(&prefix).fetch_one(pool).await.unwrap();
    // The original defect is tested through the actual API without starts_at, not SQL NOW() fixtures.
    let (status, granted) = request(
        api,
        "POST",
        "/api/admin/user-memberships",
        Some(&tokens[0]),
        json!({"user_id":users[0],"plan_id":plan,"status":"active"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{granted}");
    assert_eq!(granted["membership"]["user_id"], users[0]);
    assert_eq!(granted["membership"]["plan_id"], plan);
    let id = granted["membership"]["id"].as_i64().unwrap();
    let exact:bool=sqlx::query_scalar("SELECT abs(EXTRACT(EPOCH FROM(starts_at-(NOW() AT TIME ZONE 'UTC'))))<30 AND abs(EXTRACT(EPOCH FROM(created_at-(NOW() AT TIME ZONE 'UTC'))))<30 AND abs(EXTRACT(EPOCH FROM(updated_at-(NOW() AT TIME ZONE 'UTC'))))<30 FROM user_memberships WHERE id=$1").bind(id).fetch_one(pool).await.unwrap();
    assert!(
        exact,
        "Default grant must store operational UTC under a real New York session"
    );
    // Expired/future bounds deliberately fall inside the old four/five-hour session offset.
    for (index, status, start_delta, end_delta, trial_delta, deleted) in [
        (1, "active", -600_i32, Some(-60_i32), None, false),
        (2, "active", 60, Some(600), None, false),
        (3, "trial", -600, Some(600), Some(600), false),
        (4, "trial", -600, Some(600), Some(-60), false),
        (5, "active", -600, Some(600), None, true),
        (6, "active", -600, Some(600), None, false),
        (6, "trial", -600, Some(600), Some(600), false),
        (8, "active", -600, Some(600), None, false),
    ] {
        sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,trial_ends_at,deleted_at) VALUES($1,$2,$3,(NOW() AT TIME ZONE 'UTC')+$4*INTERVAL '1 second',(NOW() AT TIME ZONE 'UTC')+$5*INTERVAL '1 second',(NOW() AT TIME ZONE 'UTC')+$6*INTERVAL '1 second',CASE WHEN $7 THEN NOW() ELSE NULL END)").bind(users[index]).bind(plan).bind(status).bind(start_delta).bind(end_delta).bind(trial_delta).bind(deleted).execute(pool).await.unwrap();
    }
    // Actual new DB defaults also follow UTC, even when no application timestamp was supplied.
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status) VALUES($1,$2,'active')")
        .bind(users[7])
        .bind(plan)
        .execute(pool)
        .await
        .unwrap();
    let defaults:bool=sqlx::query_scalar("SELECT abs(EXTRACT(EPOCH FROM(starts_at-(NOW() AT TIME ZONE 'UTC'))))<30 FROM user_memberships WHERE user_id=$1").bind(users[7]).fetch_one(pool).await.unwrap();
    assert!(defaults);
    sqlx::query("UPDATE users SET deleted_at=NOW() WHERE id=$1")
        .bind(users[8])
        .execute(pool)
        .await
        .unwrap();
    // One intentionally ambiguous historical wall-clock value remains unchanged by the forward DDL.
    sqlx::query("UPDATE user_memberships SET starts_at=TIMESTAMP '2020-11-01 01:30:00',expires_at=TIMESTAMP '2020-11-01 01:31:00' WHERE user_id=$1").bind(users[1]).execute(pool).await.unwrap();
    let ny = snapshot(api, pool, &tokens[0], &prefix, plan, &users).await;
    timezone(pool, "UTC").await;
    let utc = snapshot(api, pool, &tokens[0], &prefix, plan, &users).await;
    assert_eq!(ny, utc);
    timezone(pool, "Pacific/Kiritimati").await;
    let positive = snapshot(api, pool, &tokens[0], &prefix, plan, &users).await;
    assert_eq!(ny, positive);
    // Replaying the exact forward statements inside a rollback proves defaults/view changes do not rewrite existing date bytes.
    let mut tx = pool.begin().await.unwrap();
    let mut segment_versions = Vec::new();
    for (suffix, rules, affected) in [
        (
            "status-array",
            json!([{"field":"status","operator":"eq","value":"active"}]),
            true,
        ),
        (
            "current-membership-object",
            json!({"conditions":[{"field":"subscription_status","operator":"eq","value":"active"}]}),
            true,
        ),
        (
            "recorded-spend",
            json!([{"field":"total_spent","operator":"gte","value":100}]),
            false,
        ),
    ] {
        let name = format!("{prefix}-{suffix}");
        let id: i64 = sqlx::query_scalar(
            "INSERT INTO member_segments(name,slug,filter_criteria,authoring_version) VALUES($1,$1,$2,17) RETURNING id",
        )
        .bind(name)
        .bind(rules)
        .fetch_one(&mut *tx)
        .await
        .unwrap();
        segment_versions.push((id, affected));
    }
    let observations: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM member_segment_observations")
        .fetch_one(&mut *tx)
        .await
        .unwrap();
    let before: Value = sqlx::query_scalar(
        "SELECT jsonb_agg(to_jsonb(s) ORDER BY id) FROM user_memberships s WHERE user_id=ANY($1)",
    )
    .bind(&users)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    sqlx::raw_sql(include_str!(
        "../../migrations/115_member_operational_utc.sql"
    ))
    .execute(&mut *tx)
    .await
    .unwrap();
    let after: Value = sqlx::query_scalar(
        "SELECT jsonb_agg(to_jsonb(s) ORDER BY id) FROM user_memberships s WHERE user_id=ANY($1)",
    )
    .bind(&users)
    .fetch_one(&mut *tx)
    .await
    .unwrap();
    assert_eq!(before, after, "No historical membership timestamp rewrite");
    for (id, affected) in segment_versions {
        let version: i64 =
            sqlx::query_scalar("SELECT authoring_version FROM member_segments WHERE id=$1")
                .bind(id)
                .fetch_one(&mut *tx)
                .await
                .unwrap();
        assert_eq!(
            version,
            if affected { 18 } else { 17 },
            "Only operational-UTC-dependent rules receive a new definition version"
        );
    }
    let retained_observations: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM member_segment_observations")
            .fetch_one(&mut *tx)
            .await
            .unwrap();
    assert_eq!(observations, retained_observations);
    tx.rollback().await.unwrap();
    let aware:i64=sqlx::query_scalar("SELECT COUNT(*) FROM information_schema.columns WHERE table_schema='public' AND table_name='user_memberships' AND column_name IN('deleted_at','last_payment_failure','renewal_reminder_sent_at','trial_ending_reminder_sent_at') AND data_type='timestamp with time zone'").fetch_one(pool).await.unwrap();
    assert_eq!(aware, 4);
    timezone(pool, &original).await;
    sqlx::query("DELETE FROM users WHERE id=ANY($1)")
        .bind(&users)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM membership_plans WHERE id=$1")
        .bind(plan)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Operational UTC member proof passed: real API default grant under America/New_York, DB defaults, near-offset bounds, UTC/+14 session equivalence, current facts/catalog/local cohorts/segments, aware-column preservation and unchanged historical timestamp bytes."
    );
}
