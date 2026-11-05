//! Actual downloadable member snapshots, with fixtures retained for independent readers.
use super::request;
use axum::{
    Router,
    body::Body,
    extract::ConnectInfo,
    http::{HeaderMap, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::Value;
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::net::SocketAddr;
use tower::ServiceExt;
use uuid::Uuid;

async fn download(
    api: &Router,
    path: &str,
    token: Option<&str>,
) -> (StatusCode, HeaderMap, Vec<u8>) {
    let mut req = Request::builder()
        .uri(path)
        .extension(ConnectInfo(SocketAddr::from(([127, 0, 0, 1], 45001))));
    if let Some(token) = token {
        req = req.header("authorization", format!("Bearer {token}"));
    }
    let response = api
        .clone()
        .oneshot(req.body(Body::empty()).unwrap())
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
pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    const ROOT: &str = "/api/admin/member-management/export";
    let admin = Some(tokens[0].as_str());
    for format in ["csv", "xlsx", "pdf"] {
        let path = format!("{ROOT}?format={format}");
        assert_eq!(download(api, &path, None).await.0, StatusCode::UNAUTHORIZED);
        assert_eq!(
            download(api, &path, Some(&tokens[1])).await.0,
            StatusCode::FORBIDDEN
        );
    }
    for query in [
        "format=xml",
        "status=unknown",
        "date_from=not-a-date",
        "date_to=2020-02-30",
        "date_from=2020-2-01",
        "date_from=2020-03-01&date_to=2020-02-29",
        "page=2",
    ] {
        let (status, body) =
            request(api, "GET", &format!("{ROOT}?{query}"), admin, Value::Null).await;
        assert_eq!(status, StatusCode::BAD_REQUEST, "{query}: {body}");
    }
    let names = [
        "=SUM(1,2)".to_owned(),
        "D'Ávila,\nquoted \"name\"".to_owned(),
        "Мария Παπαδοπούλου".into(),
        "李明 山田太郎".into(),
        "ليلى أحمد لا".into(),
        "שרה כהן".into(),
        "अर्जुन शर्मा".into(),
        "Long member name ".repeat(20).chars().take(255).collect(),
    ];
    let ids:Vec<i64>=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,role,created_at,updated_at) SELECT CASE WHEN n<=8 THEN ($1::text[])[n] ELSE 'Export member '||n END,'member-export-'||n||'@example.test','not-a-login-password','user',TIMESTAMP '2020-02-29 12:34:56.123456',TIMESTAMP '2020-02-29 20:30:00' FROM generate_series(1,1105)n RETURNING id")
        .bind(names.to_vec()).fetch_all(pool).await.unwrap();
    assert_eq!(ids.len(), 1105);
    let outside:Vec<i64>=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,created_at) VALUES('Before range','member-export-before@example.test','not-a-login-password',TIMESTAMP '2020-02-28 23:59:59.999999'),('After range','member-export-after@example.test','not-a-login-password',TIMESTAMP '2020-03-01 00:00:00'),('Deleted row','member-export-deleted@example.test','not-a-login-password',TIMESTAMP '2020-02-29 14:00:00') RETURNING id").fetch_all(pool).await.unwrap();
    sqlx::query("UPDATE users SET deleted_at=NOW() WHERE id=$1")
        .bind(outside[2])
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("UPDATE users SET created_at=TIMESTAMP '2020-02-29 23:59:59.999999' WHERE id=$1")
        .bind(ids[0])
        .execute(pool)
        .await
        .unwrap();
    let plan:i64=sqlx::query_scalar("INSERT INTO membership_plans(name,slug,price,billing_cycle) VALUES('Export facts fixture','export-facts-fixture',10,'monthly') RETURNING id").fetch_one(pool).await.unwrap();
    for (id, status, expires) in [
        (ids[0], "active", 1_i32),
        (ids[1], "trial", 1),
        (ids[2], "active", -1),
    ] {
        sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,trial_ends_at) VALUES($1,$2,$3,NOW()-INTERVAL '10 days',NOW()+$4*INTERVAL '1 day',NOW()+$4*INTERVAL '1 day')").bind(id).bind(plan).bind(status).bind(expires).execute(pool).await.unwrap();
    }
    // A later cancelled record must not hide a current active membership.
    sqlx::query("INSERT INTO user_memberships(user_id,plan_id,status,starts_at,expires_at,created_at) VALUES($1,$2,'cancelled',NOW()-INTERVAL '2 days',NOW()-INTERVAL '1 day',NOW())").bind(ids[0]).bind(plan).execute(pool).await.unwrap();
    let tag:i64=sqlx::query_scalar("INSERT INTO member_tags(name,slug) VALUES('Export cohort','export-cohort-fixture') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query(
        "INSERT INTO user_member_tags(user_id,tag_id) SELECT id,$2 FROM unnest($1::bigint[])id",
    )
    .bind(&ids[..4])
    .bind(tag)
    .execute(pool)
    .await
    .unwrap();
    let segment:i64=sqlx::query_scalar("INSERT INTO member_segments(name,slug,is_dynamic,is_active,filter_criteria) VALUES('Export cohort','export-cohort-fixture',false,true,'[]') RETURNING id").fetch_one(pool).await.unwrap();
    sqlx::query("INSERT INTO user_member_segments(segment_id,user_id) SELECT $1,id FROM unnest($2::bigint[])id").bind(segment).bind(&ids[..5]).execute(pool).await.unwrap();
    let product:i64=sqlx::query_scalar("INSERT INTO products(name,slug,price,type) VALUES('Export proof product','export-proof-product',25,'course') RETURNING id").fetch_one(pool).await.unwrap();
    let mut orders = Vec::new();
    for (id, total) in ids.iter().take(4).zip([1_i64, 100, 1000, 5000]) {
        let order:i64=sqlx::query_scalar("INSERT INTO orders(user_id,order_number,total,status,currency,completed_at) VALUES($1,$2,$3::bigint::numeric,'completed','USD',NOW()) RETURNING id").bind(id).bind(Uuid::new_v4().to_string()).bind(total).fetch_one(pool).await.unwrap();
        sqlx::query("INSERT INTO order_items(order_id,product_id,name,quantity,unit_price,total) VALUES($1,$2,'Export proof product',1,25,25)").bind(order).bind(product).execute(pool).await.unwrap();
        orders.push(order);
    }
    let expected:Vec<Value>=sqlx::query_scalar("SELECT to_jsonb(q) FROM(SELECT u.id::text AS id,u.name,u.email,u.role,u.created_at::text AS created_at,u.updated_at::text AS updated_at,m.status FROM users u JOIN admin_member_facts m ON m.id=u.id WHERE u.created_at>=DATE '2020-02-29' AND u.created_at<DATE '2020-03-01' ORDER BY u.created_at DESC,u.id)q").fetch_all(pool).await.unwrap();
    assert_eq!(expected.len(), 1105);
    let directory =
        std::env::temp_dir().join(format!("revolution-member-export-proof-{}", Uuid::new_v4()));
    std::fs::create_dir(&directory).unwrap();
    std::fs::write(
        directory.join("expected.json"),
        serde_json::to_vec_pretty(&expected).unwrap(),
    )
    .unwrap();
    let range = "date_from=2020-02-29&date_to=2020-02-29";
    for format in ["csv", "xlsx", "pdf"] {
        let (status, headers, bytes) =
            download(api, &format!("{ROOT}?format={format}&{range}"), admin).await;
        assert_eq!(
            status,
            StatusCode::OK,
            "{format}: {}",
            String::from_utf8_lossy(&bytes)
        );
        assert_eq!(headers["x-export-row-count"], "1105");
        assert!(
            headers["cache-control"]
                .to_str()
                .unwrap()
                .contains("no-store")
        );
        let digest = hex::encode(Sha256::digest(&bytes));
        assert_eq!(headers["x-export-sha256"], digest);
        let audit:Value=sqlx::query_scalar("SELECT context FROM crm_operational_logs WHERE category='system' AND context->>'operation'='member_export' AND context->>'sha256'=$1 ORDER BY id DESC LIMIT 1").bind(&digest).fetch_one(pool).await.unwrap();
        assert_eq!(audit["rows"], 1105);
        assert_eq!(audit["bytes"], bytes.len());
        assert_eq!(audit["format"], format);
        match format {
            "xlsx" => assert!(bytes.starts_with(b"PK\x03\x04")),
            "pdf" => assert!(bytes.starts_with(b"%PDF-")),
            _ => {
                let mut reader = csv::Reader::from_reader(bytes.as_slice());
                assert_eq!(reader.headers().unwrap().get(0), Some("ID"));
                let rows = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
                assert_eq!(rows.len(), 1105);
                for (row, expected) in rows.iter().zip(&expected) {
                    assert_eq!(row.get(0), expected["id"].as_str());
                    let name = expected["name"].as_str().unwrap();
                    assert_eq!(
                        row.get(1),
                        Some(if name.starts_with('=') {
                            format!("'{name}")
                        } else {
                            name.to_owned()
                        })
                        .as_deref()
                    );
                    assert_eq!(row.get(2), expected["email"].as_str());
                }
            }
        }
        std::fs::write(directory.join(format!("members.{format}")), bytes).unwrap();
    }
    for status in ["active", "trial", "churned", "never_subscribed"] {
        let (code, headers, bytes) =
            download(api, &format!("{ROOT}?{range}&status={status}"), admin).await;
        assert_eq!(code, StatusCode::OK);
        let count = expected
            .iter()
            .filter(|row| row["status"] == status)
            .count();
        assert_eq!(headers["x-export-row-count"], count.to_string());
        let rows = csv::Reader::from_reader(bytes.as_slice())
            .records()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();
        assert_eq!(rows.len(), count);
    }
    // Every established filter and sort must select the exact same IDs/order
    // as the real member directory; exporting must not broaden a UI audience.
    for filters in [
        format!("tag={tag}"),
        format!("segment={segment}"),
        format!("plan_id={plan}"),
        format!("product_id={product}"),
        "search=member-export-1%40example.test".into(),
        format!("tag={tag}&spending_tier=low"),
        format!("tag={tag}&spending_tier=medium"),
        format!("tag={tag}&spending_tier=high"),
        format!("tag={tag}&spending_tier=whale"),
        format!(
            "segment={segment}&tag={tag}&product_id={product}&sort_by=total_spent&sort_dir=asc"
        ),
        format!("tag={tag}&status=active&sort_by=email&sort_dir=desc"),
    ] {
        let (code, list) = request(
            api,
            "GET",
            &format!("/api/admin/members?{range}&{filters}&per_page=100"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(code, StatusCode::OK, "list {filters}: {list}");
        let expected = list["members"]
            .as_array()
            .unwrap()
            .iter()
            .map(|row| row["id"].as_i64().unwrap().to_string())
            .collect::<Vec<_>>();
        assert!(expected.len() < 100);
        let (code, headers, bytes) =
            download(api, &format!("{ROOT}?{range}&{filters}"), admin).await;
        assert_eq!(
            code,
            StatusCode::OK,
            "export {filters}: {}",
            String::from_utf8_lossy(&bytes)
        );
        assert_eq!(headers["x-export-row-count"], expected.len().to_string());
        let actual = csv::Reader::from_reader(bytes.as_slice())
            .records()
            .map(|row| row.unwrap()[0].to_owned())
            .collect::<Vec<_>>();
        assert_eq!(actual, expected, "Export filter mismatch: {filters}");
    }
    let (status, _, bytes) =
        download(api, &format!("/api/admin/members/export?{range}"), admin).await;
    assert_eq!(status, StatusCode::OK);
    let mut legacy = csv::Reader::from_reader(bytes.as_slice());
    assert_eq!(
        legacy.headers().unwrap(),
        &csv::StringRecord::from(vec!["id", "name", "email", "created_at", "updated_at"])
    );
    assert_eq!(legacy.records().count(), 1105);
    assert_eq!(
        download(
            api,
            &format!("/api/admin/members/export?format=xlsx&{range}"),
            admin
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let (_, headers, empty) = download(
        api,
        &format!("{ROOT}?format=pdf&date_from=2019-02-28&date_to=2019-02-28"),
        admin,
    )
    .await;
    assert_eq!(headers["x-export-row-count"], "0");
    std::fs::write(directory.join("empty.pdf"), empty).unwrap();
    // A real value within VARCHAR(255) can still require several PDF pages.
    // Excel has a hard row-height limit: fail explicitly instead of clipping it.
    let wrapped_name = "A\n".repeat(120) + "End";
    let wrapped:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,created_at) VALUES($1,'member-export-wrapped@example.test','not-a-login-password',TIMESTAMP '2020-02-27 12:00:00') RETURNING id").bind(&wrapped_name).fetch_one(pool).await.unwrap();
    let wrapped_range = "date_from=2020-02-27&date_to=2020-02-27";
    let (status, headers, bytes) =
        download(api, &format!("{ROOT}?format=pdf&{wrapped_range}"), admin).await;
    assert_eq!(
        status,
        StatusCode::OK,
        "Wrapped PDF: {}",
        String::from_utf8_lossy(&bytes)
    );
    assert_eq!(headers["x-export-row-count"], "1");
    std::fs::write(directory.join("wrapped.pdf"), bytes).unwrap();
    std::fs::write(directory.join("wrapped-name.txt"), &wrapped_name).unwrap();
    let (status, body) = request(
        api,
        "GET",
        &format!("{ROOT}?format=xlsx&{wrapped_range}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body["message"]
            .as_str()
            .unwrap()
            .contains("Excel can display")
    );
    let (status, _, bytes) = download(api, &format!("{ROOT}?{wrapped_range}"), admin).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(
        &csv::Reader::from_reader(bytes.as_slice())
            .records()
            .next()
            .unwrap()
            .unwrap()[1],
        wrapped_name
    );
    // Exact 10000 works. The next matching row fails explicitly, not silently.
    let overflow:Vec<i64>=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,created_at) SELECT 'Bounded export fixture','member-export-bound-'||n||'@example.test','not-a-login-password',TIMESTAMP '2020-02-29 12:00:00' FROM generate_series(1,8895)n RETURNING id").fetch_all(pool).await.unwrap();
    let (status, headers, _) = download(api, &format!("{ROOT}?{range}"), admin).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(headers["x-export-row-count"], "10000");
    let extra:i64=sqlx::query_scalar("INSERT INTO users(name,email,password_hash,created_at) VALUES('Overflow fixture','member-export-overflow@example.test','not-a-login-password',TIMESTAMP '2020-02-29 12:00:00') RETURNING id").fetch_one(pool).await.unwrap();
    let (status, body) = request(api, "GET", &format!("{ROOT}?{range}"), admin, Value::Null).await;
    assert_eq!(status, StatusCode::BAD_REQUEST);
    assert!(
        body["message"]
            .as_str()
            .unwrap()
            .contains("no partial export")
    );
    sqlx::raw_sql("CREATE FUNCTION fixture_export_audit_failure() RETURNS trigger LANGUAGE plpgsql AS $$BEGIN IF NEW.category='system' AND NEW.context->>'operation'='member_export' THEN RAISE EXCEPTION 'Controlled export audit failure'; END IF; RETURN NEW; END$$; CREATE TRIGGER fixture_export_audit_failure BEFORE INSERT ON crm_operational_logs FOR EACH ROW EXECUTE FUNCTION fixture_export_audit_failure()")
        .execute(pool).await.unwrap();
    let (status, _, _) = download(api, &format!("{ROOT}?{range}&status=active"), admin).await;
    assert_eq!(
        status,
        StatusCode::INTERNAL_SERVER_ERROR,
        "An export without its durable audit must not be downloaded"
    );
    sqlx::raw_sql("DROP TRIGGER fixture_export_audit_failure ON crm_operational_logs; DROP FUNCTION fixture_export_audit_failure()")
        .execute(pool).await.unwrap();
    sqlx::query("DELETE FROM user_memberships WHERE user_id=ANY($1)")
        .bind(&ids)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM order_items WHERE order_id=ANY($1)")
        .bind(&orders)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM orders WHERE id=ANY($1)")
        .bind(orders)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM products WHERE id=$1")
        .bind(product)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_segments WHERE id=$1")
        .bind(segment)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM member_tags WHERE id=$1")
        .bind(tag)
        .execute(pool)
        .await
        .unwrap();
    let mut cleanup = ids;
    cleanup.extend(outside);
    cleanup.extend(overflow);
    cleanup.push(extra);
    cleanup.push(wrapped);
    sqlx::query("DELETE FROM users WHERE id=ANY($1)")
        .bind(cleanup)
        .execute(pool)
        .await
        .unwrap();
    sqlx::query("DELETE FROM membership_plans WHERE id=$1")
        .bind(plan)
        .execute(pool)
        .await
        .unwrap();
    println!(
        "Independent member export fixtures: {}",
        directory.display()
    );
}
