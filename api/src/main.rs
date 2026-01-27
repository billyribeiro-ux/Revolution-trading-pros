//! Revolution Trading Pros API
//!
//! High-performance Rust backend with Axum
//! Stack: Fly.io PostgreSQL, Upstash Redis, Cloudflare R2

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

mod config;
mod db;
mod docs;
mod domain;
mod middleware;
mod models;
mod monitoring;
mod queue;
mod routes;
mod services;
mod utils;

use axum::{
    http::{header, HeaderName, HeaderValue, Method},
    middleware as axum_middleware, Router,
};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer, cors::CorsLayer, set_header::SetResponseHeaderLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::db::Database;
use crate::routes::realtime::EventBroadcaster;
use crate::services::Services;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub services: Services,
    pub config: Config,
    pub event_broadcaster: EventBroadcaster,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "revolution_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    dotenvy::dotenv().ok();
    let config = Config::from_env()?;

    tracing::info!("Starting Revolution Trading Pros API");
    tracing::info!("Environment: {}", config.environment);

    // Initialize database
    let db = Database::new(&config).await?;
    tracing::info!("Database connected");

    // Run migrations (skip if SKIP_MIGRATIONS=true or if they fail on existing schema)
    let skip_migrations = std::env::var("SKIP_MIGRATIONS").unwrap_or_default() == "true";
    if skip_migrations {
        tracing::info!("Skipping migrations (SKIP_MIGRATIONS=true)");
    } else {
        match db.migrate().await {
            Ok(_) => tracing::info!("✅ Migrations completed successfully"),
            Err(e) => {
                tracing::warn!(
                    "Migration error (may be expected with existing schema): {}",
                    e
                );
                tracing::info!("Continuing without migrations - using existing schema");
            }
        }
    }

    // ICT 7 Principal Engineer: Bootstrap developer account from environment variables
    // No hardcoded credentials - reads from DEVELOPER_BOOTSTRAP_* env vars
    match db.bootstrap_developer(&config).await {
        Ok(_) => tracing::info!("✅ Developer bootstrap complete"),
        Err(e) => tracing::warn!("Developer bootstrap skipped or failed: {}", e),
    }

    // Initialize services
    let services = Services::new(&config).await?;
    tracing::info!("Services initialized");

    // Background job processor - ICT11+ Production Ready
    let job_db = db.clone();
    tokio::spawn(async move {
        tracing::info!("Starting background job worker");
        queue::worker::run(job_db).await;
    });
    tracing::info!("Job queue worker started");

    // Initialize real-time event broadcaster - ICT 11+ SSE Updates
    let event_broadcaster = EventBroadcaster::new();
    tracing::info!("Real-time event broadcaster initialized");

    // Create app state
    let state = AppState {
        db,
        services,
        config: config.clone(),
        event_broadcaster,
    };

    // Build CORS layer - ICT 11+ CORB Fix: explicit headers required when using credentials
    // Log configured CORS origins for debugging
    tracing::info!("CORS origins configured: {:?}", config.cors_origins);

    let parsed_origins: Vec<HeaderValue> = config
        .cors_origins
        .iter()
        .filter_map(|o| match o.parse::<HeaderValue>() {
            Ok(hv) => {
                tracing::debug!("CORS origin parsed successfully: {}", o);
                Some(hv)
            }
            Err(e) => {
                tracing::error!("Failed to parse CORS origin '{}': {}", o, e);
                None
            }
        })
        .collect();

    tracing::info!("CORS layer allowing {} origins", parsed_origins.len());

    let cors = CorsLayer::new()
        .allow_origin(parsed_origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::DELETE,
            Method::PATCH,
            Method::OPTIONS,
        ])
        .allow_headers([
            header::CONTENT_TYPE,
            header::AUTHORIZATION,
            header::ACCEPT,
            header::ORIGIN,
            header::COOKIE,
            header::SET_COOKIE,
            HeaderName::from_static("x-api-version"),
            HeaderName::from_static("x-session-id"),
        ])
        // ICT 11+ CORB Fix: Expose headers so browser can read cross-origin responses
        .expose_headers([
            header::CONTENT_TYPE,
            header::SET_COOKIE,
            header::AUTHORIZATION,
            HeaderName::from_static("x-api-version"),
            HeaderName::from_static("x-session-id"),
        ])
        .allow_credentials(true)
        .max_age(std::time::Duration::from_secs(3600)); // Cache preflight for 1 hour

    // ICT L11+ Security Headers
    // These headers protect against XSS, clickjacking, MIME sniffing, and enforce HTTPS
    let security_headers = tower::ServiceBuilder::new()
        // Prevent MIME type sniffing
        .layer(SetResponseHeaderLayer::overriding(
            header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        // Prevent clickjacking
        .layer(SetResponseHeaderLayer::overriding(
            header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        // XSS Protection (legacy browsers)
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("x-xss-protection"),
            HeaderValue::from_static("1; mode=block"),
        ))
        // Referrer Policy
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("referrer-policy"),
            HeaderValue::from_static("strict-origin-when-cross-origin"),
        ))
        // Content Security Policy
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("content-security-policy"),
            HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; media-src 'self' https://simpler-options.s3.amazonaws.com https://cdn.simplertrading.com https://simpler-cdn.s3.amazonaws.com; connect-src 'self' https://revolution-trading-pros-api.fly.dev wss://revolution-trading-pros-api.fly.dev http://localhost:* ws://localhost:*"),
        ))
        // Strict Transport Security (HSTS) - 1 year, include subdomains
        .layer(SetResponseHeaderLayer::overriding(
            header::STRICT_TRANSPORT_SECURITY,
            HeaderValue::from_static("max-age=31536000; includeSubDomains; preload"),
        ))
        // Permissions Policy (disable sensitive features)
        .layer(SetResponseHeaderLayer::overriding(
            HeaderName::from_static("permissions-policy"),
            HeaderValue::from_static("accelerometer=(), camera=(), geolocation=(), gyroscope=(), magnetometer=(), microphone=(), payment=(), usb=()"),
        ));

    tracing::info!("Security headers configured");

    // Initialize metrics
    let metrics = monitoring::Metrics::default();

    // ICT 11+ ENHANCEMENT: Swagger UI for API documentation
    use utoipa::OpenApi;
    use utoipa_swagger_ui::SwaggerUi;

    let swagger_router =
        SwaggerUi::new("/swagger-ui").url("/api-docs/openapi.json", docs::ApiDoc::openapi());

    // Build router with security layers
    // ICT 11+ Enhancement: Add metrics middleware to track all requests
    // ICT 11+ CORB Fix: Middleware ordering is CRITICAL
    // Axum applies layers in REVERSE order (bottom to top in execution)
    // Execution order: TraceLayer → CompressionLayer → ensure_content_type → CORS → security_headers → metrics → routes
    // This ensures Content-Type is set BEFORE CORS headers are evaluated by the browser
    let app = Router::new()
        .merge(routes::health::router())
        .nest("/api", routes::api_router())
        .merge(swagger_router)
        .nest(
            "/monitoring",
            monitoring::router().with_state(metrics.clone()),
        )
        .layer(axum_middleware::from_fn_with_state(
            metrics.clone(),
            monitoring::metrics_middleware,
        ))
        .layer(security_headers)
        .layer(cors)
        .layer(axum_middleware::from_fn(middleware::ensure_content_type))
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Listening on {}", addr);
    tracing::info!(
        "📚 API Documentation: http://{}:{}/swagger-ui",
        addr.ip(),
        addr.port()
    );
    tracing::info!(
        "📊 Metrics: http://{}:{}/monitoring/metrics",
        addr.ip(),
        addr.port()
    );

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
