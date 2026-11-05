//! Factual member profiles and journals: persisted evidence, aliases and atomic notes.
use super::{Adapter, request};
use axum::{Router, http::StatusCode};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use tower::ServiceExt;
use uuid::Uuid;

const ROOTS: [&str; 2] = ["/api/admin/members", "/api/admin/member-management"];
const PANELS: [&str; 6] = [
    "notes",
    "activity",
    "emails",
    "memberships",
    "orders",
    "timeline",
];
struct Fixture {
    users: Vec<i64>,
    plan: i64,
    tag: i64,
    admin: i64,
    email: String,
    prefix: String,
    note_ids: Vec<i64>,
    email_ids: Vec<i64>,
}
fn requests(adapter: &Adapter) -> usize {
    let held = adapter.requests.lock().unwrap();
    held.len()
}
async fn ok(api: &Router, path: &str, token: &str) -> Value {
    let result = request(api, "GET", path, Some(token), Value::Null).await;
    assert_eq!(result.0, StatusCode::OK, "{path}: {result:?}");
    result.1
}
async fn fixture(pool: &PgPool) -> Fixture {
    let prefix = format!("member-profile-{}", Uuid::new_v4());
    let email = format!("{prefix}@example.test");
    let mut users = Vec::new();
    for (suffix, name) in [
        ("", "Recorded <Profile & Name>"),
        ("-other", "Other member"),
        ("-wide", "Wide recorded date"),
        ("-positive-infinity", "Unbounded future recorded date"),
        ("-negative-infinity", "Unbounded past recorded date"),
    ] {
        let id:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,created_at,updated_at,is_active,mfa_enabled,metadata) VALUES($1,$2,'not-a-login-password','user',(NOW() AT TIME ZONE 'UTC')-INTERVAL '3 days',(NOW() AT TIME ZONE 'UTC')-INTERVAL '1 day',NULL,NULL,$3) RETURNING id")
            .bind(name).bind(format!("{prefix}{suffix}@example.test")).bind(json!({"first_name":"Recorded","last_name":"Member"})).fetch_one(pool).await.unwrap();
        users.push(id);
    }
    sqlx::query(
        "UPDATE users SET created_at=(NOW() AT TIME ZONE 'UTC')+INTERVAL '1 day' WHERE id=$1",
    )
    .bind(users[1])
    .execute(pool)
    .await
    .unwrap();
    sqlx::query("UPDATE users SET created_at=TIMESTAMP '12000-01-01 00:00:00' WHERE id=$1")
        .bind(users[2])
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE users SET created_at=TIMESTAMP 'infinity',metadata=NULL WHERE id=$1")
        .bind(users[3])
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE users SET created_at=TIMESTAMP '-infinity',metadata=NULL WHERE id=$1")
        .bind(users[4])
        .execute(pool)
        .await
        .unwrap();
    let admin: i64 =
        sqlx::query_scalar("SELECT id FROM users WHERE email='admin@email-completion.test'")
            .fetch_one(pool)
            .await
            .unwrap();
    sqlx::query("INSERT INTO user_status(user_id,status,status_reason,ban_reason,ban_expires_at) VALUES($1,'suspended','Recorded restriction','Recorded reason',NOW()+INTERVAL '1 day')")
        .bind(users[0]).execute(pool).await.unwrap();
    let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle,interval_count,is_active) VALUES($1,$1,999.99,'monthly',1,false) RETURNING id")
        .bind(&prefix).fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,current_period_start,current_period_end,created_at,updated_at) VALUES($1,$2,'active',(NOW() AT TIME ZONE 'UTC')-INTERVAL '1 day',(NOW() AT TIME ZONE 'UTC')+INTERVAL '30 days',(NOW() AT TIME ZONE 'UTC')-INTERVAL '1 day',(NOW() AT TIME ZONE 'UTC')+INTERVAL '10 days',(NOW() AT TIME ZONE 'UTC'),NULL)")
        .bind(users[0]).bind(plan).execute(pool).await.unwrap();
    // Cancelled history is legal alongside one current active row under 063/070.
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,cancelled_at,created_at,updated_at,deleted_at) SELECT $1,$2,'cancelled',TIMESTAMP '2001-01-01',TIMESTAMP '2001-02-01',TIMESTAMP '2001-01-15',CASE WHEN n=1 THEN NULL ELSE TIMESTAMP '2001-01-01' END,NULL,CASE WHEN n=2 THEN NOW() ELSE NULL END FROM generate_series(1,31)n")
        .bind(users[0]).bind(plan).execute(pool).await.unwrap();
    let order_prefix = Uuid::new_v4().to_string();
    sqlx::query("INSERT INTO orders(user_id,order_number,status,currency,total,refund_amount,created_at,updated_at,completed_at,refunded_at) SELECT $1,$2||'-'||n,CASE n WHEN 1 THEN 'partial_refund' WHEN 2 THEN 'partially_refunded' WHEN 3 THEN 'refunded' WHEN 6 THEN 'pending' ELSE 'paid' END,CASE n WHEN 4 THEN 'EUR' WHEN 5 THEN NULL ELSE 'USD' END,CASE n WHEN 1 THEN 100 WHEN 2 THEN 50 WHEN 3 THEN 20 WHEN 4 THEN 123 WHEN 5 THEN 55 WHEN 6 THEN 500 ELSE 0 END,CASE n WHEN 1 THEN 25 WHEN 2 THEN 5 WHEN 3 THEN 20 ELSE 0 END,CASE WHEN n=1 THEN NULL ELSE TIMESTAMP '2001-01-01' END,NULL,CASE WHEN n=6 THEN NULL ELSE TIMESTAMP '2001-01-02' END,CASE WHEN n<=3 THEN TIMESTAMP '2001-01-03' ELSE NULL END FROM generate_series(1,31)n")
        .bind(users[0]).bind(&order_prefix).execute(pool).await.unwrap();
    let note_ids:Vec<i64>=sqlx::query_scalar("INSERT INTO member_notes(user_id,note,created_by,created_at,updated_at) SELECT $1,$2||' legacy note '||n,$3,CASE WHEN n=1 THEN NULL ELSE TIMESTAMPTZ '2001-01-01 00:00:00+00' END,NULL FROM generate_series(1,31)n RETURNING id")
        .bind(users[0]).bind(&prefix).bind(admin).fetch_all(pool).await.unwrap();
    sqlx::query("INSERT INTO user_activity_log(user_id,action,details,created_at) SELECT $1,CASE WHEN n%2=0 THEN 'profile_alpha' ELSE 'profile_beta' END,jsonb_build_object('description','Recorded activity '||n,'ordinal',n),CASE WHEN n=1 THEN NULL ELSE TIMESTAMPTZ '2001-01-01 00:00:00+00' END FROM generate_series(1,31)n")
        .bind(users[0]).execute(pool).await.unwrap();
    // Complete/incomplete recorded receipt states are distinct test fixtures; no read may resubmit.
    let mut email_ids:Vec<i64>=sqlx::query_scalar("INSERT INTO email_logs(user_id,email,to_email,subject,status,provider,provider_message_id,sent_at,queued_at,created_at,opened_at,clicked_at,error_message) SELECT $1,$2,CASE WHEN n=1 THEN NULL ELSE $2 END,$3||' receipt '||n,CASE n WHEN 4 THEN 'failed' WHEN 5 THEN 'unavailable' WHEN 6 THEN 'queued' WHEN 7 THEN 'delivered' WHEN 8 THEN 'clicked' ELSE 'sent' END,'fixture',CASE WHEN n=2 THEN '   ' WHEN n IN(3,8) THEN $3||'-recorded-'||n ELSE NULL END,CASE WHEN n IN(2,3,8) THEN TIMESTAMPTZ '2001-01-02 00:00:00+00' ELSE NULL END,TIMESTAMPTZ '2001-01-01 00:00:00+00',CASE WHEN n=11 THEN NULL WHEN n=12 THEN TIMESTAMP '12000-01-01' ELSE TIMESTAMP '2001-01-01' END,CASE WHEN n=8 THEN TIMESTAMP '2001-01-03' ELSE NULL END,CASE WHEN n=8 THEN TIMESTAMP '2001-01-03' ELSE NULL END,CASE WHEN n=4 THEN 'Recorded rejection' ELSE NULL END FROM generate_series(1,31)n RETURNING id")
        .bind(users[0]).bind(&email).bind(&prefix).fetch_all(pool).await.unwrap();
    let excluded:Vec<i64>=sqlx::query_scalar("INSERT INTO email_logs(user_id,email,to_email,subject,status) VALUES(NULL,$1,$1,'Unlinked same address must stay absent','sent'),($2,$1,$1,'Another recorded user must stay absent','sent') RETURNING id")
        .bind(&email).bind(users[1]).fetch_all(pool).await.unwrap();
    email_ids.extend(excluded);
    let tag: i64 =
        sqlx::query_scalar("INSERT INTO member_tags(name,slug) VALUES($1,$1) RETURNING id")
            .bind(&prefix)
            .fetch_one(pool)
            .await
            .unwrap();
    sqlx::query("INSERT INTO user_member_tags(user_id,tag_id) VALUES($1,$2)")
        .bind(users[0])
        .bind(tag)
        .execute(pool)
        .await
        .unwrap();
    Fixture {
        users,
        plan,
        tag,
        admin,
        email,
        prefix,
        note_ids,
        email_ids,
    }
}
async fn auth_and_missing(api: &Router, tokens: &[String], member: i64) {
    for base in ROOTS {
        let root = format!("{base}/{member}");
        for (method, path, body) in std::iter::once(("GET", root.clone(), Value::Null))
            .chain(
                PANELS
                    .into_iter()
                    .map(|panel| ("GET", format!("{root}/{panel}"), Value::Null)),
            )
            .chain([
                (
                    "POST",
                    format!("{root}/notes"),
                    json!({"content":"Denied note"}),
                ),
                (
                    "DELETE",
                    format!("{root}/notes/9223372036854775807"),
                    Value::Null,
                ),
            ])
        {
            for (token, status) in [
                (None, StatusCode::UNAUTHORIZED),
                (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
            ] {
                let response = request(api, method, &path, token, body.clone()).await;
                assert_eq!(response.0, status, "{method} {path}: {response:?}");
            }
        }
        for suffix in std::iter::once(String::new())
            .chain(PANELS.into_iter().map(|panel| format!("/{panel}")))
        {
            let path = format!("{base}/9223372036854775807{suffix}");
            let response = request(api, "GET", &path, Some(&tokens[0]), Value::Null).await;
            assert_eq!(response.0, StatusCode::NOT_FOUND, "{path}: {response:?}");
        }
    }
}
async fn current_profile(api: &Router, token: &str, f: &Fixture) {
    let first = ok(api, &format!("{}/{id}", ROOTS[0], id = f.users[0]), token).await;
    let second = ok(api, &format!("{}/{id}", ROOTS[1], id = f.users[0]), token).await;
    for body in [&first, &second] {
        assert_eq!(body["schema_version"], 1);
        assert_eq!(body["metrics_scope"], "member_profile");
        assert_eq!(body["member_id"], f.users[0]);
        let evaluated =
            chrono::DateTime::parse_from_rfc3339(body["evaluated_at"].as_str().unwrap()).unwrap();
        assert!((chrono::Utc::now().timestamp() - evaluated.timestamp()).abs() < 10);
        let member = &body["member"];
        assert_eq!(member["email"], f.email);
        assert_eq!(member["status"], "active");
        assert_eq!(member["active_subscriptions_count"], 1);
        assert_eq!(member["account"]["recorded_restriction"], "suspended");
        assert!(member["account"]["is_active"].is_null());
        assert!(member["account"]["mfa_enabled"].is_null());
        assert!(member["created_at"].is_string());
        assert!(member["updated_at"].is_string());
        assert_eq!(member["recorded_age_days"], 3);
        assert!(member["avatar"].is_null());
        assert_eq!(member["email_verified"], false);
        assert_eq!(member["tag_records"][0]["id"], f.tag);
        assert_eq!(member["tags"], json!([f.prefix]));
        assert!(
            member["account"]["banned_until"]
                .as_str()
                .unwrap()
                .ends_with('Z')
        );
        assert_eq!(body["recorded_order_value"]["currency"], "USD");
        assert_eq!(body["recorded_order_value"]["net_cents"], 12000);
        assert_eq!(body["recorded_order_value"]["order_count"], 28);
        assert_eq!(body["recorded_order_value"]["excluded_non_usd_orders"], 1);
        assert_eq!(
            body["recorded_order_value"]["excluded_unknown_currency_orders"],
            1
        );
        assert!(body["engagement"]["score"].is_null());
        assert_eq!(
            body["engagement"]["observations"]["linked_email_receipts"],
            31
        );
        assert_eq!(body["engagement"]["observations"]["stored_open_signals"], 1);
        assert_eq!(
            body["engagement"]["observations"]["stored_click_signals"],
            1
        );
        for (name, total) in [
            ("memberships", 32),
            ("orders", 31),
            ("notes", 31),
            ("activity", 31),
            ("emails", 31),
        ] {
            assert_eq!(body[name]["pagination"]["total"], total, "{name}: {body}");
            assert_eq!(body[name][name].as_array().unwrap().len(), 25);
        }
    }
    for key in [
        "member",
        "recorded_order_value",
        "memberships",
        "orders",
        "notes",
        "activity",
        "emails",
        "timeline",
        "engagement",
        "timestamp_basis",
        "billing_coverage",
    ] {
        let mut left = first[key].clone();
        let mut right = second[key].clone();
        if let Some(object) = left.as_object_mut() {
            object.remove("evaluated_at");
        }
        if let Some(object) = right.as_object_mut() {
            object.remove("evaluated_at");
        }
        assert_eq!(
            left, right,
            "Both existing route families read the same canonical {key}"
        );
    }
    for user in &f.users[1..] {
        for base in ROOTS {
            let body = ok(api, &format!("{base}/{user}"), token).await;
            assert!(body["member"]["recorded_age_days"].is_null());
            assert!(body["member"]["created_at"].is_string());
            assert!(body["engagement"]["score"].is_null());
            if *user == f.users[3] || *user == f.users[4] {
                assert!(body["member"]["first_name"].is_null());
                assert!(body["member"]["last_name"].is_null());
            }
        }
    }
}
async fn all_rows(api: &Router, path: &str, token: &str, key: &str) -> Vec<Value> {
    let first = ok(api, &format!("{path}?per_page=100"), token).await;
    let total = first["pagination"]["total"].as_i64().unwrap();
    let pages = first["pagination"]["last_page"].as_i64().unwrap();
    let mut rows = first[key].as_array().unwrap().clone();
    for page in 2..=pages {
        let next = ok(api, &format!("{path}?per_page=100&page={page}"), token).await;
        assert_eq!(next["pagination"]["total"], total);
        rows.extend(next[key].as_array().unwrap().iter().cloned());
    }
    assert_eq!(rows.len() as i64, total);
    let ids = rows
        .iter()
        .map(|row| row["id"].to_string())
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(
        ids.len(),
        rows.len(),
        "Stable pages contain each recorded identity once: {path}"
    );
    rows
}
async fn exported(
    api: &Router,
    path: &str,
    token: Option<&str>,
) -> (StatusCode, axum::http::HeaderMap, Vec<u8>) {
    let mut request = axum::http::Request::builder().uri(path);
    if let Some(token) = token {
        request = request.header("authorization", format!("Bearer {token}"));
    }
    let response = api
        .clone()
        .oneshot(request.body(axum::body::Body::empty()).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let bytes = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (status, headers, bytes)
}
fn csv_rows(bytes: &[u8]) -> Vec<csv::StringRecord> {
    assert!(bytes.starts_with(&[0xEF, 0xBB, 0xBF]));
    let mut reader = csv::Reader::from_reader(&bytes[3..]);
    assert_eq!(reader.headers().unwrap().len(), 12);
    reader.records().map(Result::unwrap).collect()
}
async fn order_export(api: &Router, pool: &PgPool, token: &str, ordinary: &str, f: &Fixture) {
    let source = all_rows(
        api,
        &format!("{}/{}/orders", ROOTS[0], f.users[0]),
        token,
        "orders",
    )
    .await;
    for base in ROOTS {
        let path = format!("{base}/{}/orders/export?format=csv", f.users[0]);
        assert_eq!(exported(api, &path, None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(
            exported(api, &path, Some(ordinary)).await.0,
            StatusCode::FORBIDDEN
        );
        let (status, headers, bytes) = exported(api, &path, Some(token)).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "{}",
            String::from_utf8_lossy(&bytes)
        );
        assert_eq!(headers["x-export-member-id"], f.users[0].to_string());
        assert_eq!(headers["x-export-row-count"], "31");
        assert_eq!(headers["x-export-scope"], "all_recorded_member_orders");
        assert_eq!(
            headers["x-export-sha256"],
            hex::encode(Sha256::digest(&bytes))
        );
        let actual = csv_rows(&bytes);
        assert_eq!(actual.len(), source.len());
        for (row, expected) in actual.iter().zip(&source) {
            assert_eq!(
                row.get(0).unwrap(),
                expected["id"].as_i64().unwrap().to_string()
            );
            assert_eq!(row.get(1).unwrap(), f.users[0].to_string());
            for (index, key) in [
                (2, "order_number"),
                (3, "status"),
                (4, "currency"),
                (5, "total_units"),
                (6, "refund_units"),
                (7, "net_recorded_units"),
                (8, "created_at"),
                (9, "completed_at"),
                (10, "refunded_at"),
            ] {
                assert_eq!(
                    row.get(index).unwrap(),
                    expected[key].as_str().unwrap_or(""),
                    "{key}"
                );
            }
        }
        let bad = exported(
            api,
            &format!("{base}/{}/orders/export?page=1", f.users[0]),
            Some(token),
        )
        .await;
        assert_eq!(bad.0, StatusCode::BAD_REQUEST);
        assert!(!bad.1.contains_key("content-disposition"));
    }
    let formula = format!("=owned-{}", Uuid::new_v4());
    let id:i64=sqlx::query_scalar("UPDATE orders SET order_number=$2 WHERE id=(SELECT id FROM orders WHERE user_id=$1 ORDER BY id LIMIT 1) RETURNING id").bind(f.users[0]).bind(&formula).fetch_one(pool).await.unwrap();
    let (_, _, bytes) = exported(
        api,
        &format!("{}/{}/orders/export", ROOTS[1], f.users[0]),
        Some(token),
    )
    .await;
    let rows = csv_rows(&bytes);
    let escaped = rows
        .iter()
        .find(|row| row.get(0).unwrap() == id.to_string())
        .unwrap();
    assert_eq!(escaped.get(2).unwrap(), format!("'{formula}"));
    let audit_before:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_operational_logs WHERE context->>'operation'='member_order_export'").fetch_one(pool).await.unwrap();
    rename_fixture_table(pool, 0, true).await;
    let failed = exported(
        api,
        &format!("{}/{}/orders/export", ROOTS[0], f.users[0]),
        Some(token),
    )
    .await;
    rename_fixture_table(pool, 0, false).await;
    assert_eq!(failed.0, StatusCode::SERVICE_UNAVAILABLE);
    assert!(!failed.1.contains_key("content-disposition"));
    let audit_after:i64=sqlx::query_scalar("SELECT COUNT(*) FROM crm_operational_logs WHERE context->>'operation'='member_order_export'").fetch_one(pool).await.unwrap();
    assert_eq!(audit_before, audit_after);
    let path = format!("{}/{}/orders/export", ROOTS[0], f.users[1]);
    let empty = exported(api, &path, Some(token)).await;
    assert_eq!(empty.0, StatusCode::OK);
    assert_eq!(empty.1["x-export-row-count"], "0");
    assert!(csv_rows(&empty.2).is_empty());
    let prefix = format!("o-{}", Uuid::new_v4());
    sqlx::query("INSERT INTO orders(user_id,order_number,status) SELECT $1,$2||'-'||n,'pending' FROM generate_series(1,10000)n").bind(f.users[1]).bind(&prefix).execute(pool).await.unwrap();
    let bounded = exported(api, &path, Some(token)).await;
    assert_eq!(bounded.0, StatusCode::OK);
    assert_eq!(bounded.1["x-export-row-count"], "10000");
    assert_eq!(csv_rows(&bounded.2).len(), 10000);
    sqlx::query(
        "INSERT INTO orders(user_id,order_number,status) VALUES($1,$2||'-10001','pending')",
    )
    .bind(f.users[1])
    .bind(&prefix)
    .execute(pool)
    .await
    .unwrap();
    let oversized = exported(api, &path, Some(token)).await;
    assert_eq!(oversized.0, StatusCode::BAD_REQUEST);
    assert!(!oversized.1.contains_key("content-disposition"));
    assert_eq!(
        sqlx::query("DELETE FROM orders WHERE user_id=$1 AND order_number LIKE $2||'-%'")
            .bind(f.users[1])
            .bind(&prefix)
            .execute(pool)
            .await
            .unwrap()
            .rows_affected(),
        10001
    );
}
async fn history_panels(api: &Router, token: &str, f: &Fixture) {
    for base in ROOTS {
        for (panel, total) in [
            ("notes", 31),
            ("activity", 31),
            ("emails", 31),
            ("memberships", 32),
            ("orders", 31),
            ("timeline", 156),
        ] {
            let path = format!("{base}/{}/{panel}", f.users[0]);
            let key = if panel == "timeline" { "data" } else { panel };
            let first = ok(api, &format!("{path}?per_page=10"), token).await;
            let second = ok(api, &format!("{path}?per_page=10&page=2"), token).await;
            assert_eq!(first["schema_version"], 1);
            assert_eq!(first["member_id"], f.users[0]);
            assert_eq!(first["pagination"]["total"], total, "{path}: {first}");
            assert_eq!(second["pagination"]["total"], total);
            assert_eq!(first[key].as_array().unwrap().len(), 10);
            assert_eq!(second[key].as_array().unwrap().len(), 10);
            let repeated = ok(api, &format!("{path}?per_page=10"), token).await;
            assert_eq!(
                repeated[key], first[key],
                "Tied recorded dates have a stable identity tie-breaker"
            );
            let ids = first[key]
                .as_array()
                .unwrap()
                .iter()
                .map(|row| row["id"].to_string())
                .collect::<std::collections::BTreeSet<_>>();
            assert!(
                second[key]
                    .as_array()
                    .unwrap()
                    .iter()
                    .all(|row| !ids.contains(&row["id"].to_string()))
            );
            let all = all_rows(api, &path, token, key).await;
            if panel == "timeline" {
                assert!(
                    all.iter().any(|row| row["date"].is_null()),
                    "Missing recorded times remain missing"
                );
                assert!(
                    all.iter()
                        .filter(|row| row["id"].as_str().unwrap().starts_with("order:"))
                        .all(|row| row["title"] == "Order record created")
                );
            } else {
                assert!(
                    all.iter().any(|row| row["created_at"].is_null()),
                    "Unknown legacy dates are not replaced with NOW: {panel}"
                );
            }
            for bad in [
                "page=0",
                "per_page=0",
                "per_page=101",
                "page=9223372036854775807&per_page=100",
                "unsupported=true",
            ] {
                let result = request(
                    api,
                    "GET",
                    &format!("{path}?{bad}"),
                    Some(token),
                    Value::Null,
                )
                .await;
                assert_eq!(
                    result.0,
                    StatusCode::BAD_REQUEST,
                    "Invalid bounded history query {path}?{bad}: {result:?}"
                );
            }
        }
        let filtered = ok(
            api,
            &format!(
                "{base}/{}/activity?action=profile_alpha&per_page=10&page=2",
                f.users[0]
            ),
            token,
        )
        .await;
        assert_eq!(filtered["pagination"]["total"], 15);
        assert_eq!(filtered["activity"].as_array().unwrap().len(), 5);
        assert!(
            filtered["activity"]
                .as_array()
                .unwrap()
                .iter()
                .all(|row| row["action"] == "profile_alpha")
        );
        let empty = ok(
            api,
            &format!("{base}/{}/activity?action=no_recorded_action", f.users[0]),
            token,
        )
        .await;
        assert_eq!(empty["pagination"]["total"], 0);
        assert_eq!(empty["pagination"]["last_page"], 1);
        assert_eq!(empty["activity"], json!([]));
        for panel in ["notes", "emails", "memberships", "orders", "timeline"] {
            let result = request(
                api,
                "GET",
                &format!("{base}/{}/{panel}?action=profile_alpha", f.users[0]),
                Some(token),
                Value::Null,
            )
            .await;
            assert_eq!(
                result.0,
                StatusCode::BAD_REQUEST,
                "Action filters apply only to actual activity: {result:?}"
            );
        }
    }
    let rows = all_rows(
        api,
        &format!("{}/{}/emails", ROOTS[0], f.users[0]),
        token,
        "emails",
    )
    .await;
    for (number, outcome) in [
        (1, "unknown"),
        (2, "unknown"),
        (3, "accepted"),
        (4, "rejected"),
        (5, "unavailable"),
        (6, "queued"),
        (7, "unknown"),
        (8, "accepted"),
    ] {
        let subject = format!("{} receipt {number}", f.prefix);
        let row = rows.iter().find(|row| row["subject"] == subject).unwrap();
        assert_eq!(
            row["outcome"], outcome,
            "Incomplete raw sent/delivered records never become acceptance evidence: {row}"
        );
        assert_eq!(
            row["recipient"], f.email,
            "Legacy email is recorded fallback when to_email is absent"
        );
        if outcome == "accepted" {
            assert!(
                !row["provider_message_id"]
                    .as_str()
                    .unwrap()
                    .trim()
                    .is_empty()
            );
            assert!(row["sent_at"].as_str().unwrap().ends_with('Z'));
        }
    }
    assert_eq!(
        rows.iter()
            .find(|row| row["subject"] == format!("{} receipt 1", f.prefix))
            .unwrap()["recorded_status"],
        "sent"
    );
    assert!(rows.iter().all(
        |row| row["subject"] != "Unlinked same address must stay absent"
            && row["subject"] != "Another recorded user must stay absent"
    ));
    let memberships = all_rows(
        api,
        &format!("{}/{}/memberships", ROOTS[1], f.users[0]),
        token,
        "memberships",
    )
    .await;
    assert!(memberships.iter().any(|row| !row["deleted_at"].is_null()));
    for row in memberships {
        assert_eq!(row["plan_id"], f.plan);
        assert!(row["catalog_currency"].is_null());
        assert!(row["total_paid"].is_null());
        assert!(row["next_payment_at"].is_null());
        assert!(!row["unmeasured_reason"].as_str().unwrap().is_empty());
    }
}
async fn journal_snapshot(pool: &PgPool, member: i64) -> Value {
    let mut tx = pool.begin().await.unwrap();
    sqlx::query("SET LOCAL TIME ZONE 'UTC'")
        .execute(&mut *tx)
        .await
        .unwrap();
    let value=sqlx::query_scalar("SELECT jsonb_build_object('notes',(SELECT COALESCE(jsonb_agg(to_jsonb(n) ORDER BY id),'[]'::jsonb) FROM member_notes n WHERE user_id=$1),'activity',(SELECT COALESCE(jsonb_agg(to_jsonb(a) ORDER BY id),'[]'::jsonb) FROM user_activity_log a WHERE user_id=$1),'audit',(SELECT COUNT(*) FROM crm_operational_logs WHERE context->>'member_id'=$1::text AND context->>'operation' IN('member_note_created','member_note_deleted')))")
        .bind(member).fetch_one(&mut *tx).await.unwrap();
    tx.commit().await.unwrap();
    value
}
async fn note_mutations(api: &Router, pool: &PgPool, token: &str, f: &Fixture) {
    let primary = format!("{}/{}", ROOTS[0], f.users[0]);
    let rich = format!("{}/{}", ROOTS[1], f.users[0]);
    for body in [
        json!({"content":"  "}),
        json!({"content":"x".repeat(10_001)}),
        json!({"content":"invalid\u{0000}control"}),
    ] {
        let before = journal_snapshot(pool, f.users[0]).await;
        let result = request(api, "POST", &format!("{primary}/notes"), Some(token), body).await;
        assert_eq!(result.0, StatusCode::BAD_REQUEST, "{result:?}");
        assert_eq!(journal_snapshot(pool, f.users[0]).await, before);
    }
    let structural = request(
        api,
        "POST",
        &format!("{primary}/notes"),
        Some(token),
        json!({}),
    )
    .await;
    assert_eq!(
        structural.0,
        StatusCode::UNPROCESSABLE_ENTITY,
        "{structural:?}"
    );
    let content = "Actual <note> & Unicode 🧭\nSecond recorded line";
    let added = request(
        api,
        "POST",
        &format!("{primary}/notes"),
        Some(token),
        json!({"content":content}),
    )
    .await;
    assert_eq!(added.0, StatusCode::OK, "{added:?}");
    assert_eq!(added.1["metrics_scope"], "member_note_mutation");
    assert_eq!(added.1["member_id"], f.users[0]);
    let note = &added.1["note"];
    let id = note["id"].as_i64().unwrap();
    assert_eq!(note["content"], content);
    assert_eq!(note["created_by"], f.admin);
    assert_eq!(note["created_by_name"], "Fixture");
    let at = chrono::DateTime::parse_from_rfc3339(note["created_at"].as_str().unwrap()).unwrap();
    assert!(
        (chrono::Utc::now().timestamp() - at.timestamp()).abs() < 10,
        "Real note creation is a UTC instant even under the non-UTC fixture"
    );
    for root in [&primary, &rich] {
        let rows = all_rows(api, &format!("{root}/notes"), token, "notes").await;
        assert!(
            rows.iter()
                .any(|row| row["id"] == id && row["content"] == content)
        );
    }
    let created:i64=sqlx::query_scalar("SELECT COUNT(*) FROM user_activity_log WHERE user_id=$1 AND entity_type='member_note' AND entity_id=$2 AND action='member_note_created'").bind(f.users[0]).bind(id).fetch_one(pool).await.unwrap();
    assert_eq!(created, 1);
    let timeline = all_rows(api, &format!("{rich}/timeline"), token, "data").await;
    assert_eq!(
        timeline
            .iter()
            .filter(|row| row["meta"]["note_id"] == id && row["title"] == "member_note_created")
            .count(),
        1
    );
    assert!(
        !timeline.iter().any(|row| row["id"] == format!("note:{id}")),
        "A note's journal and creation record are one displayed creation event"
    );
    let before = journal_snapshot(pool, f.users[0]).await;
    let wrong = request(
        api,
        "DELETE",
        &format!("{}/{}/notes/{id}", ROOTS[1], f.users[1]),
        Some(token),
        Value::Null,
    )
    .await;
    assert_eq!(wrong.0, StatusCode::NOT_FOUND, "{wrong:?}");
    assert_eq!(journal_snapshot(pool, f.users[0]).await, before);
    let removed = request(
        api,
        "DELETE",
        &format!("{rich}/notes/{id}"),
        Some(token),
        Value::Null,
    )
    .await;
    assert_eq!(removed.0, StatusCode::OK, "{removed:?}");
    assert_eq!(removed.1["removed"], true);
    let after = journal_snapshot(pool, f.users[0]).await;
    let repeated = request(
        api,
        "DELETE",
        &format!("{primary}/notes/{id}"),
        Some(token),
        Value::Null,
    )
    .await;
    assert_eq!(repeated.0, StatusCode::OK, "{repeated:?}");
    assert_eq!(repeated.1["removed"], false);
    assert_eq!(
        journal_snapshot(pool, f.users[0]).await,
        after,
        "Repeated deletion creates no additional journal or audit"
    );
    let timeline = all_rows(api, &format!("{primary}/timeline"), token, "data").await;
    for action in ["member_note_created", "member_note_deleted"] {
        assert_eq!(
            timeline
                .iter()
                .filter(|row| row["meta"]["note_id"] == id && row["title"] == action)
                .count(),
            1,
            "Creation and deletion remain actual separate recorded actions"
        );
    }
}
async fn rename_fixture_table(pool: &PgPool, kind: u8, hide: bool) {
    match (kind, hide) {
        (0, true) => {
            sqlx::query(
                "ALTER TABLE crm_operational_logs RENAME TO member_profile_audit_unavailable",
            )
            .execute(pool)
            .await
            .unwrap();
        }
        (0, false) => {
            sqlx::query(
                "ALTER TABLE member_profile_audit_unavailable RENAME TO crm_operational_logs",
            )
            .execute(pool)
            .await
            .unwrap();
        }
        (1, true) => {
            sqlx::query(
                "ALTER TABLE user_activity_log RENAME TO member_profile_activity_unavailable",
            )
            .execute(pool)
            .await
            .unwrap();
        }
        (1, false) => {
            sqlx::query(
                "ALTER TABLE member_profile_activity_unavailable RENAME TO user_activity_log",
            )
            .execute(pool)
            .await
            .unwrap();
        }
        (2, true) => {
            sqlx::query("ALTER TABLE member_notes RENAME TO member_profile_notes_unavailable")
                .execute(pool)
                .await
                .unwrap();
        }
        (2, false) => {
            sqlx::query("ALTER TABLE member_profile_notes_unavailable RENAME TO member_notes")
                .execute(pool)
                .await
                .unwrap();
        }
        _ => unreachable!(),
    }
}
async fn rollback_proof(api: &Router, pool: &PgPool, token: &str, f: &Fixture) {
    let path = format!("{}/{}", ROOTS[1], f.users[0]);
    for kind in 0..=2 {
        let before = journal_snapshot(pool, f.users[0]).await;
        rename_fixture_table(pool, kind, true).await;
        let created = request(
            api,
            "POST",
            &format!("{path}/notes"),
            Some(token),
            json!({"content":"Must roll back with the journal or audit"}),
        )
        .await;
        let deleted = request(
            api,
            "DELETE",
            &format!("{path}/notes/{}", f.note_ids[0]),
            Some(token),
            Value::Null,
        )
        .await;
        let failed_read = if kind == 1 {
            Some(
                request(
                    api,
                    "GET",
                    &format!("{path}/activity"),
                    Some(token),
                    Value::Null,
                )
                .await,
            )
        } else if kind == 2 {
            Some(
                request(
                    api,
                    "GET",
                    &format!("{path}/notes"),
                    Some(token),
                    Value::Null,
                )
                .await,
            )
        } else {
            None
        };
        let failed_profile = if kind != 0 {
            Some(request(api, "GET", &path, Some(token), Value::Null).await)
        } else {
            None
        };
        // Restore the physical source before asserting failures, preserving a reviewable fixture.
        rename_fixture_table(pool, kind, false).await;
        assert_eq!(
            created.0,
            StatusCode::SERVICE_UNAVAILABLE,
            "Uncommitted note creation must fail with its required journal/audit: {created:?}"
        );
        assert_eq!(
            deleted.0,
            StatusCode::SERVICE_UNAVAILABLE,
            "Uncommitted deletion must fail with its required journal/audit: {deleted:?}"
        );
        assert_eq!(
            journal_snapshot(pool, f.users[0]).await,
            before,
            "Both note and all required journal/audit effects roll back, including failed deletion"
        );
        for failure in [failed_read, failed_profile].into_iter().flatten() {
            assert_eq!(failure.0, StatusCode::SERVICE_UNAVAILABLE, "{failure:?}");
            assert!(
                failure.1.get("pagination").is_none(),
                "A storage outage must not become successful zero history"
            );
            assert!(failure.1.get("member").is_none());
        }
        let recovered = ok(api, &path, token).await;
        assert_eq!(recovered["metrics_scope"], "member_profile");
    }
}
async fn one_connection(api: &Router, pool: &PgPool, token: &str, f: &Fixture) {
    let one = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .acquire_timeout(std::time::Duration::from_secs(2))
        .connect_with((*pool.connect_options()).clone())
        .await
        .unwrap();
    sqlx::query("SET TIME ZONE 'America/New_York'")
        .execute(&one)
        .await
        .unwrap();
    let single = super::app(revolution_api::db::Database { pool: one.clone() }, None).await;
    for base in ROOTS {
        let path = format!("{base}/{}", f.users[0]);
        for suffix in std::iter::once(String::new()).chain(
            PANELS
                .into_iter()
                .map(|panel| format!("/{panel}?per_page=5")),
        ) {
            let result = tokio::time::timeout(
                std::time::Duration::from_secs(5),
                request(
                    &single,
                    "GET",
                    &format!("{path}{suffix}"),
                    Some(token),
                    Value::Null,
                ),
            )
            .await
            .unwrap();
            assert_eq!(
                result.0,
                StatusCode::OK,
                "One connection must suffice for current identity + the entire factual snapshot: {path}{suffix}: {result:?}"
            );
        }
    }
    let path = format!("{}/{}", ROOTS[1], f.users[0]);
    let added = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        request(
            &single,
            "POST",
            &format!("{path}/notes"),
            Some(token),
            json!({"content":"Single connection actual note"}),
        ),
    )
    .await
    .unwrap();
    assert_eq!(added.0, StatusCode::OK, "{added:?}");
    let created =
        chrono::DateTime::parse_from_rfc3339(added.1["note"]["created_at"].as_str().unwrap())
            .unwrap();
    assert!((chrono::Utc::now().timestamp() - created.timestamp()).abs() < 10);
    let note = added.1["note"]["id"].as_i64().unwrap();
    let journal:i64=sqlx::query_scalar("SELECT COUNT(*) FROM user_activity_log WHERE user_id=$1 AND entity_type='member_note' AND entity_id=$2 AND action='member_note_created'").bind(f.users[0]).bind(note).fetch_one(pool).await.unwrap();
    assert_eq!(
        journal, 1,
        "The rich alias uses the same atomic note journal"
    );
    let basic = format!("{}/{}", ROOTS[0], f.users[0]);
    let deleted = tokio::time::timeout(
        std::time::Duration::from_secs(5),
        request(
            &single,
            "DELETE",
            &format!("{basic}/notes/{note}"),
            Some(token),
            Value::Null,
        ),
    )
    .await
    .unwrap();
    assert_eq!(deleted.0, StatusCode::OK, "{deleted:?}");
    assert_eq!(deleted.1["removed"], true);
    one.close().await;
    let recovered = ok(api, &path, token).await;
    assert_eq!(recovered["member"]["id"], f.users[0]);
}
async fn deleted_and_invalid(api: &Router, pool: &PgPool, token: &str, f: &Fixture) {
    sqlx::query("UPDATE users SET deleted_at=NOW() WHERE id=$1")
        .bind(f.users[1])
        .execute(pool)
        .await
        .unwrap();
    for base in ROOTS {
        for suffix in std::iter::once(String::new())
            .chain(PANELS.into_iter().map(|panel| format!("/{panel}")))
        {
            let path = format!("{base}/{}{suffix}", f.users[1]);
            let result = request(api, "GET", &path, Some(token), Value::Null).await;
            assert_eq!(
                result.0,
                StatusCode::NOT_FOUND,
                "Soft deletion is checked before an empty or preserved child history: {path}: {result:?}"
            );
        }
        for member in [f.users[1], i64::MAX] {
            let path = format!("{base}/{member}/notes");
            let before = journal_snapshot(pool, f.users[0]).await;
            let create = request(
                api,
                "POST",
                &path,
                Some(token),
                json!({"content":"No note for an unavailable member"}),
            )
            .await;
            let delete = request(
                api,
                "DELETE",
                &format!("{path}/{}", f.note_ids[0]),
                Some(token),
                Value::Null,
            )
            .await;
            assert_eq!(create.0, StatusCode::NOT_FOUND, "{create:?}");
            assert_eq!(delete.0, StatusCode::NOT_FOUND, "{delete:?}");
            assert_eq!(journal_snapshot(pool, f.users[0]).await, before);
        }
        for path in [
            format!("{base}/0"),
            format!("{base}/-1/notes"),
            format!("{base}/0/timeline"),
        ] {
            let result = request(api, "GET", &path, Some(token), Value::Null).await;
            assert_eq!(result.0, StatusCode::BAD_REQUEST, "{path}: {result:?}");
        }
    }
}
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String], adapter: &Adapter) {
    let submissions = requests(adapter);
    let f = fixture(pool).await;
    auth_and_missing(api, tokens, f.users[0]).await;
    current_profile(api, &tokens[0], &f).await;
    history_panels(api, &tokens[0], &f).await;
    order_export(api, pool, &tokens[0], &tokens[1], &f).await;
    note_mutations(api, pool, &tokens[0], &f).await;
    rollback_proof(api, pool, &tokens[0], &f).await;
    one_connection(api, pool, &tokens[0], &f).await;
    deleted_and_invalid(api, pool, &tokens[0], &f).await;
    assert_eq!(
        requests(adapter),
        submissions,
        "Reading recorded receipts and writing private notes never submit provider messages"
    );
    sqlx::query("DELETE FROM email_logs WHERE id=ANY($1)")
        .bind(&f.email_ids)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM users WHERE id=ANY($1)")
        .bind(&f.users)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM membership_plans WHERE id=$1")
        .bind(f.plan)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_tags WHERE id=$1")
        .bind(f.tag)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Member profile/history proof passed: both canonical aliases, true recorded scope, nullable/wide dates, paginated tied records, explicit receipt links/evidence, atomic private notes/journals, missing/softdeleted members and single-connection reads/writes."
    );
}
