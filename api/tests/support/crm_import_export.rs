//! Actual bounded transfers, durable snapshots, duplicate decisions and rollback.
use super::request;
use axum::{
    Router,
    body::Body,
    http::{HeaderMap, Request, StatusCode},
};
use http_body_util::BodyExt;
use serde_json::{Value, json};
use sqlx::PgPool;
use tower::ServiceExt;
const ROOT: &str = "/api/admin/crm";

async fn raw(
    api: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    content_type: Option<&str>,
    bytes: Vec<u8>,
) -> (StatusCode, HeaderMap, Vec<u8>) {
    let mut builder = Request::builder().method(method).uri(path);
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    if let Some(kind) = content_type {
        builder = builder.header("content-type", kind);
    }
    let response = api
        .clone()
        .oneshot(builder.body(Body::from(bytes)).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let headers = response.headers().clone();
    let body = response
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes()
        .to_vec();
    (status, headers, body)
}
async fn upload(api: &Router, token: Option<&str>, name: &str, data: Value) -> Value {
    let result=request(api,"POST",&format!("{ROOT}/imports"),token,json!({"type":"contacts","file_name":name,"file_type":if name.ends_with(".csv"){"csv"}else{"json"},"data":data})).await;
    assert_eq!(result.0, StatusCode::OK, "{result:?}");
    result.1
}

pub async fn proof(api: &Router, pool: &PgPool, tokens: &[String]) {
    let admin = Some(tokens[0].as_str());
    let missing = uuid::Uuid::new_v4();
    for (token, status) in [
        (None, StatusCode::UNAUTHORIZED),
        (Some(tokens[1].as_str()), StatusCode::FORBIDDEN),
    ] {
        for (method, path, body) in [
            ("GET", format!("{ROOT}/imports"), Value::Null),
            ("GET", format!("{ROOT}/exports"), Value::Null),
            ("POST", format!("{ROOT}/imports"), json!({})),
            ("POST", format!("{ROOT}/imports/{missing}/start"), json!({})),
            (
                "POST",
                format!("{ROOT}/imports/{missing}/cancel"),
                Value::Null,
            ),
            (
                "GET",
                format!("{ROOT}/imports/{missing}/preview"),
                Value::Null,
            ),
            (
                "POST",
                format!("{ROOT}/exports"),
                json!({"type":"contacts"}),
            ),
            (
                "GET",
                format!("{ROOT}/exports/{missing}/download"),
                Value::Null,
            ),
            (
                "POST",
                format!("{ROOT}/import"),
                json!({"file_type":"json","data":[]}),
            ),
        ] {
            assert_eq!(
                request(api, method, &path, token, body).await.0,
                status,
                "{method} {path}"
            );
        }
    }
    for body in [
        json!({"type":"tags","file_name":"tags.json","file_type":"json","data":[{"name":"No fake success"}]}),
        json!({"type":"contacts","file_name":"../contacts.csv","file_type":"csv","data":"email\na@example.test\n"}),
        json!({"type":"contacts","file_name":"contacts.csv","file_type":"csv","data":"email,email\na@example.test,b@example.test\n"}),
        json!({"type":"contacts","file_name":"contacts.csv","file_type":"csv","data":"email,name\na@example.test\n"}),
        json!({"type":"contacts","file_name":"contacts.json","file_type":"json","data":[{"email":"null@example.test","first_name":"bad\0value"}]}),
    ] {
        assert_eq!(
            request(api, "POST", &format!("{ROOT}/imports"), admin, body)
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
    }
    let newsletter_before: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM newsletter_subscribers")
        .fetch_one(pool)
        .await
        .unwrap();
    let tag_id:i64=sqlx::query_scalar("INSERT INTO crm_tags(name,title,slug) VALUES('Transfer actual tag','Transfer actual tag','transfer-actual-tag') RETURNING id").fetch_one(pool).await.unwrap();
    let list_id:i64=sqlx::query_scalar("INSERT INTO crm_lists(name,title,slug) VALUES('Transfer actual list','Transfer actual list','transfer-actual-list') RETURNING id").fetch_one(pool).await.unwrap();
    let csv = "Address,Given name,custom_fields\r\ntransfer-proof-a@example.test,\"Line one\nLine two, quoted\",\"{\"\"team\"\":\"\"actual\"\"}\"\r\nTRANSFER-PROOF-A@example.test,Ignored duplicate,{}\r\ntransfer-proof-b@example.test,=SUM(1+1),{}\r\n";
    let job = upload(api, admin, "actual.csv", json!(csv)).await;
    let id = job["id"].as_str().unwrap();
    assert_eq!(job["status"], "pending");
    assert_eq!(job["total_rows"], 3);
    let preview = request(
        api,
        "GET",
        &format!("{ROOT}/imports/{id}/preview"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(preview.0, StatusCode::OK);
    assert_eq!(
        preview.1["rows"][0]["Given name"],
        "Line one\nLine two, quoted"
    );
    let start = json!({"mapping":{"Address":"email","Given name":"first_name","custom_fields":"custom_fields"},"settings":{"skip_duplicates":true,"add_to_lists":[list_id.to_string()],"apply_tags":[tag_id.to_string()]}});
    let path = format!("{ROOT}/imports/{id}/start");
    let (first, replay) = tokio::join!(
        request(api, "POST", &path, admin, start.clone()),
        request(api, "POST", &path, admin, start.clone())
    );
    for result in [first, replay] {
        assert_eq!(result.0, StatusCode::OK, "{result:?}");
        assert_eq!(result.1["status"], "completed");
        assert_eq!(result.1["created_count"], 2);
        assert_eq!(result.1["success_count"], 2);
        assert_eq!(result.1["skipped_count"], 1);
    }
    let contacts:Vec<(i64,String,String,Value)>=sqlx::query_as("SELECT id,email,first_name,tags FROM contacts WHERE email LIKE 'transfer-proof-%' ORDER BY email").fetch_all(pool).await.unwrap();
    assert_eq!(contacts.len(), 2);
    assert_eq!(contacts[0].1, "transfer-proof-a@example.test");
    assert_eq!(contacts[0].2, "Line one\nLine two, quoted");
    assert_eq!(contacts[0].3, json!(["Transfer actual tag"]));
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM crm_list_members WHERE list_id=$1")
            .bind(list_id)
            .fetch_one(pool)
            .await
            .unwrap(),
        2
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM newsletter_subscribers")
            .fetch_one(pool)
            .await
            .unwrap(),
        newsletter_before,
        "Import never creates confirmation or submits messages"
    );
    assert_eq!(sqlx::query_scalar::<_,i64>("SELECT COUNT(*) FROM crm_operational_logs WHERE message='Contact import completed' AND context->>'job_id'=$1").bind(id).fetch_one(pool).await.unwrap(),1,"Concurrent start records one execution");
    assert_eq!(
        request(
            api,
            "POST",
            &path,
            admin,
            json!({"mapping":{"Address":"email"}})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/imports/{id}/cancel"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let independent = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&std::env::var("ADMIN_VERIFY_DATABASE_URL").unwrap())
        .await
        .unwrap();
    assert_eq!(
        sqlx::query_scalar::<_, String>("SELECT status FROM crm_transfer_jobs WHERE id=$1::uuid")
            .bind(id)
            .fetch_one(&independent)
            .await
            .unwrap(),
        "completed"
    );
    independent.close().await;

    let bad=upload(api,admin,"invalid.json",json!([{"email":"transfer-atomic-valid@example.test","first_name":"Valid row"},{"email":"not-an-email","first_name":"Invalid row"}])).await;
    let bad_id = bad["id"].as_str().unwrap();
    let failed = request(
        api,
        "POST",
        &format!("{ROOT}/imports/{bad_id}/start"),
        admin,
        json!({}),
    )
    .await;
    assert_eq!(failed.0, StatusCode::OK, "{failed:?}");
    assert_eq!(failed.1["status"], "failed");
    assert_eq!(failed.1["success_count"], 0);
    assert_eq!(failed.1["error_count"], 1);
    assert_eq!(failed.1["errors"][0]["row"], 2);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contacts WHERE email='transfer-atomic-valid@example.test'"
        )
        .fetch_one(pool)
        .await
        .unwrap(),
        0,
        "Validation failure rolls back the entire batch"
    );

    let update = upload(
        api,
        admin,
        "update.json",
        json!([{"email":"TRANSFER-PROOF-A@example.test","first_name":null,"status":"customer"}]),
    )
    .await;
    let updated = request(
        api,
        "POST",
        &format!("{ROOT}/imports/{}/start", update["id"].as_str().unwrap()),
        admin,
        json!({"settings":{"update_existing":true,"skip_duplicates":false}}),
    )
    .await;
    assert_eq!(updated.0, StatusCode::OK, "{updated:?}");
    assert_eq!(updated.1["updated_count"], 1);
    assert_eq!(updated.1["created_count"], 0);
    let preserved: (Option<String>, Value, Value) =
        sqlx::query_as("SELECT first_name,tags,custom_fields FROM contacts WHERE id=$1")
            .bind(contacts[0].0)
            .fetch_one(pool)
            .await
            .unwrap();
    assert!(preserved.0.is_none());
    assert_eq!(preserved.1, json!(["Transfer actual tag"]));
    assert_eq!(preserved.2, json!({"team":"actual"}));

    let pending = upload(
        api,
        admin,
        "cancel.json",
        json!([{"email":"transfer-cancel@example.test"}]),
    )
    .await;
    let pending_id = pending["id"].as_str().unwrap();
    for _ in 0..2 {
        let cancelled = request(
            api,
            "POST",
            &format!("{ROOT}/imports/{pending_id}/cancel"),
            admin,
            Value::Null,
        )
        .await;
        assert_eq!(cancelled.0, StatusCode::OK);
        assert_eq!(cancelled.1["status"], "cancelled");
    }
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/imports/{pending_id}/start"),
            admin,
            json!({})
        )
        .await
        .0,
        StatusCode::CONFLICT
    );
    let history = request(
        api,
        "GET",
        &format!("{ROOT}/imports?status=completed&per_page=1&page=2"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(history.0, StatusCode::OK);
    assert_eq!(history.1["data"].as_array().unwrap().len(), 1);
    assert_eq!(history.1["meta"]["total"], 2);
    assert_eq!(history.1["meta"]["current_page"], 2);

    let output=request(api,"POST",&format!("{ROOT}/exports"),admin,json!({"type":"contacts","filters":{"list_id":list_id.to_string()},"fields":["email","first_name","tags","custom_fields"]})).await;
    assert_eq!(output.0, StatusCode::OK, "{output:?}");
    assert_eq!(output.1["status"], "completed");
    assert_eq!(output.1["total_records"], 2);
    let export_id = output.1["id"].as_str().unwrap();
    let download_path = output.1["file_url"].as_str().unwrap();
    assert_eq!(
        download_path,
        format!("{ROOT}/exports/{export_id}/download")
    );
    let downloaded = raw(api, "GET", download_path, admin, None, Vec::new()).await;
    assert_eq!(downloaded.0, StatusCode::OK);
    assert_eq!(downloaded.1["cache-control"], "no-store");
    assert_eq!(downloaded.1["content-type"], "text/csv; charset=utf-8");
    assert!(
        downloaded.1["content-disposition"]
            .to_str()
            .unwrap()
            .ends_with(".csv\"")
    );
    let mut reader = csv::Reader::from_reader(downloaded.2.as_slice());
    let records = reader.records().collect::<Result<Vec<_>, _>>().unwrap();
    assert_eq!(records.len(), 2);
    assert_eq!(&records[1][1], "'=SUM(1+1)");
    sqlx::query("UPDATE contacts SET first_name='Changed after snapshot' WHERE id=$1")
        .bind(contacts[1].0)
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(
        raw(api, "GET", download_path, admin, None, Vec::new())
            .await
            .2,
        downloaded.2,
        "Download is the durable snapshot, not a new export"
    );
    let json_export=request(api,"POST",&format!("{ROOT}/exports"),admin,json!({"type":"contacts","file_type":"json","filters":{"list_id":list_id.to_string()},"fields":["email","first_name"]})).await;
    assert_eq!(json_export.0, StatusCode::OK);
    let json_bytes = raw(
        api,
        "GET",
        json_export.1["file_url"].as_str().unwrap(),
        admin,
        None,
        Vec::new(),
    )
    .await;
    let json_rows: Value = serde_json::from_slice(&json_bytes.2).unwrap();
    assert_eq!(json_rows[1]["first_name"], "Changed after snapshot");
    for (input, status) in [
        (json!({"type":"sequences"}), StatusCode::BAD_REQUEST),
        (
            json!({"type":"contacts","fields":["password_hash"]}),
            StatusCode::BAD_REQUEST,
        ),
        (
            json!({"type":"contacts","fields":["email","email"]}),
            StatusCode::BAD_REQUEST,
        ),
        (
            json!({"type":"contacts","filters":{"unknown":true}}),
            StatusCode::UNPROCESSABLE_ENTITY,
        ),
    ] {
        assert_eq!(
            request(api, "POST", &format!("{ROOT}/exports"), admin, input)
                .await
                .0,
            status
        );
    }

    // Real multipart upload follows the same durable preview/start path.
    let multipart=b"--transfer-boundary\r\nContent-Disposition: form-data; name=\"type\"\r\n\r\ncontacts\r\n--transfer-boundary\r\nContent-Disposition: form-data; name=\"file\"; filename=\"multipart.csv\"\r\nContent-Type: text/csv\r\n\r\nemail,first_name\ntransfer-multipart@example.test,Multipart actual\n\r\n--transfer-boundary--\r\n".to_vec();
    let uploaded = raw(
        api,
        "POST",
        &format!("{ROOT}/imports"),
        admin,
        Some("multipart/form-data; boundary=transfer-boundary"),
        multipart,
    )
    .await;
    assert_eq!(
        uploaded.0,
        StatusCode::OK,
        "{}",
        String::from_utf8_lossy(&uploaded.2)
    );
    let multipart_job: Value = serde_json::from_slice(&uploaded.2).unwrap();
    let multipart_started = request(
        api,
        "POST",
        &format!(
            "{ROOT}/imports/{}/start",
            multipart_job["id"].as_str().unwrap()
        ),
        admin,
        json!({}),
    )
    .await;
    assert_eq!(
        multipart_started.1["created_count"], 1,
        "{multipart_started:?}"
    );

    let rollback = upload(
        api,
        admin,
        "rollback.json",
        json!([{"email":"transfer-rollback@example.test"}]),
    )
    .await;
    let rollback_id = rollback["id"].as_str().unwrap();
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO transfer_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let rolled_back = request(
        api,
        "POST",
        &format!("{ROOT}/imports/{rollback_id}/start"),
        admin,
        json!({}),
    )
    .await;
    let export_failed = request(
        api,
        "POST",
        &format!("{ROOT}/exports"),
        admin,
        json!({"type":"contacts","filters":{"search":"transfer-proof"}}),
    )
    .await;
    sqlx::query("ALTER TABLE transfer_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(rolled_back.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(export_failed.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contacts WHERE email='transfer-rollback@example.test'"
        )
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/imports/{rollback_id}"),
            admin,
            Value::Null
        )
        .await
        .1["status"],
        "pending"
    );
    let recovered = request(
        api,
        "POST",
        &format!("{ROOT}/imports/{rollback_id}/start"),
        admin,
        json!({}),
    )
    .await;
    assert_eq!(recovered.1["created_count"], 1);
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/exports/{missing}/download"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(
            api,
            "GET",
            &format!("{ROOT}/exports/{id}/download"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    tag_reference_proof(api, pool, tokens[0].as_str(), tag_id, list_id).await;
}

async fn tag_reference_proof(api: &Router, pool: &PgPool, token: &str, tag_id: i64, list_id: i64) {
    let admin = Some(token);
    let options = request(
        api,
        "GET",
        &format!(
            "{ROOT}/imports/options?kind=tags&selected={tag_id},999999999&search=Transfer%20actual"
        ),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(options.0, StatusCode::OK, "{options:?}");
    assert_eq!(options.1["selected"][0]["id"], tag_id.to_string());
    assert_eq!(options.1["missing"], json!(["999999999"]));
    let list_options = request(
        api,
        "GET",
        &format!("{ROOT}/imports/options?kind=lists&selected={list_id}"),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(list_options.0, StatusCode::OK);
    assert_eq!(
        list_options.1["selected"][0]["title"],
        "Transfer actual list"
    );
    sqlx::query("INSERT INTO crm_tags(name,title,slug) SELECT 'Transfer option '||LPAD(n::text,3,'0'),'Transfer option '||LPAD(n::text,3,'0'),'transfer-option-'||n::text FROM generate_series(1,105) n").execute(pool).await.unwrap();
    let paged = request(
        api,
        "GET",
        &format!(
            "{ROOT}/imports/options?kind=tags&search=Transfer%20option%20&per_page=100&page=2"
        ),
        admin,
        Value::Null,
    )
    .await;
    assert_eq!(paged.0, StatusCode::OK, "{paged:?}");
    assert_eq!(paged.1["data"].as_array().unwrap().len(), 5);
    assert_eq!(paged.1["meta"]["total"], 105);
    assert_eq!(paged.1["meta"]["total_pages"], 2);
    let root = format!("{ROOT}/contact-tags/{tag_id}");
    let loaded = request(api, "GET", &root, admin, Value::Null).await;
    assert_eq!(loaded.0, StatusCode::OK);
    assert_eq!(
        loaded.1["contacts_count"], 2,
        "Tag counts use contact facts even though legacy counter is zero"
    );
    let shared:i64=sqlx::query_scalar("INSERT INTO crm_tags(name,title,slug) VALUES('Transfer legacy alias','Transfer actual tag','transfer-legacy-alias') RETURNING id").fetch_one(pool).await.unwrap();
    let rename =
        json!({"title":"Transfer renamed tag","description":"Atomic rename","color":"#123456"});
    assert_eq!(
        request(api, "PUT", &root, admin, rename.clone()).await.0,
        StatusCode::CONFLICT,
        "A shared legacy label cannot be split by guessing assignments"
    );
    assert_eq!(
        request(
            api,
            "DELETE",
            &format!("{ROOT}/contact-tags/{shared}"),
            admin,
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contacts WHERE tags?'Transfer actual tag'"
        )
        .fetch_one(pool)
        .await
        .unwrap(),
        2,
        "Deleting one duplicate label retains assignments for its surviving definition"
    );
    let renamed = request(api, "PUT", &root, admin, rename).await;
    assert_eq!(renamed.0, StatusCode::OK, "{renamed:?}");
    assert_eq!(renamed.1["contacts_count"], 2);
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contacts WHERE tags?'Transfer renamed tag'"
        )
        .fetch_one(pool)
        .await
        .unwrap(),
        2
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contacts WHERE tags?'Transfer actual tag'"
        )
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    sqlx::query("ALTER TABLE crm_operational_logs RENAME TO tag_audit_unavailable")
        .execute(pool)
        .await
        .unwrap();
    let rollback = request(
        api,
        "PUT",
        &root,
        admin,
        json!({"title":"Transfer rolled back tag","color":"#123456"}),
    )
    .await;
    sqlx::query("ALTER TABLE tag_audit_unavailable RENAME TO crm_operational_logs")
        .execute(pool)
        .await
        .unwrap();
    assert_eq!(rollback.0, StatusCode::INTERNAL_SERVER_ERROR);
    assert_eq!(
        request(api, "GET", &root, admin, Value::Null).await.1["tag"]["title"],
        "Transfer renamed tag"
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contacts WHERE tags?'Transfer renamed tag'"
        )
        .fetch_one(pool)
        .await
        .unwrap(),
        2
    );
    assert_eq!(
        request(
            api,
            "POST",
            &format!("{ROOT}/contact-tags"),
            admin,
            json!({"title":"Invalid transfer color","color":"red"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(api, "DELETE", &root, admin, Value::Null).await.0,
        StatusCode::OK
    );
    assert_eq!(
        sqlx::query_scalar::<_, i64>(
            "SELECT COUNT(*) FROM contacts WHERE tags?'Transfer renamed tag'"
        )
        .fetch_one(pool)
        .await
        .unwrap(),
        0
    );
    assert_eq!(
        request(api, "GET", &root, admin, Value::Null).await.0,
        StatusCode::NOT_FOUND
    );
    sqlx::query("DELETE FROM crm_tags WHERE slug LIKE 'transfer-option-%'")
        .execute(pool)
        .await
        .unwrap();
}
