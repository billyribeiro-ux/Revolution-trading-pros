//! Real router and PostgreSQL proof for public/admin forms and popups.
//! Run only against an empty local database named marketing_verify*.
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

async fn request(
    api: &Router,
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
    let response = api
        .clone()
        .oneshot(builder.body(Body::from(body.to_string())).unwrap())
        .await
        .unwrap();
    let status = response.status();
    let bytes = response.into_body().collect().await.unwrap().to_bytes();
    (
        status,
        serde_json::from_slice(&bytes).unwrap_or(Value::Null),
    )
}

#[tokio::test]
#[ignore = "requires an explicitly provisioned empty disposable database"]
async fn forms_and_popups_work_through_real_routes() {
    let url = std::env::var("MARKETING_VERIFY_DATABASE_URL")
        .expect("MARKETING_VERIFY_DATABASE_URL required");
    let parsed = reqwest::Url::parse(&url).unwrap();
    assert_eq!(parsed.host_str(), Some("127.0.0.1"));
    assert!(parsed.path().starts_with("/marketing_verify"));
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
    assert_eq!(count, 0, "database must be empty");
    let db = Database { pool: pool.clone() };
    db.migrate().await.unwrap();

    let mut config = Config::from_env().unwrap();
    config.database_url = url;
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
    let api = Router::new()
        .nest("/api", routes::api_router())
        .with_state(state);
    let hash = revolution_api::utils::hash_password("DisposableOnly123!").unwrap();
    let admin_id: i64 = sqlx::query_scalar("INSERT INTO users (email,name,password_hash,role,email_verified_at) VALUES ('admin@marketing.test','Test',$1,'admin',NOW()) RETURNING id")
        .bind(&hash).fetch_one(&pool).await.unwrap();
    let secret = std::env::var("JWT_SECRET").unwrap();
    let admin = revolution_api::utils::create_jwt(admin_id, &secret, 1).unwrap();

    assert_eq!(
        request(&api, "GET", "/api/admin/forms", None, Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        request(&api, "GET", "/api/admin/popups", None, Value::Null)
            .await
            .0,
        StatusCode::UNAUTHORIZED
    );

    let (status, form) = request(&api, "POST", "/api/admin/forms", Some(&admin), json!({
        "name": "Marketing Contact", "fields": [{"name":"email","label":"Email","field_type":"email","required":true}],
        "settings": {}, "is_published": true
    })).await;
    assert_eq!(status, StatusCode::OK, "{form}");
    let form_id = form["data"]["id"].as_i64().unwrap();
    let slug = form["data"]["slug"].as_str().unwrap();
    let (status, public_form) = request(
        &api,
        "GET",
        &format!("/api/forms/{slug}"),
        None,
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{public_form}");
    assert_eq!(public_form["id"], form_id);
    let (status, submission) = request(
        &api,
        "POST",
        &format!("/api/forms/{slug}/submit"),
        None,
        json!({"email":"reader@example.test"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{submission}");
    assert_eq!(submission["success"], true);
    assert!(submission["submission_id"].is_string());
    let submission_id = submission["submission_id"].as_str().unwrap();
    let (status, entries) = request(
        &api,
        "GET",
        &format!("/api/admin/forms/{form_id}/submissions?per_page=10"),
        Some(&admin),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{entries}");
    assert_eq!(entries["meta"]["total"], 1);
    assert_eq!(
        entries["submissions"][0]["data"]["email"],
        "reader@example.test"
    );
    assert_eq!(
        request(
            &api,
            "PUT",
            &format!("/api/admin/forms/{form_id}/submissions/{submission_id}/status"),
            Some(&admin),
            json!({"status":"read"})
        )
        .await
        .0,
        StatusCode::OK
    );

    let (status, numeric_form) = request(
        &api,
        "POST",
        "/api/admin/forms",
        Some(&admin),
        json!({
            "name":"Required Numeric Answers",
            "fields":[
                {"name":"quantity","label":"Quantity","field_type":"number","required":true,"validation":{"min":0}},
                {"name":"enabled","label":"Enabled","field_type":"toggle","required":true}
            ],
            "settings":{},"is_published":true
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{numeric_form}");
    let numeric_slug = numeric_form["data"]["slug"].as_str().unwrap();
    let (status, accepted) = request(
        &api,
        "POST",
        &format!("/api/forms/{numeric_slug}/submit"),
        None,
        json!({"quantity":0,"enabled":false}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{accepted}");
    let (status, rejected) = request(
        &api,
        "POST",
        &format!("/api/forms/{numeric_slug}/submit"),
        None,
        json!({"quantity":-1,"enabled":false}),
    )
    .await;
    assert_eq!(status, StatusCode::BAD_REQUEST, "{rejected}");
    assert_eq!(
        rejected["errors"]["quantity"][0],
        "Quantity must be at least 0"
    );

    let (status, popup) = request(
        &api,
        "POST",
        "/api/admin/popups",
        Some(&admin),
        json!({
            "name":"Welcome", "type":"timed", "status":"published", "title":"Join us",
            "content":"A short invitation", "has_form":true, "form_id":form_id,
            "trigger_rules":{"delay":0}, "frequency_rules":{"frequency":"once_per_session"},
            "display_rules":{"devices":["desktop"],"include_pages":["/blog/*"]}
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{popup}");
    let popup_id = popup["id"].as_i64().unwrap();
    let (status, active) = request(
        &api,
        "GET",
        "/api/popups/active?page=%2Fblog%2Farticle&device=desktop",
        None,
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{active}");
    assert_eq!(active["popups"][0]["id"], popup_id);
    assert_eq!(active["popups"][0]["form_id"], form_id);
    let (_, excluded) = request(
        &api,
        "GET",
        "/api/popups/active?page=%2Fpricing&device=desktop",
        None,
        Value::Null,
    )
    .await;
    assert_eq!(excluded["popups"].as_array().unwrap().len(), 0);
    let (status, impression) = request(
        &api,
        "POST",
        &format!("/api/popups/{popup_id}/impression"),
        None,
        json!({"page_url":"/blog/article?email=secret@example.test","device_type":"desktop","session_id":"bad@example.test"}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{impression}");
    let (status, conversion) = request(
        &api,
        "POST",
        &format!("/api/popups/{popup_id}/conversion"),
        None,
        json!({"page_url":"/blog/article?token=private","device_type":"desktop","metadata":{"email":"secret@example.test"}}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{conversion}");
    let (status, popup_submission) = request(
        &api,
        "POST",
        &format!("/api/popups/{popup_id}/form-submit"),
        None,
        json!({"formData":{"email":"popup@example.test"}}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{popup_submission}");
    let totals: (i64, i64) =
        sqlx::query_as("SELECT total_views, total_conversions FROM popups WHERE id=$1")
            .bind(popup_id as i32)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(totals, (1, 2));
    let stored_events: Vec<(Option<String>, Option<String>, Option<Value>)> = sqlx::query_as(
        "SELECT page_url, session_id, metadata FROM popup_events WHERE popup_id=$1 ORDER BY id",
    )
    .bind(popup_id as i32)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(stored_events[0].0.as_deref(), Some("/blog/article"));
    assert_eq!(stored_events[0].1, None);
    assert_eq!(stored_events[1].0.as_deref(), Some("/blog/article"));
    assert_eq!(stored_events[1].2, Some(json!({})));
    assert_eq!(
        request(
            &api,
            "POST",
            "/api/popups/999999/impression",
            None,
            json!({})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );

    let (status, updated) = request(&api, "PUT", &format!("/api/admin/popups/{popup_id}"), Some(&admin), json!({
        "trigger_rules":{"delay":7000}, "display_rules":{"devices":["mobile"],"include_pages":["/pricing"]},
        "design":{"background_color":"#fafafa"}, "cta_text":"Join"
    })).await;
    assert_eq!(status, StatusCode::OK, "{updated}");
    let (status, detail) = request(
        &api,
        "GET",
        &format!("/api/admin/popups/{popup_id}"),
        Some(&admin),
        Value::Null,
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{detail}");
    assert_eq!(detail["popup"]["trigger_rules"]["delay"], 7000);
    assert_eq!(detail["popup"]["design"]["background_color"], "#fafafa");
    assert_eq!(detail["popup"]["cta_text"], "Join");
    let (_, after_update) = request(
        &api,
        "GET",
        "/api/popups/active?page=%2Fblog%2Farticle&device=desktop",
        None,
        Value::Null,
    )
    .await;
    assert_eq!(after_update["popups"].as_array().unwrap().len(), 0);

    let (status, duplicated) = request(
        &api,
        "POST",
        &format!("/api/admin/popups/{popup_id}/duplicate"),
        Some(&admin),
        json!({}),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{duplicated}");
    let duplicate_id = duplicated["id"].as_i64().unwrap();
    assert_eq!(
        request(
            &api,
            "POST",
            &format!("/api/popups/{duplicate_id}/form-submit"),
            None,
            json!({"formData":{}})
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let (_, copy) = request(
        &api,
        "GET",
        &format!("/api/admin/popups/{duplicate_id}"),
        Some(&admin),
        Value::Null,
    )
    .await;
    assert_eq!(copy["popup"]["status"], "draft");
    assert_eq!(copy["popup"]["content"], "A short invitation");
    assert_eq!(
        request(
            &api,
            "DELETE",
            &format!("/api/admin/popups/{duplicate_id}"),
            Some(&admin),
            json!({})
        )
        .await
        .0,
        StatusCode::OK
    );
    assert_eq!(
        request(
            &api,
            "GET",
            &format!("/api/admin/popups/{duplicate_id}"),
            Some(&admin),
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );

    let (status, unpublished) = request(
        &api,
        "PUT",
        &format!("/api/admin/forms/{form_id}"),
        Some(&admin),
        json!({
            "name":"Marketing Contact Updated", "is_published":false
        }),
    )
    .await;
    assert_eq!(status, StatusCode::OK, "{unpublished}");
    assert_eq!(
        request(
            &api,
            "GET",
            &format!("/api/forms/{slug}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    let unpublished_slug = unpublished["data"]["slug"].as_str().unwrap();
    assert_eq!(
        request(
            &api,
            "GET",
            &format!("/api/forms/{unpublished_slug}"),
            None,
            Value::Null
        )
        .await
        .0,
        StatusCode::NOT_FOUND
    );
    assert_eq!(
        request(
            &api,
            "DELETE",
            &format!("/api/admin/forms/{form_id}"),
            Some(&admin),
            json!({})
        )
        .await
        .0,
        StatusCode::OK
    );
    pool.close().await;
}
