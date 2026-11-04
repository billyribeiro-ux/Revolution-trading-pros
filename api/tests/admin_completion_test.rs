//! Disposable database proof for the admin completion work.
//! Run explicitly with ADMIN_VERIFY_DATABASE_URL pointing to an EMPTY database
//! whose name begins admin_move_verify. No dotenv files are loaded by this test.
use axum::{
    Router,
    body::Body,
    http::{Request, StatusCode},
};
use http_body_util::BodyExt;
use revolution_api::{
    AppState,
    cache::{CacheInvalidator, CacheService},
    config::Config,
    db::Database,
    routes,
    services::Services,
};
use serde_json::{Value, json};
use sqlx::postgres::PgPoolOptions;
use tower::ServiceExt;

async fn app(db: Database) -> Router {
    let mut config = Config::from_env().expect("isolated test configuration");
    config.database_url = std::env::var("ADMIN_VERIFY_DATABASE_URL").unwrap();
    config.redis_url = "redis://127.0.0.1:1".into();
    config.meilisearch_host = "http://127.0.0.1:1".into();
    config.r2_endpoint = "http://127.0.0.1:1".into();
    config.r2_public_url = "http://127.0.0.1:1/media".into();
    let services = Services::new(&config).await.unwrap();
    let cache = CacheService::new(None, None);
    let state = AppState {
        db,
        services,
        config,
        event_broadcaster: routes::realtime::EventBroadcaster::new(),
        ws_manager: routes::websocket::WsConnectionManager::new(),
        cache: cache.clone(),
        cache_invalidator: CacheInvalidator::new(cache),
    };
    Router::new()
        .nest("/api", routes::api_router())
        .with_state(state)
}

async fn request(
    app: &Router,
    method: &str,
    path: &str,
    token: Option<&str>,
    body: Value,
) -> (StatusCode, Value) {
    let mut builder = Request::builder()
        .method(method)
        .uri(path)
        .header("content-type", "application/json");
    if let Some(token) = token {
        builder = builder.header("authorization", format!("Bearer {token}"));
    }
    let response = app
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    let value = serde_json::from_slice(&bytes)
        .unwrap_or_else(|_| json!({"text": String::from_utf8_lossy(&bytes)}));
    (status, value)
}

async fn segment_case(
    api: &Router,
    token: &str,
    name: &str,
    conditions: Value,
    match_type: &str,
    expected: i64,
) -> Value {
    let (status,segment)=request(api,"POST","/api/admin/crm/segments",Some(token),json!({"title":format!("Segment DB {name}"),"conditions":conditions,"match_type":match_type})).await;
    assert_eq!(status, StatusCode::OK, "{name}: {segment}");
    assert_eq!(segment["contacts_count"], expected, "{name}: {segment}");
    let path = format!("/api/admin/crm/segments/{}/contacts", segment["id"]);
    let (status, members) = request(api, "GET", &path, Some(token), Value::Null).await;
    assert_eq!(status, StatusCode::OK, "{name}: {members}");
    assert_eq!(members["meta"]["total"], expected, "{name}: {members}");
    assert_eq!(members["data"].as_array().unwrap().len(), expected as usize);
    segment
}

#[tokio::test]
#[ignore = "requires an explicitly provisioned empty disposable database"]
async fn fresh_install_crm_persistence_media_states_and_migration_failure() {
    let url =
        std::env::var("ADMIN_VERIFY_DATABASE_URL").expect("ADMIN_VERIFY_DATABASE_URL is required");
    let parsed = reqwest::Url::parse(&url).unwrap();
    assert_eq!(parsed.host_str(), Some("127.0.0.1"), "local test DB only");
    assert!(
        parsed.path().starts_with("/admin_move_verify"),
        "disposable DB name required"
    );
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let count: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema='public'",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(count, 0, "fresh-install proof requires an empty database");
    let db = Database { pool: pool.clone() };
    db.migrate()
        .await
        .expect("first boot initializes baseline and forwards");
    db.migrate().await.expect("second boot is idempotent");
    let settings_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM cms_site_settings")
        .fetch_one(&pool)
        .await
        .unwrap();
    let stages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM crm_pipeline_stages")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(settings_count, 1);
    assert_eq!(stages, 6);
    let hash = revolution_api::utils::hash_password("OnlyForDisposableTest123!").unwrap();
    let mut tokens = Vec::new();
    for role in ["admin", "user"] {
        let id: i64 = sqlx::query_scalar("INSERT INTO users (email, name, password_hash, role, email_verified_at) VALUES ($1, 'Test', $2, $3, NOW()) RETURNING id")
            .bind(format!("{role}@admin-completion.test")).bind(&hash).bind(role).fetch_one(&pool).await.unwrap();
        tokens.push(
            revolution_api::utils::create_jwt(id, &std::env::var("JWT_SECRET").unwrap(), 1)
                .unwrap(),
        );
    }
    let api = app(db.clone()).await;
    let path = "/api/admin/crm/settings/double-optin";
    assert_eq!(
        request(&api, "GET", path, None, Value::Null).await.0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[1]),
            json!({"enabled": true})
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(&api, "GET", path, Some(&tokens[0]), Value::Null)
            .await
            .1["enabled"],
        false
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"enabled": true, "email_subject": "Fixture subject"})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"redirect_url": "javascript:alert(1)"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"enabled": "yes"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "POST",
            path,
            Some(&tokens[0]),
            json!({"unknown": true})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let ep = "/api/admin/crm/settings/email-preferences";
    assert_eq!(
        request(
            &api,
            "POST",
            ep,
            Some(&tokens[0]),
            json!({"title": "Fixture preferences", "show_tags": true})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "PUT",
            "/api/admin/crm/settings",
            Some(&tokens[0]),
            json!({"settings": {"fixture": 42}})
        )
        .await
        .0,
        StatusCode::OK
    );
    let companies_path = "/api/admin/crm/companies";
    let company_input = json!({"name":"Company fixture", "website":"https://example.test", "industry":"technology", "size":"11-50", "description":"Durable company notes", "logo_url":""});
    assert_eq!(
        request(
            &api,
            "POST",
            companies_path,
            Some(&tokens[1]),
            company_input.clone()
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        request(
            &api,
            "POST",
            companies_path,
            Some(&tokens[0]),
            json!({"name":" "})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "POST",
            companies_path,
            Some(&tokens[0]),
            json!({"name":"Invalid URL", "website":"javascript:alert(1)"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let (status, company) = request(
        &api,
        "POST",
        companies_path,
        Some(&tokens[0]),
        company_input.clone(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let company_id = company["id"].as_i64().unwrap();
    let company_path = format!("{companies_path}/{company_id}");
    assert_eq!(
        request(&api, "GET", &company_path, Some(&tokens[0]), Value::Null)
            .await
            .1["description"],
        "Durable company notes"
    );
    let mut updated_company = company_input.clone();
    updated_company["name"] = json!("Updated company fixture");
    updated_company["industry"] = json!("finance");
    assert_eq!(
        request(
            &api,
            "PUT",
            &company_path,
            Some(&tokens[0]),
            updated_company
        )
        .await
        .0,
        StatusCode::OK
    );
    let (status, duplicate) = request(
        &api,
        "POST",
        &format!("{company_path}/duplicate"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(duplicate["name"], "Updated company fixture (copy)");
    let first_page = request(
        &api,
        "GET",
        &format!("{companies_path}?search=Updated&industry=finance&per_page=1&page=1"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await
    .1;
    let second_page = request(
        &api,
        "GET",
        &format!("{companies_path}?search=Updated&industry=finance&per_page=1&page=2"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await
    .1;
    assert_eq!(first_page["meta"]["total"], 2);
    assert_eq!(first_page["meta"]["total_pages"], 2);
    assert_ne!(first_page["data"][0]["id"], second_page["data"][0]["id"]);
    assert_eq!(
        request(
            &api,
            "DELETE",
            &format!("{companies_path}/{}", duplicate["id"]),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "GET",
            &format!("{companies_path}/{}", duplicate["id"]),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let mut contact_ids = Vec::new();
    for email in [
        "list-member-one@completion.test",
        "list-member-two@completion.test",
    ] {
        let (status, contact) = request(
            &api,
            "POST",
            "/api/admin/crm/contacts",
            Some(&tokens[0]),
            json!({"email":email,"first_name":"List fixture"}),
        )
        .await;
        assert_eq!(status, StatusCode::OK);
        contact_ids.push(contact["id"].as_i64().unwrap());
    }
    let (status, list) = request(
        &api,
        "POST",
        "/api/admin/crm/lists",
        Some(&tokens[0]),
        json!({"title":"Durable list fixture","description":"Membership test","is_public":true}),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    let list_id = list["id"].as_i64().unwrap();
    let list_path = format!("/api/admin/crm/lists/{list_id}");
    let members_path = format!("{list_path}/contacts");
    let members_input =
        json!({"contact_ids":[contact_ids[0].to_string(),contact_ids[1],contact_ids[0]]});
    assert_eq!(
        request(
            &api,
            "POST",
            &members_path,
            Some(&tokens[1]),
            members_input.clone()
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let (status, added) = request(
        &api,
        "POST",
        &members_path,
        Some(&tokens[0]),
        members_input.clone(),
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(added["added_count"], 2);
    assert_eq!(
        request(&api, "POST", &members_path, Some(&tokens[0]), members_input)
            .await
            .1["added_count"],
        0
    );
    assert_eq!(
        request(
            &api,
            "POST",
            &members_path,
            Some(&tokens[0]),
            json!({"contact_ids":[contact_ids[0],999999999]})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    let members = request(
        &api,
        "GET",
        &format!("{members_path}?per_page=1&page=2"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await
    .1;
    assert_eq!(members["meta"]["total"], 2);
    assert_eq!(members["data"].as_array().unwrap().len(), 1);
    let global_members = request(
        &api,
        "GET",
        &format!("/api/admin/crm/contacts?list_id={list_id}"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await
    .1;
    assert_eq!(global_members["meta"]["total"], 2);
    assert_eq!(request(&api,"PUT",&list_path,Some(&tokens[0]),json!({"title":"Renamed list fixture","description":"Saved list description","is_public":false})).await.0,StatusCode::OK);
    let saved_list = request(&api, "GET", &list_path, Some(&tokens[0]), Value::Null)
        .await
        .1;
    assert_eq!(saved_list["list"]["title"], "Renamed list fixture");
    assert_eq!(saved_list["contacts_count"], 2);
    assert_eq!(
        request(
            &api,
            "DELETE",
            &members_path,
            Some(&tokens[0]),
            json!({"contact_ids":[contact_ids[0].to_string()]})
        )
        .await
        .1["removed_count"],
        1
    );
    assert_eq!(
        request(
            &api,
            "GET",
            &format!("/api/admin/crm/contacts/{}", contact_ids[0]),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "DELETE",
            &format!("/api/admin/crm/contacts/{}", contact_ids[1]),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(&api, "GET", &list_path, Some(&tokens[0]), Value::Null)
            .await
            .1["contacts_count"],
        0
    );
    assert_eq!(
        request(
            &api,
            "POST",
            &members_path,
            Some(&tokens[0]),
            json!({"contact_ids":[contact_ids[0]]})
        )
        .await
        .1["added_count"],
        1
    );
    assert_eq!(
        request(&api, "DELETE", &list_path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "GET",
            &format!("/api/admin/crm/contacts/{}", contact_ids[0]),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    // Template contracts must preserve HTML, design configuration and category metadata.
    let template_input = json!({"title":"Reusable fixture", "subject":"Hello", "content":"<h1>Fixture</h1>", "category":"fixture-category", "design_template":"visual", "template_config":{"blocks":[{"text":"Fixture"}]}, "is_default":true});
    let template_base = "/api/admin/crm/templates";
    assert_eq!(
        request(
            &api,
            "POST",
            template_base,
            Some(&tokens[1]),
            template_input.clone()
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    let (status, template) = request(
        &api,
        "POST",
        template_base,
        Some(&tokens[0]),
        template_input.clone(),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{template}");
    assert_eq!(template["content"], template_input["content"]);
    assert_eq!(
        template["template_config"],
        template_input["template_config"]
    );
    assert_eq!(template["is_default"], true);
    let template_id = template["id"].as_str().unwrap();
    let template_path = format!("{template_base}/{template_id}");
    let (status, categories) = request(
        &api,
        "GET",
        &format!("{template_base}/categories"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert!(
        categories
            .as_array()
            .unwrap()
            .iter()
            .any(|c| c["name"] == "fixture-category" && c["templates_count"] == 1)
    );
    let (_, empty) = request(
        &api,
        "GET",
        &format!("{template_base}?category=absent"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(empty["meta"]["total"], 0);
    let mut edited = template_input.clone();
    edited["content"] = json!("<p>Persisted edit</p>");
    assert_eq!(
        request(&api, "PUT", &template_path, Some(&tokens[0]), edited)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(&api, "GET", &template_path, Some(&tokens[0]), Value::Null)
            .await
            .1["content"],
        "<p>Persisted edit</p>"
    );
    sqlx::query("UPDATE crm_templates SET is_global=true WHERE id=$1")
        .bind(template_id.parse::<i64>().unwrap())
        .execute(&pool)
        .await
        .unwrap();
    let (_, prebuilt) = request(
        &api,
        "GET",
        &format!("{template_base}/prebuilt"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert!(
        prebuilt
            .as_array()
            .unwrap()
            .iter()
            .any(|t| t["id"] == template_id)
    );
    let (status, copy) = request(
        &api,
        "POST",
        &format!("{template_path}/duplicate"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{copy}");
    assert_eq!(copy["title"], "Reusable fixture (copy)");
    assert_eq!(copy["is_default"], false);
    assert_eq!(copy["is_global"], false);
    assert_eq!(copy["template_config"], template_input["template_config"]);
    let (_, second_page) = request(
        &api,
        "GET",
        &format!("{template_base}?category=fixture-category&per_page=1&page=2"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(second_page["meta"]["total"], 2);
    assert_eq!(second_page["data"].as_array().unwrap().len(), 1);
    let mut invalid = template_input.clone();
    invalid["content"] = json!("");
    assert_eq!(
        request(&api, "POST", template_base, Some(&tokens[0]), invalid)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "DELETE",
            &template_path,
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(&api, "GET", &template_path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::NOT_FOUND
    );
    // Run the actual recurring worker and observe durable effects, including retries.
    let content_id: uuid::Uuid=sqlx::query_scalar("INSERT INTO cms_content(content_type,slug,title) VALUES('page','scheduler-fixture','Scheduled fixture') RETURNING id").fetch_one(&pool).await.unwrap();
    let schedule_id: uuid::Uuid=sqlx::query_scalar("INSERT INTO cms_schedules(content_id,action,scheduled_at,created_at) VALUES($1,'publish',NOW()-INTERVAL '1 minute',NOW()-INTERVAL '2 minutes') RETURNING id").bind(content_id).fetch_one(&pool).await.unwrap();
    let future_id: uuid::Uuid=sqlx::query_scalar("INSERT INTO cms_schedules(content_id,action,scheduled_at) VALUES($1,'archive',NOW()+INTERVAL '1 day') RETURNING id").bind(content_id).fetch_one(&pool).await.unwrap();
    let worker = tokio::spawn(revolution_api::services::cms_scheduler::run(pool.clone()));
    tokio::time::timeout(std::time::Duration::from_secs(12), async {
        loop {
            let status: String =
                sqlx::query_scalar("SELECT status::text FROM cms_schedules WHERE id=$1")
                    .bind(schedule_id)
                    .fetch_one(&pool)
                    .await
                    .unwrap();
            if status == "completed" {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("recurring worker must execute due content");
    worker.abort();
    let _ = worker.await;
    let (content_status, version): (String, i32) =
        sqlx::query_as("SELECT status::text,version FROM cms_content WHERE id=$1")
            .bind(content_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!((content_status.as_str(), version), ("published", 2));
    let future_status: String =
        sqlx::query_scalar("SELECT status::text FROM cms_schedules WHERE id=$1")
            .bind(future_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(future_status, "pending");
    let invalid_id:uuid::Uuid=sqlx::query_scalar("INSERT INTO cms_schedules(content_id,action,staged_data,scheduled_at,created_at) VALUES($1,'update','{}'::jsonb,NOW()-INTERVAL '1 minute',NOW()-INTERVAL '2 minutes') RETURNING id").bind(content_id).fetch_one(&pool).await.unwrap();
    let (first, second) = tokio::join!(
        revolution_api::services::cms_scheduler::process_schedules(&pool),
        revolution_api::services::cms_scheduler::process_schedules(&pool)
    );
    assert_eq!(
        first.unwrap().processed + second.unwrap().processed,
        1,
        "two workers claim each schedule once"
    );
    let (status,retries,error,future):(String,i32,Option<String>,bool)=sqlx::query_as("SELECT status::text,retry_count,error_message,scheduled_at>NOW() FROM cms_schedules WHERE id=$1").bind(invalid_id).fetch_one(&pool).await.unwrap();
    assert_eq!((status.as_str(), retries, future), ("pending", 1, true));
    assert!(error.is_some());
    let release_id:uuid::Uuid=sqlx::query_scalar("INSERT INTO cms_releases(name,status,scheduled_at,notify_on_complete) VALUES('Fixture release','scheduled',NOW()-INTERVAL '1 minute',false) RETURNING id").fetch_one(&pool).await.unwrap();
    sqlx::query("INSERT INTO cms_release_items(release_id,content_id,action,staged_data) VALUES($1,$2,'update',$3)").bind(release_id).bind(content_id).bind(json!({"title":"Released fixture","content":"Persisted scheduled HTML"})).execute(&pool).await.unwrap();
    assert_eq!(
        revolution_api::services::cms_scheduler::process_releases(&pool)
            .await
            .unwrap()
            .successful,
        1
    );
    let (release_status, completed): (String, i32) =
        sqlx::query_as("SELECT status::text,completed_items FROM cms_releases WHERE id=$1")
            .bind(release_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!((release_status.as_str(), completed), ("completed", 1));
    let content_title: String = sqlx::query_scalar("SELECT title FROM cms_content WHERE id=$1")
        .bind(content_id)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(content_title, "Released fixture");
    assert!(
        std::env::var("POSTMARK_TOKEN").is_err() && std::env::var("POSTMARK_API_KEY").is_err(),
        "this proof must never send real emails"
    );
    let delayed = revolution_api::queue::enqueue_delayed(
        &pool,
        "fixture",
        "unknown-fixture-job",
        json!({}),
        chrono::Utc::now() + chrono::Duration::hours(1),
    )
    .await
    .unwrap();
    assert!(
        revolution_api::queue::worker::process_next_job(&db)
            .await
            .unwrap()
            .is_none()
    );
    let email_job = revolution_api::queue::enqueue(
        &pool,
        "fixture",
        "send_email",
        json!({"to":"worker@completion.test","subject":"Fixture","body":"Fixture"}),
    )
    .await
    .unwrap();
    assert_eq!(
        revolution_api::queue::worker::process_next_job(&db)
            .await
            .unwrap(),
        Some(email_job)
    );
    let (status, attempts, error): (String, i32, Option<String>) =
        sqlx::query_as("SELECT status,attempts,error FROM jobs WHERE id=$1")
            .bind(email_job)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!((status.as_str(), attempts), ("pending", 1));
    assert!(error.unwrap().contains("Postmark is unavailable"));
    sqlx::query("UPDATE jobs SET available_at=NOW(),max_attempts=2 WHERE id=$1")
        .bind(email_job)
        .execute(&pool)
        .await
        .unwrap();
    revolution_api::queue::worker::process_next_job(&db)
        .await
        .unwrap();
    let status: String = sqlx::query_scalar("SELECT status FROM jobs WHERE id=$1")
        .bind(email_job)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(status, "failed");
    sqlx::query("UPDATE jobs SET available_at=NOW(),max_attempts=1 WHERE id=$1")
        .bind(delayed)
        .execute(&pool)
        .await
        .unwrap();
    revolution_api::queue::worker::process_next_job(&db)
        .await
        .unwrap();
    let (status, error): (String, Option<String>) =
        sqlx::query_as("SELECT status,error FROM jobs WHERE id=$1")
            .bind(delayed)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "failed");
    assert!(error.unwrap().contains("Unknown job type"));
    let product_id:i64=sqlx::query_scalar("INSERT INTO products(name,slug,type) VALUES('Scheduled product fixture','scheduled-product-fixture','course') RETURNING id").fetch_one(&pool).await.unwrap();
    let legacy_id:i64=sqlx::query_scalar("INSERT INTO scheduled_content(content_type,content_id,scheduled_action,scheduled_at) VALUES('product',$1,'unpublish',NOW()-INTERVAL '1 minute') RETURNING id").bind(product_id).fetch_one(&pool).await.unwrap();
    let legacy_update:i64=sqlx::query_scalar("INSERT INTO scheduled_content(content_type,content_id,scheduled_action,scheduled_at,payload) VALUES('product',$1,'update',NOW()-INTERVAL '1 minute',$2) RETURNING id").bind(product_id).bind(json!({"name":"Updated scheduled product"})).fetch_one(&pool).await.unwrap();
    let legacy_unknown:i64=sqlx::query_scalar("INSERT INTO scheduled_content(content_type,content_id,scheduled_action,scheduled_at) VALUES('unknown-fixture',1,'publish',NOW()-INTERVAL '1 minute') RETURNING id").fetch_one(&pool).await.unwrap();
    let webhook_id:i64=sqlx::query_scalar("INSERT INTO webhooks(name,url) VALUES('Blocked local fixture','https://127.0.0.1:1/webhook') RETURNING id").fetch_one(&pool).await.unwrap();
    let delivery_id:i64=sqlx::query_scalar("INSERT INTO webhook_deliveries(webhook_id,event_type,payload) VALUES($1,'fixture.event','{}') RETURNING id").bind(webhook_id).fetch_one(&pool).await.unwrap();
    let expired_id:i64=sqlx::query_scalar("INSERT INTO webhook_deliveries(webhook_id,event_type,payload,status,attempt_count,next_retry_at) VALUES($1,'fixture.expired','{}','processing',3,NOW()-INTERVAL '1 minute') RETURNING id").bind(webhook_id).fetch_one(&pool).await.unwrap();
    let worker = tokio::spawn(revolution_api::queue::worker::run(db.clone()));
    tokio::time::timeout(std::time::Duration::from_secs(18), async {
        loop {
            let pending: i64 = sqlx::query_scalar(
                "SELECT COUNT(*) FROM scheduled_content WHERE status='scheduled'",
            )
            .fetch_one(&pool)
            .await
            .unwrap();
            if pending == 0 {
                break;
            }
            tokio::time::sleep(std::time::Duration::from_millis(100)).await;
        }
    })
    .await
    .expect("actual queue worker executes legacy schedules");
    worker.abort();
    let _ = worker.await;
    let (name, active): (String, bool) =
        sqlx::query_as("SELECT name,is_active FROM products WHERE id=$1")
            .bind(product_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        (name.as_str(), active),
        ("Updated scheduled product", false)
    );
    for id in [legacy_id, legacy_update] {
        let status: String = sqlx::query_scalar("SELECT status FROM scheduled_content WHERE id=$1")
            .bind(id)
            .fetch_one(&pool)
            .await
            .unwrap();
        assert_eq!(status, "executed");
    }
    let (status, error): (String, Option<String>) =
        sqlx::query_as("SELECT status,error_message FROM scheduled_content WHERE id=$1")
            .bind(legacy_unknown)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "failed");
    assert!(error.unwrap().contains("Unsupported content type"));
    let (status,attempts,compat_attempts,error,delivered):(String,i32,i32,Option<String>,Option<chrono::DateTime<chrono::Utc>>)=sqlx::query_as("SELECT status,attempt_count,attempts,error_message,delivered_at FROM webhook_deliveries WHERE id=$1").bind(delivery_id).fetch_one(&pool).await.unwrap();
    assert_eq!(
        (status.as_str(), attempts, compat_attempts),
        ("failed", 1, 1)
    );
    assert!(error.unwrap().contains("SSRF"));
    assert!(delivered.is_none());
    let (status, error): (String, Option<String>) =
        sqlx::query_as("SELECT status,error_message FROM webhook_deliveries WHERE id=$1")
            .bind(expired_id)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(status, "failed");
    assert!(error.unwrap().contains("lease expired"));
    let contacts_base = "/api/admin/crm/contacts";
    for invalid in [
        json!({"email":"invalid"}),
        json!({"email":"validation@completion.test","phone":"x".repeat(51)}),
        json!({"email":"validation@completion.test","status":"invalid"}),
    ] {
        assert_eq!(
            request(&api, "POST", contacts_base, Some(&tokens[0]), invalid)
                .await
                .0,
            StatusCode::BAD_REQUEST
        );
    }
    let (status,contact)=request(&api,"POST",contacts_base,Some(&tokens[0]),json!({"email":"editable@completion.test","company":"Original fixture","tags":["fixture"],"custom_fields":{"preserve":true}})).await;
    assert_eq!(status, StatusCode::OK);
    let edit_path = format!("{contacts_base}/{}", contact["id"]);
    assert_eq!(
        request(
            &api,
            "PUT",
            &edit_path,
            Some(&tokens[0]),
            json!({"email":"invalid"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "PUT",
            &edit_path,
            Some(&tokens[0]),
            json!({"company":"Updated contact fixture","status":"prospect"})
        )
        .await
        .0,
        StatusCode::OK
    );
    let (_, edited) = request(&api, "GET", &edit_path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(edited["company"], "Updated contact fixture");
    assert_eq!(edited["tags"], json!(["fixture"]));
    assert_eq!(edited["custom_fields"]["preserve"], true);
    let (status,lead)=request(&api,"POST","/api/admin/crm/leads",Some(&tokens[0]),json!({"email":"editable-lead@completion.test","company_name":"Alias fixture","status":"new"})).await;
    assert_eq!(status, StatusCode::OK);
    let lead_path = format!("/api/admin/crm/leads/{}", lead["id"].as_str().unwrap());
    let (status, lead) = request(&api, "GET", &lead_path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(lead["company_name"], "Alias fixture");
    assert_eq!(
        request(
            &api,
            "PUT",
            &lead_path,
            Some(&tokens[0]),
            json!({"status":"customer"})
        )
        .await
        .0,
        StatusCode::BAD_REQUEST
    );
    assert_eq!(
        request(
            &api,
            "PUT",
            &lead_path,
            Some(&tokens[0]),
            json!({"status":"qualified"})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(&api, "GET", &lead_path, Some(&tokens[0]), Value::Null)
            .await
            .1["status"],
        "qualified"
    );
    sqlx::query("INSERT INTO crm_deals(name,pipeline_id,stage_id,amount,currency,status) SELECT 'Stats fixture',p.id,s.id,12.34,'USD','open' FROM crm_pipelines p JOIN crm_pipeline_stages s ON s.pipeline_id=p.id ORDER BY p.id,s.id LIMIT 1").execute(&pool).await.unwrap();
    let (status, stats) = request(
        &api,
        "GET",
        "/api/admin/crm/stats",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{stats}");
    assert_eq!(stats["open_deals"], 1);
    assert_eq!(stats["deals_value_cents"], 1234);
    assert_eq!(stats["deal_value"], json!(12.34));
    sqlx::query("ALTER TABLE crm_campaigns RENAME TO crm_campaigns_unavailable_fixture")
        .execute(&pool)
        .await
        .unwrap();
    let unavailable = request(
        &api,
        "GET",
        "/api/admin/crm/stats",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    sqlx::query("ALTER TABLE crm_campaigns_unavailable_fixture RENAME TO crm_campaigns")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        unavailable.0,
        StatusCode::INTERNAL_SERVER_ERROR,
        "statistics failure must not be fabricated zeroes"
    );
    let mut segment_contacts = Vec::new();
    for (suffix, status, tags, phone, score) in [
        ("a", "lead", json!(["segment-alpha"]), Value::Null, 10),
        ("b", "prospect", json!(["segment-alpha"]), json!("123"), 80),
        ("c", "customer", json!(["beta"]), Value::Null, 90),
    ] {
        let (status_code,contact)=request(&api,"POST",contacts_base,Some(&tokens[0]),json!({"email":format!("seg-fixture-{suffix}@completion.test"),"status":status,"source":"segment-fixture","tags":tags,"phone":phone})).await;
        assert_eq!(status_code, StatusCode::OK, "{contact}");
        let id = contact["id"].as_i64().unwrap();
        segment_contacts.push(id);
        sqlx::query("UPDATE contacts SET score=$2 WHERE id=$1")
            .bind(id)
            .bind(score)
            .execute(&pool)
            .await
            .unwrap();
    }
    let source = json!({"field":"source","operator":"equals","value":"segment-fixture"});
    let segment = segment_case(
        &api,
        &tokens[0],
        "numeric",
        json!([source,{"field":"score","operator":"greater_than","value":50}]),
        "all",
        2,
    )
    .await;
    let segment_path = format!("/api/admin/crm/segments/{}", segment["id"]);
    assert_eq!(
        request(
            &api,
            "POST",
            "/api/admin/crm/segments",
            Some(&tokens[1]),
            json!({"title":"Denied segment"})
        )
        .await
        .0,
        StatusCode::FORBIDDEN
    );
    segment_case(
        &api,
        &tokens[0],
        "text",
        json!([source,{"field":"email","operator":"contains","value":"SEG-FIXTURE-"}]),
        "all",
        3,
    )
    .await;
    segment_case(
        &api,
        &tokens[0],
        "literal-text",
        json!([source,{"field":"email","operator":"contains","value":"_%"}]),
        "all",
        0,
    )
    .await;
    segment_case(
        &api,
        &tokens[0],
        "null",
        json!([source,{"field":"phone","operator":"is_null"}]),
        "all",
        2,
    )
    .await;
    segment_case(
        &api,
        &tokens[0],
        "membership",
        json!([source,{"field":"status","operator":"in","value":["lead","prospect"]}]),
        "all",
        2,
    )
    .await;
    segment_case(
        &api,
        &tokens[0],
        "tags",
        json!([source,{"field":"tags","operator":"contains","value":"segment-alpha"}]),
        "all",
        2,
    )
    .await;
    segment_case(
        &api,
        &tokens[0],
        "tag-list",
        json!([source,{"field":"tags","operator":"in","value":["segment-alpha","beta"]}]),
        "all",
        3,
    )
    .await;
    segment_case(
        &api,
        &tokens[0],
        "between",
        json!([source,{"field":"score","operator":"between","value":[10,80]}]),
        "all",
        2,
    )
    .await;
    segment_case(
        &api,
        &tokens[0],
        "date",
        json!([source,{"field":"created_at","operator":"greater_than","value":"2000-01-01"}]),
        "all",
        3,
    )
    .await;
    segment_case(&api,&tokens[0],"any",json!([{"field":"email","operator":"equals","value":"seg-fixture-a@completion.test"},{"field":"email","operator":"equals","value":"seg-fixture-b@completion.test"}]),"any",2).await;
    segment_case(&api,&tokens[0],"mixed",json!([source,{"field":"score","operator":"greater_than","value":50,"logic":"or"},{"field":"email","operator":"equals","value":"seg-fixture-a@completion.test","logic":"and"}]),"all",1).await;
    segment_case(
        &api,
        &tokens[0],
        "sql-value",
        json!([{"field":"email","operator":"equals","value":"x' OR TRUE --"}]),
        "all",
        0,
    )
    .await;
    assert_eq!(request(&api,"POST","/api/admin/crm/segments",Some(&tokens[0]),json!({"title":"Rejected rule","conditions":[{"field":"email; DELETE FROM contacts; --","operator":"equals","value":"x"}]})).await.0,StatusCode::BAD_REQUEST);
    let (_, page2) = request(
        &api,
        "GET",
        &format!("{segment_path}/contacts?per_page=1&page=2"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(page2["meta"]["total"], 2);
    assert_eq!(page2["data"].as_array().unwrap().len(), 1);
    sqlx::query("UPDATE contacts SET score=0 WHERE id=$1")
        .bind(segment_contacts[2])
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(&api, "GET", &segment_path, Some(&tokens[0]), Value::Null)
            .await
            .1["contacts_count"],
        1,
        "dynamic counts reflect current contact values without a fake sync"
    );
    let (_, synced) = request(
        &api,
        "POST",
        &format!("{segment_path}/sync"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(synced["contacts_count"], 1);
    assert!(synced["last_synced_at"].is_string());
    let stored: i32 = sqlx::query_scalar("SELECT contacts_count FROM crm_segments WHERE id=$1")
        .bind(segment["id"].as_i64().unwrap())
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(stored, 1);
    let (status, updated) = request(
        &api,
        "PUT",
        &segment_path,
        Some(&tokens[0]),
        json!({"title":"Segment DB renamed"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{updated}");
    assert_eq!(updated["conditions"], segment["conditions"]);
    let (status, copy) = request(
        &api,
        "POST",
        &format!("{segment_path}/duplicate"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{copy}");
    assert_eq!(copy["is_active"], false);
    assert_eq!(copy["contacts_count"], 1);
    let (_, page2) = request(
        &api,
        "GET",
        "/api/admin/crm/segments?search=Segment%20DB&per_page=1&page=2",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(page2["meta"]["total"], 13);
    assert_eq!(page2["data"].as_array().unwrap().len(), 1);
    assert_eq!(
        request(&api, "DELETE", &segment_path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "GET",
            &format!("{contacts_base}/{}", segment_contacts[0]),
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(request(&api,"POST","/api/admin/crm/segments",Some(&tokens[0]),json!({"title":"Rejected range","conditions":[{"field":"score","operator":"between","value":[80,10]}]})).await.0,StatusCode::BAD_REQUEST);
    let legacy_empty:i64=sqlx::query_scalar("INSERT INTO crm_segments(name,conditions,is_active,contacts_count) VALUES('Legacy empty fixture',NULL,NULL,777) RETURNING id").fetch_one(&pool).await.unwrap();
    let legacy_bad:i64=sqlx::query_scalar("INSERT INTO crm_segments(name,conditions,contacts_count) VALUES('Legacy invalid fixture',$1,777) RETURNING id").bind(json!([{"field":"unsupported_legacy_field","operator":"equals","value":"x"}])).fetch_one(&pool).await.unwrap();
    let (_, legacy) = request(
        &api,
        "GET",
        &format!("/api/admin/crm/segments/{legacy_empty}"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    let actual_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM contacts")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(legacy["contacts_count"], actual_count);
    assert_eq!(legacy["conditions"], json!([]));
    let legacy_path = format!("/api/admin/crm/segments/{legacy_bad}");
    let (status, legacy) = request(&api, "GET", &legacy_path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert!(legacy["contacts_count"].is_null());
    assert!(legacy["validation_error"].is_string());
    let (_, definitions) = request(
        &api,
        "GET",
        &format!("{legacy_path}?definition=true"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(definitions["title"], "Legacy invalid fixture");
    assert!(definitions["conditions"].is_array());
    let (_, listing) = request(
        &api,
        "GET",
        "/api/admin/crm/segments?search=Legacy&is_active=true",
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(listing["meta"]["total"], 2);
    assert_eq!(listing["data"].as_array().unwrap().len(), 2);
    let (status, repaired) = request(
        &api,
        "PUT",
        &legacy_path,
        Some(&tokens[0]),
        json!({"conditions":[source],"match_type":"all"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{repaired}");
    assert_eq!(repaired["contacts_count"], 3);
    assert!(repaired["validation_error"].is_null());
    let batch = json!({"session_id":"fixture-session", "visitor_id":"fixture-visitor", "user_id":"999999", "events":[
        {"event_type":"page_view", "timestamp":chrono::Utc::now().timestamp_millis(), "page_url":"https://example.test/pricing?email=private@example.test#token"},
        {"event_type":"click", "timestamp":chrono::Utc::now().timestamp_millis(), "page_url":"https://example.test/pricing", "element_selector":"#subscribe", "event_value":"private form value", "coordinates":{"x":12,"y":24}},
        {"event_type":"scroll_depth", "timestamp":chrono::Utc::now().timestamp_millis(), "page_url":"https://example.test/pricing", "event_value":75}
    ]});
    let (status, ingested) =
        request(&api, "POST", "/api/behavior/events", None, batch.clone()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(ingested["persisted"], 3);
    let retry = request(&api, "POST", "/api/behavior/events", None, batch.clone())
        .await
        .1;
    assert_eq!(
        retry["persisted"], 0,
        "retries must not double-count events"
    );
    let mut invalid_scroll = batch.clone();
    invalid_scroll["events"][2]["event_value"] = json!(101);
    assert_eq!(
        request(&api, "POST", "/api/behavior/events", None, invalid_scroll)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let mut invalid = batch.clone();
    invalid["events"][1]["event_type"] = json!("unknown-event");
    assert_eq!(
        request(&api, "POST", "/api/behavior/events", None, invalid)
            .await
            .0,
        StatusCode::BAD_REQUEST
    );
    let summary_path = "/api/analytics/behavior?period=7d";
    assert_eq!(
        request(&api, "GET", summary_path, None, Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(&api, "GET", summary_path, Some(&tokens[1]), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    let (status, summary) = request(&api, "GET", summary_path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(summary["total_sessions"], 1);
    assert_eq!(summary["heatmap_views"], 0, "clicks are not heatmap views");
    assert_eq!(summary["avg_scroll_depth"], 75.0);
    assert_eq!(summary["top_clicks"][0]["element"], "#subscribe");
    let saved: Vec<(String, Option<f64>)> =
        sqlx::query_as("SELECT page_url,event_value FROM behavior_events ORDER BY id")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(saved.len(), 3, "invalid batches must not partially persist");
    assert!(
        saved.iter().all(|(path, _)| path == "/pricing"),
        "strip sensitive query/fragment data"
    );
    assert!(saved[1].1.is_none(), "never store form input values");
    drop(api);
    // Rebuild services/router and reopen the DB pool, proving state does not
    // depend on process-local memory or the old connection pool.
    pool.close().await;
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await
        .unwrap();
    let db = Database { pool: pool.clone() };
    let api = app(db.clone()).await;
    let (status, doi) = request(&api, "GET", path, Some(&tokens[0]), Value::Null).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(doi["enabled"], true);
    assert_eq!(doi["email_subject"], "Fixture subject");
    assert_eq!(
        request(&api, "GET", ep, Some(&tokens[0]), Value::Null)
            .await
            .1["title"],
        "Fixture preferences"
    );
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/admin/crm/settings",
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .1["fixture"],
        42
    );
    let id: i64 = sqlx::query_scalar("INSERT INTO media (filename, path, size) VALUES ('fixture.txt', 'fixtures/test.txt', 12) RETURNING id")
        .fetch_one(&pool).await.unwrap();
    let detail = format!("/api/admin/media/{id}");
    assert_eq!(
        request(&api, "GET", &detail, Some(&tokens[0]), Value::Null)
            .await
            .1["data"]["scan_status"],
        "pending"
    );
    let (status, result) = request(
        &api,
        "POST",
        &format!("/api/admin/media/scan/{id}"),
        Some(&tokens[0]),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(result["success"], false);
    assert_eq!(result["data"]["scan_result"]["status"], "unavailable");
    assert_eq!(result["data"]["scan_result"]["is_clean"], false);
    let stored = request(&api, "GET", &detail, Some(&tokens[0]), Value::Null)
        .await
        .1;
    assert_eq!(stored["data"]["scan_status"], "unavailable");
    assert!(stored["data"]["scan_result"]["scanned_at"].is_null());
    assert_eq!(
        request(
            &api,
            "GET",
            "/api/analytics/behavior?period=7d",
            Some(&tokens[0]),
            Value::Null
        )
        .await
        .1["total_sessions"],
        1
    );
    for role in [
        "admin",
        "administrator",
        "developer",
        "super-admin",
        "super_admin",
        "superadmin",
    ] {
        sqlx::query("UPDATE users SET role=$1 WHERE email='admin@admin-completion.test'")
            .bind(role)
            .execute(&pool)
            .await
            .unwrap();
        assert_eq!(
            request(&api, "GET", path, Some(&tokens[0]), Value::Null)
                .await
                .0,
            StatusCode::OK,
            "admin role {role}"
        );
    }
    sqlx::query("UPDATE users SET role='editor' WHERE email='admin@admin-completion.test'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(&api, "GET", path, Some(&tokens[0]), Value::Null)
            .await
            .0,
        StatusCode::FORBIDDEN
    );
    sqlx::query("UPDATE users SET role='admin' WHERE email='admin@admin-completion.test'")
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        request(&api, "GET", &company_path, Some(&tokens[0]), Value::Null)
            .await
            .1["name"],
        "Updated company fixture"
    );
    // A corrupt migration checksum must fail readiness, never be ignored.
    let checksum: Vec<u8> =
        sqlx::query_scalar("SELECT checksum FROM _sqlx_migrations WHERE version=74")
            .fetch_one(&pool)
            .await
            .unwrap();
    sqlx::query("UPDATE _sqlx_migrations SET checksum = '\\x00'::bytea WHERE version=74")
        .execute(&pool)
        .await
        .unwrap();
    let failure = db
        .migrate()
        .await
        .expect_err("required migration mismatch must abort");
    assert!(failure.to_string().contains("startup aborted"));
    sqlx::query("UPDATE _sqlx_migrations SET checksum=$1 WHERE version=74")
        .bind(checksum)
        .execute(&pool)
        .await
        .unwrap();
    db.migrate().await.unwrap();
}
