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
    http::{header, HeaderValue, Method},
    Router,
};
use std::net::SocketAddr;
use tower_http::{
    compression::CompressionLayer,
    cors::CorsLayer,
    trace::TraceLayer,
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

    // Start background job processor
    let job_db = db.clone();
    tokio::spawn(async move {
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
        ])
        .expose_headers([header::SET_COOKIE])
        .allow_credentials(true);

    // Build router
    let app = Router::new()
        .merge(routes::health::router())
        .nest("/api", routes::api_router())
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
