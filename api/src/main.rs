//! Revolution Trading Pros API
//!
//! High-performance Rust backend with Axum
//! Stack: Neon PostgreSQL, Upstash Redis, Cloudflare R2

mod config;
mod db;
mod middleware;
mod models;
mod queue;
mod routes;
mod services;
mod utils;

use axum::{
    http::{header, HeaderName, HeaderValue, Method},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
    set_header::SetResponseHeaderLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::Config;
use crate::db::Database;
use crate::services::Services;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: Database,
    pub services: Services,
    pub config: Config,
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
            Ok(_) => tracing::info!("Migrations completed"),
            Err(e) => {
                tracing::warn!("Migration error (may be expected with existing schema): {}", e);
                tracing::info!("Continuing without migrations - using existing schema");
            }
        }
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

    // Create app state
    let state = AppState {
        db,
        services,
        config: config.clone(),
    };

    // Build CORS layer - explicit headers required when using credentials
    let cors = CorsLayer::new()
        .allow_origin(
            config
                .cors_origins
                .iter()
                .map(|o| o.parse::<HeaderValue>().unwrap())
                .collect::<Vec<_>>(),
        )
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH, Method::OPTIONS])
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
        .expose_headers([header::SET_COOKIE])
        .allow_credentials(true);

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
            HeaderValue::from_static("default-src 'self'; script-src 'self' 'unsafe-inline'; style-src 'self' 'unsafe-inline'; img-src 'self' data: https:; font-src 'self' data:; connect-src 'self' https://revolution-trading-pros-api.fly.dev wss://revolution-trading-pros-api.fly.dev http://localhost:* ws://localhost:*"),
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

    // Build router with security layers
    let app = Router::new()
        .merge(routes::health::router())
        .nest("/api", routes::api_router())
        .layer(security_headers)
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    // Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
