//! Revolution Trading Pros - Rust/Axum API
//! 
//! Apple ICT 11+ Principal Engineer Grade
//! High-performance backend replacing Laravel PHP
//!
//! # Architecture
//! - Axum 0.8 web framework
//! - SQLx for compile-time verified database queries
//! - JWT authentication with Argon2 password hashing
//! - Stripe integration for payments
//! - Redis for caching and rate limiting

use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use axum::{http::StatusCode, Router};
use sqlx::postgres::PgPoolOptions;
use tokio::net::TcpListener;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer},
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use revolution_trading_pros_api::{
    config::AppConfig,
    routes,
    AppState,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,sqlx=warn,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer().json())
        .init();

    tracing::info!("Starting Revolution Trading Pros API");

    // Load configuration
    dotenvy::dotenv().ok();
    let config = AppConfig::from_env()?;

    tracing::info!(
        "Configuration loaded: {}:{}", 
        config.server.host, 
        config.server.port
    );

    // Create database connection pool with resilient settings
    // ICT 11+: Increased timeouts for stability under load
    let db_pool = PgPoolOptions::new()
        .max_connections(config.database.max_connections)
        .min_connections(config.database.min_connections)
        .acquire_timeout(Duration::from_secs(10))  // Increased from 3s
        .idle_timeout(Duration::from_secs(300))    // Reduced to recycle connections faster
        .max_lifetime(Duration::from_secs(900))    // Reduced to prevent stale connections
        .test_before_acquire(true)                 // Verify connection is valid
        .connect(&config.database.url)
        .await?;

    tracing::info!("Database connection pool established");

    // Run migrations (ICT 11+ approach: graceful handling of migration issues)
    match sqlx::migrate!("./migrations").run(&db_pool).await {
        Ok(_) => tracing::info!("Database migrations completed"),
        Err(e) => {
            // Log warning but don't crash - migrations may have been applied differently
            tracing::warn!("Migration check failed: {}. Continuing with existing schema.", e);
        }
    }

    // Create Redis connection (optional)
    let redis_client = if let Some(ref redis_url) = config.redis.url {
        match redis::Client::open(redis_url.as_str()) {
            Ok(client) => {
                tracing::info!("Redis connection established");
                Some(client)
            }
            Err(e) => {
                tracing::warn!("Redis connection failed: {}, continuing without cache", e);
                None
            }
        }
    } else {
        tracing::info!("Redis not configured, continuing without cache");
        None
    };

    // Create application state
    let state = AppState {
        db: db_pool,
        redis: redis_client,
        config: Arc::new(config.clone()),
    };

    // Build application router
    let app = Router::new()
        .merge(routes::api_routes(state.clone()))
        // Request ID propagation
        .layer(SetRequestIdLayer::x_request_id(MakeRequestUuid))
        .layer(PropagateRequestIdLayer::x_request_id())
        // Compression
        .layer(CompressionLayer::new())
        // Timeout
        .layer(TimeoutLayer::with_status_code(
            StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30)
        ))
        // CORS
        .layer(build_cors_layer(&config))
        // Tracing
        .layer(TraceLayer::new_for_http());

    // Create socket address
    let addr = SocketAddr::from((
        config.server.host.parse::<std::net::IpAddr>()?,
        config.server.port,
    ));

    tracing::info!("Listening on {}", addr);

    // Start server
    let listener = TcpListener::bind(addr).await?;
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    tracing::info!("Server shutdown complete");
    Ok(())
}

/// Build CORS layer from configuration
/// ICT 11+: Properly configured to prevent CORB blocking
fn build_cors_layer(config: &AppConfig) -> CorsLayer {
    use axum::http::{header, Method, HeaderName};

    let origins: Vec<_> = config
        .cors
        .allowed_origins
        .iter()
        .filter_map(|o| o.parse().ok())
        .collect();

    tracing::info!("CORS allowed origins: {:?}", config.cors.allowed_origins);

    CorsLayer::new()
        .allow_origin(origins)
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PUT,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
            Method::HEAD,
        ])
        .allow_headers([
            header::AUTHORIZATION,
            header::CONTENT_TYPE,
            header::ACCEPT,
            header::ORIGIN,
            header::CACHE_CONTROL,
            HeaderName::from_static("x-requested-with"),
        ])
        .expose_headers([
            header::CONTENT_TYPE,
            header::CONTENT_LENGTH,
            HeaderName::from_static("x-request-id"),
        ])
        .allow_credentials(true)
        .max_age(Duration::from_secs(config.cors.max_age))
}

/// Graceful shutdown signal handler
async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install SIGTERM handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            tracing::info!("Received Ctrl+C, starting graceful shutdown");
        }
        _ = terminate => {
            tracing::info!("Received SIGTERM, starting graceful shutdown");
        }
    }
}
