//! Revolution Trading Pros API - Library exports for testing
//!
//! This file exposes internal modules for integration tests.
//! The actual application entry point is in main.rs.

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

pub mod config;
pub mod db;
pub mod docs;
pub mod domain;
pub mod middleware;
pub mod models;
pub mod monitoring;
pub mod queue;
pub mod routes;
pub mod services;
pub mod utils;

use routes::realtime::EventBroadcaster;

/// Application state shared across all routes
#[derive(Clone)]
pub struct AppState {
    pub db: db::Database,
    pub services: services::Services,
    pub config: config::Config,
    pub event_broadcaster: EventBroadcaster,
}
