//! Real current-cohort, recorded-money and plan/product-identity workflow proof.
use super::request;
use axum::{Router, http::StatusCode};
use serde_json::Value;
use sqlx::PgPool;
use uuid::Uuid;

async fn get(api: &Router, path: &str, token: Option<&str>) -> (StatusCode, Value) {
    request(api, "GET", path, token, Value::Null).await
}
async fn membership(pool: &PgPool, user: i64, plan: i64, status: &str, days: i32) -> i64 {
    sqlx::query_scalar("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,trial_ends_at) VALUES($1,$2,$3,(NOW() AT TIME ZONE 'UTC')-INTERVAL '10 days',(NOW() AT TIME ZONE 'UTC')+$4*INTERVAL '1 day',(NOW() AT TIME ZONE 'UTC')+$4*INTERVAL '1 day') RETURNING id").bind(user).bind(plan).bind(status).bind(days).fetch_one(pool).await.unwrap()
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = Some(tokens[0].as_str());
    for path in [
        "/api/admin/members/stats",
        "/api/admin/members/services",
        "/api/admin/members/service/114000114",
    ] {
        assert_eq!(get(api, path, None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(
            get(api, path, Some(&tokens[1])).await.0,
            StatusCode::FORBIDDEN
        );
    }
    let (status, before) = get(api, "/api/admin/members/stats", admin).await;
    assert_eq!(status, StatusCode::OK, "{before}");
    let prefix = format!("member-facts-{}", Uuid::new_v4());
    let users:Vec<i64>=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,created_at) SELECT $1||'-'||n,$1||'-'||n||'@example.test','not-a-login-password','user',(NOW() AT TIME ZONE 'UTC') FROM generate_series(1,8)n RETURNING id").bind(&prefix).fetch_all(pool).await.unwrap();
    sqlx::query("UPDATE users SET deleted_at=NOW() WHERE id=$1")
        .bind(users[7])
        .execute(pool)
        .await
        .unwrap();
    sqlx::query(
        "UPDATE users SET created_at=(NOW() AT TIME ZONE 'UTC')+INTERVAL '1 month' WHERE id=$1",
    )
    .bind(users[6])
    .execute(pool)
    .await
    .unwrap();
    let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(id,name,slug,price,billing_cycle,interval_count,is_active) VALUES(114000114,$1,$1,120,'annual',12,false) RETURNING id").bind(&prefix).fetch_one(pool).await.unwrap();
    let other:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle,interval_count) VALUES($1,$1,30,'quarterly',3) RETURNING id").bind(format!("{prefix}-other")).fetch_one(pool).await.unwrap();
    membership(pool, users[0], plan, "active", 30).await;
    membership(pool, users[0], plan, "trial", 30).await; // legal active/trial overlap; distinct members once.
    membership(pool, users[0], plan, "cancelled", -1).await;
    membership(pool, users[1], plan, "active", 30).await;
    membership(pool, users[2], plan, "trial", 30).await;
    membership(pool, users[3], plan, "expired", -1).await;
    membership(pool, users[3], other, "active", 30).await; // globally active, expired in this plan.
    let future = membership(pool, users[4], plan, "active", 30).await;
    sqlx::query("UPDATE user_memberships SET starts_at=(NOW() AT TIME ZONE 'UTC')+INTERVAL '1 day' WHERE id=$1")
        .bind(future)
        .execute(pool)
        .await
        .unwrap();
    let deleted = membership(pool, users[5], plan, "active", 30).await;
    sqlx::query("UPDATE user_memberships SET deleted_at=NOW() WHERE id=$1")
        .bind(deleted)
        .execute(pool)
        .await
        .unwrap();
    membership(pool, users[6], other, "cancelled", -1).await;
    membership(pool, users[7], plan, "active", 30).await;
    for (status, currency, total, refund) in [
        ("partial_refund", Some("USD"), 100_i64, 20_i64),
        ("partially_refunded", Some("USD"), 50, 10),
        ("refunded", Some("USD"), 30, 30),
        ("refunded", Some("USD"), 20, 50),
        ("paid", Some("USD"), 10, 0),
        ("paid", Some("EUR"), 100, 0),
        ("paid", None, 100, 0),
        ("pending", Some("USD"), 100, 0),
    ] {
        sqlx::query("INSERT INTO orders(user_id,order_number,status,currency,total,refund_amount,completed_at) VALUES($1,$2,$3,$4,$5::bigint::numeric,$6::bigint::numeric,(NOW() AT TIME ZONE 'UTC'))").bind(users[0]).bind(Uuid::new_v4().to_string()).bind(status).bind(currency).bind(total).bind(refund).execute(pool).await.unwrap();
    }
    let (status, after) = get(api, "/api/admin/members/stats", admin).await;
    assert_eq!(status, StatusCode::OK, "{after}");
    assert_eq!(after["schema_version"], 1);
    assert_eq!(
        after["overview"]["total_members"].as_i64().unwrap()
            - before["overview"]["total_members"].as_i64().unwrap(),
        7
    );
    assert_eq!(
        after["overview"]["new_this_month"].as_i64().unwrap()
            - before["overview"]["new_this_month"].as_i64().unwrap(),
        6
    );
    for (key, delta) in [("unknown_members", 0), ("future_members", 1)] {
        assert_eq!(
            after["growth_timestamp_coverage"][key].as_i64().unwrap()
                - before["growth_timestamp_coverage"][key].as_i64().unwrap(),
            delta
        );
    }
    for (key, delta) in [
        ("active", 3),
        ("trial", 1),
        ("churned", 1),
        ("never_subscribed", 2),
        ("active_membership_records", 3),
    ] {
        assert_eq!(
            after["subscriptions"][key].as_i64().unwrap()
                - before["subscriptions"][key].as_i64().unwrap(),
            delta,
            "{key}: {after}"
        );
    }
    assert!(after["subscriptions"]["churn_rate"].is_null());
    assert!(after["revenue"]["mrr_cents"].is_null());
    assert!(after["revenue"]["avg_ltv_cents"].is_null());
    assert!(after["revenue"]["catalog_monthly_estimate"]["currency"].is_null());
    let money = &after["revenue"]["recorded_net_usd_order_value"];
    let previous = &before["revenue"]["recorded_net_usd_order_value"];
    for (key, delta) in [
        ("net_cents", 13000),
        ("order_count", 5),
        ("excluded_non_usd_orders", 1),
        ("excluded_unknown_currency_orders", 1),
    ] {
        assert_eq!(
            money[key].as_i64().unwrap() - previous[key].as_i64().unwrap(),
            delta,
            "{key}: {money}"
        );
    }
    let spent: f64 =
        sqlx::query_scalar("SELECT total_spent::float8 FROM admin_member_facts WHERE id=$1")
            .bind(users[0])
            .fetch_one(pool)
            .await
            .unwrap();
    assert_eq!(spent, 130.0);
    let unchanged: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM orders WHERE user_id=$1 AND status='partial_refund'",
    )
    .bind(users[0])
    .fetch_one(pool)
    .await
    .unwrap();
    assert_eq!(unchanged, 1);
    assert_eq!(after["growth_trend"].as_array().unwrap().len(), 12);
    assert!(
        after["recorded_timestamp_basis"]
            .as_str()
            .unwrap()
            .contains("without time zone")
    );
    let path = format!("/api/admin/members/service/{plan}");
    let (status, all) = get(api, &path, admin).await;
    assert_eq!(status, StatusCode::OK, "{all}");
    assert_eq!(all["pagination"]["total"], 5);
    assert_eq!(all["stats"]["active_members"], 2);
    assert_eq!(all["stats"]["trial_members"], 1);
    assert_eq!(all["stats"]["expired_members"], 1);
    assert_eq!(all["stats"]["other_members"], 1);
    assert_eq!(
        all["stats"]["whole_member_recorded_net_usd_order_value"]["net_cents"],
        13000
    );
    for (filter, total) in [
        ("status=active", 2),
        ("status=trial", 1),
        ("status=expired", 1),
        ("status=scheduled", 1),
        ("status=cancelled", 0),
    ] {
        let (status, body) = get(api, &format!("{path}?{filter}&per_page=1"), admin).await;
        assert_eq!(status, StatusCode::OK, "{body}");
        assert_eq!(body["pagination"]["total"], total);
        assert_eq!(body["stats"]["total_members"], total);
        if total > 0 {
            assert_eq!(body["members"].as_array().unwrap().len(), 1);
        }
    }
    let (_, expired) = get(api, &format!("{path}?status=expired"), admin).await;
    assert_eq!(expired["members"][0]["id"], users[3]);
    assert_eq!(expired["members"][0]["status"], "active");
    assert_eq!(expired["members"][0]["service_status"], "expired");
    let (_, page2) = get(
        api,
        &format!("{path}?search={prefix}&status=active&per_page=1&page=2"),
        admin,
    )
    .await;
    assert_eq!(page2["pagination"]["total"], 2);
    assert_eq!(page2["members"].as_array().unwrap().len(), 1);
    for bad in ["status=bad", "per_page=0", "page=0", "unsupported=true"] {
        let (status, body) = get(api, &format!("{path}?{bad}"), admin).await;
        assert!(status.is_client_error(), "{bad}: {body}");
    }
    assert_eq!(
        get(api, "/api/admin/members/service/9223372036854775807", admin)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    // A purchased product with the exact same numeric ID must remain a different namespace.
    sqlx::query("INSERT INTO products(id,name,slug,price,type) VALUES($1,$2,$2,10,'course')")
        .bind(plan)
        .bind(&prefix)
        .execute(pool)
        .await
        .unwrap();
    let order:i64=sqlx::query_scalar("INSERT INTO orders(user_id,order_number,status,total,currency) VALUES($1,$2,'paid',0,'USD') RETURNING id").bind(users[5]).bind(Uuid::new_v4().to_string()).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO order_items(order_id,product_id,name,quantity,unit_price,total) VALUES($1,$2,'Identity fixture',1,0,0)").bind(order).bind(plan).execute(pool).await.unwrap();
    let (_, plans) = get(
        api,
        &format!("/api/admin/members?plan_id={plan}&search={prefix}"),
        admin,
    )
    .await;
    assert_eq!(plans["pagination"]["total"], 5);
    let (_, products) = get(
        api,
        &format!("/api/admin/members?product_id={plan}&search={prefix}"),
        admin,
    )
    .await;
    assert_eq!(products["pagination"]["total"], 1);
    assert_eq!(products["members"][0]["id"], users[5]);
    let catalog_prefix = format!("{prefix}-catalog");
    let plan_ids:Vec<i64>=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) SELECT $1||'-'||lpad(n::text,3,'0'),$1||'-'||n,10,'monthly' FROM generate_series(1,61)n RETURNING id").bind(&catalog_prefix).fetch_all(pool).await.unwrap();
    let (_, catalog) = get(
        api,
        &format!(
            "/api/admin/members/services?search={catalog_prefix}&per_page=50&selected_id={plan}"
        ),
        admin,
    )
    .await;
    assert_eq!(catalog["pagination"]["total"], 61);
    assert_eq!(catalog["services"].as_array().unwrap().len(), 50);
    assert_eq!(catalog["selected"]["id"], plan);
    assert_eq!(catalog["selected"]["is_active"], false);
    assert_eq!(catalog["selected"]["members_count"], 3);
    let (_, catalog2) = get(
        api,
        &format!("/api/admin/members/services?search={catalog_prefix}&per_page=50&page=2"),
        admin,
    )
    .await;
    assert_eq!(catalog2["services"].as_array().unwrap().len(), 11);
    assert_eq!(
        get(
            api,
            "/api/admin/members/services?selected_id=9223372036854775807",
            admin
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    // Actual storage outage must cross neither backend nor proxy as successful zero facts.
    sqlx::query("ALTER VIEW admin_member_facts RENAME TO member_statistics_unavailable_fixture")
        .execute(pool)
        .await
        .unwrap();
    let failed = get(api, "/api/admin/members/stats", admin).await;
    sqlx::query("ALTER VIEW member_statistics_unavailable_fixture RENAME TO admin_member_facts")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::INTERNAL_SERVER_ERROR, "{}", failed.1);
    assert!(!failed.1.is_object() || failed.1.get("overview").is_none());
    assert_eq!(
        get(api, "/api/admin/members/stats", admin).await.0,
        StatusCode::OK
    );
    // Keep later global fixture proofs independent from this audience.
    sqlx::query("DELETE FROM orders WHERE user_id=ANY($1)")
        .bind(&users)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE id=ANY($1)")
        .bind(&users)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM membership_plans WHERE id=ANY($1)")
        .bind([vec![plan, other], plan_ids].concat())
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM products WHERE id=$1")
        .bind(plan)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Current Members statistics/service proof passed: cohorts, record counts, refunds/currency, identities, >50 catalog, filters/paging, real outage/recovery."
    );
}
