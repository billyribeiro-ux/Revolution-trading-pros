//! Revolution Trading Pros API - Library exports for testing
//!
//! This file exposes internal modules for integration tests.
//! The actual application entry point is in main.rs.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod cache;
pub mod config;
pub mod db;
pub mod docs;
pub mod domain;
pub mod middleware;
pub mod models;
pub mod monitoring;
pub mod openapi;
pub mod queue;
pub mod routes;
pub mod services;
pub mod utils;

use cache::{CacheInvalidator, CacheService};
use routes::realtime::EventBroadcaster;
use routes::websocket::WsConnectionManager;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: db::Database,
    pub services: services::Services,
    pub config: config::Config,
    pub event_broadcaster: EventBroadcaster,
    /// WebSocket connection manager for real-time alerts (ICT 7+ Phase 3)
    pub ws_manager: WsConnectionManager,
    /// Cache service for Explosive Swings room content caching (L1 + Redis L2)
    pub cache: CacheService,
    /// Cache invalidator for managing cache lifecycle
    pub cache_invalidator: CacheInvalidator,
}
