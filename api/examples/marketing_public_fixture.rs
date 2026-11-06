//! Owned, loopback-only public API fixture. Never loads a .env or starts workers.
use anyhow::{Context, ensure};
use axum::{
    Router,
    body::Body,
    http::{Method, Request, StatusCode},
    middleware::{self, Next},
    response::Response,
};
use revolution_api::{
    AppState, cache::CacheInvalidator, config::Config, db::Database, routes, services::Services,
};

async fn public_reads_only(request: Request<Body>, next: Next) -> Response {
    let path = request.uri().path();
    if request.method() != Method::GET
        || !(path.starts_with("/api/seo/")
            || path == "/api/posts"
            || path.starts_with("/api/posts/"))
    {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::empty())
            .expect("static response");
    }
    next.run(request).await
}

/// Seed only after the disposable database's owner marker has been checked.
/// Writes use the real admin handlers in process; HTTP remains read-only.
async fn seed(api: &Router, pool: &sqlx::PgPool, secret: &str) -> anyhow::Result<()> {
    use serde_json::{Value, json};
    use tower::ServiceExt;
    async fn request(
        api: &Router,
        method: &str,
        path: &str,
        token: &str,
        value: Value,
    ) -> anyhow::Result<Value> {
        let response = api
            .clone()
            .oneshot(
                Request::builder()
                    .method(method)
                    .uri(path)
                    .header("authorization", format!("Bearer {token}"))
                    .header("content-type", "application/json")
                    .body(Body::from(serde_json::to_vec(&value)?))?,
            )
            .await?;
        let status = response.status();
        let bytes = axum::body::to_bytes(response.into_body(), 1_000_000).await?;
        ensure!(
            status.is_success(),
            "Fixture request {path} failed: {status}: {}",
            String::from_utf8_lossy(&bytes)
        );
        Ok(serde_json::from_slice(&bytes)?)
    }
    let authors: Value =
        serde_json::from_str(include_str!("../../frontend/src/lib/authors/authors.json"))?;
    let author = &authors[0];
    let user: i64 = sqlx::query_scalar("INSERT INTO users(email,name,password_hash,role,email_verified_at) VALUES('marketing@example.test',$1,'unused-disposable-fixture-hash','admin',NOW()) RETURNING id")
        .bind(author["name"].as_str().context("Author name")?).fetch_one(pool).await?;
    let token = revolution_api::utils::create_jwt(user, secret, 1)?;
    let date = chrono::Utc::now() - chrono::Duration::minutes(5);
    for n in 1..=21 {
        request(api, "POST", "/api/admin/posts", &token, json!({
            "title":format!("Marketing fixture article {n:02}"),
            "excerpt":"An isolated test article used to verify public rendering, pagination, and navigation against the actual API.",
            "content_blocks":[], "featured_image":"https://marketing.example.test/image.png",
            "status":"published", "published_at":date.naive_utc(), "indexable":true
        })).await?;
    }
    let trader: i64 = sqlx::query_scalar(
        "INSERT INTO traders(name,slug) VALUES($1,'marketing-fixture-author') RETURNING id",
    )
    .bind(author["name"].as_str().context("Author name")?)
    .fetch_one(pool)
    .await?;
    let video = request(api, "POST", "/api/admin/videos", &token, json!({
        "title":"Marketing fixture educational video", "description":"An isolated educational test video verifying that the public library uses approved, published sources with real author metadata.",
        "video_url":"https://marketing-fixture.b-cdn.net/educational.mp4", "video_platform":"self-hosted",
        "thumbnail_url":"https://marketing.example.test/image.png", "duration":600,
        "video_date":date.date_naive().to_string(), "published_at":date.to_rfc3339(),
        "trader_id":trader,"content_type":"learning_center","is_published":true,"room_ids":[]
    })).await?;
    let id = video["data"]["id"].as_i64().context("Video ID")?;
    let source = request(
        api,
        "GET",
        &format!("/api/admin/seo/video-publications/{id}"),
        &token,
        Value::Null,
    )
    .await?;
    request(api,"POST","/api/admin/seo/video-publications",&token,json!({"video_id":id,"author_slug":author["slug"],"expected_source_hash":source["record"]["source_hash"]})).await?;
    println!("Seeded 21 actual fixture articles and one approved public video");
    Ok(())
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut config = Config::from_env()?;
    ensure!(
        config.environment == "development",
        "Fixture requires development mode"
    );
    let owner = std::env::var("MARKETING_FIXTURE_OWNER")?;
    ensure!(
        owner.len() == 32 && owner.bytes().all(|c| c.is_ascii_hexdigit()),
        "Missing fixture owner"
    );
    let db_url = reqwest::Url::parse(&config.database_url)?;
    ensure!(
        db_url.host_str() == Some("127.0.0.1")
            && db_url.port() == Some(55485)
            && db_url.path() == "/marketing_fixture",
        "Refuse any other database"
    );
    config.redis_url = "redis://127.0.0.1:1".into();
    config.meilisearch_host = "http://127.0.0.1:1".into();
    config.r2_endpoint = "http://127.0.0.1:1".into();
    let db = Database::new(&config).await?;
    let marker: Option<String> = sqlx::query_scalar("SELECT shobj_description(oid, 'pg_database') FROM pg_database WHERE datname=current_database()")
        .fetch_one(&db.pool).await?;
    ensure!(
        marker.as_deref() == Some(owner.as_str()),
        "Database owner marker did not match"
    );
    db.migrate().await?;
    let services = Services::new(&config).await?;
    let cache = services.cache.clone();
    let state = AppState {
        db,
        services,
        config,
        event_broadcaster: routes::realtime::EventBroadcaster::new(),
        ws_manager: routes::websocket::WsConnectionManager::new(),
        cache: cache.clone(),
        cache_invalidator: CacheInvalidator::new(cache),
    };
    let pool = state.db.pool.clone();
    let secret = state.config.jwt_secret.clone();
    let app = Router::new()
        .nest("/api", routes::api_router())
        .with_state(state);
    if std::env::var("MARKETING_FIXTURE_SEED").as_deref() == Ok("published") {
        seed(&app, &pool, &secret).await?;
    }
    let app = app.layer(middleware::from_fn(public_reads_only));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:19085")
        .await
        .context("Fixture port unavailable")?;
    println!("Owned public API ready on 127.0.0.1:19085");
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<std::net::SocketAddr>(),
    )
    .await?;
    Ok(())
}
