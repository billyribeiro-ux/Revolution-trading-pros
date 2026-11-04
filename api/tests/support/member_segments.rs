//! Rules, actual member data, authoring and authorization against a fresh schema.
use super::request;
use axum::{Router, http::StatusCode};
use revolution_api::services::member_segmentation as evaluation;
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = Some(tokens[0].as_str());
    let super_id:i64=sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES('segment-superadmin@example.test','Segment privileged fixture','not-a-login-password','super_admin',NOW()) RETURNING id").fetch_one(pool).await.unwrap();
    let super_token =
        revolution_api::utils::create_jwt(super_id, &std::env::var("JWT_SECRET").unwrap(), 1)
            .unwrap();
    let superadmin = Some(super_token.as_str());

    let root = "/api/admin/members/segments";
    let tag:i64=sqlx::query_scalar("INSERT INTO member_tags(name,slug,color) VALUES('Segment proof cohort','segment-proof-cohort','#123456') RETURNING id").fetch_one(pool).await.unwrap();
    let mut users = vec![];
    for index in 0..3 {
        let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,created_at) VALUES($1,$2,'disposable-no-login',NOW()-INTERVAL '40 days') RETURNING id").bind(format!("Segment fixture {index}")).bind(format!("segment-{index}@example.test")).fetch_one(pool).await.unwrap();
        sqlx::query("INSERT INTO user_member_tags(user_id,tag_id) VALUES($1,$2)")
            .bind(id)
            .bind(tag)
            .execute(pool)
            .await
            .unwrap();
        users.push(id);
    }
    for (user, total, refund, status, currency) in [
        (users[0], "500", "20", "completed", "USD"),
        (users[1], "150", "0", "paid", "USD"),
        (users[2], "9000", "0", "pending", "USD"),
        (users[2], "9000", "0", "paid", "EUR"),
    ] {
        sqlx::query("INSERT INTO orders(user_id,order_number,total,refund_amount,status,currency,completed_at) VALUES($1,$2,$3::text::numeric,$4::text::numeric,$5,$6,NOW()-INTERVAL '10 days')").bind(user).bind(uuid::Uuid::new_v4().to_string()).bind(total).bind(refund).bind(status).bind(currency).execute(pool).await.unwrap();
    }
    let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES('Segment proof plan','segment-proof-plan',20,'monthly') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,expires_at) VALUES($1,$2,'trialing',NOW()+INTERVAL '10 days')").bind(users[0]).bind(plan).execute(pool).await.unwrap();
    let product:i64=sqlx::query_scalar("INSERT INTO products(name,slug,price,type) VALUES('Segment proof product','segment-proof-product',25,'course') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO order_items(order_id,product_id,name,quantity,unit_price,total) SELECT id,$1,'Segment proof product',1,25,25 FROM orders WHERE user_id=$2 AND currency='USD'").bind(product).bind(users[0]).execute(pool).await.unwrap();
    let cohort = json!({"field":"tag","operator":"has","value":tag});
    let conditions = json!([cohort,{"field":"total_spent","operator":"gt","value":"400.00"}]);
    let payload = json!({"name":"Segment persistence fixture","slug":"segment-persistence-fixture","rules":conditions});
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(api, "POST", root, token, payload.clone()).await.0,
            status
        );
    }
    let created = request(api, "POST", root, admin, payload.clone()).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    assert_eq!(created.1["memberCount"], 1);
    assert_eq!(created.1["conditions"], conditions);
    let id = created.1["id"].as_i64().unwrap();
    let path = format!("{root}/{id}");
    let cold = super::app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    assert_eq!(
        request(&cold, "GET", &path, admin, Value::Null).await.1["member_count"],
        1
    );
    let listed = request(
        api,
        "GET",
        &format!("/api/admin/members?segment={id}&per_page=1"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(listed.0, StatusCode::OK, "{listed:?}");
    assert_eq!(listed.1["pagination"]["total"], 1);
    assert_eq!(listed.1["members"][0]["id"], users[0]);
    assert_eq!(listed.1["members"][0]["total_spent"].as_f64(), Some(480.0));
    assert_eq!(listed.1["members"][0]["status"], "trial");
    for (field, operator, value, expected) in [
        ("days_since_signup", "gt", json!(30), 3),
        ("days_since_last_order", "gt", json!(5), 3),
        ("subscription_status", "eq", json!("trialing"), 1),
        ("email_engagement", "eq", json!("engaged"), 0),
        ("product", "has", json!(product), 1),
    ] {
        let rules = evaluation::rules(Some(
            json!([cohort,{"field":field,"operator":operator,"value":value}]),
        ))
        .unwrap();
        assert_eq!(
            evaluation::count(pool, &rules).await.unwrap(),
            expected,
            "{field}"
        );
    }
    for invalid in [
        json!([{ "field":"email);DROP TABLE users;--","operator":"eq","value":"x"}]),
        json!([{ "field":"total_spent","operator":"eq","value":"NaN"}]),
        json!([{ "field":"status","operator":"gt","value":"active"}]),
        json!([{ "field":"tag","operator":"has","value":"x\0y"}]),
    ] {
        let response = request(api, "PUT", &path, admin, json!({"rules":invalid})).await;
        assert_eq!(response.0, StatusCode::BAD_REQUEST, "{response:?}");
    }
    let injection = evaluation::rules(Some(
        json!([{"field":"tag","operator":"has","value":"' OR true --"}]),
    ))
    .unwrap();
    assert_eq!(evaluation::count(pool, &injection).await.unwrap(), 0);
    let static_id:i64=sqlx::query_scalar("INSERT INTO member_segments(name,slug,is_dynamic) VALUES('Static proof','static-segment-proof',false) RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_member_segments(user_id,segment_id) VALUES($1,$2)")
        .bind(users[1])
        .bind(static_id)
        .execute(pool)
        .await
        .unwrap();
    let static_row = request(
        api,
        "GET",
        &format!("{root}/{static_id}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(static_row.0, StatusCode::OK, "{static_row:?}");
    assert_eq!(static_row.1["memberCount"], 1);
    assert_eq!(static_row.1["type"], "static");
    let by_tag=request(api,"GET",&format!("/api/admin/members?tag={tag}&spending_tier=medium&sort_by=total_spent&sort_dir=asc&per_page=1&page=2"),admin,Value::Null).await;
    assert_eq!(by_tag.0, StatusCode::OK, "{by_tag:?}");
    assert_eq!(by_tag.1["pagination"]["total"], 2);
    assert_eq!(by_tag.1["members"][0]["id"], users[0]);
    assert_eq!(
        request(
            api,
            "GET",
            "/api/admin/members?date_from=2026-02-30",
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let updated = request(
        api,
        "PUT",
        &path,
        admin,
        json!({"name":"Updated persisted fixture","rules":[cohort]}),
    )
    .await;
    assert_eq!(updated.0, StatusCode::OK, "{updated:?}");
    assert_eq!(updated.1["memberCount"], 3);
    super::member_segment_analytics_proof::proof(api, pool, tokens, id, users[0]).await;
    let system: i64 = sqlx::query_scalar("SELECT id FROM member_segments WHERE slug='trial-users'")
        .fetch_one(pool)
        .await
        .unwrap();
    assert_eq!(
        request(
            api,
            "PUT",
            &format!("{root}/{system}"),
            admin,
            json!({"name":"Forbidden"})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{root}/{system}"),
            superadmin,
            Value::Null
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO segment_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(api, "PUT", &path, admin, json!({"name":"Should roll back"})).await;
    sqlx::query("ALTER TABLE segment_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await.1["name"],
        "Updated persisted fixture"
    );
    assert_eq!(
        request(api, "DELETE", &path, Some(&tokens[1]), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(api, "DELETE", &path, admin, Value::Null).await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(api, "DELETE", &path, superadmin, Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await.0,
        StatusCode::NOT_FOUND
    );
}
