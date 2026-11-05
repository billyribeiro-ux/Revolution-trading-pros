//! Persisted tags, idempotent assignments, recorded filter applies and rollback.
use super::request;
use axum::{Router, http::StatusCode};
use serde_json::{Value, json};
use sqlx::PgPool;

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = Some(tokens[0].as_str());
    let super_id: i64 =
        sqlx::query_scalar("SELECT id FROM users WHERE email='segment-superadmin@example.test'")
            .fetch_one(pool)
            .await
            .unwrap();
    let token =
        revolution_api::utils::create_jwt(super_id, &std::env::var("JWT_SECRET").unwrap(), 1)
            .unwrap();
    let superadmin = Some(token.as_str());
    let mut users = vec![];
    for suffix in ["A", "B", "C"] {
        let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash) VALUES($1,$2,'not-a-login-password') RETURNING id").bind(format!("Config member {suffix}")).bind(format!("member-config-{suffix}@example.test")).fetch_one(pool).await.unwrap();
        users.push(id);
    }
    let tag_root = "/api/admin/members/tags";
    for bad in [
        json!({"name":"","color":"#123456"}),
        json!({"name":"Invalid color","color":"red"}),
        json!({"name":"Bad\0name"}),
    ] {
        assert_eq!(
            request(api, "POST", tag_root, admin, bad).await.0,
            StatusCode::BAD_REQUEST
        );
    }
    let mut tags = vec![];
    for suffix in ["A", "B"] {
        let created = request(
            api,
            "POST",
            tag_root,
            admin,
            json!({"name":format!("Configuration tag {suffix}"),"color":"#123456"}),
        )
        .await;
        assert_eq!(created.0, StatusCode::OK, "{created:?}");
        tags.push(created.1["id"].as_i64().unwrap());
        assert_eq!(created.1["member_count"], 0);
    }
    let assign = format!("{tag_root}/bulk-assign");
    let payload = json!({"user_ids":[users[0],users[1],users[0]],"tag_ids":tags});
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        assert_eq!(
            request(api, "POST", &assign, token, payload.clone())
                .await
                .0,
            status
        );
    }
    let first = request(api, "POST", &assign, admin, payload.clone()).await;
    assert_eq!(first.0, StatusCode::OK, "{first:?}");
    assert_eq!(first.1["assigned"], 4);
    assert_eq!(first.1["requested"], 4);
    let repeated = request(api, "POST", &assign, admin, payload).await;
    assert_eq!(repeated.0, StatusCode::OK, "{repeated:?}");
    assert_eq!(repeated.1["assigned"], 0);
    assert_eq!(repeated.1["already_assigned"], 4);
    for invalid in [
        json!({"user_ids":[0],"tag_ids":tags}),
        json!({"user_ids":users,"tag_ids":[]}),
        json!({"user_ids":vec![users[0];1001],"tag_ids":tags}),
    ] {
        assert_eq!(
            request(api, "POST", &assign, admin, invalid).await.0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{tag_root}/assign"),
            admin,
            json!({"user_id":999999,"tag_id":tags[0]})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let tag_path = format!("{tag_root}/{}", tags[0]);
    assert_eq!(
        request(api, "GET", &tag_path, admin, Value::Null).await.1["member_count"],
        2
    );
    sqlx::query("UPDATE users SET deleted_at=NOW() WHERE id=$1")
        .bind(users[1])
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        request(api, "GET", &tag_path, admin, Value::Null).await.1["member_count"],
        1,
        "Read counts use actual live members, not stale cached metadata"
    );
    sqlx::query("UPDATE users SET deleted_at=NULL WHERE id=$1")
        .bind(users[1])
        .execute(pool)
        .await
        .unwrap();
    let listing = request(
        api,
        "GET",
        &format!("{tag_root}?search=Configuration%20tag&per_page=1&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(listing.0, StatusCode::OK, "{listing:?}");
    assert_eq!(listing.1["meta"]["total"], 2);
    assert_eq!(listing.1["tags"].as_array().unwrap().len(), 1);
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO member_config_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(
        api,
        "POST",
        &format!("{tag_root}/assign"),
        admin,
        json!({"user_id":users[2],"tag_id":tags[0]}),
    )
    .await;
    let removal = request(
        api,
        "DELETE",
        &format!("{tag_root}/unassign"),
        admin,
        json!({"user_id":users[0],"tag_id":tags[0]}),
    )
    .await;
    sqlx::query("ALTER TABLE member_config_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(removal.0, StatusCode::SERVICE_UNAVAILABLE);
    let extra: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM user_member_tags WHERE user_id=$1 AND tag_id=$2)",
    )
    .bind(users[2])
    .bind(tags[0])
    .fetch_one(pool)
    .await
    .unwrap();
    assert!(!extra);
    assert_eq!(
        request(api, "GET", &tag_path, admin, Value::Null).await.1["member_count"],
        2
    );
    let root = "/api/admin/members/filters";
    let created=request(api,"POST",root,admin,json!({"name":"Configuration saved filter","filters":{"tag":tags[1].to_string(),"search":"Config member","spending_tier":"low"}})).await;
    assert_eq!(created.0, StatusCode::OK, "{created:?}");
    let id = created.1["id"].as_i64().unwrap();
    let path = format!("{root}/{id}");
    assert_eq!(created.1["filters"]["tag"], tags[1]);
    assert_eq!(created.1["usage_count"], 0);
    assert!(created.1["last_applied_at"].is_null());
    for bad in [
        json!({"unknown":"x"}),
        json!({"date_from":"2026-02-30"}),
        json!({"date_from":"2026-05-01","date_to":"2026-04-01"}),
        json!({"tag":999999}),
        json!({"page":0}),
    ] {
        assert_eq!(
            request(api, "PUT", &path, admin, json!({"filters":bad}))
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
    }
    assert_eq!(
        request(api, "PUT", &path, admin, json!({"usage_count":999}))
            .await
            .0,
        StatusCode::UNPROCESSABLE_ENTITY
    );
    let applied = request(api, "POST", &format!("{path}/apply"), admin, Value::Null).await;
    assert_eq!(applied.0, StatusCode::OK, "{applied:?}");
    assert_eq!(applied.1["usage_count"], 1);
    assert_eq!(applied.1["applied"], true);
    let members = request(
        api,
        "GET",
        &format!(
            "/api/admin/members?tag={}&spending_tier=low&search=Config%20member",
            tags[1]
        ),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(members.0, StatusCode::OK, "{members:?}");
    assert_eq!(members.1["pagination"]["total"], 2);
    let mut copies = vec![];
    for _ in 0..2 {
        let copy = request(
            api,
            "POST",
            &format!("{path}/duplicate"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(copy.0, StatusCode::OK, "{copy:?}");
        let copied = copy.1["id"].as_i64().unwrap();
        assert_ne!(copied, id);
        assert!(!copies.contains(&copied));
        copies.push(copied);
        assert_eq!(copy.1["usage_count"], 0);
        assert_eq!(copy.1["filters"], created.1["filters"]);
    }
    assert_eq!(
        request(
            api,
            "PUT",
            &path,
            admin,
            json!({"name":"Updated configuration filter"})
        )
        .await
        .0,
        StatusCode::OK
    );
    let cold = super::app(revolution_api::db::Database { pool: pool.clone() }, None).await;
    assert_eq!(
        request(&cold, "GET", &path, admin, Value::Null).await.1["name"],
        "Updated configuration filter"
    );
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO member_config_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let failed = request(api, "POST", &format!("{path}/apply"), admin, Value::Null).await;
    sqlx::query("ALTER TABLE member_config_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await.1["usage_count"],
        1
    );
    // A single-connection installation must not deadlock on reference validation inside a transaction.
    let single = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("DATABASE_URL").unwrap())
        .await
        .unwrap();
    let one = super::app(
        revolution_api::db::Database {
            pool: single.clone(),
        },
        None,
    )
    .await;
    let applied = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        request(&one, "POST", &format!("{path}/apply"), admin, Value::Null),
    )
    .await
    .unwrap();
    assert_eq!(applied.0, StatusCode::OK, "{applied:?}");
    assert_eq!(applied.1["usage_count"], 2);
    drop(one);
    single.close().await;
    assert_eq!(
        request(api, "DELETE", &path, admin, Value::Null).await.0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(api, "DELETE", &tag_path, admin, Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    let deleted = request(api, "DELETE", &tag_path, superadmin, Value::Null).await;
    assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
    assert_eq!(deleted.1["removed_assignments"], 2);
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{tag_root}/{}", tags[1]),
            superadmin,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(api, "POST", &format!("{path}/apply"), admin, Value::Null)
            .await
            .0,
        StatusCode::BAD_REQUEST,
        "A deleted reference never broadens a saved audience"
    );
    assert_eq!(
        request(api, "GET", &path, admin, Value::Null).await.1["usage_count"],
        2
    );
    for selected in copies.into_iter().chain(std::iter::once(id)) {
        assert_eq!(
            request(
                api,
                "DELETE",
                &format!("{root}/{selected}"),
                superadmin,
                Value::Null
            )
            .await
            .0,
            StatusCode::OK
        );
    }
}
